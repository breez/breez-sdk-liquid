import type { ExpoConfig } from 'expo/config';
import { createRunOncePlugin } from 'expo/config-plugins';
import { warnOnce } from './utils';
import { withNotificationServiceExtension } from './withBreezIOS';
const pkg = require("@breeztech/react-native-breez-sdk-liquid");

type PluginProps = {
  apiKey: string;
  keyService?: string;
  mnemonicKeyName?: string;
}

function withBreezPlugin(config: ExpoConfig, props?: PluginProps): ExpoConfig {
  const apiKey = props?.apiKey

  if(apiKey === undefined) {
    warnOnce("API Key not set.");
    return config;
  }

  const keyService = props?.keyService ?? "app:no-auth"; // This is the default name in expo-secure-store
  const mnemonicKeyName = props?.keyService ?? "mnemonic";

  // iOS Configuration
  config = withNotificationServiceExtension(config, { apiKey, keyService, mnemonicKeyName});

  // TODO: Android Configuration

  return config;
}

export default createRunOncePlugin(withBreezPlugin, pkg.name, pkg.version)
