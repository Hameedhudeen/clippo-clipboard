// swift-tools-version: 5.10
import PackageDescription

let package = Package(
    name: "ClippoMac",
    platforms: [
        .macOS(.v14)
    ],
    products: [
        .executable(name: "ClippoMac", targets: ["ClippoMac"])
    ],
    targets: [
        .executableTarget(
            name: "ClippoMac",
            path: "Sources/ClippoMac"
        )
    ]
)
