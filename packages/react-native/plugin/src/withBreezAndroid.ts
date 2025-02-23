import type { ExpoConfig } from "@expo/config"
import * as fs from "fs";
import * as path from "path";
import { withGradleProperties, withSettingsGradle, withDangerousMod } from "@expo/config-plugins"

export type AndroidConfigProps = {
    apiKey: string
    mnemonicKeyName: string
}

const EXTENSION_NAME = "breezsdkliquid_notifications";

const SETTINGS_GRADLE_LIB = "\ninclude ':breezsdkliquid_notifications'\nproject(':breezsdkliquid_notifications').projectDir = new File(rootProject.projectDir, 'libs/breezsdkliquid_notifications')"

export function withAndroidConfig(config: ExpoConfig, props: AndroidConfigProps): ExpoConfig {

  config = withDangerousMod(config, ["android", (config) => {
    const libraryDir = path.join(
      config.modRequest.platformProjectRoot,
      path.join("libs", EXTENSION_NAME)
    )
    if (!fs.existsSync(libraryDir)) {
      fs.mkdirSync(libraryDir, {recursive: true});
    }
    fs.cpSync(path.join(__dirname, "./../android"), libraryDir, { recursive: true})
    return config
    },
  ])

  config = withSettingsGradle(config, (config) => {
    const settings = config.modResults
    settings.contents += SETTINGS_GRADLE_LIB
    return config
  })

  config = withGradleProperties(config, (config) => {
    const gradleProperties = config.modResults;
    const {apiKey, mnemonicKeyName} = props

    gradleProperties.push({
      type: "property",
      key: "breezApiKey",
      value: apiKey,
    });

    gradleProperties.push({
      type: "property",
      key: "mnemonicKeyName",
      value: mnemonicKeyName,
    });

    return config
  })

  return config;
}
