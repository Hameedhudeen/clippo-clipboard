import AppKit
import ApplicationServices
import Carbon
import ServiceManagement
import SwiftUI
import UserNotifications

@main
struct ClippoMacApp: App {
    @StateObject private var model = ClippoMacModel()

    var body: some Scene {
        MenuBarExtra("Clippo", systemImage: "clipboard") {
            Button("Open History") {
                model.showHistoryWindow()
            }
            .keyboardShortcut("c", modifiers: [.shift, .command])

            Divider()

            Button(model.capturePaused ? "Resume Capture" : "Pause Capture") {
                model.capturePaused.toggle()
            }

            Button("Ignore Next Copy") {
                model.ignoreNextCopy = true
            }

            Divider()

            SettingsLink {
                Text("Preferences")
            }

            Button("Quit Clippo") {
                NSApplication.shared.terminate(nil)
            }
            .keyboardShortcut("q", modifiers: [.command])
        }
        .menuBarExtraStyle(.menu)

        Window("Clippo History", id: ClippoWindowID.history.rawValue) {
            HistoryPopupView(model: model)
                .frame(width: 420, height: 520)
        }
        .defaultPosition(.topTrailing)
        .defaultSize(width: 420, height: 520)
        .commands {
            CommandMenu("Clippo") {
                SettingsLink {
                    Text("Preferences")
                }
                .keyboardShortcut(",", modifiers: [.command])

                Divider()

                Button("Copy Selection") {
                    model.copySelection()
                }
                .keyboardShortcut("c", modifiers: [.command])

                Button("Select Item") {
                    model.copySelection()
                }
                .keyboardShortcut(.return, modifiers: [])

                Button("Paste Selection") {
                    model.pasteSelection()
                }
                .keyboardShortcut(.return, modifiers: [.option])

                Button("Paste Without Formatting") {
                    model.pasteSelectionWithoutFormatting()
                }
                .keyboardShortcut(.return, modifiers: [.option, .shift])

                ForEach(1...9, id: \.self) { index in
                    let shortcut = Character(String(index))
                    Button("Copy Item \(index)") {
                        model.copyVisibleShortcut(shortcut)
                    }
                    .keyboardShortcut(KeyEquivalent(shortcut), modifiers: [])

                    Button("Paste Item \(index)") {
                        model.pasteVisibleShortcut(shortcut)
                    }
                    .keyboardShortcut(KeyEquivalent(shortcut), modifiers: [.option])

                    Button("Paste Item \(index) Without Formatting") {
                        model.pasteVisibleShortcutWithoutFormatting(shortcut)
                    }
                    .keyboardShortcut(KeyEquivalent(shortcut), modifiers: [.option, .shift])
                }

                Divider()

                Button("Pin or Unpin") {
                    model.toggleSelectedPin()
                }
                .keyboardShortcut("p", modifiers: [.option])

                Button("Delete") {
                    model.deleteSelected()
                }
                .keyboardShortcut(.delete, modifiers: [.option])

                Divider()

                Button("Clear Unpinned") {
                    model.clearUnpinned()
                }
                .keyboardShortcut(.delete, modifiers: [.option, .command])

                Button("Clear All") {
                    model.clearAll()
                }
                .keyboardShortcut(.delete, modifiers: [.option, .command, .shift])
            }
        }

        Settings {
            PreferencesView(model: model)
                .frame(width: 520, height: 360)
        }
    }
}

enum ClippoWindowID: String {
    case history = "history"
}

final class ClippoMacModel: ObservableObject {
    @Published var searchQuery = ""
    @Published var capturePaused = false
    @Published var ignoreNextCopy = false
    @Published var launchAtLogin = SMAppService.mainApp.status == .enabled
    @Published var selectedItemID: UUID?
    @Published var items: [ClippoMacHistoryItem] = []
    private let historyStore = MacHistoryStore()
    private var pasteboardChangeCount = NSPasteboard.general.changeCount
    private var clipboardTimer: Timer?
    private var globalHotKey: GlobalHotKey?
    private var notificationObservers: [NSObjectProtocol] = []

