/**
 * Sample React Native App
 * https://github.com/facebook/react-native
 *
 * @format
 * @flow strict-local
 */

import React, { useState } from "react"
import { SafeAreaView, ScrollView, StatusBar, Text, TouchableOpacity, View } from "react-native"
import {
    addEventListener,
    connect,
    defaultConfig,
    getInfo,
    LiquidNetwork,
    listPayments,
    removeEventListener,
    prepareReceivePayment,
    prepareSendPayment,
    receivePayment,
    sendPayment
} from "@breeztech/react-native-breez-sdk-liquid"
import { generateMnemonic } from "@dreson4/react-native-quick-bip39"
import { getSecureItem, setSecureItem } from "./utils/storage"

const MNEMONIC_STORE = "MNEMONIC_SECURE_STORE"

const DebugLine = ({ title, text }) => {
    return (
        <TouchableOpacity style={{ flex: 1 }}>
            <View style={{ margin: 5 }}>
                <Text style={{ fontWeight: "bold" }}>{title}</Text>
                {text && text.length > 0 ? <Text>{text}</Text> : <></>}
            </View>
        </TouchableOpacity>
    )
}

const App = () => {
    const [lines, setLines] = useState([])

    const addLine = (title, text) => {
        setLines((lines) => [{ at: new Date().getTime(), title, text }, ...lines])
        console.log(`${title}${text && text.length > 0 ? ": " + text : ""}`)
    }

    const eventHandler = (e) => {
        addLine("event", JSON.stringify(e))
    }

    React.useEffect(() => {
        let listenerId = null
        let bolt11Invoice = null

        const asyncFn = async () => {
            try {
                // Get the mnemonic
                let mnemonic = await getSecureItem(MNEMONIC_STORE)

                if (mnemonic == null) {
                    mnemonic = generateMnemonic(256)
                    setSecureItem(MNEMONIC_STORE, mnemonic)
                }

                // Connect using the config
                const config = await defaultConfig(LiquidNetwork.MAINNET)
                addLine("defaultConfig", JSON.stringify(config))

                await connect({ config, mnemonic })
                addLine("connect", null)

                // Get wallet info
                let getInfoRes = await getInfo()
                addLine("getInfo", JSON.stringify(getInfoRes))

                // Historical payments list
                let payments = listPayments({})

                // Register for events
                listenerId = await addEventListener(eventHandler)
                addLine("addEventListener", listenerId)

                /* Receive lightning payment */

                let prepareReceiveRes = await prepareReceivePayment({ payerAmountSat: 1000 })
                addLine("prepareReceivePayment", JSON.stringify(prepareReceiveRes))
                // Get the fees required for this payment
                addLine("Payment fees", `${prepareReceiveRes.feesSat}`)

                let receivePaymentRes = await receivePayment(prepareReceiveRes)
                addLine("receivePayment", JSON.stringify(receivePaymentRes))
                // Wait for payer to pay.... once successfully paid an event of `paymentSucceeded` will be emitted.
                addLine("Bolt11 invoice", `${receivePaymentRes.invoice}`)

                /* Send lightning payment */

                // Set the `bolt11Invoice` to enable sending in the example app
                if (bolt11Invoice) {
                    let prepareSendRes = await prepareSendPayment({ invoice: bolt11Invoice })
                    addLine("prepareSendPayment", JSON.stringify(prepareSendRes))
                    // Get the fees required for this payment
                    addLine("Payment fees", `${prepareSendRes.feesSat}`)

                    let sendPaymentRes = await sendPayment(prepareSendRes)
                    addLine("sendPayment", JSON.stringify(sendPaymentRes))
                    // Once successfully paid an event of `paymentSucceeded` will be emitted.
                    addLine("Payment", `${sendPaymentRes.payment}`)
                }
            } catch (e) {
                addLine("error", e.toString())
                console.log(`Error: ${JSON.stringify(e)}`)
            }
        }

        asyncFn()

        return () => {
            if (listenerId) {
                removeEventListener(listenerId)
            }
        }
    }, [])

    return (
        <SafeAreaView>
            <StatusBar />
            <ScrollView style={{ padding: 5 }}>
                {lines.map((line) => (
                    <DebugLine key={line.at} title={line.title} text={line.text} />
                ))}
            </ScrollView>
        </SafeAreaView>
    )
}

export default App
