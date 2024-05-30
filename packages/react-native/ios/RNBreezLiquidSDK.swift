import BreezLiquidSDK
import Foundation

@objc(RNBreezLiquidSDK)
class RNBreezLiquidSDK: RCTEventEmitter {
    static let TAG: String = "BreezLiquidSDK"

    public static var emitter: RCTEventEmitter!
    public static var hasListeners: Bool = false
    public static var supportedEvents: [String] = []

    private var bindingLiquidSdk: BindingLiquidSdk!

    static var defaultDataDir: URL {
        let applicationDirectory = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first!

        return applicationDirectory.appendingPathComponent("breezLiquidSdk", isDirectory: true)
    }

    override init() {
        super.init()
        RNBreezLiquidSDK.emitter = self
    }

    @objc
    override static func moduleName() -> String! {
        TAG
    }

    static func addSupportedEvent(name: String) {
        RNBreezLiquidSDK.supportedEvents.append(name)
    }

    override func supportedEvents() -> [String]! {
        return RNBreezLiquidSDK.supportedEvents
    }

    override func startObserving() {
        RNBreezLiquidSDK.hasListeners = true
    }

    override func stopObserving() {
        RNBreezLiquidSDK.hasListeners = false
    }

    @objc
    override static func requiresMainQueueSetup() -> Bool {
        return false
    }

    func getBindingLiquidSdk() throws -> BindingLiquidSdk {
        if bindingLiquidSdk != nil {
            return bindingLiquidSdk
        }

        throw LiquidSdkError.Generic(message: "Not initialized")
    }

    @objc(connect:resolve:reject:)
    func connect(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        if bindingLiquidSdk != nil {
            reject("Generic", "Already initialized", nil)
            return
        }

        do {
            var connectRequest = try BreezLiquidSDKMapper.asConnectRequest(connectRequest: req)
            connectRequest.dataDir = connectRequest.dataDir == nil || connectRequest.dataDir!.isEmpty ? RNBreezLiquidSDK.defaultDataDir.path : connectRequest.dataDir
            bindingLiquidSdk = try BreezLiquidSDK.connect(req: connectRequest)
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(addEventListener:reject:)
    func addEventListener(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var eventListener = BreezLiquidSDKEventListener()
            var res = try getBindingLiquidSdk().addEventListener(listener: eventListener)

            eventListener.setId(id: res)
            resolve(res)
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(removeEventListener:resolve:reject:)
    func removeEventListener(_ id: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            try getBindingLiquidSdk().removeEventListener(id: id)
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(getInfo:resolve:reject:)
    func getInfo(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let getInfoRequest = try BreezLiquidSDKMapper.asGetInfoRequest(getInfoRequest: req)
            var res = try getBindingLiquidSdk().getInfo(req: getInfoRequest)
            resolve(BreezLiquidSDKMapper.dictionaryOf(getInfoResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(prepareSendPayment:resolve:reject:)
    func prepareSendPayment(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let prepareSendRequest = try BreezLiquidSDKMapper.asPrepareSendRequest(prepareSendRequest: req)
            var res = try getBindingLiquidSdk().prepareSendPayment(req: prepareSendRequest)
            resolve(BreezLiquidSDKMapper.dictionaryOf(prepareSendResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(sendPayment:resolve:reject:)
    func sendPayment(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let prepareSendResponse = try BreezLiquidSDKMapper.asPrepareSendResponse(prepareSendResponse: req)
            var res = try getBindingLiquidSdk().sendPayment(req: prepareSendResponse)
            resolve(BreezLiquidSDKMapper.dictionaryOf(sendPaymentResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(prepareReceivePayment:resolve:reject:)
    func prepareReceivePayment(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let prepareReceiveRequest = try BreezLiquidSDKMapper.asPrepareReceiveRequest(prepareReceiveRequest: req)
            var res = try getBindingLiquidSdk().prepareReceivePayment(req: prepareReceiveRequest)
            resolve(BreezLiquidSDKMapper.dictionaryOf(prepareReceiveResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(receivePayment:resolve:reject:)
    func receivePayment(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let prepareReceiveResponse = try BreezLiquidSDKMapper.asPrepareReceiveResponse(prepareReceiveResponse: req)
            var res = try getBindingLiquidSdk().receivePayment(req: prepareReceiveResponse)
            resolve(BreezLiquidSDKMapper.dictionaryOf(receivePaymentResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(listPayments:reject:)
    func listPayments(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBindingLiquidSdk().listPayments()
            resolve(BreezLiquidSDKMapper.arrayOf(paymentList: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(sync:reject:)
    func sync(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            try getBindingLiquidSdk().sync()
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(backup:resolve:reject:)
    func backup(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let backupRequest = try BreezLiquidSDKMapper.asBackupRequest(backupRequest: req)
            try getBindingLiquidSdk().backup(req: backupRequest)
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(restore:resolve:reject:)
    func restore(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let restoreRequest = try BreezLiquidSDKMapper.asRestoreRequest(restoreRequest: req)
            try getBindingLiquidSdk().restore(req: restoreRequest)
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(disconnect:reject:)
    func disconnect(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            try getBindingLiquidSdk().disconnect()
            bindingLiquidSdk = nil
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    func rejectErr(err: Error, reject: @escaping RCTPromiseRejectBlock) {
        var errorName = "Generic"
        var message = "\(err)"
        if let errAssociated = Mirror(reflecting: err).children.first {
            errorName = errAssociated.label ?? errorName
            if let associatedMessage = Mirror(reflecting: errAssociated.value).children.first {
                message = associatedMessage.value as! String
            }
        }
        reject(errorName, message, err)
    }
}
