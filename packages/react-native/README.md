# React Native Breez SDK - Nodeless *(Liquid Implementation)*

The Breez SDK provides developers with a end-to-end solution for integrating self-custodial Lightning payments into their apps and services. It eliminates the need for third parties, simplifies the complexities of Bitcoin and Lightning, and enables seamless onboarding for billions of users to the future of peer-to-peer payments.

## **What Is the Breez SDK - Nodeless *(Liquid Implementation)*?**

It’s a nodeless integration that offers a self-custodial, end-to-end solution for integrating Lightning payments, utilizing the Liquid Network with on-chain interoperability and third-party fiat on-ramps. Using the SDK you'll able to:

- **Send payments** via various protocols such as: Bolt11, Bolt12, BIP353, LNURL-Pay, Lightning address, BTC address
- **Receive payments** via various protocols such as: Bolt11, LNURL-Withdraw, LNURL-Pay, Lightning address, BTC address

**Key Features**

- [x] Send and receive Lightning payments
- [x] On-chain interoperability
- [x] Complete LNURL functionality
- [x] Multi-app support
- [x] Multi-device support
- [x] Real-time state backup
- [x] Keys are only held by users
- [x] USDT and multi-asset support on Liquid
- [x] Built-in fiat on-ramp
- [x] Free open-source solution

## Getting Started
```bash
npm install @breeztech/breez-sdk-liquid-react-native
```
or

```bash
yarn add @breeztech/breez-sdk-liquid-react-native
```

## Usage
Head over to the [Breez SDK - Nodeless *(Liquid Implementation)* documentation](https://sdk-doc-liquid.breez.technology/) to start implementing Lightning in your app.

You'll need an API key to use the Breez SDK - Nodeless *(Liquid Implementation)*. To request an API key is free — you just need to [complete this simple form.](https://breez.technology/request-api-key/#contact-us-form-sdk)

```ts
import React, { useEffect } from "react"
import {
    addEventListener,
    connect,
    defaultConfig,
    LiquidNetwork,
    NodeConfigVariant,
    prepareSendPayment,
    SdkEvent,
    sendPayment
} from "@breeztech/react-native-breez-sdk-liquid";
import BuildConfig from "react-native-build-config"

const App = () => (
    ...

    const eventHandler = (sdkEvent: SdkEvent) => {
        console.log(`${JSON.stringify(sdkEvent)}`)
    }

    const payInvoice = async (bolt11: string) => {
        // Pay invoice
        let prepareSendRes = await prepareSendPayment({ destination: bolt11 })
        let sendPaymentRes = await sendPayment({ prepareResponse: prepareSendRes })
    }

    useEffect(() => {
        const asyncFn = async () => {
            // Construct the sdk default config
            const config = await defaultConfig(LiquidNetwork.MAINNET, BuildConfig.BREEZ_API_KEY)

            // Connect to the Breez SDK make it ready to use
            await connect({ config, mnemonic })

            // Add event handler
            await addEventListener(eventHandler)
        }

        asyncFn()
    }, [])

    ...
)

export default App
```

## Troubleshooting
### Important fix for React Native versions below 0.71.0

If your project uses a React Native version less < 0.71.0, and you want to build your app for Android, you might run into an error like this:

```
 2 files found with path 'lib/arm64-v8a/libc++_shared.so' from inputs:
      - /(...)/.gradle/caches/transforms-3/c476ede63d070b991438fe0d1c323931/transformed/jetified-react-native-0.68.6/jni/arm64-v8a/libc++_shared.so
      - /(...)/.gradle/caches/transforms-3/7c318ac8dd87c1f0c7540616d6d47bd8/transformed/jetified-breez-sdk-0.1.3/jni/arm64-v8a/libc++_shared.so
```

To fix this you need to disambiguate which file to use by adding the following snippet to your app's `android/app/build.gradle`:

```gradle
android {
    // ...
    packagingOptions {
        pickFirst 'lib/x86/libc++_shared.so'
        pickFirst 'lib/x86_64/libc++_shared.so'
        pickFirst 'lib/armeabi-v7a/libc++_shared.so'
        pickFirst 'lib/arm64-v8a/libc++_shared.so'
    }
}
```

Both the Breez SDK as well as React Native package the `libc++_shared.so` native library.
React Native versions below 0.71.0 have a [bug](https://github.com/facebook/react-native/issues/30297) where they cannot automatically handle multiple versions of this file.
This has been [fixed](https://github.com/facebook/react-native/pull/35093) in React Native 0.71.0 and thus the above snippet only needs to be added to projects using React Native < 0.71.0.
