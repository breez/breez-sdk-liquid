import UserNotifications
import Foundation

struct LnurlErrorResponse: Decodable, Encodable {
    let status: String
    let reason: String
    
    init(status: String, reason: String) {
        self.status = status
        self.reason = reason
    }
}

class LnurlPayTask : TaskProtocol {
    var payload: String
    var contentHandler: ((UNNotificationContent) -> Void)?
    var bestAttemptContent: UNMutableNotificationContent?
    var logger: ServiceLogger
    var successNotificationTitle: String
    var failNotificationTitle: String
    
    init(payload: String, logger: ServiceLogger, contentHandler: ((UNNotificationContent) -> Void)? = nil, bestAttemptContent: UNMutableNotificationContent? = nil, successNotificationTitle: String, failNotificationTitle: String) {
        self.payload = payload
        self.contentHandler = contentHandler
        self.bestAttemptContent = bestAttemptContent
        self.logger = logger
        self.successNotificationTitle = successNotificationTitle;
        self.failNotificationTitle = failNotificationTitle;
    }
    
    func start(liquidSDK: BindingLiquidSdk) throws {}

    public func onEvent(e: SdkEvent) {}
    
    func onShutdown() {
        displayPushNotification(title: self.failNotificationTitle, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_LNURL_PAY)
    }
    
    func replyServer(encodable: Encodable, replyURL: String) {
        guard let serverReplyURL = URL(string: replyURL) else {
            self.displayPushNotification(title: self.failNotificationTitle, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_LNURL_PAY)
            return
        }
        var request = URLRequest(url: serverReplyURL)
        request.httpMethod = "POST"
        request.httpBody = try! JSONEncoder().encode(encodable)
        let task = URLSession.shared.dataTask(with: request) { data, response, error in
            let statusCode = (response as! HTTPURLResponse).statusCode
            
            if statusCode == 200 {
                self.displayPushNotification(title: self.successNotificationTitle, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_LNURL_PAY)
            } else {
                self.displayPushNotification(title: self.failNotificationTitle, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_LNURL_PAY)
                return
            }
        }
        task.resume()
    }
    
    func fail(withError: String, replyURL: String, failNotificationTitle: String? = nil) {
        if let serverReplyURL = URL(string: replyURL) {
            var request = URLRequest(url: serverReplyURL)
            request.httpMethod = "POST"
            request.httpBody = try! JSONEncoder().encode(LnurlErrorResponse(status: "ERROR", reason: withError))
            let task = URLSession.shared.dataTask(with: request) { data, response, error in
                let _ = (response as! HTTPURLResponse).statusCode
            }
            task.resume()
        }
        let title = failNotificationTitle != nil ? failNotificationTitle! : self.failNotificationTitle
        self.displayPushNotification(title: title, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_LNURL_PAY)
    }
}

enum InvalidLnurlPayError: Error {
    case minSendable
    case amount(amount: UInt64)
}

extension InvalidLnurlPayError: LocalizedError {
    public var errorDescription: String? {
        switch self {
        case .minSendable:
            return NSLocalizedString("Minimum sendable amount is invalid", comment: "InvalidLnurlPayError")
        case .amount(amount: let amount):
            return NSLocalizedString("Invalid amount requested \(amount)", comment: "InvalidLnurlPayError")
        }
    }
}
