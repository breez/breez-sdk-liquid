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

const NOTIFICATION_SERVICE_PODS = (targetName: string) => `
target '${targetName}' do
  pod 'BreezSDKLiquid'
  pod 'KeychainAccess'
end
`;

const NOTIFICATION_SERVICE_ENTITLEMENT = `
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
</dict>
</plist>
`;

type EASAppExtension = {
  targetName: string;
  bundleIdentifier: string;
  entitlements: Record<string, Array<string>>;
};

function generateEntitlementsXML(
  entitlements: Record<string, Array<string>>,
): string {
  const dictEntries = Object.entries(entitlements)
    .map(
      ([key, values]) => `
  <key>${key}</key>
  <array>
    ${values.map((value) => `<string>${value}</string>`).join("\n    ")}
  </array>`,
    )
    .join("\n");

  return NOTIFICATION_SERVICE_ENTITLEMENT.replace(
    "<dict>",
    `<dict>\n${dictEntries}`,
  );
}

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

      const entitlementsPath = path.join(
        extensionDir,
        `${targetName}.entitlements`,
      );
      if (!fs.existsSync(entitlementsPath)) {
        fs.writeFileSync(
          entitlementsPath,
          generateEntitlementsXML(entitlements),
        );
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
  // TODO: Support dynamic Pods
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
