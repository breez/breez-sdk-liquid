import Foundation
import LiquidSwapSDK

@objc(RNLiquidSwapSDK)
class RNLiquidSwapSDK: RCTEventEmitter {
    static let TAG: String = "LiquidSwapSDK"

    public static var emitter: RCTEventEmitter!
    public static var hasListeners: Bool = false

    private var bindingWallet: BindingWallet!

    static var defaultDataDir: URL {
        let applicationDirectory = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first!

        return applicationDirectory.appendingPathComponent("lsSdk", isDirectory: true)
    }

    override init() {
        super.init()
        RNLiquidSwapSDK.emitter = self
    }

    @objc
    override static func moduleName() -> String! {
        TAG
    }

    override func supportedEvents() -> [String]! {
        return []
    }

    override func startObserving() {
        RNLiquidSwapSDK.hasListeners = true
    }

    override func stopObserving() {
        RNLiquidSwapSDK.hasListeners = false
    }

    @objc
    override static func requiresMainQueueSetup() -> Bool {
        return false
    }

    func getBindingWallet() throws -> BindingWallet {
        if bindingWallet != nil {
            return bindingWallet
        }

        throw LsSdkError.Generic(message: "Not initialized")
    }

    @objc(initBindingWallet:dataDir:network:resolve:reject:)
    func initBindingWallet(_ mnemonic: String, dataDir: String, network: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        if bindingWallet != nil {
            reject("Generic", "Already initialized", nil)
            return
        }

        do {
            let dataDirTmp = dataDir.isEmpty ? RNLiquidSwapSDK.defaultDataDir.path : dataDir
            let networkTmp = try LiquidSwapSDKMapper.asNetwork(network: network)
            bindingWallet = try LiquidSwapSDK.`init`(mnemonic: mnemonic, dataDir: dataDirTmp, network: networkTmp)
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(getInfo:resolve:reject:)
    func getInfo(_ withScan: Bool, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBindingWallet().getInfo(withScan: withScan)
            resolve(LiquidSwapSDKMapper.dictionaryOf(walletInfo: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(prepareSendPayment:resolve:reject:)
    func prepareSendPayment(_ invoice: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBindingWallet().prepareSendPayment(invoice: invoice)
            resolve(LiquidSwapSDKMapper.dictionaryOf(prepareSendResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(sendPayment:resolve:reject:)
    func sendPayment(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let prepareSendResponse = try LiquidSwapSDKMapper.asPrepareSendResponse(prepareSendResponse: req)
            var res = try getBindingWallet().sendPayment(req: prepareSendResponse)
            resolve(LiquidSwapSDKMapper.dictionaryOf(sendPaymentResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(prepareReceivePayment:resolve:reject:)
    func prepareReceivePayment(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let prepareReceiveRequest = try LiquidSwapSDKMapper.asPrepareReceiveRequest(prepareReceiveRequest: req)
            var res = try getBindingWallet().prepareReceivePayment(req: prepareReceiveRequest)
            resolve(LiquidSwapSDKMapper.dictionaryOf(prepareReceiveResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(receivePayment:resolve:reject:)
    func receivePayment(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let prepareReceiveResponse = try LiquidSwapSDKMapper.asPrepareReceiveResponse(prepareReceiveResponse: req)
            var res = try getBindingWallet().receivePayment(req: prepareReceiveResponse)
            resolve(LiquidSwapSDKMapper.dictionaryOf(receivePaymentResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(backup:reject:)
    func backup(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            try getBindingWallet().backup()
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(restore:resolve:reject:)
    func restore(_ backupPath: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let backupPathTmp = backupPath.isEmpty ? nil : backupPath
            try getBindingWallet().restore(backupPath: backupPathTmp)
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
