import type { ExpoConfig } from "@expo/config"
import { createRunOncePlugin } from "@expo/config-plugins"
import { warnOnce, sdkPackage } from "./utils"
import { withNotificationServiceExtension } from "./withBreezIOS"
import { withAndroidConfig } from "./withBreezAndroid"

type PluginProps = {
    apiKey: string
    keyService?: string
    mnemonicKeyName?: string
}

function withBreezPlugin(config: ExpoConfig, props?: PluginProps): ExpoConfig {
    const apiKey = props?.apiKey

    if (apiKey === undefined) {
        warnOnce("API Key not set.")
        return config
    }

    const keyService = props?.keyService ?? "app:no-auth" // This is the default name in expo-secure-store
    const mnemonicKeyName = props?.keyService ?? "mnemonic"

    // iOS Configuration
    config = withNotificationServiceExtension(config, { apiKey, keyService, mnemonicKeyName })

    // Android Configuration
    config = withAndroidConfig(config, { apiKey, mnemonicKeyName})

    return config
}

export default createRunOncePlugin(withBreezPlugin, sdkPackage.name, sdkPackage.version)
