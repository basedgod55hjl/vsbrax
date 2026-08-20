// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "AbrasaxKit",
    platforms: [
        .iOS(.v17),
        .macOS(.v14),
    ],
    products: [
        .library(name: "AbrasaxKit", targets: ["AbrasaxKit"])
    ],
    targets: [
        .target(
            name: "AbrasaxKit",
            swiftSettings: [.enableUpcomingFeature("StrictConcurrency")]
        ),
        .testTarget(
            name: "AbrasaxKitTests",
            dependencies: ["AbrasaxKit"]
        ),
    ]
)
