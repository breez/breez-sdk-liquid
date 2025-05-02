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

struct InvoiceErrorResponse: Decodable, Encodable {
    let error: String
    
    init(error: String) {
        self.error = error
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
            let createBolt12InvoiceRes = try liquidSDK.createBolt12Invoice(req: CreateBolt12InvoiceRequest(offer: request!.offer, invoiceRequest: request!.invoice_request))
            self.replyServer(encodable: InvoiceRequestResponse(invoice: createBolt12InvoiceRes.invoice), replyURL: request!.reply_url)
        } catch let e as PaymentError {
            self.logger.log(tag: TAG, line: "failed to process invoice request: \(e)", level: "ERROR")
            let error = e.localizedDescription
            self.notifyError(request: request, error: error)
        } catch let e {
            self.logger.log(tag: TAG, line: "failed to process invoice request: \(e)", level: "ERROR")
            self.notifyError(request: request, error: "Failed to process invoice request")
        } 
    }

    func notifyError(request: InvoiceRequestRequest?, error: String) {
        if request != nil {
            self.replyServer(encodable: InvoiceErrorResponse(error: error), replyURL: request!.reply_url)
        }
        self.displayPushNotification(title: self.failNotificationTitle, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_REPLACEABLE)
    }
}
