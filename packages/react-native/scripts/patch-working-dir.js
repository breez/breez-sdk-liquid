#!/usr/bin/env node
/**
 * This script patches the generated breez_sdk_liquid.js files to set workingDir
 * It modifies the defaultConfig function to:
 * 1. Store the result in a 'config' variable instead of returning directly
 * 2. Set config.workingDir using react-native-fs
 * 3. Return the modified config
 */

const fs = require('fs');

const FILES = [
  'lib/commonjs/generated/breez_sdk_liquid.js',
  'lib/module/generated/breez_sdk_liquid.js'
];

function patchFile(filePath) {
  if (!fs.existsSync(filePath)) {
    console.error(`Error: ${filePath} not found`);
    process.exit(1);
  }

  let content = fs.readFileSync(filePath, 'utf8');

  // The generated code structure (formatted with newlines):
  // function defaultConfig(network, breezApiKey) /*throws*/{
  //   return FfiConverterTypeConfig.lift(uniffiCaller.rustCallWithError(...
  //   }, /*liftString:*/FfiConverterString.lift));
  // }
  //
  // We need to:
  // 1. Change "return FfiConverterTypeConfig.lift(" to "const config = FfiConverterTypeConfig.lift("
  // 2. Add the workingDir assignment and return statement before the closing brace

  // Pattern to match the defaultConfig function with multiline support
  // Using [\s\S] to match any character including newlines
  const pattern = /(function defaultConfig\(network,\s*breezApiKey\)\s*\/\*throws\*\/\s*\{\s*)(return)(\s+FfiConverterTypeConfig\.lift\([\s\S]+?\/\*liftString:\*\/FfiConverterString\.lift\)\);)(\s*\})/g;

  const replacement = '$1const config =$3\n  var fs = require(\'react-native-fs\');\n  config.workingDir = `${fs.DocumentDirectoryPath}/breezSdkLiquid`;\n  return config;$4';

  const newContent = content.replace(pattern, replacement);

  if (newContent === content) {
    console.error(`Error: Could not find pattern to patch in ${filePath}`);
    console.error('The generated code structure may have changed.');

    // Debug: show what we're looking for
    if (content.includes('function defaultConfig')) {
      const idx = content.indexOf('function defaultConfig');
      console.error('\nFound "function defaultConfig" at position', idx);
      console.error('Context (500 chars):');
      console.error(content.substring(idx, idx + 500));
    } else {
      console.error('\nCould not find "function defaultConfig" at all!');
    }
    process.exit(1);
  }

  fs.writeFileSync(filePath, newContent, 'utf8');
  console.log(`Patched ${filePath}`);
}

function verifyPatch(filePath) {
  const content = fs.readFileSync(filePath, 'utf8');
  let errors = 0;

  console.log(`Verifying ${filePath}...`);

  if (!content.includes('const config = FfiConverterTypeConfig.lift')) {
    console.error(`ERROR: Missing 'const config = FfiConverterTypeConfig.lift' in ${filePath}`);
    errors++;
  }

  if (!content.includes('config.workingDir')) {
    console.error(`ERROR: Missing 'config.workingDir' assignment in ${filePath}`);
    errors++;
  }

  if (!content.includes("require('react-native-fs'")) {
    console.error(`ERROR: Missing react-native-fs require in ${filePath}`);
    errors++;
  }

  if (errors > 0) {
    return false;
  }

  console.log(`Verification passed for ${filePath}`);
  return true;
}

// Main
console.log('Patching files...');
for (const file of FILES) {
  patchFile(file);
}

console.log('\nVerifying patches were applied correctly...');
let allPassed = true;
for (const file of FILES) {
  if (!verifyPatch(file)) {
    allPassed = false;
  }
}

if (!allPassed) {
  console.error('\nPatch verification failed!');
  process.exit(1);
}

console.log('\nWorking directory patch applied and verified successfully');
