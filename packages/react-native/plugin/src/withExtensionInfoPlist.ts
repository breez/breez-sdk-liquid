import * as fs from "fs"
import * as path from "path"
import type { ExpoConfig } from "@expo/config"
import type { InfoPlist } from "@expo/config-plugins"
import { IOSConfig, withDangerousMod } from "@expo/config-plugins"
import plist from "@expo/plist"
import { NotificationServiceExtensionProps } from "./withBreezIOS"

type InfoPlistProps = NotificationServiceExtensionProps & {
    targetName: string
    appGroup: string
}

export function withTargetPlist(config: ExpoConfig, { targetName, appGroup, apiKey, keyService, mnemonicKeyName }: InfoPlistProps): ExpoConfig {
    return withDangerousMod(config, [
        "ios",
        (config) => {
            const extensionDir = path.join(config.modRequest.platformProjectRoot, targetName)
            if (!fs.existsSync(extensionDir)) {
                fs.mkdirSync(extensionDir)
            }
            const infoPlistPath = path.join(extensionDir, "Info.plist")

            const targetPlist: InfoPlist = {
                CFBundleDevelopmentRegion: "$(DEVELOPMENT_LANGUAGE)",
                CFBundleExecutable: "$(EXECUTABLE_NAME)",
                CFBundleIdentifier: "$(PRODUCT_BUNDLE_IDENTIFIER)",
                CFBundleInfoDictionaryVersion: "6.0",
                CFBundleName: "$(PRODUCT_NAME)",
                CFBundlePackageType: "$(PRODUCT_BUNDLE_PACKAGE_TYPE)",
                CFBundleDisplayName: targetName,
                CFBundleVersion: IOSConfig.Version.getBuildNumber(config),
                CFBundleShortVersionString: IOSConfig.Version.getVersion(config),
                NSExtension: {
                    NSExtensionPointIdentifier: "com.apple.usernotifications.service",
                    NSExtensionPrincipalClass: "$(PRODUCT_MODULE_NAME).NotificationService"
                },
                AppGroup: appGroup,
                ApiKey: apiKey,
                KeyService: keyService,
                MnemonicKeyName: mnemonicKeyName
            }

            fs.writeFileSync(infoPlistPath, plist.build(targetPlist))

            return config
        }
    ])
}