    init() {
        items = historyStore.load()
        startClipboardMonitoring()
        requestNotificationAuthorization()
        let workspaceNotifications = NSWorkspace.shared.notificationCenter
        notificationObservers.append(workspaceNotifications.addObserver(
            forName: NSWorkspace.willSleepNotification,
            object: nil,
            queue: .main
        ) { [weak self] _ in
            self?.workspaceWillSleep()
        })
        notificationObservers.append(workspaceNotifications.addObserver(
            forName: NSWorkspace.didWakeNotification,
            object: nil,
            queue: .main
        ) { [weak self] _ in
            self?.workspaceDidWake()
        })
        notificationObservers.append(NotificationCenter.default.addObserver(
            forName: NSApplication.willTerminateNotification,
            object: nil,
            queue: .main
        ) { [weak self] _ in
            self?.saveHistory()
        })
        globalHotKey = GlobalHotKey(keyCode: UInt32(kVK_ANSI_C), modifiers: UInt32(cmdKey | shiftKey)) { [weak self] in
            self?.showHistoryWindow()
        }
        DispatchQueue.main.async { [weak self] in
            self?.maybePromptLaunchAtLoginOnFirstRun()
        }
    }

    deinit {
        clipboardTimer?.invalidate()
        globalHotKey?.unregister()
        for observer in notificationObservers {
            NotificationCenter.default.removeObserver(observer)
            NSWorkspace.shared.notificationCenter.removeObserver(observer)
        }
    }

    var filteredItems: [ClippoMacHistoryItem] {
        let orderedItems = items.sorted { left, right in
            if left.pinned != right.pinned {
                return left.pinned && !right.pinned
            }
            return left.createdAt > right.createdAt
        }

        guard !searchQuery.isEmpty else {
            return orderedItems
        }

        return orderedItems.filter {
            $0.text.localizedCaseInsensitiveContains(searchQuery)
        }
    }

    func showHistoryWindow() {
        NSApp.activate(ignoringOtherApps: true)
        NSApp.sendAction(Selector(("showWindow:")), to: nil, from: nil)
        DispatchQueue.main.async {
            self.positionHistoryWindow()
        }
    }

    func positionHistoryWindow() {
        guard let window = NSApp.windows.first(where: { $0.title == "Clippo History" }) else {
            return
        }

        window.collectionBehavior.insert(.moveToActiveSpace)
        window.collectionBehavior.insert(.transient)
        window.contentMinSize = NSSize(width: 360, height: 420)
        window.setContentSize(NSSize(width: 420, height: 520))

        let mouseLocation = NSEvent.mouseLocation
        let screen = NSScreen.screens.first { screen in
            screen.frame.contains(mouseLocation)
        } ?? NSScreen.main

        guard let visibleFrame = screen?.visibleFrame else {
            window.center()
            return
        }

        let scale = window.backingScaleFactor
        let pointAlignedWidth = round(420 * scale) / scale
        let pointAlignedHeight = round(520 * scale) / scale
        let origin = NSPoint(
            x: visibleFrame.maxX - pointAlignedWidth - 16,
            y: visibleFrame.maxY - pointAlignedHeight - 16
        )
        window.setFrame(
            NSRect(x: origin.x, y: origin.y, width: pointAlignedWidth, height: pointAlignedHeight),
            display: true
        )
    }

    func requestNotificationAuthorization() {
        UNUserNotificationCenter.current().requestAuthorization(options: [.alert, .sound]) { _, _ in }
    }

    func startClipboardMonitoring() {
        clipboardTimer?.invalidate()
        clipboardTimer = Timer.scheduledTimer(withTimeInterval: 0.5, repeats: true) { [weak self] _ in
            DispatchQueue.main.async {
                self?.pollClipboard()
            }
        }
    }

    func pollClipboard() {
        guard !capturePaused else {
            return
        }

        let pasteboard = NSPasteboard.general
        guard pasteboard.changeCount != pasteboardChangeCount else {
            return
        }
        pasteboardChangeCount = pasteboard.changeCount

        if ignoreNextCopy {
            ignoreNextCopy = false
            return
        }

        guard let text = pasteboard.string(forType: .string), !text.isEmpty else {
            return
        }

        if let existingIndex = items.firstIndex(where: { $0.text == text }) {
            items[existingIndex].createdAt = Date()
        } else {
            items.insert(ClippoMacHistoryItem(text: text), at: 0)
        }
        saveHistory()
    }

    var selectedItem: ClippoMacHistoryItem? {
        let selectedItemID = selectedItemID ?? filteredItems.first?.id
        return filteredItems.first { $0.id == selectedItemID }
    }

