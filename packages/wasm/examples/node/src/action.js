const { connect, defaultConfig, setLogger } = require('@breeztech/breez-sdk-liquid/node')
const { confirm, question } = require('./prompt.js')
const fs = require('fs')
const qrcode = require('qrcode')
require('dotenv').config()

const logFile = fs.createWriteStream(__dirname + '/../sdk.log', { flags: 'a' })

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

const initSdk = async () => {
    if (sdk) return sdk

    // Set the logger to trace
    setLogger(fileLogger)

    // Get the mnemonic
    const breezApiKey = process.env.BREEZ_API_KEY
    const mnemonic = process.env.MNEMONIC

    // Connect using the config
    let config = defaultConfig('mainnet', breezApiKey)
    config.workingDir = './.data'

    sdk = await connect({ config, mnemonic })

    await sdk.addEventListener(eventListener)
    return sdk
}

const disconnect = () => {
    if (sdk) {
        sdk.disconnect()
    }
    process.exit(0)
}

const buyBitcoin = async (provider, amountSat) => {
    const sdk = await initSdk()
    const prepareResponse = await sdk.prepareBuyBitcoin({
        provider,
        amountSat
    })
    const message = `Fees: ${prepareResponse.feesSat} sat. Are the fees acceptable?`
    if (await confirm(message)) {
        const res = await sdk.buyBitcoin({ prepareResponse })
        console.log(res)
        qrcode.toString(res, { type: 'terminal', small: true }, (err, url) => {
            console.log(url)
        })
    }
}

const checkMessage = async (message, pubkey, signature) => {
    const sdk = await initSdk()
    const res = await sdk.checkMessage({ message, pubkey, signature })
    console.log(JSON.stringify(res, null, 2))
}

const getInfo = async () => {
    const sdk = await initSdk()
    const res = await sdk.getInfo()
    console.log(JSON.stringify(res, null, 2))
}

const getPayment = async (options) => {
    const sdk = await initSdk()
    const req = options.paymentHash
        ? { type: 'paymentHash', paymentHash: options.paymentHash }
        : options.swapId
        ? { type: 'swapId', swapId: options.swapId }
        : undefined
    if (!req) {
        console.error('Please provide either a payment hash or swap id')
        return
    }
    const res = await sdk.getPayment(req)
    console.log(JSON.stringify(res, null, 2))
}

const fetchFiatRates = async () => {
    const sdk = await initSdk()
    const res = await sdk.fetchFiatRates()
    console.log(JSON.stringify(res, null, 2))
}

const fetchLightningLimits = async () => {
    const sdk = await initSdk()
    const res = await sdk.fetchLightningLimits()
    console.log(JSON.stringify(res, null, 2))
}

const fetchOnchainLimits = async () => {
    const sdk = await initSdk()
    const res = await sdk.fetchOnchainLimits()
    console.log(JSON.stringify(res, null, 2))
}

const listFiat = async () => {
    const sdk = await initSdk()
    const res = await sdk.listFiatCurrencies()
    console.log(JSON.stringify(res, null, 2))
}

const listPayments = async (options) => {
    const sdk = await initSdk()
    const res = await sdk.listPayments({
        filters: options.filter,
        states: options.state,
        fromTimestamp: options.fromTimestamp,
        toTimestamp: options.toTimestamp,
        limit: options.limit,
        offset: options.offset,
        details:
            options.asset || options.destination
                ? { type: 'liquid', assetId: options.asset, destination: options.destination }
                : options.address
                ? { type: 'bitcoin', address: options.address }
                : undefined,
        sortAscending: options.ascending
    })
    console.log(JSON.stringify(res, null, 2))
}

const listRefundables = async () => {
    const sdk = await initSdk()
    const res = await sdk.listRefundables()
    console.log(JSON.stringify(res, null, 2))
}

const lnurlAuth = async (lnurl) => {
    const sdk = await initSdk()
    const res = await sdk.parse(lnurl)
    if (res.type === 'lnUrlAuth') {
        const authRes = await sdk.lnurlAuth(res.data)
        console.log(JSON.stringify(authRes, null, 2))
    } else {
        console.log('Not a valid LNURL-auth')
    }
}

const lnurlPay = async (lnurl, options) => {
    const sdk = await initSdk()
    const res = await sdk.parse(lnurl)
    if (res.type === 'lnUrlPay') {
        const data = res.data
        let amount = options.drain ? { type: 'drain' } : { type: 'bitcoin', receiverAmountSat: 0 }
        if (!options.drain) {
            const minSendable = Math.ceil(data.minSendable / 1000.0)
            const maxSendable = Math.floor(data.maxSendable / 1000.0)
            const message = `Amount to pay (min ${minSendable} sat, max ${maxSendable} sat)`
            const receiverAmountSat = await question(message, parseInt)
            amount = { type: 'bitcoin', receiverAmountSat }
        }
        const prepareResponse = await sdk.prepareLnurlPay({
            data,
            amount,
            bip353Address: res.bip353Address,
            validateSuccessActionUrl: options.validate
        })
        const message = `Fees: ${prepareResponse.feesSat} sat. Are the fees acceptable?`
        if (await confirm(message)) {
            const payRes = await sdk.lnurlPay({ prepareResponse })
            console.log(JSON.stringify(res, null, 2))
        }
    } else {
        console.log('Not a valid LNURL-pay')
    }
}

const lnurlWithdraw = async (lnurl) => {
    const sdk = await initSdk()
    const res = await sdk.parse(lnurl)
    if (res.type === 'lnUrlWithdraw') {
        const data = res.data
        const message = `Amount to withdraw in millisatoshi (min ${data.minWithdrawable} msat, max ${data.maxWithdrawable} msat)`
        const amountMsat = await question(message, parseInt)
        const withdrawRes = await sdk.lnurlWithdraw({ data, amountMsat, description: 'LNURL-withdraw' })
        console.log(JSON.stringify(withdrawRes, null, 2))
    } else {
        console.log('Not a valid LNURL-withdraw')
    }
}

