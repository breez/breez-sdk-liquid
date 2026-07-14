import KeychainAccess
import BreezSDKLiquid

enum NotificationServiceError: Error {
  case keychainValuesNotFound
  case infoDictionaryValueNotFound
  case containerUrlNotFound
}

class NotificationService: SDKNotificationService {

  override func getConnectRequest() -> ConnectRequest? {
    do {
      let keychainService = Bundle.main.infoDictionary?["KeyService"] as? String
      let keychainKeyMnemonic = Bundle.main.infoDictionary?["MnemonicKeyName"] as? String
      guard let keychainService, let keychainKeyMnemonic else {
        throw NotificationServiceError.infoDictionaryValueNotFound
      }
      
      let keychain = Keychain(service: keychainService)
      let mnemonic = try keychain.get(keychainKeyMnemonic)

      guard let mnemonic else {
        throw NotificationServiceError.keychainValuesNotFound
      }

      let apiKey = Bundle.main.infoDictionary?["ApiKey"] as? String
      let appGroup = Bundle.main.infoDictionary?["AppGroup"] as? String

      guard let apiKey, let appGroup else {
        throw NotificationServiceError.infoDictionaryValueNotFound
      }

      guard let containerURL = FileManager.default.containerURL(forSecurityApplicationGroupIdentifier: appGroup) else {
        throw NotificationServiceError.containerUrlNotFound
      }

      let workDir = containerURL.appendingPathComponent("breezSdkLiquid", isDirectory: true).path

      var config = try defaultConfig(network: LiquidNetwork.mainnet, breezApiKey: apiKey)
      config.workingDir = workDir

      return ConnectRequest(config: config, mnemonic: mnemonic)
    } catch {
      // TODO: Add logging
      print("BreezNotificationService: Error getting connect request: \(error.localizedDescription)")
      return nil
    }
  }
}
