import Foundation
import os.log

#if DEBUG && true
fileprivate var logger = OSLog(
    subsystem: Bundle.main.bundleIdentifier!,
    category: "PluginManager"
)
#else
fileprivate var logger = OSLog.disabled
#endif

public class PluginManager {
    private static var nwcPlugin: BindingNwcService? = nil

    fileprivate static var queue = DispatchQueue(label: "PluginManager")

    static func nwc(liquidSDK: BindingLiquidSdk, pluginConfigs: PluginConfigs) throws -> BindingNwcService? {
        try PluginManager.queue.sync { [] in
            if PluginManager.nwcPlugin == nil {
                if pluginConfigs.nwc == nil {
                    return nil;
                }
                os_log("Starting NWC service", log: logger, type: .debug)
                PluginManager.nwcPlugin = try liquidSDK.useNwcPlugin(config: pluginConfigs.nwc!)
                os_log("Successfully started NWC service", log: logger, type: .debug)
            }
            return PluginManager.nwcPlugin
        }
    }

    static func shutdown() {
        PluginManager.queue.sync { [] in
            os_log("Shutting down the plugin manager", log: logger, type: .debug)
            PluginManager.nwcPlugin?.stop()
            PluginManager.nwcPlugin = nil
            os_log("Successfully shut down the plugin manager", log: logger, type: .debug)
        }
    }
}

public class PluginConfigs {
    public var nwc: NwcConfig?
    init(nwc: NwcConfig?) {
        self.nwc = nwc;
    }
}
