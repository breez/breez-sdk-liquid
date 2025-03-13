# Breez Liquid SDK Expo Example

## Prerequisites
You need to set your Breez API key as the following environment property: `EXPO_PUBLIC_BREEZ_LIQUID_API_KEY`.
Ideally, the environment variable should be added to your EAS Project config as well.

## Expo managed workflow

This library includes a Expo Plugin that builds all the required notification services within EAS managed workflow. Make sure to add the Breez Liquid Plugin within your `app.config` as shown in this example.

### iOS Entitlements

The Breez Liquid Notification Service Extension requires specific entitlements that need to be added to the Application Provisioning Profile. The EAS Credentials tool is able to create and manage them for you after adding the entitlements configuration within the experimental section in `app.config` as show in this example.

Run the EAS CLI Credentials tool after adding the required configurations as shown above:

```bash
eas credentials
```

Go through the credentials setup for each profile your managed workflow offers. EAS is able to create the entitlements and attach them to the corresponding provisioning profiles for you.

## Build

Like any Expo Managed Workflow, you can build your app locally or within EAS Cloud service as long as you have all entitlements and provisioning profiles correctly set up.


## Development

To develop the Breez Liquid SDK alongside the React Native module and example app, please read [DEVELOPING.md](../DEVELOPING.md) for details on how to setup your development environment.