    func selectNext() {
        moveSelection(delta: 1)
    }

    func selectPrevious() {
        moveSelection(delta: -1)
    }

    func moveSelection(delta: Int) {
        let visibleItems = filteredItems
        guard !visibleItems.isEmpty else {
            selectedItemID = nil
            return
        }

        let currentIndex = selectedItemID.flatMap { id in
            visibleItems.firstIndex { $0.id == id }
        } ?? 0
        let nextIndex = min(max(currentIndex + delta, 0), visibleItems.count - 1)
        selectedItemID = visibleItems[nextIndex].id
    }

    func copySelection() {
        guard let item = selectedItem else {
            return
        }

        writeToPasteboard(item.text)
    }

    func pasteSelection() {
        guard let item = selectedItem else {
            return
        }

        writeToPasteboard(item.text)
        sendPasteShortcutIfTrusted()
    }

    func pasteSelectionWithoutFormatting() {
        pasteSelection()
    }

    func itemForVisibleShortcut(_ shortcut: Character) -> ClippoMacHistoryItem? {
        filteredItems.first { visibleShortcut(for: $0) == shortcut }
    }

    func copyVisibleShortcut(_ shortcut: Character) {
        guard let item = itemForVisibleShortcut(shortcut) else {
            return
        }
        selectedItemID = item.id
        writeToPasteboard(item.text)
    }

    func pasteVisibleShortcut(_ shortcut: Character) {
        guard let item = itemForVisibleShortcut(shortcut) else {
            return
        }
        selectedItemID = item.id
        writeToPasteboard(item.text)
        sendPasteShortcutIfTrusted()
    }

    func pasteVisibleShortcutWithoutFormatting(_ shortcut: Character) {
        pasteVisibleShortcut(shortcut)
    }

    func writeToPasteboard(_ text: String) {
        let pasteboard = NSPasteboard.general
        pasteboard.clearContents()
        pasteboard.setString(text, forType: .string)
        pasteboardChangeCount = pasteboard.changeCount
    }

    func sendPasteShortcutIfTrusted() {
        guard AXIsProcessTrusted() else {
            requestAccessibilityPermission()
            return
        }

        let source = CGEventSource(stateID: .combinedSessionState)
        let keyDown = CGEvent(keyboardEventSource: source, virtualKey: 9, keyDown: true)
        let keyUp = CGEvent(keyboardEventSource: source, virtualKey: 9, keyDown: false)
        keyDown?.flags = .maskCommand
        keyUp?.flags = .maskCommand
        keyDown?.post(tap: .cghidEventTap)
        keyUp?.post(tap: .cghidEventTap)
    }

    func requestAccessibilityPermission() {
        let options = [kAXTrustedCheckOptionPrompt.takeUnretainedValue() as String: true] as CFDictionary
        AXIsProcessTrustedWithOptions(options)
    }

    func togglePin(_ item: ClippoMacHistoryItem) {
        guard let index = items.firstIndex(where: { $0.id == item.id }) else {
            return
        }
        if items[index].pinned {
            items[index].pinned = false
            items[index].pinnedShortcut = nil
        } else {
            items[index].pinned = true
            items[index].pinnedShortcut = nextPinnedShortcut()
        }
        saveHistory()
        notify(title: items[index].pinned ? "Pinned" : "Unpinned", body: preview(items[index].text))
    }

    func toggleSelectedPin() {
        guard let item = selectedItem else {
            return
        }
        togglePin(item)
    }

    func delete(_ item: ClippoMacHistoryItem) {
        items.removeAll { $0.id == item.id }
        if selectedItemID == item.id {
            selectedItemID = nil
        }
        saveHistory()
        notify(title: "Deleted", body: preview(item.text))
    }

    func deleteSelected() {
        guard let item = selectedItem else {
            return
        }
        delete(item)
    }

    func clearUnpinned() {
        items.removeAll { !$0.pinned }
        saveHistory()
        notify(title: "Cleared", body: "Unpinned clipboard history was cleared.")
    }

    func clearAll() {
        items.removeAll()
        selectedItemID = nil
        saveHistory()
        notify(title: "Cleared", body: "All clipboard history was cleared.")
    }

