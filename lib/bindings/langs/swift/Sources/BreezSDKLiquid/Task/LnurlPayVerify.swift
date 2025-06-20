import UserNotifications
import Foundation

struct LnurlVerifyRequest: Codable {
    let payment_hash: String
    let reply_url: String
}

struct LnurlVerifyResponse: Decodable, Encodable {
    let status: String
    let settled: Bool
    let preimage: String?
    let pr: String
    
    init(settled: Bool, preimage: String?, pr: String) {
        self.status = "OK"
        self.settled = settled
        self.preimage = preimage
        self.pr = pr
    }
}

class LnurlPayVerifyTask : LnurlPayTask {
    fileprivate let TAG = "LnurlPayVerifyTask"
    
    init(payload: String, logger: ServiceLogger, contentHandler: ((UNNotificationContent) -> Void)? = nil, bestAttemptContent: UNMutableNotificationContent? = nil) {
        let successNotificationTitle = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_VERIFY_NOTIFICATION_TITLE, fallback: Constants.DEFAULT_LNURL_PAY_VERIFY_NOTIFICATION_TITLE)
        let failNotificationTitle = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_VERIFY_NOTIFICATION_FAILURE_TITLE, fallback: Constants.DEFAULT_LNURL_PAY_VERIFY_NOTIFICATION_FAILURE_TITLE)
        super.init(payload: payload, logger: logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent, successNotificationTitle: successNotificationTitle, failNotificationTitle: failNotificationTitle)
    }
    
    override func start(liquidSDK: BindingLiquidSdk) throws {
        var request: LnurlVerifyRequest? = nil
        do {
            request = try JSONDecoder().decode(LnurlVerifyRequest.self, from: self.payload.data(using: .utf8)!)
        } catch let e {
            self.logger.log(tag: TAG, line: "failed to decode payload: \(e)", level: "ERROR")
            self.displayPushNotification(title: self.failNotificationTitle, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_REPLACEABLE)
            throw e
        }
        
        do {
            // Get the payment by payment hash
            let getPaymentReq = GetPaymentRequest.paymentHash(paymentHash: request!.payment_hash)
            guard let payment = try liquidSDK.getPayment(req: getPaymentReq) else {
                throw InvalidLnurlPayError.notFound
            }
            var response: LnurlVerifyResponse? = nil
            switch payment.details {
                case let .lightning(_, _, _, preimage, invoice, _, _, _, _, _, _, claimTxId, _, _):
                    // In the case of a Lightning payment, if it's paid via Lightning or MRH,
                    // we can release the preimage
                    let settled = switch payment.status {
                        case .pending:
                            // If the payment is pending, we need to check if it's paid via Lightning or MRH
                            claimTxId != nil
                        case .complete:
                            true
                        default:
                            false
                    }
                    response = LnurlVerifyResponse(settled: settled, preimage: settled ? preimage : nil, pr: invoice!)
                default: 
                    break
            }
            if response == nil {
                throw InvalidLnurlPayError.notFound
            }
            let maxAge = response!.settled ? Constants.CACHE_CONTROL_MAX_AGE_WEEK : Constants.CACHE_CONTROL_MAX_AGE_THREE_SEC
            replyServer(encodable: response, replyURL: request!.reply_url, maxAge: maxAge)
        } catch let e {
            self.logger.log(tag: TAG, line: "failed to process lnurl verify: \(e)", level: "ERROR")
            fail(withError: e.localizedDescription, replyURL: request!.reply_url)
        }
    }
}
