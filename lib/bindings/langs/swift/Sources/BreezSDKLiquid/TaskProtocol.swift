import UserNotifications
import Foundation
import Security

/// Result of an HTTP request made through NSEURLSessionDelegate
struct NSEHTTPResult {
    let data: Data?
    let response: URLResponse?
    let error: Error?

    var httpResponse: HTTPURLResponse? {
        return response as? HTTPURLResponse
    }

    var statusCode: Int? {
        return httpResponse?.statusCode
    }

    var isSuccess: Bool {
        return error == nil && statusCode == 200
    }
}

/// Callback type for HTTP request completion
typealias NSEHTTPCallback = (NSEHTTPResult) -> Void

/// URLSession delegate that handles server trust evaluation for NSE
/// The NSE sandbox may not have full access to the system trust store,
/// so we embed the ISRG Root X1 certificate (Let's Encrypt's root CA) as a trust anchor.
///
/// IMPORTANT: We implement URLSessionDataDelegate to handle data tasks without completion handlers.
/// When using dataTask(with:) WITHOUT a completion handler, the delegate receives ALL callbacks
/// including authentication challenges. Using dataTask(with:completionHandler:) bypasses
/// the auth challenge delegate methods entirely, which is why we must use delegate-based handling.
class NSEURLSessionDelegate: NSObject, URLSessionDelegate, URLSessionDataDelegate {

    // ISRG Root X1 certificate (Let's Encrypt root CA) in DER format, base64 encoded
    // This certificate is used by breez.fun and many other services
    // Valid until 2035-06-04
    private static let isrgRootX1Base64 = """
        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
        """

    private static var isrgRootX1Certificate: SecCertificate? = {
        guard let certData = Data(base64Encoded: isrgRootX1Base64, options: .ignoreUnknownCharacters) else {
            return nil
        }
        return SecCertificateCreateWithData(nil, certData as CFData)
    }()

    // Thread-safe storage for task callbacks and data
    private let lock = NSLock()
    private var taskCallbacks: [Int: NSEHTTPCallback] = [:]
    private var taskData: [Int: Data] = [:]
    private var taskResponses: [Int: URLResponse] = [:]

    /// Register a callback for a task
    func registerCallback(for task: URLSessionTask, callback: @escaping NSEHTTPCallback) {
        lock.lock()
        defer { lock.unlock() }
        taskCallbacks[task.taskIdentifier] = callback
        taskData[task.taskIdentifier] = Data()
    }

    /// Clean up storage for a task
    private func cleanupTask(_ taskId: Int) {
        taskCallbacks.removeValue(forKey: taskId)
        taskData.removeValue(forKey: taskId)
        taskResponses.removeValue(forKey: taskId)
    }

    // MARK: - URLSessionDelegate

    /// Handle server trust evaluation at session level
    func urlSession(
        _ session: URLSession,
        didReceive challenge: URLAuthenticationChallenge,
        completionHandler: @escaping (URLSession.AuthChallengeDisposition, URLCredential?) -> Void
    ) {
        handleServerTrust(challenge: challenge, completionHandler: completionHandler)
    }

    // MARK: - URLSessionTaskDelegate

    /// Handle server trust evaluation for a specific task
    /// This is called for HTTPS connections when the server presents its certificate
    func urlSession(
        _ session: URLSession,
        task: URLSessionTask,
        didReceive challenge: URLAuthenticationChallenge,
        completionHandler: @escaping (URLSession.AuthChallengeDisposition, URLCredential?) -> Void
    ) {
        handleServerTrust(challenge: challenge, completionHandler: completionHandler)
    }

    /// Called when task completes (success or failure)
    func urlSession(_ session: URLSession, task: URLSessionTask, didCompleteWithError error: Error?) {
        lock.lock()
        let taskId = task.taskIdentifier
        let callback = taskCallbacks[taskId]
        let data = taskData[taskId]
        let response = taskResponses[taskId]
        cleanupTask(taskId)
        lock.unlock()

        let result = NSEHTTPResult(data: data, response: response, error: error)
        callback?(result)
    }

    // MARK: - URLSessionDataDelegate

