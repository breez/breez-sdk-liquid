import UserNotifications
import os.log

open class SDKNotificationService: UNNotificationServiceExtension {
    fileprivate let TAG = "SDKNotificationService"

    // NSE memory limit is ~24MB - trigger graceful shutdown before iOS kills us
    private let memoryWarningThresholdMB: Double = 20.0
    private let memoryCriticalThresholdMB: Double = 22.0
    private var memoryMonitorTimer: DispatchSourceTimer?
    private var isShuttingDown = false

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
        self.logger.log(tag: TAG, line: "Notification received - identifier: \(request.identifier)", level: "INFO")
        self.contentHandler = contentHandler
        self.bestAttemptContent = (request.content.mutableCopy() as? UNMutableNotificationContent)

        // Start memory monitoring to prevent EXC_RESOURCE crashes
        startMemoryMonitor()

        guard let connectRequest = self.getConnectRequest() else {
            self.logger.log(tag: TAG, line: "getConnectRequest() returned nil, delivering original content", level: "WARN")
            if let content = bestAttemptContent {
                contentHandler(content)
            }
            return
        }
        self.logger.log(tag: TAG, line: "ConnectRequest obtained successfully", level: "DEBUG")

        if let currentTask = self.getTaskFromNotification() {
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

    // MARK: - Memory Monitoring

    private func startMemoryMonitor() {
        let timer = DispatchSource.makeTimerSource(queue: DispatchQueue.global(qos: .utility))
        timer.schedule(deadline: .now(), repeating: .milliseconds(500))
        timer.setEventHandler { [weak self] in
            self?.checkMemoryUsage()
        }
        timer.resume()
        memoryMonitorTimer = timer
        self.logger.log(tag: TAG, line: "Memory monitor started (warning: \(memoryWarningThresholdMB)MB, critical: \(memoryCriticalThresholdMB)MB)", level: "DEBUG")
    }

    private func stopMemoryMonitor() {
        memoryMonitorTimer?.cancel()
        memoryMonitorTimer = nil
    }

    private func checkMemoryUsage() {
        let memoryMB = currentMemoryUsageMB()

        if memoryMB >= memoryCriticalThresholdMB && !isShuttingDown {
            isShuttingDown = true
            self.logger.log(tag: TAG, line: "⚠️ CRITICAL: Memory at \(String(format: "%.1f", memoryMB))MB - initiating emergency shutdown to prevent EXC_RESOURCE crash", level: "ERROR")
            DispatchQueue.main.async { [weak self] in
                self?.emergencyShutdown(reason: "Memory limit exceeded (\(String(format: "%.1f", memoryMB))MB)")
            }
        } else if memoryMB >= memoryWarningThresholdMB {
            self.logger.log(tag: TAG, line: "⚠️ WARNING: Memory at \(String(format: "%.1f", memoryMB))MB - approaching limit", level: "WARN")
        }
    }

    private func currentMemoryUsageMB() -> Double {
        // Use task_vm_info with phys_footprint - this is what iOS uses for memory limits/jetsam
        var info = task_vm_info_data_t()
        var count = mach_msg_type_number_t(MemoryLayout<task_vm_info_data_t>.size / MemoryLayout<natural_t>.size)
        let result = withUnsafeMutablePointer(to: &info) {
            $0.withMemoryRebound(to: integer_t.self, capacity: Int(count)) {
                task_info(mach_task_self_, task_flavor_t(TASK_VM_INFO), $0, &count)
            }
        }
        if result == KERN_SUCCESS {
            return Double(info.phys_footprint) / (1024 * 1024)
        }
        return 0
    }

    private func emergencyShutdown(reason: String) {
        self.logger.log(tag: TAG, line: "Emergency shutdown: \(reason)", level: "ERROR")

        // Display notification about the issue
        if let content = bestAttemptContent {
            content.title = "Payment Processing Interrupted"
            content.body = "Please open the app to complete any pending operations."
            contentHandler?(content)
        }

        shutdown()
    }

    private func shutdown() -> Void {
        stopMemoryMonitor()
        self.logger.log(tag: TAG, line: "shutdown() started", level: "DEBUG")
        PluginManager.shutdown()
        self.logger.log(tag: TAG, line: "PluginManager.shutdown() completed", level: "DEBUG")
        BreezSDKLiquidConnector.unregister()
        self.logger.log(tag: TAG, line: "BreezSDKLiquidConnector.unregister() completed", level: "DEBUG")
        self.currentTask?.onShutdown()
        self.logger.log(tag: TAG, line: "shutdown() completed", level: "DEBUG")
    }
    
    public func setServiceLogger(logger: Logger) {
        self.logger = ServiceLogger(logStream: logger)
    }
}
