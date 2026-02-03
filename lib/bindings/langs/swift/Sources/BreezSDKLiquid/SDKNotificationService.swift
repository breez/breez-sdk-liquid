import UserNotifications
import os.log

open class SDKNotificationService: UNNotificationServiceExtension {
    fileprivate let TAG = "SDKNotificationService"

    var liquidSDK: BindingLiquidSdk?
    var contentHandler: ((UNNotificationContent) -> Void)?
    var bestAttemptContent: UNMutableNotificationContent?
    var currentTask: TaskProtocol?
    /// Tracks whether the task completed normally (vs being terminated by iOS)
    var taskCompletedNormally: Bool = false
    public var logger: ServiceLogger = ServiceLogger(logStream: nil)

    override public init() { }

    override open func didReceive(
        _ request: UNNotificationRequest,
        withContentHandler contentHandler: @escaping (UNNotificationContent) -> Void
    ) {
        self.logger.log(tag: TAG, line: "Notification received - identifier: \(request.identifier)", level: "INFO")
        self.contentHandler = contentHandler
        self.bestAttemptContent = (request.content.mutableCopy() as? UNMutableNotificationContent)

        guard let connectRequest = self.getConnectRequest() else {
            self.logger.log(tag: TAG, line: "getConnectRequest() returned nil, delivering original content", level: "WARN")
            if let content = bestAttemptContent {
                contentHandler(content)
            }
            return
        }
        self.logger.log(tag: TAG, line: "ConnectRequest obtained successfully", level: "DEBUG")

        if var currentTask = self.getTaskFromNotification() {
            // Set completion callback to cleanup resources when task finishes
            currentTask.onComplete = { [weak self] in
                self?.logger.log(tag: self?.TAG ?? "SDKNotificationService", line: "Task completed normally, shutting down", level: "INFO")
                self?.taskCompletedNormally = true
                self?.shutdown()
            }
            self.currentTask = currentTask
            self.logger.log(tag: TAG, line: "Task created: \(type(of: currentTask))", level: "DEBUG")

            DispatchQueue.main.async { [self] in
                do {
                    logger.log(tag: TAG, line: "Breez Liquid SDK is not connected, connecting...", level: "INFO")
                    let connectStartTime = Date()
                    liquidSDK = try BreezSDKLiquidConnector.register(connectRequest: connectRequest, listener: currentTask)
                    let connectElapsed = Date().timeIntervalSince(connectStartTime)
                    logger.log(tag: TAG, line: "Breez Liquid SDK connected successfully in \(String(format: "%.3f", connectElapsed))s", level: "INFO")

                    logger.log(tag: TAG, line: "Starting task: \(type(of: currentTask))", level: "DEBUG")
                    let taskStartTime = Date()
                    try currentTask.start(liquidSDK: liquidSDK!, pluginConfigs: getPluginConfigs())
                    let taskElapsed = Date().timeIntervalSince(taskStartTime)
                    logger.log(tag: TAG, line: "Task start() completed in \(String(format: "%.3f", taskElapsed))s", level: "DEBUG")
                } catch {
                    logger.log(tag: TAG, line: "Breez Liquid SDK connection failed \(error)", level: "ERROR")
                    shutdown()
                }
            }
        } else {
            self.logger.log(tag: TAG, line: "getTaskFromNotification() returned nil", level: "WARN")
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
        self.logger.log(tag: TAG, line: "Notification payload: \(content.userInfo)", level: "INFO")
        self.logger.log(tag: TAG, line: "Notification type: \(notificationType)", level: "INFO")
        
        guard let payload = userInfo[Constants.MESSAGE_DATA_PAYLOAD] as? String else {
            contentHandler!(content)
            return nil
        }
        
        self.logger.log(tag: TAG, line: "\(notificationType) data string: \(payload)", level: "INFO")
        switch(notificationType) {
        case Constants.MESSAGE_TYPE_INVOICE_REQUEST:
            return InvoiceRequestTask(payload: payload, logger: self.logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent)
        case Constants.MESSAGE_TYPE_SWAP_UPDATED:
            return SwapUpdatedTask(payload: payload, logger: self.logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent)
        case Constants.MESSAGE_TYPE_LNURL_PAY_INFO:
            return LnurlPayInfoTask(payload: payload, logger: self.logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent)
        case Constants.MESSAGE_TYPE_LNURL_PAY_INVOICE:
            return LnurlPayInvoiceTask(payload: payload, logger: self.logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent)
        case Constants.MESSAGE_TYPE_LNURL_PAY_VERIFY:
            return LnurlPayVerifyTask(payload: payload, logger: self.logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent)
        case Constants.MESSAGE_TYPE_NWC_EVENT:
            return NwcEventTask(payload: payload, logger: self.logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent)
        default:
            return nil
        }
    }
    
    override open func serviceExtensionTimeWillExpire() {
        self.logger.log(tag: TAG, line: "serviceExtensionTimeWillExpire() - iOS is about to terminate the extension", level: "WARN")

        // iOS calls this function just before the extension will be terminated by the system.
        // Use this as an opportunity to deliver your "best attempt" at modified content,
        // otherwise the original push payload will be used.
        self.shutdown()
    }

    private func shutdown() -> Void {
        self.logger.log(tag: TAG, line: "shutdown() started", level: "DEBUG")
        PluginManager.shutdown()
        self.logger.log(tag: TAG, line: "PluginManager.shutdown() completed", level: "DEBUG")
        BreezSDKLiquidConnector.unregister()
        self.logger.log(tag: TAG, line: "BreezSDKLiquidConnector.unregister() completed", level: "DEBUG")
        // Only call onShutdown if task didn't complete normally (e.g., iOS terminated us early)
        // This prevents showing "Receive Payment Failed" after successful task completion
        if !self.taskCompletedNormally {
            self.currentTask?.onShutdown()
        }
        self.logger.log(tag: TAG, line: "shutdown() completed", level: "DEBUG")
    }
    
    public func setServiceLogger(logger: Logger) {
        self.logger = ServiceLogger(logStream: logger)
    }
}
