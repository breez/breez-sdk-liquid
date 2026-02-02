import Foundation
import os.log

#if DEBUG && true
fileprivate var logger = OSLog(
    subsystem: Bundle.main.bundleIdentifier!,
    category: "BreezSDKLiquidConnector"
)
#else
fileprivate var logger = OSLog.disabled
#endif

class BreezSDKLiquidConnector {
    private static var liquidSDK: BindingLiquidSdk? = nil
    fileprivate static var queue = DispatchQueue(label: "BreezSDKLiquidConnector")
    fileprivate static var sdkListener: EventListener? = nil

    static func register(connectRequest: ConnectRequest, listener: EventListener) throws -> BindingLiquidSdk? {
        os_log("register() called - SDK already connected: %{public}@", log: logger, type: .debug, String(BreezSDKLiquidConnector.liquidSDK != nil))
        return try BreezSDKLiquidConnector.queue.sync { [] in
            BreezSDKLiquidConnector.sdkListener = listener
            if BreezSDKLiquidConnector.liquidSDK == nil {
                os_log("SDK is nil, calling connectSDK()", log: logger, type: .debug)
                BreezSDKLiquidConnector.liquidSDK = try BreezSDKLiquidConnector.connectSDK(connectRequest: connectRequest)
            } else {
                os_log("SDK already connected, reusing existing instance", log: logger, type: .debug)
            }
            return BreezSDKLiquidConnector.liquidSDK
        }
    }

    static func unregister() {
        os_log("unregister() called", log: logger, type: .debug)
        BreezSDKLiquidConnector.queue.sync { [] in
            BreezSDKLiquidConnector.sdkListener = nil
            if let sdk = BreezSDKLiquidConnector.liquidSDK {
                os_log("Disconnecting SDK...", log: logger, type: .debug)
                do {
                    try sdk.disconnect()
                    os_log("SDK disconnected successfully", log: logger, type: .debug)
                } catch {
                    os_log("Failed to disconnect SDK: %@", log: logger, type: .error, error.localizedDescription)
                }
                BreezSDKLiquidConnector.liquidSDK = nil
            } else {
                os_log("SDK was already nil", log: logger, type: .debug)
            }
        }
        os_log("unregister() completed", log: logger, type: .debug)
    }

    static func connectSDK(connectRequest: ConnectRequest) throws -> BindingLiquidSdk? {
        // Connect to the Breez Liquid SDK make it ready for use
        os_log("connectSDK() starting - workingDir: %{public}@", log: logger, type: .debug, connectRequest.config.workingDir)
        let startTime = Date()
        let liquidSDK = try connect(req: connectRequest)
        let elapsed = Date().timeIntervalSince(startTime)
        os_log("connect() completed in %.3fs", log: logger, type: .debug, elapsed)
        let _ = try liquidSDK.addEventListener(listener: BreezSDKEventListener())
        os_log("addEventListener() completed", log: logger, type: .debug)
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
