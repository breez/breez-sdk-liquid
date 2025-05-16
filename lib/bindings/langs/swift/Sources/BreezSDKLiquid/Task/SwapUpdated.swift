import UserNotifications
import Foundation

struct SwapUpdatedRequest: Codable {
    let id: String
    let status: String
}

class SwapUpdatedTask : TaskProtocol {
    fileprivate let TAG = "SwapUpdatedTask"
    
    private let pollingInterval: TimeInterval = 5.0
    
    internal var payload: String
    internal var contentHandler: ((UNNotificationContent) -> Void)?
    internal var bestAttemptContent: UNMutableNotificationContent?
    internal var logger: ServiceLogger
    internal var request: SwapUpdatedRequest? = nil
    internal var notified: Bool = false
    private var pollingTimer: Timer?
    
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

        startPolling(liquidSDK: liquidSDK)
    }

    func startPolling(liquidSDK: BindingLiquidSdk) {
        pollingTimer = Timer.scheduledTimer(withTimeInterval: pollingInterval, repeats: true) { [weak self] _ in
            guard let self = self else { return }
            do {
                guard let request = self.request else {
                    self.stopPolling(withError: NSError(domain: "SwapUpdatedTask", code: -1, userInfo: [NSLocalizedDescriptionKey: "Missing swap updated request"]))
                    return
                }

                if let payment = try liquidSDK.getPayment(req: .swapId(swapId: request.id)) {
                    switch payment.status {
                    case .complete:
                        onEvent(e: SdkEvent.paymentSucceeded(details: payment))
                        self.stopPolling()
                    case .waitingFeeAcceptance:
                        onEvent(e: SdkEvent.paymentWaitingFeeAcceptance(details: payment))
                        self.stopPolling()
                    case .pending:
                        if paymentClaimIsBroadcasted(details: payment.details) {
                            onEvent(e: SdkEvent.paymentWaitingConfirmation(details: payment))
                            self.stopPolling()
                        }
                    default:
                        break
                    }
                }
            } catch {
                self.stopPolling(withError: error)
            }
        }

        pollingTimer?.fire()
    }
    
    private func stopPolling(withError error: Error? = nil) {
        pollingTimer?.invalidate()
        pollingTimer = nil
        
        if let error = error {
            logger.log(tag: TAG, line: "Polling stopped with error: \(error)", level: "ERROR")
            onShutdown()
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
            case .paymentWaitingFeeAcceptance(details: let payment):
                let swapId = self.getSwapId(details: payment.details)
                if swapIdHash == swapId?.sha256() {
                    self.logger.log(tag: TAG, line: "Received payment event: \(swapIdHash) \(payment.status)", level: "INFO")
                    self.notifyPaymentWaitingFeeAcceptance(payment: payment)
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
            case let .bitcoin(swapId, _, _, _, _, _, _, _, _):
                return swapId
            case let .lightning(swapId, _, _, _, _, _, _, _, _, _, _, _, _):
                return swapId
            default:
                break
            }
        }
        return nil
    }

    func paymentClaimIsBroadcasted(details: PaymentDetails) -> Bool {
        switch details {
        case let .bitcoin(_, _, _, _, _, _, claimTxId, _, _):
            return claimTxId != nil
        case let .lightning(_, _, _, _, _, _, _, _, _, _, claimTxId, _, _):
            return claimTxId != nil
        default:
            return false
        }
    }

    func onShutdown() {
        let notificationTitle = ResourceHelper.shared.getString(key: Constants.SWAP_CONFIRMED_NOTIFICATION_FAILURE_TITLE, fallback: Constants.DEFAULT_SWAP_CONFIRMED_NOTIFICATION_FAILURE_TITLE)
        let notificationBody = ResourceHelper.shared.getString(key: Constants.SWAP_CONFIRMED_NOTIFICATION_FAILURE_TEXT, fallback: Constants.DEFAULT_SWAP_CONFIRMED_NOTIFICATION_FAILURE_TEXT)
        self.displayPushNotification(title: notificationTitle, body: notificationBody, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_DISMISSIBLE)
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
            self.displayPushNotification(title: String(format: notificationTitle, payment.amountSat), logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_DISMISSIBLE)
        }
    }
    
    func notifyPaymentWaitingFeeAcceptance(payment: Payment) {
        if !self.notified {
            self.logger.log(tag: TAG, line: "Payment \(self.getSwapId(details: payment.details) ?? "") requires fee acceptance", level: "INFO")
            let notificationTitle = ResourceHelper.shared.getString(
                key: Constants.PAYMENT_WAITING_FEE_ACCEPTANCE_TITLE,
                fallback: Constants.DEFAULT_PAYMENT_WAITING_FEE_ACCEPTANCE_TITLE)
            let notificationBody = ResourceHelper.shared.getString(
                key: Constants.PAYMENT_WAITING_FEE_ACCEPTANCE_TEXT,
                fallback: Constants.DEFAULT_PAYMENT_WAITING_FEE_ACCEPTANCE_TEXT)
            self.notified = true
            self.displayPushNotification(title: notificationTitle, body: notificationBody, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_DISMISSIBLE)
        }
    }
}
