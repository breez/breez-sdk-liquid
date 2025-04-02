# Breez SDK - Nodeless *(Liquid Implementation)*

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
npm install @breeztech/breez-sdk-liquid
```
or

```bash
yarn add @breeztech/breez-sdk-liquid
```

## Usage
Head over to the [Breez SDK - Nodeless *(Liquid Implementation)* documentation](https://sdk-doc-liquid.breez.technology/) to start implementing Lightning in your app.

You'll need an API key to use the Breez SDK - Nodeless *(Liquid Implementation)*. To request an API key is free — you just need to [complete this simple form.](https://breez.technology/request-api-key/#contact-us-form-sdk)

### Web
When developing a browser application you should import `@breeztech/breez-sdk-liquid` (or the explicit `@breeztech/breez-sdk-liquid/web` submodule). 

It's important to first initialise the WebAssembly module by using `await init()` before making any other calls to the module.

```ts
import init, {
    connect,
    defaultConfig,
    SdkEvent
} from '@breeztech/breez-sdk-liquid/web'

// Initialise the WebAssembly module
await init()
```

### Node.js
When developing a node.js application you should require `@breeztech/breez-sdk-liquid` (or the explicit `@breeztech/breez-sdk-liquid/node` submodule).
```js
const { connect, defaultConfig, setLogger } = require('@breeztech/breez-sdk-liquid/node')
const { Command } = require('commander')
require('dotenv').config()

class JsEventListener {
    onEvent = (event) => {
        console.log(`EVENT RECEIVED: ${JSON.stringify(event)}`)
    }
}

class JsLogger {
    log = (logEntry) => {
        console.log(`[${logEntry.level}]: ${logEntry.line}`)
    }
}

const program = new Command()
const eventListener = new JsEventListener()
const logger = new JsLogger()

const initSdk = async () => {
    // Set the logger to trace
    setLogger(logger)

    // Get the mnemonic
    const breezApiKey = process.env.BREEZ_API_KEY
    const mnemonic = process.env.MNEMONIC

    // Connect using the config
    const config = await defaultConfig('mainnet', breezApiKey)
    console.log(`defaultConfig: ${JSON.stringify(config)}`)

    const sdk = await connect({ config, mnemonic })
    console.log(`connect`)

    const listenerId = await sdk.addEventListener(eventListener)
    console.log(`addEventListener: ${listenerId}`)
    return sdk
}

program.name('nodeless-wasm-cli').description('CLI for Breez SDK - Nodeless Wasm')

program.command('get-info').action(async () => {
    let sdk = await initSdk()
    let getInfoRes = await sdk.getInfo()
    console.log(`getInfo: ${JSON.stringify(getInfoRes)}`)
})

program.parse()
```

### Deno
When developing a Deno application you should import `@breeztech/breez-sdk-liquid` (or the explicit `@breeztech/breez-sdk-liquid/deno` submodule).

## Troubleshooting

- Node.js version 19 is the first version to add `global.crypto`. When using less than version 19 this has to be polyfilled.
