import UserNotifications

public protocol TaskProtocol : EventListener {
    var payload: String { get set }
    var contentHandler: ((UNNotificationContent) -> Void)? { get set }
    var bestAttemptContent: UNMutableNotificationContent? { get set }
    
    func start(liquidSDK: BindingLiquidSdk) throws
    func onShutdown()
}

extension TaskProtocol {
    func removePushNotifications(threadIdentifier: String, logger: ServiceLogger) {
        let semaphore = DispatchSemaphore(value: 0)
        let notificationCenter = UNUserNotificationCenter.current()

        notificationCenter.getDeliveredNotifications(completionHandler: { notifications in
            defer {
                semaphore.signal()
            }

            let removableNotifications = notifications.filter({ $0.request.content.threadIdentifier == threadIdentifier })
            guard !removableNotifications.isEmpty else {
                return
            }
            // The call to removeDeliveredNotifications() is async in a background thread and
            // needs to be complete before calling contentHandler()
            notificationCenter.removeDeliveredNotifications(withIdentifiers: removableNotifications.map({ $0.request.identifier }))
            logger.log(tag: "TaskProtocol", line:"removePushNotifications: \(removableNotifications.count)", level: "INFO")
        })

        semaphore.wait()
    }

    func displayPushNotification(title: String, body: String? = nil, logger: ServiceLogger, threadIdentifier: String? = nil) {
        logger.log(tag: "TaskProtocol", line:"displayPushNotification \(title)", level: "INFO")
        guard
            let contentHandler = contentHandler,
            let bestAttemptContent = bestAttemptContent
        else {
            return
        }

        removePushNotifications(threadIdentifier: Constants.NOTIFICATION_THREAD_REPLACEABLE, logger: logger)
        
        if let body = body {
            bestAttemptContent.body = body
        }

        if let threadIdentifier = threadIdentifier {
            bestAttemptContent.threadIdentifier = threadIdentifier
        }
        
        bestAttemptContent.title = title
        // The call to contentHandler() needs to be done with a slight delay otherwise
        // it will be killed before its finished removing the notifications
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) {
            contentHandler(bestAttemptContent)
        }
    }
}

class ReplyableTask : TaskProtocol {
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
        displayPushNotification(title: self.failNotificationTitle, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_REPLACEABLE)
    }
    
    func replyServer(encodable: Encodable, replyURL: String) {
        guard let serverReplyURL = URL(string: replyURL) else {
            self.displayPushNotification(title: self.failNotificationTitle, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_REPLACEABLE)
            return
        }
        var request = URLRequest(url: serverReplyURL)
        request.httpMethod = "POST"
        let encoder = JSONEncoder()
        encoder.outputFormatting = .withoutEscapingSlashes
        request.httpBody = try! encoder.encode(encodable)
        let task = URLSession.shared.dataTask(with: request) { data, response, error in
            let statusCode = (response as! HTTPURLResponse).statusCode
            
            if statusCode == 200 {
                self.displayPushNotification(title: self.successNotificationTitle, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_REPLACEABLE)
            } else {
                self.displayPushNotification(title: self.failNotificationTitle, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_REPLACEABLE)
                return
            }
        }
        task.resume()
    }
}
