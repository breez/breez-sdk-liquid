import { NativeModules, Platform } from "react-native"

const LINKING_ERROR =
    `The package 'react-native-liquid-swap-sdk' doesn't seem to be linked. Make sure: \n\n` +
    Platform.select({ ios: "- You have run 'pod install'\n", default: "" }) +
    "- You rebuilt the app after installing the package\n" +
    "- You are not using Expo managed workflow\n"

const LiquidSwapSDK = NativeModules.RNLiquidSwapSDK
    ? NativeModules.RNLiquidSwapSDK
    : new Proxy(
          {},
          {
              get() {
                  throw new Error(LINKING_ERROR)
              }
          }
      )

export interface PrepareReceiveRequest {
    payerAmountSat?: number
    receiverAmountSat?: number
}

export interface PrepareReceiveResponse {
    pairHash: string
    payerAmountSat: number
    feesSat: number
}

export interface PrepareSendResponse {
    id: string
    payerAmountSat: number
    receiverAmountSat: number
    totalFees: number
    fundingAddress: string
    invoice: string
}

export interface ReceivePaymentResponse {
    id: string
    invoice: string
}

export interface SendPaymentResponse {
    txid: string
}

export interface WalletInfo {
    balanceSat: number
    pubkey: string
}

export enum Network {
    LIQUID = "liquid",
    LIQUID_TESTNET = "liquidTestnet"
}

export const init = async (mnemonic: string, dataDir: string = "", network: Network): Promise<void> => {
    const response = await LiquidSwapSDK.initBindingWallet(mnemonic, dataDir, network)
    return response
}
export const connect = async (mnemonic: string, dataDir: string = "", network: Network): Promise<BindingWallet> => {
    const response = await LiquidSwapSDK.connect(mnemonic, dataDir, network)
    return response
}


export const getInfo = async (withScan: boolean): Promise<WalletInfo> => {
    const response = await LiquidSwapSDK.getInfo(withScan)
    return response
}

export const prepareSendPayment = async (invoice: string): Promise<PrepareSendResponse> => {
    const response = await LiquidSwapSDK.prepareSendPayment(invoice)
    return response
}

export const sendPayment = async (req: PrepareSendResponse): Promise<SendPaymentResponse> => {
    const response = await LiquidSwapSDK.sendPayment(req)
    return response
}

export const prepareReceivePayment = async (req: PrepareReceiveRequest): Promise<PrepareReceiveResponse> => {
    const response = await LiquidSwapSDK.prepareReceivePayment(req)
    return response
}

export const receivePayment = async (req: PrepareReceiveResponse): Promise<ReceivePaymentResponse> => {
    const response = await LiquidSwapSDK.receivePayment(req)
    return response
}

export const backup = async (): Promise<void> => {
    await LiquidSwapSDK.backup()
}

export const restore = async (backupPath: string = ""): Promise<void> => {
    await LiquidSwapSDK.restore(backupPath)
}
