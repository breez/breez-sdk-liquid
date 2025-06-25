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

class LnurlPayTask : ReplyableTask {
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
        self.displayPushNotification(title: title, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_REPLACEABLE)
    }
}

enum InvalidLnurlPayError: Error {
    case amount(amount: UInt64)
    case comment
    case minSendable
    case notFound
}

extension InvalidLnurlPayError: LocalizedError {
    public var errorDescription: String? {
        switch self {
        case .amount(amount: let amount):
            return NSLocalizedString("Invalid amount requested \(amount)", comment: "InvalidLnurlPayError")
        case .comment:
            return NSLocalizedString("Comment is too long", comment: "InvalidLnurlPayError")
        case .minSendable:
            return NSLocalizedString("Minimum sendable amount is invalid", comment: "InvalidLnurlPayError")
        case .notFound:
            return NSLocalizedString("Not found", comment: "InvalidLnurlPayError")
        }
    }
}
