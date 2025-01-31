import UserNotifications

public protocol TaskProtocol : EventListener {
    var payload: String { get set }
    var contentHandler: ((UNNotificationContent) -> Void)? { get set }
    var bestAttemptContent: UNMutableNotificationContent? { get set }
    
    func start(liquidSDK: BindingLiquidSdk) throws
    func onShutdown()
}

extension TaskProtocol {
    func removePushNotifications(threadIdentifier: String, logger: ServiceLogger) async -> Void {
        let notifications = await UNUserNotificationCenter.current().deliveredNotifications();
        let removableNotifications = notifications.filter({ $0.request.content.threadIdentifier == threadIdentifier });
        guard !removableNotifications.isEmpty else {
            return;
        }
        UNUserNotificationCenter.current().removeDeliveredNotifications(withIdentifiers: removableNotifications.map({ $0.request.identifier }));
    }

    func displayPushNotification(title: String, body: String? = nil, logger: ServiceLogger, threadIdentifier: String? = nil) {
        logger.log(tag: "TaskProtocol", line:"displayPushNotification \(title)", level: "INFO")
        guard
            let contentHandler = contentHandler,
            let bestAttemptContent = bestAttemptContent
        else {
            return
        }

        if let body = body {
            bestAttemptContent.body = body
        }

        if let threadIdentifier = threadIdentifier {
            bestAttemptContent.threadIdentifier = threadIdentifier
        }
        
        bestAttemptContent.title = title

        Task {
            await removePushNotifications(threadIdentifier: Constants.NOTIFICATION_THREAD_REPLACEABLE, logger: logger);
            contentHandler(bestAttemptContent);
        }
    }
}
