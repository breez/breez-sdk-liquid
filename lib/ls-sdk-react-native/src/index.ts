import { NativeModules, Platform } from "react-native"

const LINKING_ERROR =
    `The package 'react-native-breez-liquid-sdk' doesn't seem to be linked. Make sure: \n\n` +
    Platform.select({ ios: "- You have run 'pod install'\n", default: "" }) +
    "- You rebuilt the app after installing the package\n" +
    "- You are not using Expo managed workflow\n"

const BreezLiquidSDK = NativeModules.RNBreezLiquidSDK
    ? NativeModules.RNBreezLiquidSDK
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

export const connect = async (mnemonic: string, dataDir: string = "", network: Network): Promise<void> => {
    const response = await BreezLiquidSDK.connect(mnemonic, dataDir, network)
    return response
}

export const getInfo = async (withScan: boolean): Promise<WalletInfo> => {
    const response = await BreezLiquidSDK.getInfo(withScan)
    return response
}

export const prepareSendPayment = async (invoice: string): Promise<PrepareSendResponse> => {
    const response = await BreezLiquidSDK.prepareSendPayment(invoice)
    return response
}

export const sendPayment = async (req: PrepareSendResponse): Promise<SendPaymentResponse> => {
    const response = await BreezLiquidSDK.sendPayment(req)
    return response
}

export const prepareReceivePayment = async (req: PrepareReceiveRequest): Promise<PrepareReceiveResponse> => {
    const response = await BreezLiquidSDK.prepareReceivePayment(req)
    return response
}

export const receivePayment = async (req: PrepareReceiveResponse): Promise<ReceivePaymentResponse> => {
    const response = await BreezLiquidSDK.receivePayment(req)
    return response
}

export const backup = async (): Promise<void> => {
    await BreezLiquidSDK.backup()
}

export const restore = async (backupPath: string = ""): Promise<void> => {
    await BreezLiquidSDK.restore(backupPath)
}
