import Foundation
import os.log

#if DEBUG && true
fileprivate var logger = OSLog(
    subsystem: Bundle.main.bundleIdentifier!,
    category: "NwcPlugin"
)
#else
fileprivate var logger = OSLog.disabled
#endif

public class NwcPlugin {
    private static var service: BindingNwcService? = nil
    fileprivate static var queue = DispatchQueue(label: "NwcPlugin")
    fileprivate static var listener: NwcEventListener? = nil

    static func register(sdk: BindingLiquidSdk, config: NwcConfig, listener: NwcEventListener) throws -> BindingNwcService? {
        try NwcPlugin.queue.sync { [] in
            NwcPlugin.listener = listener
            if NwcPlugin.service == nil {
                NwcPlugin.service = try NwcPlugin.newService(sdk: sdk, config: config)
            }
            return NwcPlugin.service
        }
    }

    static func unregister() {
        NwcPlugin.queue.sync { [] in
            NwcPlugin.listener = nil
            if let service = NwcPlugin.service {
                service.stop()
                NwcPlugin.service = nil
            }
        }
    }

    static func newService(sdk: BindingLiquidSdk, config: NwcConfig) throws -> BindingNwcService? {
        os_log("Starting NWC service", log: logger, type: .debug)
        let nwcService = try sdk.useNwcPlugin(config: config)
        os_log("Successfully started NWC service", log: logger, type: .debug)
        let _ = nwcService.addEventListener(listener: BreezNwcEventListener())
        return nwcService
    }
}

public class BreezNwcEventListener: NwcEventListener {
    public func onEvent(event: NwcEvent) {
        NwcPlugin.queue.async { [] in
            NwcPlugin.listener?.onEvent(event: event)
        }
    }
}
