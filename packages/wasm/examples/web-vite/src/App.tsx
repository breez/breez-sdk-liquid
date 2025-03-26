import init, {
    connect,
    defaultConfig,
    LiquidNetwork,
    PaymentMethod,
    SdkEvent,
    setLogger
} from '@breeztech/breez-sdk-liquid/web'
import { useState, useEffect } from 'react'

const DebugLine = ({ title, text }: { title: string; text?: string }) => {
    return (
        <div style={{ flex: 1 }}>
            <div style={{ margin: 5 }}>
                <div style={{ fontWeight: 'bold' }}>{title}</div>
                {text && text.length > 0 ? <div>{text}</div> : <></>}
            </div>
        </div>
    )
}

type Line = {
    at: number
    title: string
    text?: string
}

class JsEventListener {
    constructor(private callback: (title: string, text?: string) => void) {}

    onEvent = (event: SdkEvent) => {
        this.callback('EVENT RECEIVED', JSON.stringify(event))
    }
}

function App() {
    const [lines, setLines] = useState<Line[]>([])

    const addLine = (title: string, text?: string) => {
        setLines((lines: Line[]) => [{ at: new Date().getTime(), title, text }, ...lines])
        console.log(`${title}${text && text.length > 0 ? ': ' + text : ''}`)
    }

    const eventListener = new JsEventListener(addLine)

    const asyncFn = async () => {
        let listenerId = null
        let bolt11Invoice = null

        try {
            // Initialize the Wasm module
            // This is required to be called before any other SDK function
            await init()

            // Set the logger to trace
            setLogger('trace')

            // Get the mnemonic
            const breez_api_key = import.meta.env.VITE_BREEZ_API_KEY
            const mnemonic = import.meta.env.VITE_MNEMONIC

            // Connect using the config
            const config = await defaultConfig(LiquidNetwork.MAINNET, breez_api_key)
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
        } catch (e: any) {
            addLine('error', e.toString())
            console.log(`Error: ${JSON.stringify(e)}`)
        }
    }

    useEffect(() => {
        asyncFn()
    }, [])

    return (
        <>
            <div>
                {lines.map((line: Line) => (
                    <DebugLine key={line.at} title={line.title} text={line.text} />
                ))}
            </div>
        </>
    )
}

export default App
