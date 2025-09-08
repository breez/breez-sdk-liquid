import BreezSDKLiquid
import Foundation

@objc(RNBreezSDKLiquid)
class RNBreezSDKLiquid: RCTEventEmitter {
    static let TAG: String = "BreezSDKLiquid"

    static var emitter: RCTEventEmitter!
    static var hasListeners: Bool = false
    static var supportedEvents: [String] = ["breezSdkLiquidLog"]

    private var bindingLiquidSdk: BindingLiquidSdk!

    static var breezSdkLiquidDirectory: URL {
        let applicationDirectory = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first!
        let breezSdkLiquidDirectory = applicationDirectory.appendingPathComponent("breezSdkLiquid", isDirectory: true)

        if !FileManager.default.fileExists(atPath: breezSdkLiquidDirectory.path) {
            try! FileManager.default.createDirectory(atPath: breezSdkLiquidDirectory.path, withIntermediateDirectories: true)
        }

        return breezSdkLiquidDirectory
    }

    override init() {
        super.init()
        RNBreezSDKLiquid.emitter = self
    }

    @objc
    override static func moduleName() -> String! {
        TAG
    }

    static func addSupportedEvent(name: String) {
        RNBreezSDKLiquid.supportedEvents.append(name)
    }

    override func supportedEvents() -> [String]! {
        return RNBreezSDKLiquid.supportedEvents
    }

    override func startObserving() {
        RNBreezSDKLiquid.hasListeners = true
    }

    override func stopObserving() {
        RNBreezSDKLiquid.hasListeners = false
    }

    @objc
    override static func requiresMainQueueSetup() -> Bool {
        return false
    }

    func getBindingLiquidSdk() throws -> BindingLiquidSdk {
        if bindingLiquidSdk != nil {
            return bindingLiquidSdk
        }

        throw SdkError.Generic(message: "Not initialized")
    }

    private func ensureWorkingDir(workingDir: String) throws {
        do {
            if !FileManager.default.fileExists(atPath: workingDir) {
                try FileManager.default.createDirectory(atPath: workingDir, withIntermediateDirectories: true)
            }
        } catch {
            throw SdkError.Generic(message: "Mandatory field workingDir must contain a writable directory")
        }
    }

