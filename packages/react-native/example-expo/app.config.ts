import type { ConfigContext, ExpoConfig } from "expo/config"
export default function defineConfig({ config }: ConfigContext): ExpoConfig {
    const apiKey = process.env.EXPO_PUBLIC_BREEZ_LIQUID_API_KEY ?? ""
    return {
        ...config,
        name: "expo-breez-sdk-liquid-example",
        slug: "expo-breez-sdk-liquid-example",
        version: "1.0.0",
        orientation: "portrait",
        icon: "./assets/icon.png",
        userInterfaceStyle: "light",
        newArchEnabled: true,
        splash: {
            image: "./assets/splash-icon.png",
            resizeMode: "contain",
            backgroundColor: "#ffffff"
        },
        ios: {
            supportsTablet: true,
            bundleIdentifier: "com.breeztech.expo-breez-sdk-liquid-example"
        },
        android: {
            adaptiveIcon: {
                foregroundImage: "./assets/adaptive-icon.png",
                backgroundColor: "#ffffff"
            }
        },
        web: {
            favicon: "./assets/favicon.png"
        },
        extra: {
            eas: {
                build: {
                    experimental: {
                        ios: {
                            appExtensions: [
                                {
                                    targetName: "BreezNotificationService",
                                    bundleIdentifier: "com.breeztech.expo-breez-sdk-liquid-example.notificationextension",
                                    entitlements: {
                                        "com.apple.security.application-groups": ["group.com.breeztech.expo-breez-sdk-liquid-example"],
                                        "keychain-access-groups": ["$(AppIdentifierPrefix)sharedkey"]
                                    }
                                }
                            ]
                        }
                    }
                }
            }
        },
        plugins: [["@breeztech/react-native-breez-sdk-liquid", { apiKey }]]
    }
}