    /// Called when response headers are received
    func urlSession(_ session: URLSession, dataTask: URLSessionDataTask, didReceive response: URLResponse, completionHandler: @escaping (URLSession.ResponseDisposition) -> Void) {
        lock.lock()
        taskResponses[dataTask.taskIdentifier] = response
        lock.unlock()
        completionHandler(.allow)
    }

    /// Called when data is received
    func urlSession(_ session: URLSession, dataTask: URLSessionDataTask, didReceive data: Data) {
        lock.lock()
        if taskData[dataTask.taskIdentifier] != nil {
            taskData[dataTask.taskIdentifier]?.append(data)
        }
        lock.unlock()
    }

    // MARK: - Server Trust Handling

    /// Common server trust evaluation logic
    private func handleServerTrust(
        challenge: URLAuthenticationChallenge,
        completionHandler: @escaping (URLSession.AuthChallengeDisposition, URLCredential?) -> Void
    ) {
        guard challenge.protectionSpace.authenticationMethod == NSURLAuthenticationMethodServerTrust,
              let serverTrust = challenge.protectionSpace.serverTrust else {
            completionHandler(.performDefaultHandling, nil)
            return
        }

        // Set SSL policy for the host
        let policy = SecPolicyCreateSSL(true, challenge.protectionSpace.host as CFString)
        SecTrustSetPolicies(serverTrust, policy)

        // Add ISRG Root X1 as a trusted anchor for Let's Encrypt certificates
        if let rootCert = NSEURLSessionDelegate.isrgRootX1Certificate {
            SecTrustSetAnchorCertificates(serverTrust, [rootCert] as CFArray)
            // Also trust the built-in anchors (system certificates)
            SecTrustSetAnchorCertificatesOnly(serverTrust, false)
        }

        var error: CFError?
        let isValid = SecTrustEvaluateWithError(serverTrust, &error)

        if isValid {
            let credential = URLCredential(trust: serverTrust)
            completionHandler(.useCredential, credential)
        } else {
            // Certificate validation failed - perform default handling which will reject
            completionHandler(.performDefaultHandling, nil)
        }
    }
}

public protocol TaskProtocol : EventListener {
    var payload: String { get set }
    var contentHandler: ((UNNotificationContent) -> Void)? { get set }
    var bestAttemptContent: UNMutableNotificationContent? { get set }

    func start(liquidSDK: BindingLiquidSdk, pluginConfigs: PluginConfigs) throws
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

    /// Creates a fresh URLSession for each request to avoid stale TLS session issues
    /// when the NSE process is reused across multiple notifications.
    ///
    /// The session is invalidated immediately after the request completes to release resources.
    internal static func createURLSession(delegate: NSEURLSessionDelegate) -> URLSession {
        let config = URLSessionConfiguration.ephemeral
        config.timeoutIntervalForRequest = 25
        config.timeoutIntervalForResource = 25
        // Ensure TLS 1.2+ is used
        config.tlsMinimumSupportedProtocolVersion = .TLSv12
        // Allow cellular and expensive networks in background
        config.allowsCellularAccess = true
        config.allowsExpensiveNetworkAccess = true
        config.allowsConstrainedNetworkAccess = true
        // Disable caching for fresh requests
        config.requestCachePolicy = .reloadIgnoringLocalCacheData
        config.urlCache = nil

        // Create dedicated queue for this session's delegate callbacks
        let delegateQueue = OperationQueue()
        delegateQueue.name = "com.breez.sdk.nse.urlsession"
        delegateQueue.maxConcurrentOperationCount = 1
        delegateQueue.underlyingQueue = DispatchQueue(label: "com.breez.sdk.nse.urlsession.dispatch")

        return URLSession(configuration: config, delegate: delegate, delegateQueue: delegateQueue)
    }

    init(payload: String, logger: ServiceLogger, contentHandler: ((UNNotificationContent) -> Void)? = nil, bestAttemptContent: UNMutableNotificationContent? = nil, successNotificationTitle: String, failNotificationTitle: String) {
        self.payload = payload
        self.contentHandler = contentHandler
        self.bestAttemptContent = bestAttemptContent
        self.logger = logger
        self.successNotificationTitle = successNotificationTitle;
        self.failNotificationTitle = failNotificationTitle;
    }