const parse = async (input) => {
    const sdk = await initSdk()
    const res = await sdk.parse(input)
    console.log(JSON.stringify(res, null, 2))
}

const prepareRefund = async (swapAddress, refundAddress, feeRateSatPerVbyte) => {
    const sdk = await initSdk()
    const res = await sdk.prepareRefund({
        swapAddress,
        refundAddress,
        feeRateSatPerVbyte
    })
    console.log(JSON.stringify(res, null, 2))
}

const receivePayment = async (options) => {
    const sdk = await initSdk()
    const prepareResponse = await sdk.prepareReceivePayment({
        paymentMethod: options.paymentMethod,
        amount: options.asset
            ? { type: 'asset', assetId: options.asset, payerAmount: options.amount }
            : { type: 'bitcoin', payerAmountSat: options.amountSat }
    })
    const fees = prepareResponse.feesSat
    const message = options.amount
        ? `Fees: ${fees} sat + ${prepareResponse.swapperFeerate}% of the sent amount. Sender should send between ${prepareResponse.minPayerAmountSat} sat and ${prepareResponse.maxPayerAmountSat} sat. Are the fees acceptable?`
        : `Fees: ${fees} sat. Are the fees acceptable?`
    if (await confirm(message)) {
        const res = await sdk.receivePayment({ prepareResponse })
        console.log(JSON.stringify(res, null, 2))
        qrcode.toString(res.destination, { type: 'terminal', small: true }, (err, url) => {
            console.log(url)
        })
    }
}

const recommendedFees = async () => {
    const sdk = await initSdk()
    const res = await sdk.recommendedFees()
    console.log(JSON.stringify(res, null, 2))
}

const refund = async (swapAddress, refundAddress, feeRateSatPerVbyte) => {
    const sdk = await initSdk()
    const res = await sdk.refund({
        swapAddress,
        refundAddress,
        feeRateSatPerVbyte
    })
    console.log(JSON.stringify(res, null, 2))
}

const registerWebhook = async (url) => {
    const sdk = await initSdk()
    const res = await sdk.registerWebhook(url)
    console.log(JSON.stringify(res, null, 2))
}

const rescanOnchainSwaps = async () => {
    const sdk = await initSdk()
    const res = await sdk.rescanOnchainSwaps()
    console.log(JSON.stringify(res, null, 2))
}

const reviewPaymentProposedFees = async (swapId) => {
    const sdk = await initSdk()
    const response = await sdk.fetchPaymentProposedFees({ swapId })
    const message = `Payer amount: ${response.payerAmountSat} sat. Fees: ${response.feesSat} sat. Resulting received amount: ${response.receiverAmountSat} sat. Are the fees acceptable?`
    if (await confirm(message)) {
        const res = await sdk.acceptPaymentProposedFees({ response })
        console.log(JSON.stringify(res, null, 2))
    }
}

const sendOnchainPayment = async (address, options) => {
    const sdk = await initSdk()
    const amount = options.drain
        ? { type: 'drain' }
        : options.receiverAmountSat
        ? { type: 'bitcoin', receiverAmountSat: options.receiverAmountSat }
        : undefined
    if (!amount) {
        console.error('Must specify a receiver amount if not draining')
        return
    }
    const prepareResponse = await sdk.preparePayOnchain({
        amount,
        feeRateSatPerVbyte: options.feeRate
    })
    const message = `Fees: ${prepareResponse.totalFeesSat} sat (incl claim fee: ${prepareResponse.claimFeesSat} sat). Receiver amount: ${prepareResponse.receiverAmountSat} sat. Are the fees acceptable?`
    if (await confirm(message)) {
        const res = await sdk.payOnchain({ address, prepareResponse })
        console.log(JSON.stringify(res, null, 2))
    }
}

const sendPayment = async (options) => {
    const sdk = await initSdk()
    const destination = options.invoice || options.offer || options.address
    const amount = options.drain
        ? { type: 'drain' }
        : options.asset
        ? { type: 'asset', assetId: options.asset, receiverAmount: options.amount }
        : options.amountSat
        ? { type: 'bitcoin', receiverAmountSat: options.amountSat }
        : undefined
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
    if (await confirm(message)) {
        const res = await sdk.sendPayment({ prepareResponse })
        console.log(JSON.stringify(res, null, 2))
    }
}

const signMessage = async (message) => {
    const sdk = await initSdk()
    const res = await sdk.signMessage({ message })
    console.log(JSON.stringify(res, null, 2))
}

const sync = async () => {
    const sdk = await initSdk()
    const res = await sdk.sync()
    console.log(JSON.stringify(res, null, 2))
}

const unregisterWebhook = async (url) => {
    const sdk = await initSdk()
    const res = await sdk.unregisterWebhook()
    console.log(JSON.stringify(res, null, 2))
}

module.exports = {
    buyBitcoin,
    checkMessage,
    disconnect,
    getInfo,
    getPayment,
    fetchFiatRates,
    fetchLightningLimits,
    fetchOnchainLimits,
    listFiat,
    listPayments,
    listRefundables,
    lnurlAuth,
    lnurlPay,
    lnurlWithdraw,
    parse,
    prepareRefund,
    receivePayment,
    recommendedFees,
    refund,
    registerWebhook,
    rescanOnchainSwaps,
    reviewPaymentProposedFees,
    sendOnchainPayment,
    sendPayment,
    signMessage,
    sync,
    unregisterWebhook
}
