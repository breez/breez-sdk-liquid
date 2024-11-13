import * as path from "path";
import type { XcodeProject } from "expo/config-plugins";

const isaXCBuildConfiguration = "XCBuildConfiguration";
const pbxTargetDependency = "PBXTargetDependency";
const pbxContainerItemProxy = "PBXContainerItemProxy";

type AddExtensionProps = {
  project: XcodeProject;
  targetName: string;
  bundleIdentifier: string;
};

export function addExtension({
  project,
  targetName,
  bundleIdentifier,
}: AddExtensionProps) {
  // Check if target already exists, if so, cancel

  const existing = project.hash.project.objects[isaXCBuildConfiguration];

  for (const [, config] of Object.entries(existing)) {
    if (typeof config === "string") continue;

    if (
      (config as any).buildSettings.PRODUCT_BUNDLE_IDENTIFIER &&
      (config as any).buildSettings.PRODUCT_BUNDLE_IDENTIFIER ===
        bundleIdentifier
    ) {
      return false;
    }
  }

  const targetUuid = project.generateUuid();

  const PRODUCT_BUNDLE_IDENTIFIER = bundleIdentifier;
  const INFOPLIST_FILE = `${targetName}/Info.plist`;
  const CODE_SIGN_ENTITLEMENTS = `${targetName}/${targetName}.entitlements`;

  // Create Build Configurations

  const commonBuildSettings = {
    CODE_SIGN_ENTITLEMENTS,
    CODE_SIGN_STYLE: "Automatic",
    DEBUG_INFORMATION_FORMAT: "dwarf",
    GCC_C_LANGUAGE_STANDARD: "gnu11",
    INFOPLIST_FILE,
    PRODUCT_BUNDLE_IDENTIFIER,
    PRODUCT_NAME: `"$(TARGET_NAME)"`,
    SKIP_INSTALL: "YES",
    SWIFT_VERSION: "5.0", // TODO: Use main target version
    TARGETED_DEVICE_FAMILY: `"1,2"`,
  };

  const buildConfigurationsList = [
    {
      name: "Debug",
      isa: isaXCBuildConfiguration,
      buildSettings: {
        ...commonBuildSettings,
        MTL_ENABLE_DEBUG_INFO: "INCLUDE_SOURCE",
      },
    },
    {
      name: "Release",
      isa: isaXCBuildConfiguration,
      buildSettings: {
        ...commonBuildSettings,
        COPY_PHASE_STRIP: "NO",
      },
    },
  ];

  const buildConfigurations = project.addXCConfigurationList(
    buildConfigurationsList,
    "Release",
    `Build configuration list for PBXNativeTarget "${targetName}"`,
  );

  // Create Product

  const productName = targetName;
  const productType = "com.apple.product-type.app-extension";
  const productFileType = '"wrapper.app-extension"';
  const productFile = project.addProductFile(productName, {
    group: "Embed Foundation Extensions",
    target: targetUuid,
    explicitFileType: productFileType,
  });

  productFile.settings = productFile.settings || {};
  productFile.settings.ATTRIBUTES = ["RemoveHeadersOnCopy"];

  project.addToPbxBuildFileSection(productFile);

  const strippedTargetName = path.basename(targetName, ".appex").trim();
  const quotedTargetName = `"${strippedTargetName}"`;

  // Create Target
  const target = {
    uuid: targetUuid,
    pbxNativeTarget: {
      isa: "PBXNativeTarget",
      name: strippedTargetName,
      productName: quotedTargetName,
      productReference: productFile.fileRef,
      productType: `"${productType}"`,
      buildConfigurationList: buildConfigurations.uuid,
      buildPhases: [],
      buildRules: [],
      dependencies: [],
    },
  };

  project.addToPbxNativeTargetSection(target);

  // Add Extension files
  const extensionGroup = project.addPbxGroup(
    [],
    targetName,
    targetName,
    '"<group>"',
  );
  const notificationServiceSwiftFile = project.addFile(
    "BreezNotificationService.swift",
    extensionGroup.uuid,
    {
      lastKnownFileType: "sourcecode.swift",
      defaultEncoding: 4,
      sourceTree: '"<group>"',
    },
  );

  notificationServiceSwiftFile.target = targetUuid;
  notificationServiceSwiftFile.uuid = project.generateUuid();

  project.addToPbxBuildFileSection(notificationServiceSwiftFile);
  const extensionSourcesBuildPhase = project.addBuildPhase(
    [],
    "PBXSourcesBuildPhase",
    "Sources",
    targetUuid,
  );
  extensionSourcesBuildPhase.buildPhase.files.push(
    notificationServiceSwiftFile.uuid,
  );

  const mainGroupUUID = project.getFirstProject().firstProject.mainGroup;
  const mainGroup = project.getPBXGroupByKey(mainGroupUUID);
  mainGroup.children.push({ value: extensionGroup.uuid, comment: targetName });

  // Create Build Phases
  const extensionCopyBuildPhase = project.addBuildPhase(
    [],
    "PBXCopyFilesBuildPhase",
    "Embed Foundation Extension",
    project.getFirstTarget().uuid,
    // targetType,
    "app_extension",
  );

  extensionCopyBuildPhase.buildPhase.dstSubfolderSpec = 13;

  addToPbxCopyfilesBuildPhase(
    project,
    productFile,
    "Embed Foundation Extension",
  );
  project.addBuildPhase([], "PBXResourcesBuildPhase", targetName, targetUuid);
  project.addToPbxProjectSection(target);

  if (!project.hash.project.objects[pbxTargetDependency]) {
    project.hash.project.objects[pbxTargetDependency] = {};
  }
  if (!project.hash.project.objects[pbxContainerItemProxy]) {
    project.hash.project.objects[pbxContainerItemProxy] = {};
  }

  project.addTargetDependency(project.getFirstTarget().uuid, [target.uuid]);

  return target;
}

function pbxBuildPhaseObj(file: any) {
  const obj = Object.create(null);

  obj.value = file.uuid;
  obj.comment = `${file.basename} in ${file.group}`;

  return obj;
}

function addToPbxCopyfilesBuildPhase(
  project: XcodeProject,
  file: any,
  name: string,
) {
  const sources = project.buildPhaseObject(
    "PBXCopyFilesBuildPhase",
    name || "Copy Files",
    file.target,
  );
  sources.files.push(pbxBuildPhaseObj(file));
}
