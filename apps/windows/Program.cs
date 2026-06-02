using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Text.Json;
using Microsoft.Win32;

namespace Clippo.Windows;

internal static class Program
{
    [STAThread]
    private static void Main()
    {
        ApplicationConfiguration.Initialize();
        Application.SetHighDpiMode(HighDpiMode.PerMonitorV2);
        using var context = new ClippoApplicationContext();
        Application.Run(context);
    }
}

internal sealed class ClippoApplicationContext : ApplicationContext
{
    private readonly NotifyIcon trayIcon;
    private readonly HistoryWindow historyWindow;
    private readonly PreferencesWindow preferencesWindow;
    private readonly ClipboardMessageWindow messageWindow;
    private readonly HistoryStore historyStore = new();
    private readonly List<HistoryItem> history = [];
    private bool capturePaused;
    private bool ignoreNextCopy;
    private bool suppressNextClipboardCapture;
    private bool isExiting;

    public ClippoApplicationContext()
    {
        history.AddRange(historyStore.Load());

        historyWindow = new HistoryWindow(this);
        preferencesWindow = new PreferencesWindow(this);
        messageWindow = new ClipboardMessageWindow(this);

        trayIcon = new NotifyIcon
        {
            Icon = SystemIcons.Application,
            Text = "Clippo",
            Visible = true,
            ContextMenuStrip = BuildTrayMenu()
        };
        trayIcon.DoubleClick += (_, _) => ShowHistory();
        PromptForLaunchAtLoginIfNeeded();
    }

    public IReadOnlyList<HistoryItem> History => history;
    public bool CapturePaused => capturePaused;
    public bool IgnoreNextCopy => ignoreNextCopy;
    public bool LaunchAtLogin => StartupRegistration.IsEnabled();
    public bool IsExiting => isExiting;

    public void ShowHistory()
    {
        historyWindow.RefreshItems();
        historyWindow.WindowState = FormWindowState.Normal;
        historyWindow.StartPosition = FormStartPosition.Manual;
        historyWindow.Location = PopupPlacement.NearCursor(historyWindow.Size);
        historyWindow.Show();
        historyWindow.Activate();
        historyWindow.FocusSearch();
    }

    public void ShowPreferences()
    {
        preferencesWindow.RefreshState();
        preferencesWindow.Show();
        preferencesWindow.Activate();
    }

    public void CaptureClipboardText()
    {
        if (capturePaused)
        {
            return;
        }

        if (ignoreNextCopy)
        {
            ignoreNextCopy = false;
            RefreshUi();
            return;
        }

        if (suppressNextClipboardCapture)
        {
            suppressNextClipboardCapture = false;
            return;
        }

        if (!Clipboard.ContainsText())
        {
            return;
        }

        var text = Clipboard.GetText(TextDataFormat.UnicodeText);
        if (string.IsNullOrWhiteSpace(text))
        {
            return;
        }

        var existing = history.Find(item => item.Text == text);
        if (existing is not null)
        {
            existing.LastUsedAt = DateTimeOffset.Now;
        }
        else
        {
            history.Insert(0, new HistoryItem(text));
        }

        SaveHistory();
        RefreshUi();
    }

    public void ToggleCapturePaused()
    {
        capturePaused = !capturePaused;
        RefreshUi();
    }

    public void IgnoreNextClipboardCopy()
    {
        ignoreNextCopy = true;
        RefreshUi();
    }

    public void Copy(HistoryItem? item)
    {
        if (item is null)
        {
            return;
        }

        Clipboard.SetText(item.Text, TextDataFormat.UnicodeText);
        suppressNextClipboardCapture = true;
    }

    public void Paste(HistoryItem? item)
    {
        if (item is null)
        {
            return;
        }

        Copy(item);
        var result = InputAutomation.Paste();
        if (result == PasteAutomationResult.BlockedByElevatedTarget)
        {
            Notify("Paste blocked", "Clippo copied the item. Press Ctrl+V in the elevated app to paste it.");
        }
    }

    public void PasteWithoutFormatting(HistoryItem? item)
    {
        Paste(item);
    }

