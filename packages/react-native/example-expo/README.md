# Breez Liquid SDK Expo Example

## Prerequisites
You need to set your Breez API key as the following environment property: `EXPO_PUBLIC_BREEZ_LIQUID_API_KEY`.
Ideally, the environment variable should be added to your EAS Project config as well.

## Expo managed workflow

This library includes a Expo Plugin that builds all the required notification services within [EAS managed workflow](https://docs.expo.dev/eas/). Make sure to add the Breez Liquid Plugin within your `app.config` as shown in this example.

### iOS Entitlements

The Breez Liquid Notification Service Extension requires specific entitlements that need to be added to the Application Provisioning Profile. The EAS Credentials tool is able to create and manage them for you after adding the entitlements configuration within the experimental section in `app.config` as show in this example.

Run the EAS CLI Credentials tool after adding the required configurations as shown above:

```bash
eas credentials
```

1. Select iOS (Android does not require extra entitlements).
2. Select desired profile.
3. Login with your Apple account.
4. Selected "Build Credentials". This will updated the provisioning profile with all required entitlements.
5. HODL.

## Build

Like any Expo Managed Workflow, you can build your app locally or within EAS Cloud service as long as you have all entitlements and provisioning profiles correctly set up.

### Cloud build

```bash
eas build
```

### Local build

```bash
eas build --local
```

### Unmanaged build

This project also includes a script to "pre-build" the app locally for un-managed deployment. This can be useful for adding additional native features to the Breez Notification Extension.

```bash
yarn prebuild:ios
```

```bash
yarn prebuild:android
```

## Roadmap

- [ ] Add logging capabilities to Notification Extension.

## Development

To develop the Breez Liquid SDK alongside the React Native module and example app, please read [DEVELOPING.md](../DEVELOPING.md) for details on how to setup your development environment.
