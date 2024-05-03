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

export interface ConnectRequest {
    mnemonic: string
    dataDir?: string
    network: Network
}

export interface GetInfoRequest {
    withScan: boolean
}

export interface GetInfoResponse {
    balanceSat: number
    pubkey: string
}

export interface PrepareReceiveRequest {
    payerAmountSat: number
}

export interface PrepareReceiveResponse {
    pairHash: string
    payerAmountSat: number
    feesSat: number
}

export interface PrepareSendRequest {
    invoice: string
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

export interface RestoreRequest {
    backupPath?: string
}

export interface SendPaymentResponse {
    txid: string
}

export enum Network {
    LIQUID = "liquid",
    LIQUID_TESTNET = "liquidTestnet"
}

export const connect = async (req: ConnectRequest): Promise<void> => {
    const response = await BreezLiquidSDK.connect(req)
    return response
}

export const getInfo = async (req: GetInfoRequest): Promise<GetInfoResponse> => {
    const response = await BreezLiquidSDK.getInfo(req)
    return response
}

export const prepareSendPayment = async (req: PrepareSendRequest): Promise<PrepareSendResponse> => {
    const response = await BreezLiquidSDK.prepareSendPayment(req)
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

export const restore = async (req: RestoreRequest): Promise<void> => {
    await BreezLiquidSDK.restore(req)
}
