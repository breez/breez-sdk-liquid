# Breez SDK - Nodeless *(Liquid Implementation)*

## **Overview**

The Breez SDK provides developers with a end-to-end solution for integrating self-custodial Lightning payments into their apps and services. It eliminates the need for third parties, simplifies the complexities of Bitcoin and Lightning, and enables seamless onboarding for billions of users to the future of peer-to-peer payments.

To provide the best experience for their end-users, developers can choose between the following implementations:

- [Breez SDK -  Nodeless *(Liquid Implementation)*](https://sdk-doc-liquid.breez.technology/)
- [Breez SDK - Native *(Greenlight Implementation)*](https://sdk-doc.breez.technology/)

**The Breez SDK is free for developers.**

## **What Is the Breez SDK - Nodeless *(Liquid Implementation)*?**

It’s a nodeless Lightning integration that offers a self-custodial, end-to-end solution for integrating Lightning payments, utilizing the Liquid Network with on-chain interoperability and third-party fiat on-ramps.

**Core Functions**

- **Sending payments** *via protocols such as: bolt11, lnurl-pay, lightning address, btc address.*
- **Receiving payments** *via protocols such as: bolt11, lnurl-withdraw, btc address.*
- **Interacting with a wallet** *e.g. balance, max allow to pay, max allow to receive, on-chain balance.*

**Key Features**

- [x]  On-chain interoperability
- [x]  LNURL functionality
- [x]  Multi-app support
- [x]  Multi-device support
- [x]  Real-time state backup
- [x]  Keys are only held by users
- [x]  Fiat on-ramps
- [x]  Open-source

## Getting Started 

Head over to the [Breez SDK - Nodeless *(Liquid Implementation)* documentation](https://sdk-doc-liquid.breez.technology/) to start implementing Lightning in your app.

You'll need an API key to use the Breez SDK - Nodeless *(Liquid Implementation)*. To request an API key is free — you just need to [complete this simple form.](https://breez.technology/request-api-key/#contact-us-form-sdk)

## **API**

API documentation is [here](https://breez.github.io/breez-sdk-liquid/breez_sdk_liquid/).

## **Command Line**

The [Breez SDK - Nodeless *(Liquid Implementation)* cli](https://github.com/breez/breez-sdk-liquid/tree/main/cli) is a command line client that allows you to interact with and test the functionality of the SDK.

## **Support**

Have a question for the team? Join our [Telegram channel](https://t.me/breezsdk) or email us at [contact@breez.technology](mailto:contact@breez.technology) 

## How Does Nodeless *(Liquid Implementation)* Work?

The Breez SDK - Nodeless *(Liquid Implementation)* uses submarine swaps and reverse submarine swaps to send and receive payments, enabling funds to move frictionlessly between the Lightning Network and the Liquid sidechain.

![Breez SDK - Liquid](https://github.com/breez/breez-sdk-liquid-docs/raw/main/src/images/BreezSDK_Liquid.png)

When sending a payment the SDK performs a submarine swap, converting L-BTC from a user’s Liquid wallet into sats on the Lightning Network, and sends them to the recipient.

When receiving a payment, the SDK performs a reverse submarine swap, converting incoming sats into L-BTC, and then deposits them in the user’s Liquid wallet.

## **Build & Test**

- **cli**:  Contains the Rust command line interface client for the SDK - *Liquid*.
- **lib**: Contains the root Rust cargo workspace.
    - **bindings**: The ffi bindings for Kotlin, Flutter, Python, React Native, and Swift.
    - **core**: The core SDK - *Liquid* rust library.
- **packages**: Contains the plugin packages for Dart, Flutter, and React Native.

Within each sub-project readme, there are instructions on how to build, test, and run.

## **SDK Development Roadmap**

- [x]  Send/Receive Lightning payments
- [x]  CLI Interface
- [x]  Foreign languages bindings
- [x]  Export/Import SDK data
- [x]  Pay BTC on-chain
- [x]  Receive via on-chain address
- [x]  LNURL-Pay
- [x]  LNURL-Withdraw
- [x]  Send to a Lightning address
- [x]  Receive via Lightning address
- [x]  Webhook for receiving payments
- [x]  Offline receive via notifications
- [x]  Offline swaps via notifications
- [ ]  Real-time sync
- [ ]  External input parsers
- [ ]  Bolt12
- [ ]  Amountless BTC swaps
- [ ]  Support USTD as an asset
- [ ]  TLV support
- [ ]  WebAssembly 
