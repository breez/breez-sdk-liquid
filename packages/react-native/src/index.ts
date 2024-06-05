import { NativeModules, Platform, EmitterSubscription, NativeEventEmitter } from "react-native"

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

const BreezLiquidSDKEmitter = new NativeEventEmitter(BreezLiquidSDK)

export interface BackupRequest {
    backupPath?: string
}

export interface Config {
    boltzUrl: string
    electrumUrl: string
    workingDir: string
    network: Network
    paymentTimeoutSec: number
}

export interface ConnectRequest {
    config: Config
    mnemonic: string
}

export interface GetInfoRequest {
    withScan: boolean
}

export interface GetInfoResponse {
    balanceSat: number
    pendingSendSat: number
    pendingReceiveSat: number
    pubkey: string
}

export interface LnInvoice {
    bolt11: string
    network: Network
    payeePubkey: string
    paymentHash: string
    description?: string
    descriptionHash?: string
    amountMsat?: number
    timestamp: number
    expiry: number
    routingHints: RouteHint[]
    paymentSecret: number[]
    minFinalCltvExpiryDelta: number
}

export interface LogEntry {
    line: string
    level: string
}

export interface Payment {
    txId: string
    swapId?: string
    timestamp: number
    amountSat: number
    feesSat: number
    preimage?: string
    refundTxId?: string
    refundTxAmountSat?: number
    paymentType: PaymentType
    status: PaymentState
}

export interface PrepareReceiveRequest {
    payerAmountSat: number
}

export interface PrepareReceiveResponse {
    payerAmountSat: number
    feesSat: number
}

export interface PrepareSendRequest {
    invoice: string
}

export interface PrepareSendResponse {
    invoice: string
    feesSat: number
}

export interface ReceivePaymentResponse {
    id: string
    invoice: string
}

export interface RestoreRequest {
    backupPath?: string
}

export interface RouteHint {
    hops: RouteHintHop[]
}

export interface RouteHintHop {
    srcNodeId: string
    shortChannelId: number
    feesBaseMsat: number
    feesProportionalMillionths: number
    cltvExpiryDelta: number
    htlcMinimumMsat?: number
    htlcMaximumMsat?: number
}

export interface SendPaymentResponse {
    payment: Payment
}

export enum LiquidSdkEventVariant {
    PAYMENT_FAILED = "paymentFailed",
    PAYMENT_PENDING = "paymentPending",
    PAYMENT_REFUNDED = "paymentRefunded",
    PAYMENT_REFUND_PENDING = "paymentRefundPending",
    PAYMENT_SUCCEED = "paymentSucceed",
    PAYMENT_WAITING_CONFIRMATION = "paymentWaitingConfirmation",
    SYNCED = "synced"
}

export type LiquidSdkEvent = {
    type: LiquidSdkEventVariant.PAYMENT_FAILED,
    details: Payment
} | {
    type: LiquidSdkEventVariant.PAYMENT_PENDING,
    details: Payment
} | {
    type: LiquidSdkEventVariant.PAYMENT_REFUNDED,
    details: Payment
} | {
    type: LiquidSdkEventVariant.PAYMENT_REFUND_PENDING,
    details: Payment
} | {
    type: LiquidSdkEventVariant.PAYMENT_SUCCEED,
    details: Payment
} | {
    type: LiquidSdkEventVariant.PAYMENT_WAITING_CONFIRMATION,
    details: Payment
} | {
    type: LiquidSdkEventVariant.SYNCED
}

export enum Network {
    MAINNET = "mainnet",
    TESTNET = "testnet"
}

export enum PaymentState {
    CREATED = "created",
    PENDING = "pending",
    COMPLETE = "complete",
    FAILED = "failed",
    TIMED_OUT = "timedOut"
}

export enum PaymentType {
    RECEIVE = "receive",
    SEND = "send"
}

export type EventListener = (e: LiquidSdkEvent) => void

export type Logger = (logEntry: LogEntry) => void

export const connect = async (req: ConnectRequest): Promise<void> => {
    const response = await BreezLiquidSDK.connect(req)
    return response
}

export const addEventListener = async (listener: EventListener): Promise<string> => {
    const response = await BreezLiquidSDK.addEventListener()
    BreezLiquidSDKEmitter.addListener(`event-${response}`, listener)

    return response
}

export const setLogger = async (logger: Logger): Promise<EmitterSubscription> => {
    const subscription = BreezLiquidSDKEmitter.addListener("breezLiquidSdkLog", logger)

    try {
        await BreezLiquidSDK.setLogger()
    } catch {}

    return subscription
}

export const defaultConfig = async (network: Network): Promise<Config> => {
    const response = await BreezLiquidSDK.defaultConfig(network)
    return response
}

export const parseInvoice = async (invoice: string): Promise<LnInvoice> => {
    const response = await BreezLiquidSDK.parseInvoice(invoice)
    return response
}


export const removeEventListener = async (id: string): Promise<void> => {
    await BreezLiquidSDK.removeEventListener(id)
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

export const listPayments = async (): Promise<Payment[]> => {
    const response = await BreezLiquidSDK.listPayments()
    return response
}

export const sync = async (): Promise<void> => {
    await BreezLiquidSDK.sync()
}

export const backup = async (req: BackupRequest): Promise<void> => {
    await BreezLiquidSDK.backup(req)
}

export const restore = async (req: RestoreRequest): Promise<void> => {
    await BreezLiquidSDK.restore(req)
}

export const disconnect = async (): Promise<void> => {
    await BreezLiquidSDK.disconnect()
}
