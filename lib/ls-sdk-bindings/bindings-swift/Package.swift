// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "bindings-swift",
    platforms: [
        .macOS(.v12),
        .iOS(.v11),
    ],
    products: [
        .library(name: "LiquidSwapSDK", targets: ["ls_sdkFFI", "LiquidSwapSDK"]),
    ],
    targets: [
        .binaryTarget(name: "ls_sdkFFI", path: "./ls_sdkFFI.xcframework"),
        .target(name: "LiquidSwapSDK", dependencies: ["ls_sdkFFI"]),
    ]
)