    public void TogglePin(HistoryItem? item)
    {
        if (item is null)
        {
            return;
        }

        if (item.Pinned)
        {
            item.Pinned = false;
            item.PinnedShortcut = null;
        }
        else
        {
            item.Pinned = true;
            item.PinnedShortcut = NextPinnedShortcut();
        }
        SaveHistory();
        RefreshUi();
    }

    public void Delete(HistoryItem? item)
    {
        if (item is null)
        {
            return;
        }

        history.Remove(item);
        SaveHistory();
        RefreshUi();
    }

    public void ClearUnpinned()
    {
        history.RemoveAll(item => !item.Pinned);
        SaveHistory();
        RefreshUi();
    }

    public void ClearAll()
    {
        history.Clear();
        SaveHistory();
        RefreshUi();
    }

    public void SetLaunchAtLogin(bool enabled)
    {
        StartupRegistration.SetEnabled(enabled);
        StartupRegistration.MarkFirstRunPrompted();
        RefreshUi();
    }

    public void Notify(string title, string body)
    {
        trayIcon.BalloonTipTitle = title;
        trayIcon.BalloonTipText = body;
        trayIcon.ShowBalloonTip(3000);
    }

    public void ExitClippo()
    {
        isExiting = true;
        SaveHistory();
        ExitThread();
    }

    protected override void Dispose(bool disposing)
    {
        if (disposing)
        {
            trayIcon.Visible = false;
            trayIcon.Dispose();
            messageWindow.Dispose();
            historyWindow.Dispose();
            preferencesWindow.Dispose();
        }

        base.Dispose(disposing);
    }

    private ContextMenuStrip BuildTrayMenu()
    {
        var menu = new ContextMenuStrip();
        menu.Items.Add("Open History", null, (_, _) => ShowHistory());
        menu.Items.Add("Preferences", null, (_, _) => ShowPreferences());
        menu.Items.Add(new ToolStripSeparator());
        menu.Items.Add("Pause Capture", null, (_, _) => ToggleCapturePaused());
        menu.Items.Add("Ignore Next Copy", null, (_, _) => IgnoreNextClipboardCopy());
        menu.Items.Add(new ToolStripSeparator());
        menu.Items.Add("Clear Unpinned", null, (_, _) => ClearUnpinned());
        menu.Items.Add("Clear All", null, (_, _) => ClearAll());
        menu.Items.Add(new ToolStripSeparator());
        menu.Items.Add("Exit", null, (_, _) => ExitClippo());
        return menu;
    }

    private void RefreshUi()
    {
        historyWindow.RefreshItems();
        preferencesWindow.RefreshState();
    }

    private char? NextPinnedShortcut()
    {
        var used = history
            .Where(item => item.PinnedShortcut is not null)
            .Select(item => item.PinnedShortcut!.Value)
            .ToHashSet();

        return "123456789".FirstOrDefault(shortcut => !used.Contains(shortcut)) switch
        {
            '\0' => null,
            var shortcut => shortcut
        };
    }

    private void SaveHistory()
    {
        historyStore.Save(history);
    }

    private void PromptForLaunchAtLoginIfNeeded()
    {
        if (!StartupRegistration.ShouldPromptOnFirstRun())
        {
            return;
        }

        var result = MessageBox.Show(
            "Start Clippo automatically when you sign in?\n\nClippo works best as a tray utility that keeps clipboard history available in the background.",
            "Start Clippo automatically?",
            MessageBoxButtons.YesNo,
            MessageBoxIcon.Question
        );

        StartupRegistration.MarkFirstRunPrompted();
        if (result == DialogResult.Yes)
        {
            SetLaunchAtLogin(true);
        }
    }

    protected override void ExitThreadCore()
    {
        isExiting = true;
        SaveHistory();
        base.ExitThreadCore();
    }
}

internal sealed class HistoryWindow : Form
{
    private readonly ClippoApplicationContext app;
    private readonly TextBox search = new();
    private readonly ListView list = new();

