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
    
    override func start(liquidSDK: BindingLiquidSdk) throws {
        var request: LnurlInvoiceRequest? = nil
        do {
            request = try JSONDecoder().decode(LnurlInvoiceRequest.self, from: self.payload.data(using: .utf8)!)
        } catch let e {
            self.logger.log(tag: TAG, line: "failed to decode payload: \(e)", level: "ERROR")
            self.displayPushNotification(title: self.failNotificationTitle, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_REPLACEABLE)
            throw e
        }
        
        do {
            // Get the lightning limits
            let limits = try liquidSDK.fetchLightningLimits()
            // Check amount is within limits
            let amountSat = request!.amount / UInt64(1000)
            if amountSat < limits.receive.minSat || amountSat > limits.receive.maxSat {
                throw InvalidLnurlPayError.amount(amount: request!.amount)
            }
            // Check comment length
            if request!.comment?.count ?? 0 > Constants.LNURL_PAY_COMMENT_MAX_LENGTH {
                throw InvalidLnurlPayError.comment
            }
            let plainTextMetadata = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_METADATA_PLAIN_TEXT, fallback: Constants.DEFAULT_LNURL_PAY_METADATA_PLAIN_TEXT)
            let metadata = "[[\"text/plain\",\"\(plainTextMetadata)\"]]"
            let amount = ReceiveAmount.bitcoin(payerAmountSat: amountSat)
            let prepareReceivePaymentRes = try liquidSDK.prepareReceivePayment(req: PrepareReceiveRequest(paymentMethod: PaymentMethod.lightning, amount: amount))
            let receivePaymentRes = try liquidSDK.receivePayment(req: ReceivePaymentRequest(prepareResponse: prepareReceivePaymentRes, description: metadata, useDescriptionHash: true, payerNote: request!.comment))
            // Add the verify URL
            var verify: String?
            if let verifyUrl = request!.verify_url {
                do {
                    let inputType = try liquidSDK.parse(input: receivePaymentRes.destination)
                    if case .bolt11(let invoice) = inputType {
                        verify = verifyUrl.replacingOccurrences(of: "{payment_hash}", with: invoice.paymentHash)
                    }
                } catch let e {
                    self.logger.log(tag: TAG, line: "Failed to parse destination: \(e)", level: "ERROR")
                }
            }
            self.replyServer(encodable: LnurlInvoiceResponse(pr: receivePaymentRes.destination, routes: [], verify: verify), replyURL: request!.reply_url)
        } catch let e {
            self.logger.log(tag: TAG, line: "failed to process lnurl: \(e)", level: "ERROR")
            self.fail(withError: e.localizedDescription, replyURL: request!.reply_url)
        }
    }
}
