import UserNotifications
import os.log

open class SDKNotificationService: UNNotificationServiceExtension {
    fileprivate let TAG = "SDKNotificationService"

    var liquidSDK: BindingLiquidSdk?
    var contentHandler: ((UNNotificationContent) -> Void)?
    var bestAttemptContent: UNMutableNotificationContent?
    var currentTask: TaskProtocol?
    public var logger: ServiceLogger = ServiceLogger(logStream: nil)

    override public init() { }

    override open func didReceive(
        _ request: UNNotificationRequest,
        withContentHandler contentHandler: @escaping (UNNotificationContent) -> Void
    ) {
        logger.log(tag: TAG, line: "Notification received - identifier: \(request.identifier)", level: "INFO")
        self.contentHandler = contentHandler
        bestAttemptContent = (request.content.mutableCopy() as? UNMutableNotificationContent)

        guard let connectRequest = getConnectRequest() else {
            logger.log(tag: TAG, line: "getConnectRequest() returned nil, delivering original content", level: "WARN")
            if let content = bestAttemptContent {
                contentHandler(content)
            }
            return
        }

        if let currentTask = getTaskFromNotification() {
            self.currentTask = currentTask

            DispatchQueue.main.async { [self] in
                do {
                    logger.log(tag: TAG, line: "Connecting to Breez Liquid SDK...", level: "INFO")
                    liquidSDK = try BreezSDKLiquidConnector.register(connectRequest: connectRequest, listener: currentTask)
                    logger.log(tag: TAG, line: "Breez Liquid SDK connected, starting task: \(type(of: currentTask))", level: "INFO")
                    try currentTask.start(liquidSDK: liquidSDK!, pluginConfigs: getPluginConfigs())
                } catch {
                    logger.log(tag: TAG, line: "Failed to process notification: \(error)", level: "ERROR")
                    shutdown()
                }
            }
        } else {
            logger.log(tag: TAG, line: "getTaskFromNotification() returned nil", level: "WARN")
        }
    }

    open func getConnectRequest() -> ConnectRequest? {
        return nil
    }

    open func getPluginConfigs() -> PluginConfigs {
        return PluginConfigs(nwc: nil)
    }

    open func getTaskFromNotification() -> TaskProtocol? {
        guard let content = bestAttemptContent else { return nil }
        let userInfo = content.userInfo["body"] as? NSDictionary ?? content.userInfo as NSDictionary
        guard let notificationType = userInfo[Constants.MESSAGE_DATA_TYPE] as? String else { return nil }
        logger.log(tag: TAG, line: "Notification payload: \(content.userInfo)", level: "INFO")
        logger.log(tag: TAG, line: "Notification type: \(notificationType)", level: "INFO")

        guard let payload = userInfo[Constants.MESSAGE_DATA_PAYLOAD] as? String else {
            contentHandler!(content)
            return nil
        }

        logger.log(tag: TAG, line: "\(notificationType) data string: \(payload)", level: "INFO")
        switch(notificationType) {
        case Constants.MESSAGE_TYPE_INVOICE_REQUEST:
            return InvoiceRequestTask(payload: payload, logger: logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent)
        case Constants.MESSAGE_TYPE_SWAP_UPDATED:
            return SwapUpdatedTask(payload: payload, logger: logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent)
        case Constants.MESSAGE_TYPE_LNURL_PAY_INFO:
            return LnurlPayInfoTask(payload: payload, logger: logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent)
        case Constants.MESSAGE_TYPE_LNURL_PAY_INVOICE:
            return LnurlPayInvoiceTask(payload: payload, logger: logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent)
        case Constants.MESSAGE_TYPE_LNURL_PAY_VERIFY:
            return LnurlPayVerifyTask(payload: payload, logger: logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent)
        case Constants.MESSAGE_TYPE_NWC_EVENT:
            return NwcEventTask(payload: payload, logger: logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent)
        default:
            return nil
        }
    }

    override open func serviceExtensionTimeWillExpire() {
        logger.log(tag: TAG, line: "serviceExtensionTimeWillExpire() - iOS is about to terminate the extension", level: "WARN")

        // iOS calls this function just before the extension will be terminated by the system.
        // Use this as an opportunity to deliver your "best attempt" at modified content,
        // otherwise the original push payload will be used.
        shutdown()
    }

    private func shutdown() -> Void {
        PluginManager.shutdown()
        BreezSDKLiquidConnector.unregister()
        currentTask?.onShutdown()
    }

    public func setServiceLogger(logger: Logger) {
        self.logger = ServiceLogger(logStream: logger)
        BreezSDKLiquidConnector.logger = self.logger
        PluginManager.logger = self.logger
    }
}
