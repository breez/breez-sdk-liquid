import { NativeModules, Platform, EmitterSubscription, NativeEventEmitter } from "react-native"

const LINKING_ERROR =
    `The package 'react-native-breez-sdk-liquid' doesn't seem to be linked. Make sure: \n\n` +
    Platform.select({ ios: "- You have run 'pod install'\n", default: "" }) +
    "- You rebuilt the app after installing the package\n" +
    "- You are not using Expo managed workflow\n"

const BreezSDKLiquid = NativeModules.RNBreezSDKLiquid
    ? NativeModules.RNBreezSDKLiquid
    : new Proxy(
          {},
          {
              get() {
                  throw new Error(LINKING_ERROR)
              }
          }
      )

const BreezSDKLiquidEmitter = new NativeEventEmitter(BreezSDKLiquid)

export type AesSuccessActionDataDecrypted = {
    description: string
    plaintext: string
}

export type BackupRequest = {
    backupPath?: string
}

export type BitcoinAddressData = {
    address: string
    network: Network
    amountSat?: number
    label?: string
    message?: string
}

export interface BuyBitcoinRequest {
    prepareRes: PrepareBuyBitcoinResponse
    redirectUrl?: string
}

export interface Config {
    liquidElectrumUrl: string
    bitcoinElectrumUrl: string
    mempoolspaceUrl: string
    workingDir: string
    network: LiquidNetwork
    paymentTimeoutSec: number
    zeroConfMinFeeRateMsat: number
    zeroConfMaxAmountSat?: number
}

export type ConnectRequest = {
    config: Config
    mnemonic: string
}

export type CurrencyInfo = {
    name: string
    fractionSize: number
    spacing?: number
    symbol?: SymbolType
    uniqSymbol?: SymbolType
    localizedName: LocalizedName[]
    localeOverrides: LocaleOverrides[]
}

export type FiatCurrency = {
    id: string
    info: CurrencyInfo
}

export type GetInfoResponse = {
    balanceSat: number
    pendingSendSat: number
    pendingReceiveSat: number
    pubkey: string
}

