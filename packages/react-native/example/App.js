/**
 * Sample React Native App
 * https://github.com/facebook/react-native
 *
 * @format
 * @flow strict-local
 */

import React, { useState } from "react"
import { SafeAreaView, ScrollView, StatusBar, Text, TouchableOpacity, View } from "react-native"
import { addEventListener, connect, defaultConfig, getInfo, Network, removeEventListener } from "@breeztech/react-native-breez-liquid-sdk"
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

        const asyncFn = async () => {
            try {
                let mnemonic = await getSecureItem(MNEMONIC_STORE)

                if (mnemonic == null) {
                    mnemonic = generateMnemonic(256)
                    setSecureItem(MNEMONIC_STORE, mnemonic)
                }

                const config = await defaultConfig(Network.TESTNET)
                addLine("defaultConfig", JSON.stringify(config))

                await connect({ config, mnemonic })
                addLine("connect", null)

                listenerId = await addEventListener(eventHandler)
                addLine("addEventListener", listenerId)

                let walletInfo = await getInfo({ withScan: false })
                addLine("getInfo", JSON.stringify(walletInfo))
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