    public HistoryWindow(ClippoApplicationContext app)
    {
        this.app = app;
        Text = "Clippo History";
        Width = 460;
        Height = 560;
        AutoScaleMode = AutoScaleMode.Dpi;
        KeyPreview = true;
        ShowInTaskbar = false;

        search.Dock = DockStyle.Top;
        search.PlaceholderText = "Search clipboard history";
        search.Margin = new Padding(8);
        search.AccessibleName = "Search clipboard history";
        search.AccessibleDescription = "Filters the visible clipboard history items.";
        search.TextChanged += (_, _) => RefreshItems();

        list.Dock = DockStyle.Fill;
        list.View = View.Details;
        list.FullRowSelect = true;
        list.MultiSelect = false;
        list.AccessibleName = "Clipboard history";
        list.AccessibleDescription = "Clipboard history items. Use arrow keys, Enter, numbers, or action buttons.";
        list.Columns.Add("Shortcut", 80);
        list.Columns.Add("Clip", 340);
        list.DoubleClick += (_, _) => app.Paste(SelectedItem);
        list.MouseClick += OnListMouseClick;

        var footer = BuildFooter();
        Controls.Add(list);
        Controls.Add(footer);
        Controls.Add(search);

        KeyDown += OnKeyDown;
        FormClosing += OnFormClosing;
        Resize += OnResize;
        RefreshItems();
    }

    public void FocusSearch()
    {
        search.Focus();
        search.SelectAll();
    }

    public void RefreshItems()
    {
        var selectedId = SelectedItem?.Id;
        list.BeginUpdate();
        list.Items.Clear();

        var rows = app.History
            .OrderByDescending(item => item.Pinned)
            .ThenByDescending(item => item.LastUsedAt)
            .Where(item => item.Text.Contains(search.Text, StringComparison.CurrentCultureIgnoreCase))
            .ToList();

        for (var index = 0; index < rows.Count; index++)
        {
            var item = rows[index];
            var row = new ListViewItem(item.PinnedShortcut?.ToString() ?? VisibleShortcutForIndex(index))
            {
                Tag = item,
                ToolTipText = item.Text
            };
            row.SubItems.Add(item.Text);
            list.Items.Add(row);
            row.Selected = item.Id == selectedId;
        }

        if (list.SelectedItems.Count == 0 && list.Items.Count > 0)
        {
            list.Items[0].Selected = true;
        }

        list.EndUpdate();
    }

    private HistoryItem? SelectedItem => list.SelectedItems.Count == 0
        ? null
        : list.SelectedItems[0].Tag as HistoryItem;

    private FlowLayoutPanel BuildFooter()
    {
        var footer = new FlowLayoutPanel
        {
            Dock = DockStyle.Bottom,
            Height = 76,
            FlowDirection = FlowDirection.LeftToRight,
            Padding = new Padding(8, 4, 8, 4),
            WrapContents = true
        };
        footer.Controls.Add(Button("Copy", (_, _) => app.Copy(SelectedItem)));
        footer.Controls.Add(Button("Paste", (_, _) => app.Paste(SelectedItem)));
        footer.Controls.Add(Button("Plain", (_, _) => app.PasteWithoutFormatting(SelectedItem)));
        footer.Controls.Add(Button("Pin", (_, _) => app.TogglePin(SelectedItem)));
        footer.Controls.Add(Button("Delete", (_, _) => app.Delete(SelectedItem)));
        footer.Controls.Add(Button("Clear", (_, _) => app.ClearUnpinned()));
        footer.Controls.Add(Button("Pause", (_, _) => app.ToggleCapturePaused()));
        footer.Controls.Add(Button("Ignore", (_, _) => app.IgnoreNextClipboardCopy()));
        footer.Controls.Add(Button("Prefs", (_, _) => app.ShowPreferences()));
        return footer;
    }

    private static Button Button(string text, EventHandler handler)
    {
        var button = new Button
        {
            Text = text,
            AccessibleName = text,
            AccessibleRole = AccessibleRole.PushButton,
            AutoSize = true
        };
        button.Click += handler;
        return button;
    }