    func setLaunchAtLogin(_ enabled: Bool) {
        do {
            if enabled {
                try SMAppService.mainApp.register()
            } else {
                try SMAppService.mainApp.unregister()
            }
            launchAtLogin = SMAppService.mainApp.status == .enabled
        } catch {
            launchAtLogin = SMAppService.mainApp.status == .enabled
            notify(title: "Launch at Login", body: "Could not update launch at login setting.")
        }
    }

    func maybePromptLaunchAtLoginOnFirstRun() {
        let promptKey = "app.clippo.firstRunLaunchAtLoginPrompted"
        let defaults = UserDefaults.standard
        if launchAtLogin {
            defaults.set(true, forKey: promptKey)
            return
        }
        guard !defaults.bool(forKey: promptKey) else {
            return
        }

        let alert = NSAlert()
        alert.messageText = "Start Clippo automatically?"
        alert.informativeText = "Clippo works best as a menu bar utility that starts when you sign in and keeps clipboard history available in the background."
        alert.addButton(withTitle: "Enable Launch at Login")
        alert.addButton(withTitle: "Not Now")
        NSApp.activate(ignoringOtherApps: true)
        let response = alert.runModal()
        defaults.set(true, forKey: promptKey)
        if response == .alertFirstButtonReturn {
            setLaunchAtLogin(true)
        }
    }

    func notify(title: String, body: String) {
        let content = UNMutableNotificationContent()
        content.title = title
        content.body = body
        let request = UNNotificationRequest(identifier: UUID().uuidString, content: content, trigger: nil)
        UNUserNotificationCenter.current().add(request)
    }

    private func preview(_ text: String) -> String {
        if text.count <= 80 {
            return text
        }
        return String(text.prefix(77)) + "..."
    }

    func visibleShortcut(for item: ClippoMacHistoryItem) -> Character? {
        if let pinnedShortcut = item.pinnedShortcut {
            return pinnedShortcut
        }
        guard let index = filteredItems.firstIndex(where: { $0.id == item.id }) else {
            return nil
        }
        let shortcut = index + 1
        return shortcut <= 9 ? Character(String(shortcut)) : nil
    }

    private func nextPinnedShortcut() -> Character? {
        let used = Set(items.compactMap(\.pinnedShortcut))
        return "123456789".first { !used.contains($0) }
    }

    private func saveHistory() {
        historyStore.save(items)
    }

    private func workspaceWillSleep() {
        clipboardTimer?.invalidate()
        clipboardTimer = nil
        saveHistory()
    }

    private func workspaceDidWake() {
        pasteboardChangeCount = NSPasteboard.general.changeCount
        startClipboardMonitoring()
    }
}

final class GlobalHotKey {
    private static var handlers: [UInt32: () -> Void] = [:]
    private var hotKeyRef: EventHotKeyRef?
    private var eventHandlerRef: EventHandlerRef?
    private let hotKeyID: UInt32

    fileprivate static func invoke(hotKeyID: UInt32) {
        handlers[hotKeyID]?()
    }

    init?(keyCode: UInt32, modifiers: UInt32, handler: @escaping () -> Void) {
        hotKeyID = UInt32.random(in: 1...UInt32.max)
        Self.handlers[hotKeyID] = handler

        var eventType = EventTypeSpec(eventClass: OSType(kEventClassKeyboard), eventKind: UInt32(kEventHotKeyPressed))
        let installStatus = InstallEventHandler(
            GetApplicationEventTarget(),
            clippoGlobalHotKeyCallback,
            1,
            &eventType,
            nil,
            &eventHandlerRef
        )
        guard installStatus == noErr else {
            Self.handlers.removeValue(forKey: hotKeyID)
            return nil
        }

        let carbonHotKeyID = EventHotKeyID(signature: fourCharCode("CLPO"), id: hotKeyID)
        let registerStatus = RegisterEventHotKey(
            keyCode,
            modifiers,
            carbonHotKeyID,
            GetApplicationEventTarget(),
            0,
            &hotKeyRef
        )
        guard registerStatus == noErr else {
            unregister()
            return nil
        }
    }

    func unregister() {
        if let hotKeyRef {
            UnregisterEventHotKey(hotKeyRef)
            self.hotKeyRef = nil
        }
        if let eventHandlerRef {
            RemoveEventHandler(eventHandlerRef)
            self.eventHandlerRef = nil
        }
        Self.handlers.removeValue(forKey: hotKeyID)
    }

