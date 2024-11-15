import * as fs from "fs"
import * as path from "path"
import type { ExpoConfig } from "@expo/config"
import type { InfoPlist } from "expo/config-plugins"
import { withDangerousMod } from "expo/config-plugins"
import plist from "@expo/plist"

type EntitlementsProps = {
    targetName: string
    entitlements: Record<string, Array<string>>
}

export function withTargetEntitlementsPlist(config: ExpoConfig, { targetName, entitlements }: EntitlementsProps): ExpoConfig {
    return withDangerousMod(config, [
        "ios",
        (config) => {
            const extensionDir = path.join(config.modRequest.platformProjectRoot, targetName)
            if (!fs.existsSync(extensionDir)) {
                fs.mkdirSync(extensionDir)
            }
            const entitlementsPath = path.join(extensionDir, `${targetName}.entitlements`)

            const entitlementsPlist: InfoPlist = {
                ...entitlements
            }

            fs.writeFileSync(entitlementsPath, plist.build(entitlementsPlist))

            return config
        }
    ])
}
