import UserNotifications
import Foundation

struct LnurlInvoiceRequest: Codable {
    let amount: UInt64
    let comment: String?
    let reply_url: String
    let verify_url: String?
    let nostr: String?
}

// Serialize the response according to:
// - LUD-06: https://github.com/lnurl/luds/blob/luds/06.md
// - LUD-21: https://github.com/lnurl/luds/blob/luds/21.md
struct LnurlInvoiceResponse: Decodable, Encodable {
    let pr: String
    let routes: [String]
    let verify: String?

    init(pr: String, routes: [String], verify: String?) {
        self.pr = pr
        self.routes = routes
        self.verify = verify
    }
}

class LnurlPayInvoiceTask : LnurlPayTask, NwcEventListener {
    fileprivate let TAG = "LnurlPayInvoiceTask"
    fileprivate let ZAP_TRACKING_TIMEOUT_SECS: Double = 120.0

    internal var zapTrackingInvoice: String? = nil
    private var zapReceiptTimeout: DispatchWorkItem? = nil

    init(payload: String, logger: ServiceLogger, contentHandler: ((UNNotificationContent) -> Void)? = nil, bestAttemptContent: UNMutableNotificationContent? = nil) {
        let successNotificationTitle = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_INVOICE_NOTIFICATION_TITLE, fallback: Constants.DEFAULT_LNURL_PAY_INVOICE_NOTIFICATION_TITLE)
        let failNotificationTitle = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_NOTIFICATION_FAILURE_TITLE, fallback: Constants.DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE)
        super.init(payload: payload, logger: logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent, successNotificationTitle: successNotificationTitle, failNotificationTitle: failNotificationTitle)
    }

    override func start(liquidSDK: BindingLiquidSdk, pluginConfigs: PluginConfigs) throws {
        var request: LnurlInvoiceRequest? = nil
        do {
            request = try JSONDecoder().decode(LnurlInvoiceRequest.self, from: payload.data(using: .utf8)!)
        } catch let e {
            logger.log(tag: TAG, line: "Failed to decode payload: \(e)", level: "ERROR")
            displayPushNotification(title: failNotificationTitle, logger: logger, threadIdentifier: Constants.NOTIFICATION_THREAD_REPLACEABLE)
            throw e
        }

        do {
            logger.log(tag: TAG, line: "Fetching lightning limits", level: "INFO")
            let limits = try liquidSDK.fetchLightningLimits()

            // Check amount is within limits
            let amountSat = request!.amount / UInt64(1000)
            if amountSat < limits.receive.minSat || amountSat > limits.receive.maxSat {
                logger.log(tag: TAG, line: "Amount \(amountSat) sat is outside limits [\(limits.receive.minSat), \(limits.receive.maxSat)]", level: "ERROR")
                throw InvalidLnurlPayError.amount(amount: request!.amount)
            }
            // Check comment length
            if request!.comment?.count ?? 0 > Constants.LNURL_PAY_COMMENT_MAX_LENGTH {
                logger.log(tag: TAG, line: "Comment too long: \(request!.comment?.count ?? 0) > \(Constants.LNURL_PAY_COMMENT_MAX_LENGTH)", level: "ERROR")
                throw InvalidLnurlPayError.comment
            }
            let plainTextMetadata = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_METADATA_PLAIN_TEXT, fallback: Constants.DEFAULT_LNURL_PAY_METADATA_PLAIN_TEXT)
            let metadata = "[[\"text/plain\",\"\(plainTextMetadata)\"]]"
            let amount = ReceiveAmount.bitcoin(payerAmountSat: amountSat)

            logger.log(tag: TAG, line: "Preparing receive payment for \(amountSat) sat", level: "INFO")
            let prepareReceivePaymentRes = try liquidSDK.prepareReceivePayment(req: PrepareReceiveRequest(paymentMethod: PaymentMethod.bolt11Invoice, amount: amount))

            logger.log(tag: TAG, line: "Creating invoice", level: "INFO")
            let receivePaymentRes = try liquidSDK.receivePayment(req: ReceivePaymentRequest(prepareResponse: prepareReceivePaymentRes, description: metadata, descriptionHash: DescriptionHash.useDescription, payerNote: request!.comment))
            // Add the verify URL
            var verify: String?
            if let verifyUrl = request!.verify_url {
                do {
                    let inputType = try liquidSDK.parse(input: receivePaymentRes.destination)
                    if case .bolt11(let invoice) = inputType {
                        verify = verifyUrl.replacingOccurrences(of: "{payment_hash}", with: invoice.paymentHash)
                    }
                } catch let e {
                    logger.log(tag: TAG, line: "Failed to parse destination for verify URL: \(e)", level: "WARN")
                }
            }
            logger.log(tag: TAG, line: "Sending invoice response", level: "INFO")
            replyServer(encodable: LnurlInvoiceResponse(pr: receivePaymentRes.destination, routes: [], verify: verify), replyURL: request!.reply_url)
            if let zapRequest = request!.nostr {
                do {
                    if let nwcService = try PluginManager.nwc(liquidSDK: liquidSDK, pluginConfigs: pluginConfigs) {
                        let _ = nwcService.addEventListener(listener: self)
                        zapTrackingInvoice = receivePaymentRes.destination
                        logger.log(tag: TAG, line: "Tracking zap for invoice: \(zapTrackingInvoice!)", level: "INFO")
                        try nwcService.trackZap(invoice: zapTrackingInvoice!, zapRequest: zapRequest)

                        zapReceiptTimeout = DispatchWorkItem { [weak self] in
                            guard let self = self else { return }
                            self.logger.log(tag: self.TAG, line: "Zap tracking timeout reached for invoice: \(self.zapTrackingInvoice ?? "nil")", level: "WARN")
                            self.zapTrackingInvoice = nil
                        }
                        DispatchQueue.main.asyncAfter(deadline: .now() + ZAP_TRACKING_TIMEOUT_SECS, execute: zapReceiptTimeout!)
                    }
                }
                catch let e {
                    logger.log(tag: TAG, line: "Failed to track zap: \(e)", level: "WARN")
                }
            }
        } catch let e {
            logger.log(tag: TAG, line: "Failed to process lnurl invoice: \(e)", level: "ERROR")
            fail(withError: e.localizedDescription, replyURL: request!.reply_url)
        }
    }

    func onEvent(event: NwcEvent) {
        if zapTrackingInvoice == nil {
            return
        }

        switch event.details {
            case .zapReceived(let invoice):
                if zapTrackingInvoice != invoice {
                    return
                }
                zapReceiptTimeout?.cancel()
                zapReceiptTimeout = nil
                logger.log(tag: TAG, line: "Successfully received zap for invoice: \(zapTrackingInvoice!)", level: "INFO")
            default:
                return
        }
    }
}