    private void OnKeyDown(object? sender, KeyEventArgs eventArgs)
    {
        var shortcutItem = ItemForNumberKey(eventArgs.KeyCode);
        if (shortcutItem is not null && eventArgs.Alt && eventArgs.Shift)
        {
            app.PasteWithoutFormatting(shortcutItem);
            eventArgs.Handled = true;
        }
        else if (shortcutItem is not null && eventArgs.Alt)
        {
            app.Paste(shortcutItem);
            eventArgs.Handled = true;
        }
        else if (shortcutItem is not null)
        {
            SelectItem(shortcutItem);
            app.Copy(shortcutItem);
            eventArgs.Handled = true;
        }
        else if (eventArgs.Alt && eventArgs.Shift && eventArgs.KeyCode == Keys.Enter)
        {
            app.PasteWithoutFormatting(SelectedItem);
            eventArgs.Handled = true;
        }
        else if (eventArgs.Alt && eventArgs.KeyCode == Keys.Enter)
        {
            app.Paste(SelectedItem);
            eventArgs.Handled = true;
        }
        else if (eventArgs.KeyCode == Keys.Enter)
        {
            app.Copy(SelectedItem);
            eventArgs.Handled = true;
        }
        else if (eventArgs.Control && eventArgs.KeyCode == Keys.C)
        {
            app.Copy(SelectedItem);
            eventArgs.Handled = true;
        }
        else if (eventArgs.Control && eventArgs.KeyCode == Keys.Oemcomma)
        {
            app.ShowPreferences();
            eventArgs.Handled = true;
        }
        else if (eventArgs.Alt && eventArgs.KeyCode == Keys.P)
        {
            app.TogglePin(SelectedItem);
            eventArgs.Handled = true;
        }
        else if (eventArgs.Alt && eventArgs.Control && eventArgs.Shift && eventArgs.KeyCode == Keys.Delete)
        {
            app.ClearAll();
            eventArgs.Handled = true;
        }
        else if (eventArgs.Alt && eventArgs.Control && eventArgs.KeyCode == Keys.Delete)
        {
            app.ClearUnpinned();
            eventArgs.Handled = true;
        }
        else if (eventArgs.Alt && eventArgs.KeyCode == Keys.Delete)
        {
            app.Delete(SelectedItem);
            eventArgs.Handled = true;
        }
    }

    private void OnListMouseClick(object? sender, MouseEventArgs eventArgs)
    {
        if (eventArgs.Button != MouseButtons.Left)
        {
            return;
        }

        var row = list.GetItemAt(eventArgs.X, eventArgs.Y);
        if (row?.Tag is not HistoryItem item)
        {
            return;
        }

        SelectItem(item);
        if ((ModifierKeys & Keys.Alt) == Keys.Alt && (ModifierKeys & Keys.Shift) == Keys.Shift)
        {
            app.PasteWithoutFormatting(item);
        }
        else if ((ModifierKeys & Keys.Alt) == Keys.Alt)
        {
            app.Paste(item);
        }
    }

    private void OnFormClosing(object? sender, FormClosingEventArgs eventArgs)
    {
        if (app.IsExiting || eventArgs.CloseReason != CloseReason.UserClosing)
        {
            return;
        }

        eventArgs.Cancel = true;
        Hide();
    }

    private void OnResize(object? sender, EventArgs eventArgs)
    {
        if (WindowState != FormWindowState.Minimized)
        {
            return;
        }

        Hide();
        WindowState = FormWindowState.Normal;
    }

    private HistoryItem? ItemForNumberKey(Keys keyCode)
    {
        var digit = keyCode switch
        {
            >= Keys.D1 and <= Keys.D9 => (char)('1' + (int)keyCode - (int)Keys.D1),
            >= Keys.NumPad1 and <= Keys.NumPad9 => (char)('1' + (int)keyCode - (int)Keys.NumPad1),
            _ => '\0'
        };

        if (digit == '\0')
        {
            return null;
        }

        foreach (ListViewItem row in list.Items)
        {
            if (row.Text == digit.ToString())
            {
                return row.Tag as HistoryItem;
            }
        }

        return null;
    }

    private static string VisibleShortcutForIndex(int index)
    {
        return index is >= 0 and < 9 ? (index + 1).ToString() : string.Empty;
    }

    private void SelectItem(HistoryItem item)
    {
        foreach (ListViewItem row in list.Items)
        {
            row.Selected = row.Tag == item;
        }
    }
}

internal sealed class PreferencesWindow : Form
{
    private readonly ClippoApplicationContext app;
    private readonly CheckBox launchAtLogin = new()
    {
        Text = "Launch at login",
        AccessibleName = "Launch at login",
        AccessibleRole = AccessibleRole.CheckButton,
        AutoSize = true
    };
    private readonly CheckBox pauseCapture = new()
    {
        Text = "Pause capture",
        AccessibleName = "Pause capture",
        AccessibleRole = AccessibleRole.CheckButton,
        AutoSize = true
    };

