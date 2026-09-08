import type { ExpoConfig } from "@expo/config"
import { withAppBuildGradle, withGradleProperties } from "@expo/config-plugins"

import { resolvePackageVersion, warnOnce } from "./utils"

// Expo ships these as precompiled aars whose Maven coordinates don't match their npm names, so the
// mapping has to be spelled out.
const EXPO_MAVEN_ARTIFACTS: { packageName: string; property: string }[] = [
    { packageName: "expo-notifications", property: "expoNotificationsVersion" }
]

export type AndroidConfigProps = {
    apiKey: string
    mnemonicKeyName: string
    sdkVersion: string
}

const PACKAGING_MARKER = "breez-sdk-liquid packaging"

// The react-native package ships libbreez_sdk_liquid_bindings.so for its JSI bindings, and the Breez
// Kotlin bindings aar ships one under the same name for its JNA bindings. Both come from the same
// Rust core and export an identical UniFFI ABI, so either copy serves both callers. pickFirst drops
// the duplicate rather than shipping the core twice.
//
// Written by regex against the Groovy app/build.gradle that `expo prebuild` generates. Verified
// against the Expo SDK 57 template.
const PACKAGING_RULE = `    // ${PACKAGING_MARKER}
    packaging {
        jniLibs {
            pickFirsts += ["**/libbreez_sdk_liquid_bindings.so", "**/libc++_shared.so"]
        }
    }
`

type GradleProperty = { type: string; key?: string; value?: string }

function setGradleProperty(properties: GradleProperty[], key: string, value: string): void {
    const existing = properties.find((item) => item.type === "property" && item.key === key)
    if (existing) {
        existing.value = value
        return
    }
    properties.push({ type: "property", key, value })
}

/**
 * The Gradle project itself is autolinked from expo-module.config.json, so this only has to supply
 * the configuration it reads. Setting breezApiKey is also what switches the project on: without it
 * the notification service stays out of the build.
 */
export function withAndroidConfig(config: ExpoConfig, props: AndroidConfigProps): ExpoConfig {
    config = withAppBuildGradle(config, (config) => {
        const gradle = config.modResults

        if (gradle.language !== "groovy") {
            warnOnce(
                `Cannot add packaging rules to a ${gradle.language} app/build.gradle. Add pickFirsts for ` +
                    "libbreez_sdk_liquid_bindings.so and libc++_shared.so by hand."
            )
            return config
        }

        if (!gradle.contents.includes(PACKAGING_MARKER)) {
            gradle.contents = gradle.contents.replace(/^android\s*\{/m, (match) => `${match}\n${PACKAGING_RULE}`)
        }

        return config
    })

    config = withGradleProperties(config, (config) => {
        const gradleProperties = config.modResults
        const { apiKey, mnemonicKeyName, sdkVersion } = props

        setGradleProperty(gradleProperties, "breezApiKey", apiKey)
        setGradleProperty(gradleProperties, "mnemonicKeyName", mnemonicKeyName)
        setGradleProperty(gradleProperties, "breezSdkVersion", sdkVersion)

        for (const { packageName, property } of EXPO_MAVEN_ARTIFACTS) {
            const version = resolvePackageVersion(config.modRequest.projectRoot, packageName)
            if (version === undefined) {
                warnOnce(
                    `${packageName} is not installed. The notification service needs it — run 'npx expo install ${packageName}'.`
                )
                continue
            }
            setGradleProperty(gradleProperties, property, version)
        }

        return config
    })

    return config
}
