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
    
    func start(liquidSDK: BindingLiquidSdk, pluginConfigs: PluginConfigs) throws {
        self.logger.log(tag: TAG, line: "start() called", level: "DEBUG")
        do {
            self.request = try JSONDecoder().decode(SwapUpdatedRequest.self, from: self.payload.data(using: .utf8)!)
            self.logger.log(tag: TAG, line: "Decoded request - id: \(self.request!.id), status: \(self.request!.status)", level: "DEBUG")
        } catch let e {
            self.logger.log(tag: TAG, line: "Failed to decode payload: \(e)", level: "ERROR")
            self.onShutdown()
            throw e
        }

        startPolling(liquidSDK: liquidSDK)
    }

    func startPolling(liquidSDK: BindingLiquidSdk) {
        self.logger.log(tag: TAG, line: "startPolling() - interval: \(pollingInterval)s", level: "DEBUG")
        var pollCount = 0
        pollingTimer = Timer.scheduledTimer(withTimeInterval: pollingInterval, repeats: true) { [weak self] _ in
            guard let self = self else { return }
            pollCount += 1
            do {
                guard let request = self.request else {
                    self.stopPolling(withError: NSError(domain: "SwapUpdatedTask", code: -1, userInfo: [NSLocalizedDescriptionKey: "Missing swap updated request"]))
                    return
                }

                self.logger.log(tag: TAG, line: "Polling #\(pollCount) - getPayment(swapId: \(request.id))", level: "DEBUG")
                let pollStartTime = Date()
                if let payment = try liquidSDK.getPayment(req: .swapId(swapId: request.id)) {
                    let pollElapsed = Date().timeIntervalSince(pollStartTime)
                    self.logger.log(tag: TAG, line: "getPayment() returned in \(String(format: "%.3f", pollElapsed))s - status: \(payment.status), txId: \(payment.txId ?? "nil")", level: "DEBUG")
                    switch payment.status {
                    case .complete:
                        self.logger.log(tag: TAG, line: "Payment complete, emitting paymentSucceeded event", level: "INFO")
                        onEvent(e: SdkEvent.paymentSucceeded(details: payment))
                        self.stopPolling()
                    case .waitingFeeAcceptance:
                        self.logger.log(tag: TAG, line: "Payment waiting fee acceptance", level: "INFO")
                        onEvent(e: SdkEvent.paymentWaitingFeeAcceptance(details: payment))
                        self.stopPolling()
                    case .pending:
                        if paymentClaimIsBroadcasted(details: payment.details) {
                            self.logger.log(tag: TAG, line: "Payment pending with claim broadcasted, emitting paymentWaitingConfirmation event", level: "INFO")
                            onEvent(e: SdkEvent.paymentWaitingConfirmation(details: payment))
                            self.stopPolling()
                        } else {
                            self.logger.log(tag: TAG, line: "Payment pending, claim not yet broadcasted, continuing to poll", level: "DEBUG")
                        }
                    default:
                        self.logger.log(tag: TAG, line: "Payment status: \(payment.status), continuing to poll", level: "DEBUG")
                        break
                    }
                } else {
                    let pollElapsed = Date().timeIntervalSince(pollStartTime)
                    self.logger.log(tag: TAG, line: "getPayment() returned nil in \(String(format: "%.3f", pollElapsed))s", level: "DEBUG")
                }
            } catch {
                self.stopPolling(withError: error)
            }
        }

        pollingTimer?.fire()
    }

    private func stopPolling(withError error: Error? = nil) {
        self.logger.log(tag: TAG, line: "stopPolling() called - hasError: \(error != nil)", level: "DEBUG")
        pollingTimer?.invalidate()
        pollingTimer = nil

        if let error = error {
            logger.log(tag: TAG, line: "Polling stopped with error: \(error)", level: "ERROR")
            onShutdown()
        } else {
            self.logger.log(tag: TAG, line: "Polling stopped successfully", level: "DEBUG")
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
            case let .bitcoin(swapId, _, _, _, _, _, _, _, _, _):
                return swapId
            case let .lightning(swapId, _, _, _, _, _, _, _, _, _, _, _, _, _):
                return swapId
            default:
                break
            }
        }
        return nil
    }

    func paymentClaimIsBroadcasted(details: PaymentDetails) -> Bool {
        switch details {
        case let .bitcoin(_, _, _, _, _, _, _, claimTxId, _, _):
            return claimTxId != nil
        case let .lightning(_, _, _, _, _, _, _, _, _, _, _, claimTxId, _, _):
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
