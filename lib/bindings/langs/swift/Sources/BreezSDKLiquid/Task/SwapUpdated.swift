import UserNotifications
import Foundation

struct SwapUpdatedRequest: Codable {
    let id: String
    let status: String
}

class SwapUpdatedTask : TaskProtocol {
    fileprivate let TAG = "SwapUpdatedTask"
    
    internal var payload: String
    internal var contentHandler: ((UNNotificationContent) -> Void)?
    internal var bestAttemptContent: UNMutableNotificationContent?
    internal var logger: ServiceLogger
    internal var request: SwapUpdatedRequest? = nil
    internal var notified: Bool = false
    
    init(payload: String, logger: ServiceLogger, contentHandler: ((UNNotificationContent) -> Void)? = nil, bestAttemptContent: UNMutableNotificationContent? = nil) {
        self.payload = payload
        self.contentHandler = contentHandler
        self.bestAttemptContent = bestAttemptContent
        self.logger = logger
    }
    
    func start(liquidSDK: BindingLiquidSdk) throws {
        do {
            self.request = try JSONDecoder().decode(SwapUpdatedRequest.self, from: self.payload.data(using: .utf8)!)
        } catch let e {
            self.logger.log(tag: TAG, line: "Failed to decode payload: \(e)", level: "ERROR")
            self.onShutdown()
            throw e
        }
    }

    public func onEvent(e: SdkEvent) {
        if let swapIdHash = self.request?.id {
            switch e {
            case .paymentWaitingConfirmation(details: let payment), .paymentSucceeded(details: let payment):
                let swapId = self.getSwapId(details: payment.details)
                if swapIdHash == swapId?.sha256() {
                    self.logger.log(tag: TAG, line: "Received payment event: \(swapIdHash) \(payment.status)", level: "INFO")
                    self.notifySuccess(payment: payment)
                }
                break
            default:
                break
            }
        }
    }

    func getSwapId(details: PaymentDetails?) -> String? {
        if let details = details {
            switch details {
            case let .bitcoin(swapId, _, _, _):
                return swapId
            case let .lightning(swapId, _, _, _, _, _, _, _):
                return swapId
            default:
                break
            }
        }
        return nil
    }

    func onShutdown() {
        let notificationTitle = ResourceHelper.shared.getString(key: Constants.SWAP_CONFIRMED_NOTIFICATION_FAILURE_TITLE, fallback: Constants.DEFAULT_SWAP_CONFIRMED_NOTIFICATION_FAILURE_TITLE)
        let notificationBody = ResourceHelper.shared.getString(key: Constants.SWAP_CONFIRMED_NOTIFICATION_FAILURE_TEXT, fallback: Constants.DEFAULT_SWAP_CONFIRMED_NOTIFICATION_FAILURE_TEXT)
        self.displayPushNotification(title: notificationTitle, body: notificationBody, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_SWAP_UPDATED)
    }

    func notifySuccess(payment: Payment) {
        if !self.notified {
            self.logger.log(tag: TAG, line: "Payment \(payment.txId ?? "") processing successful", level: "INFO")
            let received = payment.paymentType == PaymentType.receive
            let notificationTitle = ResourceHelper.shared.getString(
                key: received ? Constants.PAYMENT_RECEIVED_NOTIFICATION_TITLE : Constants.PAYMENT_SENT_NOTIFICATION_TITLE, 
                validateContains: "%d", 
                fallback: received ? Constants.DEFAULT_PAYMENT_RECEIVED_NOTIFICATION_TITLE: Constants.DEFAULT_PAYMENT_SENT_NOTIFICATION_TITLE)
            self.notified = true
            self.displayPushNotification(title: String(format: notificationTitle, payment.amountSat), logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_SWAP_UPDATED)
        }
    }
}
