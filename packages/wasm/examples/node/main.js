const { connect, defaultConfig, LiquidNetwork, PaymentMethod, setLogger } = require('@breeztech/breez-sdk-liquid/node')

class JsEventListener {
    onEvent = (event) => {
        console.log(`EVENT RECEIVED: ${JSON.stringify(event)}`)
    }
}

const eventListener = new JsEventListener()

const main = async () => {
    let listenerId = null
    let bolt11Invoice = null

    try {
        // Set the logger to trace
        setLogger('trace')

        // Get the mnemonic
        const breezApiKey = process.env.BREEZ_API_KEY
        const mnemonic = process.env.MNEMONIC

        // Connect using the config
        const config = await defaultConfig(LiquidNetwork.MAINNET, breezApiKey)
        addLine('defaultConfig', JSON.stringify(config))

        const sdk = await connect({ config, mnemonic })
        addLine('connect')

        // Get wallet info
        let getInfoRes = await sdk.getInfo()
        addLine('getInfo', JSON.stringify(getInfoRes))

        // Historical payments list
        await sdk.listPayments({})

        // Register for events
        listenerId = await sdk.addEventListener(eventListener)
        addLine('addEventListener', listenerId)

        /* Receive lightning payment */
        let amount = { BITCOIN: { payerAmountSat: 1000 } }
        let prepareReceiveRes = await sdk.prepareReceivePayment({
            amount,
            paymentMethod: PaymentMethod.LIGHTNING
        })
        addLine('prepareReceivePayment', JSON.stringify(prepareReceiveRes))
        // Get the fees required for this payment
        addLine('Payment fees', `${prepareReceiveRes.feesSat}`)

        let receivePaymentRes = await sdk.receivePayment({
            prepareResponse: prepareReceiveRes
        })
        addLine('receivePayment', JSON.stringify(receivePaymentRes))
        // Wait for payer to pay.... once successfully paid an event of `paymentSucceeded` will be emitted.
        addLine('Bolt11 invoice', `${receivePaymentRes.destination}`)

        /* Send lightning payment */

        // Set the `bolt11Invoice` to enable sending in the example app
        if (bolt11Invoice) {
            let prepareSendRes = await sdk.prepareSendPayment({
                destination: bolt11Invoice
            })
            addLine('prepareSendPayment', JSON.stringify(prepareSendRes))
            // Get the fees required for this payment
            addLine('Payment fees', `${prepareSendRes.feesSat}`)

            let sendPaymentRes = await sdk.sendPayment({
                prepareResponse: prepareSendRes
            })
            addLine('sendPayment', JSON.stringify(sendPaymentRes))
            // Once successfully paid an event of `paymentSucceeded` will be emitted.
            addLine('Payment', JSON.stringify(sendPaymentRes.payment))
        }
    } catch (e) {
        console.log(`Error: ${JSON.stringify(e)}`)
    }
}

main()
