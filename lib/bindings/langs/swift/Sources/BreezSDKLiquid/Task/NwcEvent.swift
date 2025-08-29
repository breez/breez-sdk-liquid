import UserNotifications
import Foundation

struct NwcEventRequest: Decodable {
  let event_id: String
}

class NwcEventTask : TaskProtocol {
  fileprivate let TAG = "NwcEventTask"
  
  private var request: NwcEventRequest?
  internal var payload: String
  internal var contentHandler: ((UNNotificationContent) -> Void)?
  internal var bestAttemptContent: UNMutableNotificationContent?
  internal var logger: ServiceLogger
  
  init(payload: String, logger: ServiceLogger, contentHandler: ((UNNotificationContent) -> Void)? = nil, bestAttemptContent: UNMutableNotificationContent? = nil) {
    self.payload = payload
    self.contentHandler = contentHandler
    self.bestAttemptContent = bestAttemptContent
    self.logger = logger
  }
  
  func start(liquidSDK: BindingLiquidSdk) throws {
    do {
      let request = try JSONDecoder().decode(NwcEventRequest.self, from: self.payload.data(using: .utf8)!)
      self.logger.log(tag: TAG, line: "Starting SDK for NWC event with ID: \(request.event_id)", level: "INFO")
    } catch let e {
      self.logger.log(tag: TAG, line: "failed to decode payload: \(e)", level: "ERROR")
      self.onShutdown()
      throw e
    }
  }
  
  public func onEvent(e: SdkEvent) { }
  
  func onShutdown() { }
}
