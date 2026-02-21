import UserNotifications
import Foundation

struct NwcEventNotification: Codable {
    let event_id: String
}

class NwcEventTask: TaskProtocol, NwcEventListener {
    fileprivate let TAG = "NwcEventTask"

    internal var payload: String
    internal var contentHandler: ((UNNotificationContent) -> Void)?
    internal var bestAttemptContent: UNMutableNotificationContent?
    internal var logger: ServiceLogger
    internal var eventId: String? = nil

    init(payload: String, logger: ServiceLogger, contentHandler: ((UNNotificationContent) -> Void)? = nil, bestAttemptContent: UNMutableNotificationContent? = nil) {
        self.payload = payload
        self.contentHandler = contentHandler
        self.bestAttemptContent = bestAttemptContent
        self.logger = logger
    }

    func start(liquidSDK: BindingLiquidSdk, pluginConfigs: PluginConfigs) throws {
        guard let nwcService = try PluginManager.nwc(liquidSDK: liquidSDK, pluginConfigs: pluginConfigs) else {
            return
        }
        _ = nwcService.addEventListener(listener: self)
        do {
            let request = try JSONDecoder().decode(NwcEventNotification.self, from: payload.data(using: .utf8)!)
            eventId = request.event_id
            try nwcService.handleEvent(eventId: request.event_id)
        } catch let e {
            logger.log(tag: TAG, line: "Failed to process NWC event: \(e)", level: "ERROR")
            onShutdown()
            throw e
        }
    }

    func onEvent(e: SdkEvent) {}

    func onEvent(event: NwcEvent) {
        if let eventId = eventId {
            if event.eventId != eventId {
                return
            }
            let eventName: String
            switch event.details {
                case .getBalance:
                    eventName = "Get Balance"
                case .getInfo:
                    eventName = "Get Info"
                case .listTransactions:
                    eventName = "List Transactions"
                case .payInvoice:
                    eventName = "Pay Invoice"
                case .makeInvoice:
                    eventName = "Make Invoice"
                default:
                    return;
            }
            let notificationTitle = ResourceHelper.shared.getString(
                key: Constants.NWC_SUCCESS_NOTIFICATION_TITLE,
                validateContains: "%@",
                fallback: Constants.DEFAULT_NWC_SUCCESS_NOTIFICATION_TITLE
            )
            displayPushNotification(title: String(format: notificationTitle, eventName), logger: logger, threadIdentifier: Constants.NOTIFICATION_THREAD_DISMISSIBLE)
        }
    }

    func onShutdown() {
        let notificationTitle = ResourceHelper.shared.getString(
            key: Constants.NWC_FAILURE_NOTIFICATION_TITLE,
            fallback: Constants.DEFAULT_NWC_FAILURE_NOTIFICATION_TITLE
        )
        displayPushNotification(title: notificationTitle, logger: logger, threadIdentifier: Constants.NOTIFICATION_THREAD_DISMISSIBLE)
    }
}
