import Foundation
import BreezLiquidSDK

class BreezLiquidSDKEventListener: EventListener {
    private var id: String?

    func setId(id: String) {
        self.id = id
        RNBreezLiquidSDK.addSupportedEvent(name: "event-\(id)")
    }

    func onEvent(e: LiquidSdkEvent) {
        if let id = self.id {
            if RNBreezLiquidSDK.hasListeners {
                RNBreezLiquidSDK.emitter.sendEvent(withName: "event-\(id)",
                                                   body: BreezLiquidSDKMapper.dictionaryOf(liquidSdkEvent: e))
            }
        }
    }
}
