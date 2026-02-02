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
        self.logger.log(tag: "LnurlPayTask", line: "fail() called with error: \(withError)", level: "ERROR")

        if let serverReplyURL = URL(string: replyURL) {
            var request = URLRequest(url: serverReplyURL)
            request.timeoutInterval = 25
            request.httpMethod = "POST"
            request.httpBody = try! JSONEncoder().encode(LnurlErrorResponse(status: "ERROR", reason: withError))

            // Use semaphore to block until HTTP response is received
            let semaphore = DispatchSemaphore(value: 0)

            let task = URLSession.shared.dataTask(with: request) { data, response, error in
                defer {
                    semaphore.signal()
                }

                if let error = error {
                    self.logger.log(tag: "LnurlPayTask", line: "fail() HTTP request failed: \(error.localizedDescription)", level: "ERROR")
                    return
                }

                if let httpResponse = response as? HTTPURLResponse {
                    self.logger.log(tag: "LnurlPayTask", line: "fail() response status code: \(httpResponse.statusCode)", level: "INFO")
                }
            }
            task.resume()

            let waitResult = semaphore.wait(timeout: .now() + 26)
            if waitResult == .timedOut {
                self.logger.log(tag: "LnurlPayTask", line: "fail() HTTP request timed out", level: "ERROR")
            }
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
