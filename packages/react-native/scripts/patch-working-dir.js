#!/usr/bin/env node
/**
 * This script patches the generated breez_sdk_liquid.js files to set workingDir
 * It modifies the defaultConfig function to:
 * 1. Store the result in a 'config' variable instead of returning directly
 * 2. Set config.workingDir using react-native-fs
 * 3. Return the modified config
 *
 * Note: The generated code is minified (all on one line), so we use regex replacement.
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

  // The generated code is minified. The defaultConfig function looks like:
  // function defaultConfig(network,breezApiKey)/*throws*/{return FfiConverterTypeConfig.lift(unifiCaller.rustCallWithError(...,FfiConverterString.lift));}
  // We need to:
  // 1. Change "return FfiConverterTypeConfig.lift(" to "const config = FfiConverterTypeConfig.lift("
  // 2. Change the ending "FfiConverterString.lift));};" to "FfiConverterString.lift))};var fs=require('react-native-fs');config.workingDir=`${fs.DocumentDirectoryPath}/breezSdkLiquid`;return config;};"

  // Pattern to match the defaultConfig function (handles both with and without spaces)
  const pattern = /(function defaultConfig\(network,\s*breezApiKey\)[^{]*\{)(return FfiConverterTypeConfig\.lift\()(.+?)(\,\s*\/\*liftString:\*\/FfiConverterString\.lift\)\);)(\})/gs;

  const replacement = '$1const config = FfiConverterTypeConfig.lift($3$4var fs=require(\'react-native-fs\');config.workingDir=`${fs.DocumentDirectoryPath}/breezSdkLiquid`;return config;$5';

  const newContent = content.replace(pattern, replacement);

  if (newContent === content) {
    console.error(`Error: Could not find pattern to patch in ${filePath}`);
    console.error('The generated code structure may have changed.');

    // Debug: show what we're looking for
    if (content.includes('function defaultConfig')) {
      const idx = content.indexOf('function defaultConfig');
      console.error('\nFound "function defaultConfig" at position', idx);
      console.error('Context (200 chars):');
      console.error(content.substring(idx, idx + 200));
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

  if (!content.includes('config.workingDir=')) {
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