    public PreferencesWindow(ClippoApplicationContext app)
    {
        this.app = app;
        Text = "Clippo Preferences";
        AccessibleName = "Clippo Preferences";
        Width = 420;
        Height = 240;
        AutoScaleMode = AutoScaleMode.Dpi;
        ShowInTaskbar = false;

        var layout = new FlowLayoutPanel
        {
            Dock = DockStyle.Fill,
            FlowDirection = FlowDirection.TopDown,
            Padding = new Padding(16)
        };
        layout.Controls.Add(launchAtLogin);
        layout.Controls.Add(pauseCapture);
        layout.Controls.Add(new Label
        {
            AutoSize = true,
            MaximumSize = new Size(360, 0),
            Text = "Paste automation uses Windows input simulation. It may not work across elevated-app boundaries."
        });

        launchAtLogin.CheckedChanged += (_, _) => app.SetLaunchAtLogin(launchAtLogin.Checked);
        pauseCapture.CheckedChanged += (_, _) =>
        {
            if (pauseCapture.Checked != app.CapturePaused)
            {
                app.ToggleCapturePaused();
            }
        };

        Controls.Add(layout);
        FormClosing += OnFormClosing;
    }

    public void RefreshState()
    {
        launchAtLogin.Checked = app.LaunchAtLogin;
        pauseCapture.Checked = app.CapturePaused;
    }

    private void OnFormClosing(object? sender, FormClosingEventArgs eventArgs)
    {
        if (app.IsExiting || eventArgs.CloseReason != CloseReason.UserClosing)
        {
            return;
        }

        eventArgs.Cancel = true;
        Hide();
    }
}

internal sealed class ClipboardMessageWindow : NativeWindow, IDisposable
{
    private const int WmClipboardUpdate = 0x031D;
    private const int WmHotKey = 0x0312;
    private readonly ClippoApplicationContext app;

    public ClipboardMessageWindow(ClippoApplicationContext app)
    {
        this.app = app;
        CreateHandle(new CreateParams());
        NativeMethods.AddClipboardFormatListener(Handle);
        // Win+V remains reserved for Windows Clipboard History. Clippo uses
        // Win+Shift+C so both history managers can coexist.
        NativeMethods.RegisterHotKey(Handle, 1, NativeMethods.ModWin | NativeMethods.ModShift, (uint)Keys.C);
    }

    protected override void WndProc(ref Message message)
    {
        if (message.Msg == WmClipboardUpdate)
        {
            app.CaptureClipboardText();
        }
        else if (message.Msg == WmHotKey)
        {
            app.ShowHistory();
        }

        base.WndProc(ref message);
    }

    public void Dispose()
    {
        NativeMethods.UnregisterHotKey(Handle, 1);
        NativeMethods.RemoveClipboardFormatListener(Handle);
        DestroyHandle();
    }
}

internal sealed class HistoryItem
{
    public HistoryItem(
        string text,
        bool pinned = false,
        Guid? id = null,
        char? pinnedShortcut = null,
        DateTimeOffset? lastUsedAt = null
    )
    {
        Id = id ?? Guid.NewGuid();
        Text = text;
        Pinned = pinned;
        PinnedShortcut = pinnedShortcut ?? (pinned ? '1' : null);
        LastUsedAt = lastUsedAt ?? DateTimeOffset.Now;
    }

    public Guid Id { get; }
    public string Text { get; }
    public bool Pinned { get; set; }
    public char? PinnedShortcut { get; set; }
    public DateTimeOffset LastUsedAt { get; set; }
}

internal sealed class PersistedHistoryItem
{
    public Guid Id { get; set; }
    public string Text { get; set; } = string.Empty;
    public bool Pinned { get; set; }
    public char? PinnedShortcut { get; set; }
    public DateTimeOffset LastUsedAt { get; set; }
}

internal sealed class HistoryStore
{
    private static readonly JsonSerializerOptions JsonOptions = new()
    {
        WriteIndented = true
    };

