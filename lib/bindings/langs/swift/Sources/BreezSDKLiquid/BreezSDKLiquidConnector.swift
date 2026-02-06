import Foundation

class BreezSDKLiquidConnector {
    fileprivate static let TAG = "BreezSDKLiquidConnector"

    private static var liquidSDK: BindingLiquidSdk? = nil
    fileprivate static var queue = DispatchQueue(label: "BreezSDKLiquidConnector")
    fileprivate static var sdkListener: EventListener? = nil
    static var logger: ServiceLogger = ServiceLogger(logStream: nil)

    static func register(connectRequest: ConnectRequest, listener: EventListener) throws -> BindingLiquidSdk? {
        return try BreezSDKLiquidConnector.queue.sync { [] in
            BreezSDKLiquidConnector.sdkListener = listener
            if BreezSDKLiquidConnector.liquidSDK == nil {
                logger.log(tag: TAG, line: "Connecting to Breez SDK", level: "DEBUG")
                BreezSDKLiquidConnector.liquidSDK = try BreezSDKLiquidConnector.connectSDK(connectRequest: connectRequest)
                logger.log(tag: TAG, line: "Connected to Breez SDK", level: "DEBUG")
            } else {
                logger.log(tag: TAG, line: "Reusing existing Breez SDK connection", level: "DEBUG")
            }
            return BreezSDKLiquidConnector.liquidSDK
        }
    }

    static func unregister() {
        BreezSDKLiquidConnector.queue.sync { [] in
            BreezSDKLiquidConnector.sdkListener = nil
            if let sdk = BreezSDKLiquidConnector.liquidSDK {
                logger.log(tag: TAG, line: "Disconnecting from Breez SDK", level: "DEBUG")
                do {
                    try sdk.disconnect()
                    logger.log(tag: TAG, line: "Disconnected from Breez SDK", level: "DEBUG")
                } catch {
                    logger.log(tag: TAG, line: "Failed to disconnect from Breez SDK: \(error.localizedDescription)", level: "ERROR")
                }
                BreezSDKLiquidConnector.liquidSDK = nil
            }
        }
    }

    private static func connectSDK(connectRequest: ConnectRequest) throws -> BindingLiquidSdk? {
        let liquidSDK = try connect(req: connectRequest)
        let _ = try liquidSDK.addEventListener(listener: BreezSDKEventListener())
        return liquidSDK
    }
}

class BreezSDKEventListener: EventListener {
    func onEvent(e: SdkEvent) {
        BreezSDKLiquidConnector.queue.async { [] in
            BreezSDKLiquidConnector.sdkListener?.onEvent(e: e)
        }
    }
}
