import UserNotifications
import Foundation

struct InvoiceRequestRequest: Codable {
    let offer: String
    let invoice_request: String
    let reply_url: String
}

struct InvoiceRequestResponse: Decodable, Encodable {
    let invoice: String
    
    init(invoice: String) {
        self.invoice = invoice
    }
}

class InvoiceRequestTask : ReplyableTask {
    fileprivate let TAG = "InvoiceRequestTask"
    
    init(payload: String, logger: ServiceLogger, contentHandler: ((UNNotificationContent) -> Void)? = nil, bestAttemptContent: UNMutableNotificationContent? = nil) {
        let successNotificationTitle = ResourceHelper.shared.getString(key: Constants.INVOICE_REQUEST_NOTIFICATION_TITLE, fallback: Constants.DEFAULT_INVOICE_REQUEST_NOTIFICATION_TITLE)
        let failNotificationTitle = ResourceHelper.shared.getString(key: Constants.INVOICE_REQUEST_NOTIFICATION_FAILURE_TITLE, fallback: Constants.DEFAULT_INVOICE_REQUEST_NOTIFICATION_FAILURE_TITLE)
        super.init(payload: payload, logger: logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent, successNotificationTitle: successNotificationTitle, failNotificationTitle: failNotificationTitle)
    }
    
    override func start(liquidSDK: BindingLiquidSdk) throws {
        var request: InvoiceRequestRequest? = nil
        do {
            request = try JSONDecoder().decode(InvoiceRequestRequest.self, from: self.payload.data(using: .utf8)!)
        } catch let e {
            self.logger.log(tag: TAG, line: "failed to decode payload: \(e)", level: "ERROR")
            self.displayPushNotification(title: self.failNotificationTitle, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_REPLACEABLE)
            throw e
        }
        
        do {
            let prepareReceivePaymentRes = try liquidSDK.prepareReceivePayment(req: PrepareReceiveRequest(paymentMethod: PaymentMethod.bolt12Invoice, offer: request!.offer, invoiceRequest: request!.invoice_request))
            let receivePaymentRes = try liquidSDK.receivePayment(req: ReceivePaymentRequest(prepareResponse: prepareReceivePaymentRes))
            self.replyServer(encodable: InvoiceRequestResponse(invoice: receivePaymentRes.destination), replyURL: request!.reply_url)
        } catch let e {
            self.logger.log(tag: TAG, line: "failed to process invoice request: \(e)", level: "ERROR")
            self.displayPushNotification(title: self.failNotificationTitle, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_REPLACEABLE)
        }
    }
}