    deinit {
        unregister()
    }
}

private func clippoGlobalHotKeyCallback(
    _: EventHandlerCallRef?,
    _ event: EventRef?,
    _: UnsafeMutableRawPointer?
) -> OSStatus {
    guard let event else {
        return noErr
    }
    var hotKeyID = EventHotKeyID()
    GetEventParameter(
        event,
        EventParamName(kEventParamDirectObject),
        EventParamType(typeEventHotKeyID),
        nil,
        MemoryLayout<EventHotKeyID>.size,
        nil,
        &hotKeyID
    )
    GlobalHotKey.invoke(hotKeyID: hotKeyID.id)
    return noErr
}

private func fourCharCode(_ string: String) -> OSType {
    string.utf8.reduce(0) { result, character in
        (result << 8) + OSType(character)
    }
}

struct ClippoMacHistoryItem: Identifiable, Equatable, Codable {
    var id = UUID()
    var text: String
    var pinned = false
    var pinnedShortcut: Character?
    var createdAt = Date()

    enum CodingKeys: String, CodingKey {
        case id
        case text
        case pinned
        case pinnedShortcut
        case createdAt
    }

    init(
        id: UUID = UUID(),
        text: String,
        pinned: Bool = false,
        pinnedShortcut: Character? = nil,
        createdAt: Date = Date()
    ) {
        self.id = id
        self.text = text
        self.pinned = pinned
        self.pinnedShortcut = pinnedShortcut
        self.createdAt = createdAt
    }

    init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        id = try container.decodeIfPresent(UUID.self, forKey: .id) ?? UUID()
        text = try container.decode(String.self, forKey: .text)
        pinned = try container.decodeIfPresent(Bool.self, forKey: .pinned) ?? false
        if let shortcut = try container.decodeIfPresent(String.self, forKey: .pinnedShortcut) {
            pinnedShortcut = shortcut.first
        } else {
            pinnedShortcut = nil
        }
        createdAt = try container.decodeIfPresent(Date.self, forKey: .createdAt) ?? Date()
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encode(id, forKey: .id)
        try container.encode(text, forKey: .text)
        try container.encode(pinned, forKey: .pinned)
        if let pinnedShortcut {
            try container.encode(String(pinnedShortcut), forKey: .pinnedShortcut)
        }
        try container.encode(createdAt, forKey: .createdAt)
    }
}

final class MacHistoryStore {
    private let fileURL: URL
    private let fileManager: FileManager

    init(fileManager: FileManager = .default) {
        self.fileManager = fileManager
        let baseDirectory = fileManager
            .urls(for: .applicationSupportDirectory, in: .userDomainMask)
            .first ?? fileManager.homeDirectoryForCurrentUser.appendingPathComponent("Library/Application Support")
        let appDirectory = baseDirectory.appendingPathComponent("Clippo", isDirectory: true)
        fileURL = appDirectory.appendingPathComponent("history.json")
    }

    func load() -> [ClippoMacHistoryItem] {
        guard let data = try? Data(contentsOf: fileURL) else {
            return []
        }

        let decoder = JSONDecoder()
        return (try? decoder.decode([ClippoMacHistoryItem].self, from: data)) ?? []
    }

    func save(_ items: [ClippoMacHistoryItem]) {
        do {
            try fileManager.createDirectory(
                at: fileURL.deletingLastPathComponent(),
                withIntermediateDirectories: true
            )
            let encoder = JSONEncoder()
            encoder.outputFormatting = [.prettyPrinted, .sortedKeys]
            let data = try encoder.encode(items)
            try data.write(to: fileURL, options: .atomic)
        } catch {
            // Clipboard contents are intentionally not logged here.
        }
    }
}

struct HistoryPopupView: View {
    @ObservedObject var model: ClippoMacModel
    @FocusState private var searchFocused: Bool

