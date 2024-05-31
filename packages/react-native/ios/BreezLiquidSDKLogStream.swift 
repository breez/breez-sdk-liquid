import Foundation
import BreezLiquidSDK

class BreezLiquidSDKLogger: Logger {
    static let emitterName: String = "breezLiquidSdkLog"

    func log(l: LogEntry) {
        if RNBreezLiquidSDK.hasListeners {
            RNBreezLiquidSDK.emitter.sendEvent(withName: BreezLiquidSDKLogger.emitterName,
                                         body: BreezLiquidSDKMapper.dictionaryOf(logEntry: l))
        }
    }
}