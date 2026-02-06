import Foundation

public class PluginManager {
    fileprivate static let TAG = "PluginManager"
    private static var nwcPlugin: BindingNwcService? = nil
    fileprivate static var queue = DispatchQueue(label: "PluginManager")
    static var logger: ServiceLogger = ServiceLogger(logStream: nil)

    static func nwc(liquidSDK: BindingLiquidSdk, pluginConfigs: PluginConfigs) throws -> BindingNwcService? {
        try PluginManager.queue.sync { [] in
            if PluginManager.nwcPlugin == nil {
                if pluginConfigs.nwc == nil {
                    return nil;
                }
                logger.log(tag: TAG, line: "Starting NWC service", level: "INFO")
                PluginManager.nwcPlugin = try liquidSDK.useNwcPlugin(config: pluginConfigs.nwc!)
            }
            return PluginManager.nwcPlugin
        }
    }

    static func shutdown() {
        PluginManager.queue.sync { [] in
            if PluginManager.nwcPlugin != nil {
                logger.log(tag: TAG, line: "Shutting down NWC service", level: "INFO")
                PluginManager.nwcPlugin?.stop()
                PluginManager.nwcPlugin = nil
            }
        }
    }
}

public class PluginConfigs {
    public var nwc: NwcConfig?
    init(nwc: NwcConfig?) {
        self.nwc = nwc;
    }
}
