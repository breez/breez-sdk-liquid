import { useState, useEffect, useRef } from "react"
import { StatusBar } from "expo-status-bar"
import { StyleSheet, Text, View, ScrollView, TouchableOpacity, SafeAreaView } from "react-native"
import { getItemAsync, setItemAsync } from "expo-secure-store"
import { Mnemonic } from "./mnemonic"
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
    sendPayment,
    PaymentMethod
} from "@breeztech/react-native-breez-sdk-liquid"
import type { SdkEvent } from "@breeztech/react-native-breez-sdk-liquid"

const MNEMONIC_STORE = "mnemonic"

const DebugLine = ({ title, text }: { title: string; text?: string }) => {
    return (
        <TouchableOpacity style={{ flex: 1 }}>
            <View style={{ margin: 5 }}>
                <Text style={{ fontWeight: "bold" }}>{title}</Text>
                {text && text.length > 0 ? <Text>{text}</Text> : <></>}
            </View>
        </TouchableOpacity>
    )
}

const mnemonicGenerator = new Mnemonic()

export default function App() {
    const [lines, setLines] = useState<Array<{ at: number; title: string; text?: string }>>([])
    const listenerIdRef = useRef<string>()

    useEffect(() => {
        const addLine = (title: string, text?: string): void => {
            setLines((lines) => [{ at: new Date().getTime(), title, text }, ...lines])
            console.log(`${title}${text && text.length > 0 ? ": " + text : ""}`)
        }

        const eventHandler = (event: SdkEvent) => {
            addLine("SDK Event", JSON.stringify(event))
        }

        const bolt11Invoice: string = ""

        const asyncFn = async () => {
            try {
                let mnemonic = await getItemAsync(MNEMONIC_STORE)
                if (!mnemonic) {
                    mnemonic = await mnemonicGenerator.generateMnemonic(256)
                    await setItemAsync(MNEMONIC_STORE, mnemonic)
                }

                // Get API Key
                const apiKey = process.env.EXPO_PUBLIC_API_KEY

                if (!apiKey) {
                    throw new Error("No API Key set")
                }

                // Connect using the config
                const config = await defaultConfig(LiquidNetwork.MAINNET, apiKey)
                addLine("defaultConfig", JSON.stringify(config))

                await connect({ config, mnemonic })
                addLine("connect")

                // Get wallet info
                const getInfoRes = await getInfo()
                addLine("getInfo", JSON.stringify(getInfoRes))

                // Historical payments list
                await listPayments({})

                // Register for events
                const listenerId = await addEventListener(eventHandler)
                listenerIdRef.current = listenerId
                addLine("addEventListener", listenerId)

                /* Receive lightning payment */

                const prepareReceiveRes = await prepareReceivePayment({ payerAmountSat: 1000, paymentMethod: PaymentMethod.LIGHTNING })
                addLine("prepareReceivePayment", JSON.stringify(prepareReceiveRes))
                // Get the fees required for this payment
                addLine("Payment fees", `${prepareReceiveRes.feesSat}`)

                const receivePaymentRes = await receivePayment({ prepareResponse: prepareReceiveRes })
                addLine("receivePayment", JSON.stringify(receivePaymentRes))
                // Wait for payer to pay.... once successfully paid an event of `paymentSucceeded` will be emitted.
                addLine("Bolt11 invoice", `${receivePaymentRes.destination}`)

                /* Send lightning payment */

                // Set the `bolt11Invoice` to enable sending in the example app
                if (bolt11Invoice) {
                    let prepareSendRes = await prepareSendPayment({ destination: bolt11Invoice })
                    addLine("prepareSendPayment", JSON.stringify(prepareSendRes))
                    // Get the fees required for this payment
                    addLine("Payment fees", `${prepareSendRes.feesSat}`)

                    let sendPaymentRes = await sendPayment({ prepareResponse: prepareSendRes })
                    addLine("sendPayment", JSON.stringify(sendPaymentRes))
                    // Once successfully paid an event of `paymentSucceeded` will be emitted.
                    addLine("Payment", JSON.stringify(sendPaymentRes.payment))
                }
            } catch (error: unknown) {
                if (error instanceof Error) {
                    addLine("error", error.message)
                }
                console.log(`Error: ${JSON.stringify(error)}`)
            }
        }

        asyncFn()

        return () => {
            const listenerId = listenerIdRef.current
            if (listenerId) {
                removeEventListener(listenerId)
                listenerIdRef.current = undefined
            }
        }
    }, [setLines])

    return (
        <SafeAreaView style={styles.container}>
            <ScrollView style={{ margin: 5 }}>
                {lines.map((line) => (
                    <DebugLine key={`${line.at}-${line.title}`} title={line.title} text={line.text} />
                ))}
            </ScrollView>
            <StatusBar style="auto" />
        </SafeAreaView>
    )
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: "#fff",
        alignItems: "center",
        justifyContent: "center"
    }
})
