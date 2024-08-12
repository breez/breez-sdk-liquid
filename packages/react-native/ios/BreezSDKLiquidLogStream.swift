import Foundation
import BreezSDKLiquid

class BreezSDKLiquidLogger: BreezSDKLiquid.Logger {
    static let emitterName: String = "breezSdkLiquidLog"

    func log(l: LogEntry) {
        if RNBreezSDKLiquid.hasListeners {
            RNBreezSDKLiquid.emitter.sendEvent(withName: BreezSDKLiquidLogger.emitterName,
                                         body: BreezSDKLiquidMapper.dictionaryOf(logEntry: l))
        }
    }
}