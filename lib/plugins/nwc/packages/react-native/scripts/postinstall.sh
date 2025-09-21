#!/bin/sh
# Download prebuilt binary artifacts from the release
REPO=https://github.com/breez/breez-sdk-liquid-nwc-rn
TAG=$(node -p "require('./package.json').version")

ANDROID_URL=$REPO/releases/download/$TAG/android-artifacts.zip
curl -L $ANDROID_URL --output android-artifacts.zip
unzip -o android-artifacts.zip
rm -rf android-artifacts.zip

IOS_URL=$REPO/releases/download/$TAG/ios-artifacts.zip
curl -L $IOS_URL --output ios-artifacts.zip
unzip -o ios-artifacts.zip
rm -rf ios-artifacts.zip

