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
        self.logger.log(tag: TAG, line: "Notification received", level: "INFO")
        self.contentHandler = contentHandler
        self.bestAttemptContent = (request.content.mutableCopy() as? UNMutableNotificationContent)
                
        guard let connectRequest = self.getConnectRequest() else {
            if let content = bestAttemptContent {
                contentHandler(content)
            }
            return
        }
        
        if let currentTask = self.getTaskFromNotification() {
            self.currentTask = currentTask
            
            DispatchQueue.main.async { [self] in
                do {
                    logger.log(tag: TAG, line: "Breez Liquid SDK is not connected, connecting...", level: "INFO")
                    liquidSDK = try BreezSDKLiquidConnector.register(connectRequest: connectRequest, listener: currentTask)
                    logger.log(tag: TAG, line: "Breez Liquid SDK connected successfully", level: "INFO")
                    try currentTask.start(liquidSDK: liquidSDK!)
                } catch {
                    logger.log(tag: TAG, line: "Breez Liquid SDK connection failed \(error)", level: "ERROR")
                    shutdown()
                }
            }
        }
    }
    
    open func getConnectRequest() -> ConnectRequest? {
        return nil
    }
        
    open func getTaskFromNotification() -> TaskProtocol? {
        guard let content = bestAttemptContent else { return nil }
        guard let notificationType = content.userInfo[Constants.MESSAGE_DATA_TYPE] as? String else { return nil }
        self.logger.log(tag: TAG, line: "Notification payload: \(content.userInfo)", level: "INFO")
        self.logger.log(tag: TAG, line: "Notification type: \(notificationType)", level: "INFO")
        
        guard let payload = content.userInfo[Constants.MESSAGE_DATA_PAYLOAD] as? String else {
            contentHandler!(content)
            return nil
        }
        
        self.logger.log(tag: TAG, line: "\(notificationType) data string: \(payload)", level: "INFO")
        switch(notificationType) {
        case Constants.MESSAGE_TYPE_SWAP_UPDATED:
            return SwapUpdatedTask(payload: payload, logger: self.logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent)
        case Constants.MESSAGE_TYPE_LNURL_PAY_INFO:
            return LnurlPayInfoTask(payload: payload, logger: self.logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent)
        case Constants.MESSAGE_TYPE_LNURL_PAY_INVOICE:
            return LnurlPayInvoiceTask(payload: payload, logger: self.logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent)
        default:
            return nil
        }
    }
    
    override open func serviceExtensionTimeWillExpire() {
        self.logger.log(tag: TAG, line: "serviceExtensionTimeWillExpire()", level: "INFO")
        
        // iOS calls this function just before the extension will be terminated by the system.
        // Use this as an opportunity to deliver your "best attempt" at modified content,
        // otherwise the original push payload will be used.
        self.shutdown()
    }
    
    private func shutdown() -> Void {
        self.logger.log(tag: TAG, line: "shutting down...", level: "INFO")
        BreezSDKLiquidConnector.unregister()
        self.logger.log(tag: TAG, line: "task unregistered", level: "INFO")
        self.currentTask?.onShutdown()
    }
    
    public func setServiceLogger(logger: Logger) {
        self.logger = ServiceLogger(logStream: logger)
    }
}
