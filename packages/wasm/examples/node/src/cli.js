const { connect, defaultConfig, setLogger } = require('@breeztech/breez-sdk-liquid/node')
const { Command, Option } = require('commander')
const fs = require('fs')
const qrcode = require('qrcode')
const { confirm, command } = require('./prompt.js');
const readline = require('readline');
const { parse } = require('shell-quote')
require('dotenv').config()

const logFile = fs.createWriteStream(__dirname + '/../sdk.log', {flags : 'a'})

class JsFileLogger {
    log = (logEntry) => {
        const logMessage = `[${new Date().toISOString()} ${logEntry.level}]: ${logEntry.line}\n`
        logFile.write(logMessage)
    }
}

const fileLogger = new JsFileLogger()

class JsEventListener {
    onEvent = (event) => {
        fileLogger.log({
            level: 'INFO',
            line: `Received event: ${JSON.stringify(event)}`
        })
    }
}

const eventListener = new JsEventListener()

let sdk = null

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
  terminal: true
})

const shutdown = () => {
    if (sdk) {
        sdk.disconnect();
    }
    rl.close()
    process.exit(0)
}

const initCommand = () => {
    const program = new Command()
    program.exitOverride()
    program.name('nodeless-wasm-cli').description('CLI for Breez SDK - Nodeless Wasm')
    
    program.command('exit').action(shutdown)

    program.command('get-info').action(async () => {
        const sdk = await initSdk()
        const res = await sdk.getInfo()
        console.log(JSON.stringify(res, null, 2))
    })
    
    program.command('list-payments').action(async () => {
        const sdk = await initSdk()
        const res = await sdk.listPayments({})
        console.log(JSON.stringify(res, null, 2))
    })
    
    program.command('receive-payment')
        .addOption(new Option('-m, --payment-method <choice>', 'The method to use when receiving')
            .makeOptionMandatory(true)
            .choices(['lightning', 'bitcoinAddress', 'liquidAddress']))
        .addOption(new Option('--amount-sat <number>', 'The amount the payer should send, in satoshi. If not specified, it will generate a BIP21 URI/address with no amount')
            .argParser(parseInt))
        .addOption(new Option('--asset <string>', 'Optional id of the asset to receive when the payment method is "liquidAddress"'))
        .addOption(new Option('--amount <number>', 'The amount the payer should send, in asset units. If not specified, it will generate a BIP21 URI/address with no amount. The asset id must also be provided').argParser(parseFloat))
        .action(async (options) => {
            const sdk = await initSdk()
            const prepareResponse = await sdk.prepareReceivePayment({
                paymentMethod: options.paymentMethod,
                amount: options.asset ? 
                    { type: 'asset', assetId: options.asset, payerAmount: options.amount } : 
                    { type: 'bitcoin', payerAmountSat: options.amountSat }
            })
            const fees = prepareResponse.feesSat
            const message = options.amount ? 
                `Fees: ${fees} sat + ${prepareResponse.swapperFeerate}% of the sent amount. Sender should send between ${prepareResponse.minPayerAmountSat} sat and ${prepareResponse.maxPayerAmountSat} sat. Are the fees acceptable?` :
                `Fees: ${fees} sat. Are the fees acceptable?`
            if (await confirm(rl, message)) {
                const res = await sdk.receivePayment({ prepareResponse })
                console.log(JSON.stringify(res, null, 2))
                qrcode.toString(res.destination, { type: 'terminal', small: true }, (err, url) => {
                    console.log(url)
                })
            }
        })

    program.command('send-payment')
        .addOption(new Option('-i, --invoice <string>', 'Invoice which has to be paid'))
        .addOption(new Option('-o, --offer <string>', 'BOLT12 offer. If specified, amount-sat must also be set'))
        .addOption(new Option('-a, --address <string>', 'Either BIP21 URI or Liquid address we intend to pay to'))
        .addOption(new Option('--amount-sat <number>', 'The amount the payer should send, in satoshi. If not specified, it will generate a BIP21 URI/address with no amount')
            .argParser(parseInt))
        .addOption(new Option('--asset <string>', 'Optional id of the asset to receive when the payment method is "liquidAddress"'))
        .addOption(new Option('--amount <number>', 'The amount the payer should send, in asset units. If not specified, it will generate a BIP21 URI/address with no amount. The asset id must also be provided').argParser(parseFloat))
        .addOption(new Option('-d --drain', 'Whether or not this is a drain operation. If true, all available funds will be used'))
        .action(async (options) => {
            const sdk = await initSdk()
            const destination = options.invoice || options.offer || options.address
            const amount = options.drain ? { type: 'drain' } : 
                options.asset ? { type: 'asset', assetId: options.asset, receiverAmount: options.amount } : 
                options.amountSat ? { type: 'bitcoin', receiverAmountSat: options.amountSat } : undefined
            if (!destination) {
                console.error('Please provide either an invoice, offer or address')
                return
            }
            if (options.offer && !options.amountSat) {
                console.error('Please provide an amount in satoshi')
                return
            }
            if (amount && amount.type === 'asset' && !amount.receiverAmount) {
                console.error('Please provide an amount in asset units')
                return
            }
            const prepareResponse = await sdk.prepareSendPayment({
                destination,
                amount
            })
            const message = `Fees: ${prepareResponse.feesSat} sat. Are the fees acceptable?`
            if (await confirm(rl, message)) {
                const res = await sdk.sendPayment({ prepareResponse })
                console.log(JSON.stringify(res, null, 2))
            }
        })

    return program
}

const initSdk = async () => {
    if (sdk) return sdk

    // Set the logger to trace
    setLogger(fileLogger)

    // Get the mnemonic
    const breezApiKey = process.env.BREEZ_API_KEY
    const mnemonic = process.env.MNEMONIC

    // Connect using the config
    const config = defaultConfig('mainnet', breezApiKey)

    sdk = await connect({ config, mnemonic })

    await sdk.addEventListener(eventListener)
    return sdk
}

const main = () => {
    return new Promise(async (resolve) => {
        while (true) {
            try {
                const res = await command(rl, 'sdk')
                if (res.trim().toLowerCase() === 'exit') {
                    shutdown()
                    resolve()
                    break
                } else {
                    const cmd = res.length > 0 ? res : '-h'
                    const program = initCommand()
                    await program.parseAsync(parse(cmd), { from: 'user' })   
                }
            } catch (e) {
                if (e.code !== 'commander.helpDisplayed') {
                    console.error('Error:', e)
                }
            }
        } 
    })   
}

main()
