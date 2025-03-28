const { connect, defaultConfig, initLogger } = require('@breeztech/breez-sdk-liquid/node')
const { Command } = require('commander')
require('dotenv').config()

class JsEventListener {
    onEvent = (event) => {
        console.log(`EVENT RECEIVED: ${JSON.stringify(event)}`)
    }
}

const program = new Command()
const eventListener = new JsEventListener()

const initSdk = async () => {
    // Set the logger to trace
    initLogger('trace')

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
