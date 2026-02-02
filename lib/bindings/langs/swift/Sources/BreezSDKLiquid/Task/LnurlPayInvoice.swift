import UserNotifications
import Foundation

struct LnurlInvoiceRequest: Codable {
    let amount: UInt64
    let comment: String?
    let reply_url: String
    let verify_url: String?
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

class LnurlPayInvoiceTask : LnurlPayTask {
    fileprivate let TAG = "LnurlPayInvoiceTask"
    
    init(payload: String, logger: ServiceLogger, contentHandler: ((UNNotificationContent) -> Void)? = nil, bestAttemptContent: UNMutableNotificationContent? = nil) {
        let successNotificationTitle = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_INVOICE_NOTIFICATION_TITLE, fallback: Constants.DEFAULT_LNURL_PAY_INVOICE_NOTIFICATION_TITLE)
        let failNotificationTitle = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_NOTIFICATION_FAILURE_TITLE, fallback: Constants.DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE)
        super.init(payload: payload, logger: logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent, successNotificationTitle: successNotificationTitle, failNotificationTitle: failNotificationTitle)
    }
    
    override func start(liquidSDK: BindingLiquidSdk, pluginConfigs: PluginConfigs) throws {
        self.logger.log(tag: TAG, line: "start() called", level: "DEBUG")
        var request: LnurlInvoiceRequest? = nil
        do {
            request = try JSONDecoder().decode(LnurlInvoiceRequest.self, from: self.payload.data(using: .utf8)!)
            self.logger.log(tag: TAG, line: "Decoded request - amount: \(request!.amount) msat, reply_url: \(request!.reply_url)", level: "DEBUG")
        } catch let e {
            self.logger.log(tag: TAG, line: "failed to decode payload: \(e)", level: "ERROR")
            self.displayPushNotification(title: self.failNotificationTitle, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_REPLACEABLE)
            throw e
        }

        do {
            // Get the lightning limits
            self.logger.log(tag: TAG, line: "Fetching lightning limits...", level: "DEBUG")
            let limitsStartTime = Date()
            let limits = try liquidSDK.fetchLightningLimits()
            let limitsElapsed = Date().timeIntervalSince(limitsStartTime)
            self.logger.log(tag: TAG, line: "fetchLightningLimits() completed in \(String(format: "%.3f", limitsElapsed))s - receive.minSat: \(limits.receive.minSat), receive.maxSat: \(limits.receive.maxSat)", level: "DEBUG")

            // Check amount is within limits
            let amountSat = request!.amount / UInt64(1000)
            self.logger.log(tag: TAG, line: "Checking amount: \(amountSat) sat against limits [\(limits.receive.minSat), \(limits.receive.maxSat)]", level: "DEBUG")
            if amountSat < limits.receive.minSat || amountSat > limits.receive.maxSat {
                self.logger.log(tag: TAG, line: "Amount \(amountSat) sat is outside limits", level: "ERROR")
                throw InvalidLnurlPayError.amount(amount: request!.amount)
            }
            // Check comment length
            if request!.comment?.count ?? 0 > Constants.LNURL_PAY_COMMENT_MAX_LENGTH {
                self.logger.log(tag: TAG, line: "Comment too long: \(request!.comment?.count ?? 0) > \(Constants.LNURL_PAY_COMMENT_MAX_LENGTH)", level: "ERROR")
                throw InvalidLnurlPayError.comment
            }
            let plainTextMetadata = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_METADATA_PLAIN_TEXT, fallback: Constants.DEFAULT_LNURL_PAY_METADATA_PLAIN_TEXT)
            let metadata = "[[\"text/plain\",\"\(plainTextMetadata)\"]]"
            let amount = ReceiveAmount.bitcoin(payerAmountSat: amountSat)

            self.logger.log(tag: TAG, line: "Calling prepareReceivePayment() for \(amountSat) sat...", level: "DEBUG")
            let prepareStartTime = Date()
            let prepareReceivePaymentRes = try liquidSDK.prepareReceivePayment(req: PrepareReceiveRequest(paymentMethod: PaymentMethod.bolt11Invoice, amount: amount))
            let prepareElapsed = Date().timeIntervalSince(prepareStartTime)
            self.logger.log(tag: TAG, line: "prepareReceivePayment() completed in \(String(format: "%.3f", prepareElapsed))s - feesSat: \(prepareReceivePaymentRes.feesSat)", level: "DEBUG")

            self.logger.log(tag: TAG, line: "Calling receivePayment()...", level: "DEBUG")
            let receiveStartTime = Date()
            let receivePaymentRes = try liquidSDK.receivePayment(req: ReceivePaymentRequest(prepareResponse: prepareReceivePaymentRes, description: metadata, descriptionHash: DescriptionHash.useDescription, payerNote: request!.comment))
            let receiveElapsed = Date().timeIntervalSince(receiveStartTime)
            self.logger.log(tag: TAG, line: "receivePayment() completed in \(String(format: "%.3f", receiveElapsed))s - destination length: \(receivePaymentRes.destination.count)", level: "DEBUG")

            // Add the verify URL
            var verify: String?
            if let verifyUrl = request!.verify_url {
                do {
                    let inputType = try liquidSDK.parse(input: receivePaymentRes.destination)
                    if case .bolt11(let invoice) = inputType {
                        verify = verifyUrl.replacingOccurrences(of: "{payment_hash}", with: invoice.paymentHash)
                        self.logger.log(tag: TAG, line: "Verify URL constructed with payment_hash: \(invoice.paymentHash)", level: "DEBUG")
                    }
                } catch let e {
                    self.logger.log(tag: TAG, line: "Failed to parse destination: \(e)", level: "ERROR")
                }
            }
            self.logger.log(tag: TAG, line: "Sending LnurlInvoiceResponse to reply_url", level: "DEBUG")
            self.replyServer(encodable: LnurlInvoiceResponse(pr: receivePaymentRes.destination, routes: [], verify: verify), replyURL: request!.reply_url)
        } catch let e {
            self.logger.log(tag: TAG, line: "failed to process lnurl: \(e)", level: "ERROR")
            self.fail(withError: e.localizedDescription, replyURL: request!.reply_url)
        }
    }
}