    private readonly string filePath = Path.Combine(
        Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData),
        "Clippo",
        "history.json"
    );

    public List<HistoryItem> Load()
    {
        if (!File.Exists(filePath))
        {
            return [];
        }

        try
        {
            var json = File.ReadAllText(filePath);
            var persisted = JsonSerializer.Deserialize<List<PersistedHistoryItem>>(json, JsonOptions) ?? [];
            return persisted
                .Where(item => !string.IsNullOrWhiteSpace(item.Text))
                .Select(item => new HistoryItem(
                    item.Text,
                    item.Pinned,
                    item.Id == Guid.Empty ? null : item.Id,
                    item.PinnedShortcut,
                    item.LastUsedAt == default ? null : item.LastUsedAt
                ))
                .ToList();
        }
        catch
        {
            return [];
        }
    }

    public void Save(IEnumerable<HistoryItem> history)
    {
        try
        {
            Directory.CreateDirectory(Path.GetDirectoryName(filePath)!);
            var persisted = history.Select(item => new PersistedHistoryItem
            {
                Id = item.Id,
                Text = item.Text,
                Pinned = item.Pinned,
                PinnedShortcut = item.PinnedShortcut,
                LastUsedAt = item.LastUsedAt
            });
            File.WriteAllText(filePath, JsonSerializer.Serialize(persisted, JsonOptions));
        }
        catch
        {
            // Clipboard contents are intentionally not logged here.
        }
    }
}

internal static class PopupPlacement
{
    public static Point NearCursor(Size windowSize)
    {
        var screen = Screen.FromPoint(Cursor.Position);
        var area = screen.WorkingArea;
        var x = Math.Min(Cursor.Position.X, area.Right - windowSize.Width);
        var y = Math.Min(Cursor.Position.Y, area.Bottom - windowSize.Height);
        return new Point(Math.Max(area.Left, x), Math.Max(area.Top, y));
    }
}

internal static class StartupRegistration
{
    private const string RunKeyPath = @"Software\Microsoft\Windows\CurrentVersion\Run";
    private const string AppKeyPath = @"Software\Clippo";
    private const string ValueName = "Clippo";
    private const string FirstRunPromptedValueName = "FirstRunAutostartPrompted";

    public static bool IsEnabled()
    {
        using var key = Registry.CurrentUser.OpenSubKey(RunKeyPath, writable: false);
        return key?.GetValue(ValueName) is string;
    }

    public static void SetEnabled(bool enabled)
    {
        using var key = Registry.CurrentUser.CreateSubKey(RunKeyPath, writable: true);
        if (key is null)
        {
            return;
        }

        if (enabled)
        {
            key.SetValue(ValueName, $"\"{Application.ExecutablePath}\"");
        }
        else
        {
            key.DeleteValue(ValueName, throwOnMissingValue: false);
        }
    }

    public static bool ShouldPromptOnFirstRun()
    {
        if (IsEnabled())
        {
            MarkFirstRunPrompted();
            return false;
        }

        using var key = Registry.CurrentUser.OpenSubKey(AppKeyPath, writable: false);
        return key?.GetValue(FirstRunPromptedValueName) is not int value || value != 1;
    }

    public static void MarkFirstRunPrompted()
    {
        using var key = Registry.CurrentUser.CreateSubKey(AppKeyPath, writable: true);
        key?.SetValue(FirstRunPromptedValueName, 1, RegistryValueKind.DWord);
    }
}

internal static class InputAutomation
{
    public static PasteAutomationResult Paste()
    {
        if (!IsCurrentProcessElevated() && ForegroundWindowInspector.IsForegroundProcessElevated())
        {
            return PasteAutomationResult.BlockedByElevatedTarget;
        }

        SendPasteKeystroke();
        return PasteAutomationResult.Sent;
    }

    private static bool IsCurrentProcessElevated()
    {
        using var identity = System.Security.Principal.WindowsIdentity.GetCurrent();
        var principal = new System.Security.Principal.WindowsPrincipal(identity);
        return principal.IsInRole(System.Security.Principal.WindowsBuiltInRole.Administrator);
    }