    @objc(defaultConfig:breezApiKey:resolve:reject:)
    func defaultConfig(_ network: String, breezApiKey: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let networkTmp = try BreezSDKLiquidMapper.asLiquidNetwork(liquidNetwork: network)
            let breezApiKeyTmp = breezApiKey.isEmpty ? nil : breezApiKey
            var res = try BreezSDKLiquid.defaultConfig(network: networkTmp, breezApiKey: breezApiKeyTmp)
            res.workingDir = RNBreezSDKLiquid.breezSdkLiquidDirectory.path
            resolve(BreezSDKLiquidMapper.dictionaryOf(config: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(parseInvoice:resolve:reject:)
    func parseInvoice(_ input: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try BreezSDKLiquid.parseInvoice(input: input)
            resolve(BreezSDKLiquidMapper.dictionaryOf(lnInvoice: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(setLogger:reject:)
    func setLogger(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            try BreezSDKLiquid.setLogger(logger: BreezSDKLiquidLogger())
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(connect:resolve:reject:)
    func connect(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        if bindingLiquidSdk != nil {
            reject("Generic", "Already initialized", nil)
            return
        }

        do {
            var connectRequest = try BreezSDKLiquidMapper.asConnectRequest(connectRequest: req)
            try ensureWorkingDir(workingDir: connectRequest.config.workingDir)

            bindingLiquidSdk = try BreezSDKLiquid.connect(req: connectRequest)
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(addEventListener:reject:)
    func addEventListener(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var eventListener = BreezSDKEventListener()
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

    @objc(getInfo:reject:)
    func getInfo(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBindingLiquidSdk().getInfo()
            resolve(BreezSDKLiquidMapper.dictionaryOf(getInfoResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(signMessage:resolve:reject:)
    func signMessage(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let signMessageRequest = try BreezSDKLiquidMapper.asSignMessageRequest(signMessageRequest: req)
            var res = try getBindingLiquidSdk().signMessage(req: signMessageRequest)
            resolve(BreezSDKLiquidMapper.dictionaryOf(signMessageResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(checkMessage:resolve:reject:)
    func checkMessage(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let checkMessageRequest = try BreezSDKLiquidMapper.asCheckMessageRequest(checkMessageRequest: req)
            var res = try getBindingLiquidSdk().checkMessage(req: checkMessageRequest)
            resolve(BreezSDKLiquidMapper.dictionaryOf(checkMessageResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(parse:resolve:reject:)
    func parse(_ input: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBindingLiquidSdk().parse(input: input)
            resolve(BreezSDKLiquidMapper.dictionaryOf(inputType: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(prepareSendPayment:resolve:reject:)
    func prepareSendPayment(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let prepareSendRequest = try BreezSDKLiquidMapper.asPrepareSendRequest(prepareSendRequest: req)
            var res = try getBindingLiquidSdk().prepareSendPayment(req: prepareSendRequest)
            resolve(BreezSDKLiquidMapper.dictionaryOf(prepareSendResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(sendPayment:resolve:reject:)
    func sendPayment(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let sendPaymentRequest = try BreezSDKLiquidMapper.asSendPaymentRequest(sendPaymentRequest: req)
            var res = try getBindingLiquidSdk().sendPayment(req: sendPaymentRequest)
            resolve(BreezSDKLiquidMapper.dictionaryOf(sendPaymentResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(prepareReceivePayment:resolve:reject:)
    func prepareReceivePayment(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let prepareReceiveRequest = try BreezSDKLiquidMapper.asPrepareReceiveRequest(prepareReceiveRequest: req)
            var res = try getBindingLiquidSdk().prepareReceivePayment(req: prepareReceiveRequest)
            resolve(BreezSDKLiquidMapper.dictionaryOf(prepareReceiveResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(receivePayment:resolve:reject:)
    func receivePayment(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let receivePaymentRequest = try BreezSDKLiquidMapper.asReceivePaymentRequest(receivePaymentRequest: req)
            var res = try getBindingLiquidSdk().receivePayment(req: receivePaymentRequest)
            resolve(BreezSDKLiquidMapper.dictionaryOf(receivePaymentResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(createBolt12Invoice:resolve:reject:)
    func createBolt12Invoice(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let createBolt12InvoiceRequest = try BreezSDKLiquidMapper.asCreateBolt12InvoiceRequest(createBolt12InvoiceRequest: req)
            var res = try getBindingLiquidSdk().createBolt12Invoice(req: createBolt12InvoiceRequest)
            resolve(BreezSDKLiquidMapper.dictionaryOf(createBolt12InvoiceResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(fetchLightningLimits:reject:)
    func fetchLightningLimits(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBindingLiquidSdk().fetchLightningLimits()
            resolve(BreezSDKLiquidMapper.dictionaryOf(lightningPaymentLimitsResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(fetchOnchainLimits:reject:)
    func fetchOnchainLimits(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBindingLiquidSdk().fetchOnchainLimits()
            resolve(BreezSDKLiquidMapper.dictionaryOf(onchainPaymentLimitsResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(preparePayOnchain:resolve:reject:)
    func preparePayOnchain(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let preparePayOnchainRequest = try BreezSDKLiquidMapper.asPreparePayOnchainRequest(preparePayOnchainRequest: req)
            var res = try getBindingLiquidSdk().preparePayOnchain(req: preparePayOnchainRequest)
            resolve(BreezSDKLiquidMapper.dictionaryOf(preparePayOnchainResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(payOnchain:resolve:reject:)
    func payOnchain(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let payOnchainRequest = try BreezSDKLiquidMapper.asPayOnchainRequest(payOnchainRequest: req)
            var res = try getBindingLiquidSdk().payOnchain(req: payOnchainRequest)
            resolve(BreezSDKLiquidMapper.dictionaryOf(sendPaymentResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(prepareBuyBitcoin:resolve:reject:)
    func prepareBuyBitcoin(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let prepareBuyBitcoinRequest = try BreezSDKLiquidMapper.asPrepareBuyBitcoinRequest(prepareBuyBitcoinRequest: req)
            var res = try getBindingLiquidSdk().prepareBuyBitcoin(req: prepareBuyBitcoinRequest)
            resolve(BreezSDKLiquidMapper.dictionaryOf(prepareBuyBitcoinResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(buyBitcoin:resolve:reject:)
    func buyBitcoin(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let buyBitcoinRequest = try BreezSDKLiquidMapper.asBuyBitcoinRequest(buyBitcoinRequest: req)
            var res = try getBindingLiquidSdk().buyBitcoin(req: buyBitcoinRequest)
            resolve(res)
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(listPayments:resolve:reject:)
    func listPayments(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let listPaymentsRequest = try BreezSDKLiquidMapper.asListPaymentsRequest(listPaymentsRequest: req)
            var res = try getBindingLiquidSdk().listPayments(req: listPaymentsRequest)
            resolve(BreezSDKLiquidMapper.arrayOf(paymentList: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(getPayment:resolve:reject:)
    func getPayment(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let reqTmp = try BreezSDKLiquidMapper.asGetPaymentRequest(getPaymentRequest: req)
            var res = try getBindingLiquidSdk().getPayment(req: reqTmp)
            if res != nil {
                resolve(BreezSDKLiquidMapper.dictionaryOf(payment: res!))
            } else {
                resolve(nil)
            }
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(fetchPaymentProposedFees:resolve:reject:)
    func fetchPaymentProposedFees(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let fetchPaymentProposedFeesRequest = try BreezSDKLiquidMapper.asFetchPaymentProposedFeesRequest(fetchPaymentProposedFeesRequest: req)
            var res = try getBindingLiquidSdk().fetchPaymentProposedFees(req: fetchPaymentProposedFeesRequest)
            resolve(BreezSDKLiquidMapper.dictionaryOf(fetchPaymentProposedFeesResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(acceptPaymentProposedFees:resolve:reject:)
    func acceptPaymentProposedFees(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let acceptPaymentProposedFeesRequest = try BreezSDKLiquidMapper.asAcceptPaymentProposedFeesRequest(acceptPaymentProposedFeesRequest: req)
            try getBindingLiquidSdk().acceptPaymentProposedFees(req: acceptPaymentProposedFeesRequest)
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(listRefundables:reject:)
    func listRefundables(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBindingLiquidSdk().listRefundables()
            resolve(BreezSDKLiquidMapper.arrayOf(refundableSwapList: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(prepareRefund:resolve:reject:)
    func prepareRefund(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let prepareRefundRequest = try BreezSDKLiquidMapper.asPrepareRefundRequest(prepareRefundRequest: req)
            var res = try getBindingLiquidSdk().prepareRefund(req: prepareRefundRequest)
            resolve(BreezSDKLiquidMapper.dictionaryOf(prepareRefundResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(refund:resolve:reject:)
    func refund(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let refundRequest = try BreezSDKLiquidMapper.asRefundRequest(refundRequest: req)
            var res = try getBindingLiquidSdk().refund(req: refundRequest)
            resolve(BreezSDKLiquidMapper.dictionaryOf(refundResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(rescanOnchainSwaps:reject:)
    func rescanOnchainSwaps(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            try getBindingLiquidSdk().rescanOnchainSwaps()
            resolve(["status": "ok"])
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

    @objc(recommendedFees:reject:)
    func recommendedFees(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBindingLiquidSdk().recommendedFees()
            resolve(BreezSDKLiquidMapper.dictionaryOf(recommendedFees: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(backup:resolve:reject:)
    func backup(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let backupRequest = try BreezSDKLiquidMapper.asBackupRequest(backupRequest: req)
            try getBindingLiquidSdk().backup(req: backupRequest)
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(restore:resolve:reject:)
    func restore(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let restoreRequest = try BreezSDKLiquidMapper.asRestoreRequest(restoreRequest: req)
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

    @objc(prepareLnurlPay:resolve:reject:)
    func prepareLnurlPay(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let prepareLnUrlPayRequest = try BreezSDKLiquidMapper.asPrepareLnUrlPayRequest(prepareLnUrlPayRequest: req)
            var res = try getBindingLiquidSdk().prepareLnurlPay(req: prepareLnUrlPayRequest)
            resolve(BreezSDKLiquidMapper.dictionaryOf(prepareLnUrlPayResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(lnurlPay:resolve:reject:)
    func lnurlPay(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let lnUrlPayRequest = try BreezSDKLiquidMapper.asLnUrlPayRequest(lnUrlPayRequest: req)
            var res = try getBindingLiquidSdk().lnurlPay(req: lnUrlPayRequest)
            resolve(BreezSDKLiquidMapper.dictionaryOf(lnUrlPayResult: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(lnurlWithdraw:resolve:reject:)
    func lnurlWithdraw(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let lnUrlWithdrawRequest = try BreezSDKLiquidMapper.asLnUrlWithdrawRequest(lnUrlWithdrawRequest: req)
            var res = try getBindingLiquidSdk().lnurlWithdraw(req: lnUrlWithdrawRequest)
            resolve(BreezSDKLiquidMapper.dictionaryOf(lnUrlWithdrawResult: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(lnurlAuth:resolve:reject:)
    func lnurlAuth(_ reqData: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let lnUrlAuthRequestData = try BreezSDKLiquidMapper.asLnUrlAuthRequestData(lnUrlAuthRequestData: reqData)
            var res = try getBindingLiquidSdk().lnurlAuth(reqData: lnUrlAuthRequestData)
            resolve(BreezSDKLiquidMapper.dictionaryOf(lnUrlCallbackStatus: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(registerWebhook:resolve:reject:)
    func registerWebhook(_ webhookUrl: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            try getBindingLiquidSdk().registerWebhook(webhookUrl: webhookUrl)
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(unregisterWebhook:reject:)
    func unregisterWebhook(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            try getBindingLiquidSdk().unregisterWebhook()
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(fetchFiatRates:reject:)
    func fetchFiatRates(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBindingLiquidSdk().fetchFiatRates()
            resolve(BreezSDKLiquidMapper.arrayOf(rateList: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(listFiatCurrencies:reject:)
    func listFiatCurrencies(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBindingLiquidSdk().listFiatCurrencies()
            resolve(BreezSDKLiquidMapper.arrayOf(fiatCurrencyList: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(setItem:value:resolve:reject:)
    func setItem(_ key: String, value: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            try getBindingLiquidSdk().setItem(key: key, value: value)
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(getItem:resolve:reject:)
    func getItem(_ key: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBindingLiquidSdk().getItem(key: key)
            if res != nil {
                resolve(res!)
            } else {
                resolve(nil)
            }
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(removeItem:resolve:reject:)
    func removeItem(_ key: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            try getBindingLiquidSdk().removeItem(key: key)
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
