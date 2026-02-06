import Foundation

class BreezSDKLiquidConnector {
    fileprivate static let TAG = "BreezSDKLiquidConnector"

    private static var liquidSDK: BindingLiquidSdk? = nil
    fileprivate static var queue = DispatchQueue(label: "BreezSDKLiquidConnector")
    fileprivate static var sdkListener: EventListener? = nil
    static var logger: ServiceLogger = ServiceLogger(logStream: nil)

    static func register(connectRequest: ConnectRequest, listener: EventListener) throws -> BindingLiquidSdk? {
        logger.log(tag: TAG, line: "register() called - SDK already connected: \(BreezSDKLiquidConnector.liquidSDK != nil)", level: "TRACE")
        return try BreezSDKLiquidConnector.queue.sync { [] in
            BreezSDKLiquidConnector.sdkListener = listener
            if BreezSDKLiquidConnector.liquidSDK == nil {
                logger.log(tag: TAG, line: "SDK is nil, calling connectSDK()", level: "TRACE")
                BreezSDKLiquidConnector.liquidSDK = try BreezSDKLiquidConnector.connectSDK(connectRequest: connectRequest)
            } else {
                logger.log(tag: TAG, line: "SDK already connected, reusing existing instance", level: "TRACE")
            }
            return BreezSDKLiquidConnector.liquidSDK
        }
    }

    static func unregister() {
        logger.log(tag: TAG, line: "unregister() called", level: "TRACE")
        BreezSDKLiquidConnector.queue.sync { [] in
            BreezSDKLiquidConnector.sdkListener = nil
            if let sdk = BreezSDKLiquidConnector.liquidSDK {
                logger.log(tag: TAG, line: "Disconnecting SDK...", level: "TRACE")
                do {
                    try sdk.disconnect()
                    logger.log(tag: TAG, line: "SDK disconnected successfully", level: "TRACE")
                } catch {
                    logger.log(tag: TAG, line: "Failed to disconnect SDK: \(error.localizedDescription)", level: "ERROR")
                }
                BreezSDKLiquidConnector.liquidSDK = nil
            } else {
                logger.log(tag: TAG, line: "SDK was already nil", level: "TRACE")
            }
        }
        logger.log(tag: TAG, line: "unregister() completed", level: "TRACE")
    }

    private static func connectSDK(connectRequest: ConnectRequest) throws -> BindingLiquidSdk? {
        logger.log(tag: TAG, line: "connectSDK() starting - workingDir: \(connectRequest.config.workingDir)", level: "TRACE")
        let liquidSDK = try connect(req: connectRequest)
        let _ = try liquidSDK.addEventListener(listener: BreezSDKEventListener())
        logger.log(tag: TAG, line: "connectSDK() completed", level: "TRACE")
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