export type LnInvoice = {
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

export type LightningPaymentLimitsResponse = {
    send: Limits
    receive: Limits
}

export type Limits = {
    minSat: number
    maxSat: number
    maxZeroConfSat: number
}

export type ListPaymentsRequest = {
    filters?: PaymentType[]
    fromTimestamp?: number
    toTimestamp?: number
    offset?: number
    limit?: number
}

export type LnUrlAuthRequestData = {
    k1: string
    domain: string
    url: string
    action?: string
}

export type LnUrlErrorData = {
    reason: string
}

export type LnUrlPayErrorData = {
    paymentHash: string
    reason: string
}

export type LnUrlPayRequest = {
    data: LnUrlPayRequestData
    amountMsat: number
    comment?: string
    paymentLabel?: string
    validateSuccessActionUrl?: boolean
}

export type LnUrlPayRequestData = {
    callback: string
    minSendable: number
    maxSendable: number
    metadataStr: string
    commentAllowed: number
    domain: string
    allowsNostr: boolean
    nostrPubkey?: string
    lnAddress?: string
}

export type LnUrlPaySuccessData = {
    successAction?: SuccessActionProcessed
    payment: Payment
}

export type LnUrlWithdrawRequest = {
    data: LnUrlWithdrawRequestData
    amountMsat: number
    description?: string
}

export type LnUrlWithdrawRequestData = {
    callback: string
    k1: string
    defaultDescription: string
    minWithdrawable: number
    maxWithdrawable: number
}

export type LnUrlWithdrawSuccessData = {
    invoice: LnInvoice
}

export type LocaleOverrides = {
    locale: string
    spacing?: number
    symbol: SymbolType
}

export type LocalizedName = {
    locale: string
    name: string
}

export type LogEntry = {
    line: string
    level: string
}

export type MessageSuccessActionData = {
    message: string
}

export type OnchainPaymentLimitsResponse = {
    send: Limits
    receive: Limits
}

export type PayOnchainRequest = {
    address: string
    prepareRes: PreparePayOnchainResponse
}

export type Payment = {
    timestamp: number
    amountSat: number
    feesSat: number
    paymentType: PaymentType
    status: PaymentState
    txId?: string
    swapId?: string
    preimage?: string
    bolt11?: string
    refundTxId?: string
    refundTxAmountSat?: number
}

export interface PrepareBuyBitcoinRequest {
    provider: BuyBitcoinProvider
    amountSat: number
}

export interface PrepareBuyBitcoinResponse {
    provider: BuyBitcoinProvider
    amountSat: number
    feesSat: number
}

export interface PreparePayOnchainRequest {
    receiverAmountSat: number
    satPerVbyte?: number
}

export type PreparePayOnchainResponse = {
    receiverAmountSat: number
    claimFeesSat: number
    totalFeesSat: number
}

export type PrepareReceiveOnchainRequest = {
    payerAmountSat: number
}

export type PrepareReceiveOnchainResponse = {
    payerAmountSat: number
    feesSat: number
}

export type PrepareReceiveRequest = {
    payerAmountSat: number
}

export type PrepareReceiveResponse = {
    payerAmountSat: number
    feesSat: number
}

export type PrepareRefundRequest = {
    swapAddress: string
    refundAddress: string
    satPerVbyte: number
}

export type PrepareRefundResponse = {
    txVsize: number
    txFeeSat: number
    refundTxId?: string
}

export type PrepareSendRequest = {
    invoice: string
}

export type PrepareSendResponse = {
    invoice: string
    feesSat: number
}

export type Rate = {
    coin: string
    value: number
}

export type ReceiveOnchainResponse = {
    address: string
    bip21: string
}

export type ReceivePaymentResponse = {
    id: string
    invoice: string
}

export type RecommendedFees = {
    fastestFee: number
    halfHourFee: number
    hourFee: number
    economyFee: number
    minimumFee: number
}

export type RefundRequest = {
    swapAddress: string
    refundAddress: string
    satPerVbyte: number
}

export type RefundResponse = {
    refundTxId: string
}

export type RefundableSwap = {
    swapAddress: string
    timestamp: number
    amountSat: number
}

export type RestoreRequest = {
    backupPath?: string
}

export type RouteHint = {
    hops: RouteHintHop[]
}

export type RouteHintHop = {
    srcNodeId: string
    shortChannelId: number
    feesBaseMsat: number
    feesProportionalMillionths: number
    cltvExpiryDelta: number
    htlcMinimumMsat?: number
    htlcMaximumMsat?: number
}

export type SendPaymentResponse = {
    payment: Payment
}

export type SymbolType = {
    grapheme?: string
    template?: string
    rtl?: boolean
    position?: number
}

export type UrlSuccessActionData = {
    description: string
    url: string
    matchesCallbackDomain: boolean
}

export enum AesSuccessActionDataResultVariant {
    DECRYPTED = "decrypted",
    ERROR_STATUS = "errorStatus"
}

export type AesSuccessActionDataResult = {
    type: AesSuccessActionDataResultVariant.DECRYPTED,
    data: AesSuccessActionDataDecrypted
} | {
    type: AesSuccessActionDataResultVariant.ERROR_STATUS,
    reason: string
}

export enum BuyBitcoinProvider {
    MOONPAY = "moonpay"
}

export enum InputTypeVariant {
    BITCOIN_ADDRESS = "bitcoinAddress",
    BOLT11 = "bolt11",
    NODE_ID = "nodeId",
    URL = "url",
    LN_URL_PAY = "lnUrlPay",
    LN_URL_WITHDRAW = "lnUrlWithdraw",
    LN_URL_AUTH = "lnUrlAuth",
    LN_URL_ERROR = "lnUrlError"
}

export type InputType = {
    type: InputTypeVariant.BITCOIN_ADDRESS,
    address: BitcoinAddressData
} | {
    type: InputTypeVariant.BOLT11,
    invoice: LnInvoice
} | {
    type: InputTypeVariant.NODE_ID,
    nodeId: string
} | {
    type: InputTypeVariant.URL,
    url: string
} | {
    type: InputTypeVariant.LN_URL_PAY,
    data: LnUrlPayRequestData
} | {
    type: InputTypeVariant.LN_URL_WITHDRAW,
    data: LnUrlWithdrawRequestData
} | {
    type: InputTypeVariant.LN_URL_AUTH,
    data: LnUrlAuthRequestData
} | {
    type: InputTypeVariant.LN_URL_ERROR,
    data: LnUrlErrorData
}

export enum LiquidNetwork {
    MAINNET = "mainnet",
    TESTNET = "testnet"
}

export enum LnUrlCallbackStatusVariant {
    OK = "ok",
    ERROR_STATUS = "errorStatus"
}

export type LnUrlCallbackStatus = {
    type: LnUrlCallbackStatusVariant.OK
} | {
    type: LnUrlCallbackStatusVariant.ERROR_STATUS,
    data: LnUrlErrorData
}

export enum LnUrlPayResultVariant {
    ENDPOINT_SUCCESS = "endpointSuccess",
    ENDPOINT_ERROR = "endpointError",
    PAY_ERROR = "payError"
}

export type LnUrlPayResult = {
    type: LnUrlPayResultVariant.ENDPOINT_SUCCESS,
    data: LnUrlPaySuccessData
} | {
    type: LnUrlPayResultVariant.ENDPOINT_ERROR,
    data: LnUrlErrorData
} | {
    type: LnUrlPayResultVariant.PAY_ERROR,
    data: LnUrlPayErrorData
}

export enum LnUrlWithdrawResultVariant {
    OK = "ok",
    TIMEOUT = "timeout",
    ERROR_STATUS = "errorStatus"
}

export type LnUrlWithdrawResult = {
    type: LnUrlWithdrawResultVariant.OK,
    data: LnUrlWithdrawSuccessData
} | {
    type: LnUrlWithdrawResultVariant.TIMEOUT,
    data: LnUrlWithdrawSuccessData
} | {
    type: LnUrlWithdrawResultVariant.ERROR_STATUS,
    data: LnUrlErrorData
}

export enum Network {
    BITCOIN = "bitcoin",
    TESTNET = "testnet",
    SIGNET = "signet",
    REGTEST = "regtest"
}

export enum PaymentState {
    CREATED = "created",
    PENDING = "pending",
    COMPLETE = "complete",
    FAILED = "failed",
    TIMED_OUT = "timedOut",
    REFUNDABLE = "refundable",
    REFUND_PENDING = "refundPending"
}

export enum PaymentType {
    RECEIVE = "receive",
    SEND = "send"
}

export enum SdkEventVariant {
    PAYMENT_FAILED = "paymentFailed",
    PAYMENT_PENDING = "paymentPending",
    PAYMENT_REFUNDED = "paymentRefunded",
    PAYMENT_REFUND_PENDING = "paymentRefundPending",
    PAYMENT_SUCCEEDED = "paymentSucceeded",
    PAYMENT_WAITING_CONFIRMATION = "paymentWaitingConfirmation",
    SYNCED = "synced"
}

export type SdkEvent = {
    type: SdkEventVariant.PAYMENT_FAILED,
    details: Payment
} | {
    type: SdkEventVariant.PAYMENT_PENDING,
    details: Payment
} | {
    type: SdkEventVariant.PAYMENT_REFUNDED,
    details: Payment
} | {
    type: SdkEventVariant.PAYMENT_REFUND_PENDING,
    details: Payment
} | {
    type: SdkEventVariant.PAYMENT_SUCCEEDED,
    details: Payment
} | {
    type: SdkEventVariant.PAYMENT_WAITING_CONFIRMATION,
    details: Payment
} | {
    type: SdkEventVariant.SYNCED
}

export enum SuccessActionProcessedVariant {
    AES = "aes",
    MESSAGE = "message",
    URL = "url"
}

export type SuccessActionProcessed = {
    type: SuccessActionProcessedVariant.AES,
    result: AesSuccessActionDataResult
} | {
    type: SuccessActionProcessedVariant.MESSAGE,
    data: MessageSuccessActionData
} | {
    type: SuccessActionProcessedVariant.URL,
    data: UrlSuccessActionData
}

export type EventListener = (e: SdkEvent) => void

export type Logger = (logEntry: LogEntry) => void

export const connect = async (req: ConnectRequest): Promise<void> => {
    const response = await BreezSDKLiquid.connect(req)
    return response
}

export const addEventListener = async (listener: EventListener): Promise<string> => {
    const response = await BreezSDKLiquid.addEventListener()
    BreezSDKLiquidEmitter.addListener(`event-${response}`, listener)
    
    return response
}

export const setLogger = async (logger: Logger): Promise<EmitterSubscription> => {
    const subscription = BreezSDKLiquidEmitter.addListener("breezSdkLiquidLog", logger)

    try {
        await BreezSDKLiquid.setLogger()
    } catch {}

    return subscription
}

export const defaultConfig = async (network: LiquidNetwork): Promise<Config> => {
    const response = await BreezSDKLiquid.defaultConfig(network)
    return response
}

export const parse = async (input: string): Promise<InputType> => {
    const response = await BreezSDKLiquid.parse(input)
    return response
}

export const parseInvoice = async (input: string): Promise<LnInvoice> => {
    const response = await BreezSDKLiquid.parseInvoice(input)
    return response
}


export const removeEventListener = async (id: string): Promise<void> => {
    await BreezSDKLiquid.removeEventListener(id)
}

export const getInfo = async (): Promise<GetInfoResponse> => {
    const response = await BreezSDKLiquid.getInfo()
    return response
}

export const prepareSendPayment = async (req: PrepareSendRequest): Promise<PrepareSendResponse> => {
    const response = await BreezSDKLiquid.prepareSendPayment(req)
    return response
}

export const sendPayment = async (req: PrepareSendResponse): Promise<SendPaymentResponse> => {
    const response = await BreezSDKLiquid.sendPayment(req)
    return response
}

export const prepareReceivePayment = async (req: PrepareReceiveRequest): Promise<PrepareReceiveResponse> => {
    const response = await BreezSDKLiquid.prepareReceivePayment(req)
    return response
}

export const receivePayment = async (req: PrepareReceiveResponse): Promise<ReceivePaymentResponse> => {
    const response = await BreezSDKLiquid.receivePayment(req)
    return response
}

export const fetchLightningLimits = async (): Promise<LightningPaymentLimitsResponse> => {
    const response = await BreezSDKLiquid.fetchLightningLimits()
    return response
}

export const fetchOnchainLimits = async (): Promise<OnchainPaymentLimitsResponse> => {
    const response = await BreezSDKLiquid.fetchOnchainLimits()
    return response
}

export const preparePayOnchain = async (req: PreparePayOnchainRequest): Promise<PreparePayOnchainResponse> => {
    const response = await BreezSDKLiquid.preparePayOnchain(req)
    return response
}

export const payOnchain = async (req: PayOnchainRequest): Promise<SendPaymentResponse> => {
    const response = await BreezSDKLiquid.payOnchain(req)
    return response
}

export const prepareReceiveOnchain = async (req: PrepareReceiveOnchainRequest): Promise<PrepareReceiveOnchainResponse> => {
    const response = await BreezSDKLiquid.prepareReceiveOnchain(req)
    return response
}

export const receiveOnchain = async (req: PrepareReceiveOnchainResponse): Promise<ReceiveOnchainResponse> => {
    const response = await BreezSDKLiquid.receiveOnchain(req)
    return response
}

export const prepareBuyBitcoin = async (req: PrepareBuyBitcoinRequest): Promise<PrepareBuyBitcoinResponse> => {
    const response = await BreezSDKLiquid.prepareBuyBitcoin(req)
    return response
}

export const buyBitcoin = async (req: BuyBitcoinRequest): Promise<string> => {
    const response = await BreezSDKLiquid.buyBitcoin(req)
    return response
}

export const listPayments = async (req: ListPaymentsRequest): Promise<Payment[]> => {
    const response = await BreezSDKLiquid.listPayments(req)
    return response
}

export const listRefundables = async (): Promise<RefundableSwap[]> => {
    const response = await BreezSDKLiquid.listRefundables()
    return response
}

export const prepareRefund = async (req: PrepareRefundRequest): Promise<PrepareRefundResponse> => {
    const response = await BreezSDKLiquid.prepareRefund(req)
    return response
}

export const refund = async (req: RefundRequest): Promise<RefundResponse> => {
    const response = await BreezSDKLiquid.refund(req)
    return response
}

export const rescanOnchainSwaps = async (): Promise<void> => {
    await BreezSDKLiquid.rescanOnchainSwaps()
}

export const sync = async (): Promise<void> => {
    await BreezSDKLiquid.sync()
}

export const recommendedFees = async (): Promise<RecommendedFees> => {
    const response = await BreezSDKLiquid.recommendedFees()
    return response
}

export const backup = async (req: BackupRequest): Promise<void> => {
    await BreezSDKLiquid.backup(req)
}

export const restore = async (req: RestoreRequest): Promise<void> => {
    await BreezSDKLiquid.restore(req)
}

export const disconnect = async (): Promise<void> => {
    await BreezSDKLiquid.disconnect()
}

export const lnurlPay = async (req: LnUrlPayRequest): Promise<LnUrlPayResult> => {
    const response = await BreezSDKLiquid.lnurlPay(req)
    return response
}

export const lnurlWithdraw = async (req: LnUrlWithdrawRequest): Promise<LnUrlWithdrawResult> => {
    const response = await BreezSDKLiquid.lnurlWithdraw(req)
    return response
}

export const lnurlAuth = async (reqData: LnUrlAuthRequestData): Promise<LnUrlCallbackStatus> => {
    const response = await BreezSDKLiquid.lnurlAuth(reqData)
    return response
}

export const fetchFiatRates = async (): Promise<Rate[]> => {
    const response = await BreezSDKLiquid.fetchFiatRates()
    return response
}

export const listFiatCurrencies = async (): Promise<FiatCurrency[]> => {
    const response = await BreezSDKLiquid.listFiatCurrencies()
    return response
}