    func start(liquidSDK: BindingLiquidSdk, pluginConfigs: PluginConfigs) throws {}

    public func onEvent(e: SdkEvent) {}
    
    func onShutdown() {
        displayPushNotification(title: self.failNotificationTitle, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_REPLACEABLE)
    }
    
    func replyServer(encodable: Encodable, replyURL: String, maxAge: Int = 0) {
        guard let serverReplyURL = URL(string: replyURL) else {
            self.logger.log(tag: "ReplyableTask", line: "Invalid reply URL: \(replyURL)", level: "ERROR")
            self.displayPushNotification(title: self.failNotificationTitle, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_REPLACEABLE)
            return
        }
        var request = URLRequest(url: serverReplyURL)
        request.timeoutInterval = 25 // Leave buffer before iOS kills NSE at ~30s
        if maxAge > 0 {
            request.setValue("max-age=\(maxAge)", forHTTPHeaderField: "Cache-Control")
        }
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        let encoder = JSONEncoder()
        encoder.outputFormatting = .withoutEscapingSlashes
        request.httpBody = try! encoder.encode(encodable)

        self.logger.log(tag: "ReplyableTask", line: "Sending POST request to: \(replyURL)", level: "INFO")

        // Use semaphore to block until HTTP response is received
        // This prevents iOS from killing the NSE before we get the response
        let semaphore = DispatchSemaphore(value: 0)
        var httpSuccess = false

        // Create fresh delegate and session for each request
        // This avoids stale TLS session issues when NSE process is reused
        let delegate = NSEURLSessionDelegate()
        let session = ReplyableTask.createURLSession(delegate: delegate)

        // IMPORTANT: Use dataTask WITHOUT completion handler so that delegate methods
        // are called for authentication challenges. Using dataTask(with:completionHandler:)
        // bypasses all delegate methods including auth challenges, causing certificate
        // validation to fail in the NSE sandbox.
        let task = session.dataTask(with: request)

        // Register callback with delegate to receive result
        delegate.registerCallback(for: task) { [weak self] result in
            defer {
                semaphore.signal()
            }

            guard let self = self else { return }

            if let error = result.error {
                let nsError = error as NSError
                self.logger.log(tag: "ReplyableTask", line: "HTTP request failed: \(error.localizedDescription) (domain: \(nsError.domain), code: \(nsError.code))", level: "ERROR")
                // Log underlying error if available
                if let underlyingError = nsError.userInfo[NSUnderlyingErrorKey] as? NSError {
                    self.logger.log(tag: "ReplyableTask", line: "Underlying error: \(underlyingError.localizedDescription) (domain: \(underlyingError.domain), code: \(underlyingError.code))", level: "ERROR")
                }
                return
            }

            guard let statusCode = result.statusCode else {
                self.logger.log(tag: "ReplyableTask", line: "Invalid response type", level: "ERROR")
                return
            }

            self.logger.log(tag: "ReplyableTask", line: "Response status code: \(statusCode)", level: "INFO")

            if statusCode == 200 {
                httpSuccess = true
            } else {
                if let data = result.data, let responseBody = String(data: data, encoding: .utf8) {
                    let truncatedBody = String(responseBody.prefix(200))
                    self.logger.log(tag: "ReplyableTask", line: "Response body: \(truncatedBody)", level: "ERROR")
                }
            }
        }

        task.resume()

        // Wait for HTTP response (with timeout matching request timeout)
        let waitResult = semaphore.wait(timeout: .now() + 26)
        if waitResult == .timedOut {
            self.logger.log(tag: "ReplyableTask", line: "HTTP request timed out waiting for response", level: "ERROR")
        }

        // Invalidate session to release resources immediately
        // This ensures clean state for next request and prevents stale TLS sessions
        session.invalidateAndCancel()

        // Now display notification after we have the response
        if httpSuccess {
            self.displayPushNotification(title: self.successNotificationTitle, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_REPLACEABLE)
        } else {
            self.displayPushNotification(title: self.failNotificationTitle, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_REPLACEABLE)
        }
    }
}
