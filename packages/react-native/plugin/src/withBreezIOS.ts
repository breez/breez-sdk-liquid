import * as fs from "fs";
import * as path from "path";
import type { ExpoConfig } from "@expo/config";
import {
  withDangerousMod,
  withEntitlementsPlist,
  withPodfile,
  withXcodeProject,
} from "@expo/config-plugins";

import { addExtension } from "./addExtension";
import { withTargetPlist } from "./withExtensionInfoPlist";
import { warnOnce } from "./utils";
import { withTargetEntitlementsPlist } from "./withExtensionEntitlementsPlist";

const NOTIFICATION_SERVICE_PODS = (targetName: string) => `
target '${targetName}' do
  pod 'BreezSDKLiquid'
  pod 'KeychainAccess'
end
`;

type EASAppExtension = {
  targetName: string;
  bundleIdentifier: string;
  entitlements: Record<string, Array<string>>;
};

export type NotificationServiceExtensionProps = {
  apiKey: string;
  keyService: string;
  mnemonicKeyName: string;
}

export function withNotificationServiceExtension(config: ExpoConfig, props: NotificationServiceExtensionProps): ExpoConfig {
  const appExtensions = (config.extra?.eas?.build?.experimental?.ios
    ?.appExtensions ?? []) as EASAppExtension[];

  // Set all required variables

  if (appExtensions.length !== 1 || appExtensions[0] === undefined) {
    warnOnce("Missing App Extension Entry. Please refer to: https://docs.expo.dev/build-reference/app-extensions/");
    return config;
  }

  const { targetName, bundleIdentifier, entitlements } = appExtensions[0];
  const appGroup = entitlements["com.apple.security.application-groups"]?.[0];

  if (appGroup === undefined) {
    warnOnce("Missing Application Groups entitlement. The extension requires at least one App Group");
    return config;
  }

  // Copy required extension files
  config = withDangerousMod(config, [
    "ios",
    (config) => {
      const extensionDir = path.join(
        config.modRequest.platformProjectRoot,
        targetName,
      );
      if (!fs.existsSync(extensionDir)) {
        fs.mkdirSync(extensionDir);
      }

      // TODO: Add multiple sources
      const swiftSource = path.join(
        __dirname,
        "./../ios/BreezNotificationService.swift",
      );
      const swiftDest = path.join(
        extensionDir,
        "BreezNotificationService.swift",
      );

      fs.copyFileSync(swiftSource, swiftDest);

      return config;
    },
  ]);

  // Create Target Info Plist
  config = withTargetPlist(config, { targetName, appGroup, ...props });

  // Create Target Entitlements
  config = withTargetEntitlementsPlist(config, { targetName, entitlements });

  // Add the same entitlements to the main target
  config = withEntitlementsPlist(config, (config) => {
    config.modResults = { ...config.modResults, ...entitlements };
    return config;
  });

  // Add Notification Service Extension to XCode
  config = withXcodeProject(config, (config) => {
    const project = config.modResults;

    addExtension({ project, targetName, bundleIdentifier });
    return config;
  });

  // Add required Pods for the extension
  config = withPodfile(config, (config) => {
    const podFile = config.modResults;
    if (podFile.contents.includes(targetName)) {
      return config;
    }
    podFile.contents = podFile.contents.replace(
      /end[\s]*$/,
      `end\n${NOTIFICATION_SERVICE_PODS(targetName)}`,
    );
    return config;
  });

  return config;
}
