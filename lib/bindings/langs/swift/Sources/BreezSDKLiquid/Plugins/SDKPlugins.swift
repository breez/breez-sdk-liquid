public class SDKPlugins {
    public var nwc: BindingNwcService?
    public init() {}

    public func stop() {
        self.nwc?.stop()
    }
}

public class PluginConfigs {
    public var nwc: NwcConfig?
}