    var body: some View {
        VStack(spacing: 0) {
            TextField("Search clipboard history", text: $model.searchQuery)
                .textFieldStyle(.roundedBorder)
                .focused($searchFocused)
                .padding(12)
                .accessibilityLabel("Search clipboard history")

            Divider()

            if model.filteredItems.isEmpty {
                ContentUnavailableView("No clipboard history yet", systemImage: "clipboard")
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
            } else {
                List(selection: $model.selectedItemID) {
                    Section("Pinned") {
                        ForEach(model.filteredItems.filter(\.pinned)) { item in
                            HistoryItemRow(item: item, model: model, shortcut: model.visibleShortcut(for: item))
                        }
                    }

                    Section("History") {
                        ForEach(model.filteredItems.filter { !$0.pinned }) { item in
                            HistoryItemRow(item: item, model: model, shortcut: model.visibleShortcut(for: item))
                        }
                    }
                }
                .listStyle(.inset)
                .accessibilityLabel("Clipboard history")
                .onMoveCommand { direction in
                    switch direction {
                    case .down:
                        model.selectNext()
                    case .up:
                        model.selectPrevious()
                    default:
                        break
                    }
                }
            }

            Divider()

            HStack {
                Button("Copy") {
                    model.copySelection()
                }
                .accessibilityLabel("Copy selected clipboard item")
                Button("Paste") {
                    model.pasteSelection()
                }
                .accessibilityLabel("Paste selected clipboard item")
                Menu("Actions") {
                    Button("Paste Without Formatting") {
                        model.pasteSelectionWithoutFormatting()
                    }
                    Button("Pin or Unpin") {
                        model.toggleSelectedPin()
                    }
                    Button("Delete", role: .destructive) {
                        model.deleteSelected()
                    }
                    Divider()
                    Button(model.capturePaused ? "Resume Capture" : "Pause Capture") {
                        model.capturePaused.toggle()
                    }
                    Button("Ignore Next Copy") {
                        model.ignoreNextCopy = true
                    }
                    Divider()
                    Button("Clear Unpinned") {
                        model.clearUnpinned()
                    }
                    Button("Clear All", role: .destructive) {
                        model.clearAll()
                    }
                    SettingsLink {
                        Text("Preferences")
                    }
                }
                Spacer()
                Button(model.capturePaused ? "Resume" : "Pause") {
                    model.capturePaused.toggle()
                }
                .accessibilityLabel(model.capturePaused ? "Resume clipboard capture" : "Pause clipboard capture")
            }
            .padding(12)
        }
        .onAppear {
            searchFocused = true
            if model.selectedItemID == nil {
                model.selectedItemID = model.filteredItems.first?.id
            }
        }
    }
}

struct HistoryItemRow: View {
    let item: ClippoMacHistoryItem
    @ObservedObject var model: ClippoMacModel
    let shortcut: Character?

    var body: some View {
        HStack {
            if let shortcut {
                Text(String(shortcut))
                    .font(.caption.monospacedDigit())
                    .foregroundStyle(.secondary)
                    .frame(width: 20, alignment: .leading)
                    .accessibilityLabel("Shortcut \(String(shortcut))")
            }
            Text(item.text)
                .lineLimit(2)
            Spacer()
            if item.pinned {
                Image(systemName: "pin.fill")
                    .accessibilityLabel("Pinned")
            }
        }
        .contentShape(Rectangle())
        .help(item.text)
        .onTapGesture {
            model.selectedItemID = item.id
            if NSApp.currentEvent?.modifierFlags.contains(.option) == true {
                model.pasteSelection()
            } else {
                model.copySelection()
            }
        }
        .contextMenu {
            Button(item.pinned ? "Unpin" : "Pin") {
                model.togglePin(item)
            }
            Button("Delete", role: .destructive) {
                model.delete(item)
            }
        }
        .accessibilityLabel(item.text)
        .accessibilityHint(item.pinned ? "Pinned clipboard history item" : "Clipboard history item")
    }
}

struct PreferencesView: View {
    @ObservedObject var model: ClippoMacModel

    var body: some View {
        Form {
            Toggle("Pause Capture", isOn: $model.capturePaused)
                .accessibilityLabel("Pause clipboard capture")
            Toggle("Ignore Next Copy", isOn: $model.ignoreNextCopy)
                .accessibilityLabel("Ignore next copy")
            Toggle("Launch at Login", isOn: Binding(
                get: { model.launchAtLogin },
                set: { model.setLaunchAtLogin($0) }
            ))
            .accessibilityLabel("Launch Clippo at login")
            Text("Global shortcut: Shift-Command-C")
                .foregroundStyle(.secondary)
            Text("Automatic paste uses macOS Accessibility permission. Clippo does not request Screen Recording or Input Monitoring for this scaffold.")
                .foregroundStyle(.secondary)
        }
        .padding(20)
    }
}