    private static void SendPasteKeystroke()
    {
        var inputs = new[]
        {
            NativeInput.KeyDown(Keys.ControlKey),
            NativeInput.KeyDown(Keys.V),
            NativeInput.KeyUp(Keys.V),
            NativeInput.KeyUp(Keys.ControlKey)
        };
        NativeMethods.SendInput((uint)inputs.Length, inputs, Marshal.SizeOf<NativeInput>());
    }
}

internal enum PasteAutomationResult
{
    Sent,
    BlockedByElevatedTarget
}

internal static class ForegroundWindowInspector
{
    public static bool IsForegroundProcessElevated()
    {
        var foregroundWindow = NativeMethods.GetForegroundWindow();
        if (foregroundWindow == IntPtr.Zero)
        {
            return false;
        }

        NativeMethods.GetWindowThreadProcessId(foregroundWindow, out var processId);
        if (processId == 0)
        {
            return false;
        }

        var processHandle = NativeMethods.OpenProcess(NativeMethods.ProcessQueryLimitedInformation, false, processId);
        if (processHandle == IntPtr.Zero)
        {
            return false;
        }

        try
        {
            if (!NativeMethods.OpenProcessToken(processHandle, NativeMethods.TokenQuery, out var tokenHandle))
            {
                return false;
            }

            try
            {
                var elevation = new TokenElevation();
                var elevationSize = Marshal.SizeOf<TokenElevation>();
                return NativeMethods.GetTokenInformation(
                    tokenHandle,
                    NativeMethods.TokenElevationClass,
                    ref elevation,
                    elevationSize,
                    out _
                ) && elevation.TokenIsElevated != 0;
            }
            finally
            {
                NativeMethods.CloseHandle(tokenHandle);
            }
        }
        finally
        {
            NativeMethods.CloseHandle(processHandle);
        }
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct TokenElevation
{
    public int TokenIsElevated;
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeInput
{
    public uint Type;
    public NativeKeyboardInput Keyboard;

    public static NativeInput KeyDown(Keys key) => new()
    {
        Type = 1,
        Keyboard = new NativeKeyboardInput
        {
            VirtualKey = (ushort)key
        }
    };

    public static NativeInput KeyUp(Keys key) => new()
    {
        Type = 1,
        Keyboard = new NativeKeyboardInput
        {
            VirtualKey = (ushort)key,
            Flags = 0x0002
        }
    };
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeKeyboardInput
{
    public ushort VirtualKey;
    public ushort Scan;
    public uint Flags;
    public uint Time;
    public IntPtr ExtraInfo;
}

internal static class NativeMethods
{
    public const uint ModShift = 0x0004;
    public const uint ModWin = 0x0008;
    public const uint ProcessQueryLimitedInformation = 0x1000;
    public const uint TokenQuery = 0x0008;
    public const int TokenElevationClass = 20;

    [DllImport("user32.dll", SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    public static extern bool AddClipboardFormatListener(IntPtr hwnd);

    [DllImport("user32.dll", SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    public static extern bool RemoveClipboardFormatListener(IntPtr hwnd);

    [DllImport("user32.dll", SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    public static extern bool RegisterHotKey(IntPtr hwnd, int id, uint fsModifiers, uint vk);

    [DllImport("user32.dll", SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    public static extern bool UnregisterHotKey(IntPtr hwnd, int id);

    [DllImport("user32.dll", SetLastError = true)]
    public static extern uint SendInput(uint numberOfInputs, NativeInput[] inputs, int size);

    [DllImport("user32.dll")]
    public static extern IntPtr GetForegroundWindow();

    [DllImport("user32.dll")]
    public static extern uint GetWindowThreadProcessId(IntPtr windowHandle, out uint processId);

    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern IntPtr OpenProcess(uint desiredAccess, bool inheritHandle, uint processId);

    [DllImport("advapi32.dll", SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    public static extern bool OpenProcessToken(IntPtr processHandle, uint desiredAccess, out IntPtr tokenHandle);

    [DllImport("advapi32.dll", SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    public static extern bool GetTokenInformation(
        IntPtr tokenHandle,
        int tokenInformationClass,
        ref TokenElevation tokenInformation,
        int tokenInformationLength,
        out int returnLength
    );

    [DllImport("kernel32.dll", SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    public static extern bool CloseHandle(IntPtr handle);
}
