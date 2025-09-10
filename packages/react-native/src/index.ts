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

export interface AcceptPaymentProposedFeesRequest {
    response: FetchPaymentProposedFeesResponse
}

export interface AesSuccessActionData {
    description: string
    ciphertext: string
    iv: string
}

export interface AesSuccessActionDataDecrypted {
    description: string
    plaintext: string
}

export interface AssetBalance {
    assetId: string
    balanceSat: number
    name?: string
    ticker?: string
    balance?: number
}

export interface AssetInfo {
    name: string
    ticker: string
    amount: number
    fees?: number
}

export interface AssetMetadata {
    assetId: string
    name: string
    ticker: string
    precision: number
    fiatId?: string
}

export interface BackupRequest {
    backupPath?: string
}

export interface BitcoinAddressData {
    address: string
    network: Network
    amountSat?: number
    label?: string
    message?: string
}

export interface BlockchainInfo {
    liquidTip: number
    bitcoinTip: number
}

export interface BuyBitcoinRequest {
    prepareResponse: PrepareBuyBitcoinResponse
    redirectUrl?: string
}

export interface CheckMessageRequest {
    message: string
    pubkey: string
    signature: string
}

export interface CheckMessageResponse {
    isValid: boolean
}

export interface Config {
    liquidExplorer: BlockchainExplorer
    bitcoinExplorer: BlockchainExplorer
    workingDir: string
    network: LiquidNetwork
    paymentTimeoutSec: number
    syncServiceUrl?: string
    breezApiKey?: string
    zeroConfMaxAmountSat?: number
    useDefaultExternalInputParsers: boolean
    useMagicRoutingHints: boolean
    externalInputParsers?: ExternalInputParser[]
    onchainFeeRateLeewaySat?: number
    assetMetadata?: AssetMetadata[]
    sideswapApiKey?: string
}

export interface ConnectRequest {
    config: Config
    mnemonic?: string
    passphrase?: string
    seed?: number[]
}

export interface ConnectWithSignerRequest {
    config: Config
}

export interface CreateBolt12InvoiceRequest {
    offer: string
    invoiceRequest: string
}

export interface CreateBolt12InvoiceResponse {
    invoice: string
}

export interface CurrencyInfo {
    name: string
    fractionSize: number
    spacing?: number
    symbol?: SymbolType
    uniqSymbol?: SymbolType
    localizedName: LocalizedName[]
    localeOverrides: LocaleOverrides[]
}

export interface ExternalInputParser {
    providerId: string
    inputRegex: string
    parserUrl: string
}

export interface FetchPaymentProposedFeesRequest {
    swapId: string
}

export interface FetchPaymentProposedFeesResponse {
    swapId: string
    feesSat: number
    payerAmountSat: number
    receiverAmountSat: number
}

export interface FiatCurrency {
    id: string
    info: CurrencyInfo
}

export interface GetInfoResponse {
    walletInfo: WalletInfo
    blockchainInfo: BlockchainInfo
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

export interface LnOffer {
    offer: string
    chains: string[]
    paths: LnOfferBlindedPath[]
    description?: string
    signingPubkey?: string
    minAmount?: Amount
    absoluteExpiry?: number
    issuer?: string
}

export interface LightningPaymentLimitsResponse {
    send: Limits
    receive: Limits
}

export interface Limits {
    minSat: number
    maxSat: number
    maxZeroConfSat: number
}

export interface LiquidAddressData {
    address: string
    network: Network
    assetId?: string
    amount?: number
    amountSat?: number
    label?: string
    message?: string
}

export interface ListPaymentsRequest {
    filters?: PaymentType[]
    states?: PaymentState[]
    fromTimestamp?: number
    toTimestamp?: number
    offset?: number
    limit?: number
    details?: ListPaymentDetails
    sortAscending?: boolean
}

export interface LnOfferBlindedPath {
    blindedHops: string[]
}

export interface LnUrlAuthRequestData {
    k1: string
    domain: string
    url: string
    action?: string
}

export interface LnUrlErrorData {
    reason: string
}

export interface LnUrlInfo {
    lnAddress?: string
    lnurlPayComment?: string
    lnurlPayDomain?: string
    lnurlPayMetadata?: string
    lnurlPaySuccessAction?: SuccessActionProcessed
    lnurlPayUnprocessedSuccessAction?: SuccessAction
    lnurlWithdrawEndpoint?: string
}

export interface LnUrlPayErrorData {
    paymentHash: string
    reason: string
}

export interface LnUrlPayRequest {
    prepareResponse: PrepareLnUrlPayResponse
}

export interface LnUrlPayRequestData {
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

export interface LnUrlPaySuccessData {
    successAction?: SuccessActionProcessed
    payment: Payment
}

export interface LnUrlWithdrawRequest {
    data: LnUrlWithdrawRequestData
    amountMsat: number
    description?: string
}

export interface LnUrlWithdrawRequestData {
    callback: string
    k1: string
    defaultDescription: string
    minWithdrawable: number
    maxWithdrawable: number
}

export interface LnUrlWithdrawSuccessData {
    invoice: LnInvoice
}

export interface LocaleOverrides {
    locale: string
    spacing?: number
    symbol: SymbolType
}

export interface LocalizedName {
    locale: string
    name: string
}

export interface LogEntry {
    line: string
    level: string
}

export interface MessageSuccessActionData {
    message: string
}

export interface OnchainPaymentLimitsResponse {
    send: Limits
    receive: Limits
}

export interface PayOnchainRequest {
    address: string
    prepareResponse: PreparePayOnchainResponse
}

export interface Payment {
    timestamp: number
    amountSat: number
    feesSat: number
    paymentType: PaymentType
    status: PaymentState
    details: PaymentDetails
    swapperFeesSat?: number
    destination?: string
    txId?: string
    unblindingData?: string
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

export interface PrepareLnUrlPayRequest {
    data: LnUrlPayRequestData
    amount: PayAmount
    bip353Address?: string
    comment?: string
    validateSuccessActionUrl?: boolean
}

export interface PrepareLnUrlPayResponse {
    destination: SendDestination
    feesSat: number
    data: LnUrlPayRequestData
    amount: PayAmount
    comment?: string
    successAction?: SuccessAction
}

export interface PreparePayOnchainRequest {
    amount: PayAmount
    feeRateSatPerVbyte?: number
}

export interface PreparePayOnchainResponse {
    receiverAmountSat: number
    claimFeesSat: number
    totalFeesSat: number
}

export interface PrepareReceiveRequest {
    paymentMethod: PaymentMethod
    amount?: ReceiveAmount
}

export interface PrepareReceiveResponse {
    paymentMethod: PaymentMethod
    feesSat: number
    amount?: ReceiveAmount
    minPayerAmountSat?: number
    maxPayerAmountSat?: number
    swapperFeerate?: number
}

export interface PrepareRefundRequest {
    swapAddress: string
    refundAddress: string
    feeRateSatPerVbyte: number
}

export interface PrepareRefundResponse {
    txVsize: number
    txFeeSat: number
    lastRefundTxId?: string
}

export interface PrepareSendRequest {
    destination: string
    amount?: PayAmount
}

export interface PrepareSendResponse {
    destination: SendDestination
    amount?: PayAmount
    feesSat?: number
    estimatedAssetFees?: number
    exchangeAmountSat?: number
}

export interface Rate {
    coin: string
    value: number
}

export interface ReceivePaymentRequest {
    prepareResponse: PrepareReceiveResponse
    description?: string
    useDescriptionHash?: boolean
    payerNote?: string
}

export interface ReceivePaymentResponse {
    destination: string
    liquidExpirationBlockheight?: number
    bitcoinExpirationBlockheight?: number
}

export interface RecommendedFees {
    fastestFee: number
    halfHourFee: number
    hourFee: number
    economyFee: number
    minimumFee: number
}

export interface RefundRequest {
    swapAddress: string
    refundAddress: string
    feeRateSatPerVbyte: number
}

export interface RefundResponse {
    refundTxId: string
}

export interface RefundableSwap {
    swapAddress: string
    timestamp: number
    amountSat: number
    lastRefundTxId?: string
}

export interface RestoreRequest {
    backupPath?: string
}

export interface RouteHint {
    hops: RouteHintHop[]
}

export interface RouteHintHop {
    srcNodeId: string
    shortChannelId: string
    feesBaseMsat: number
    feesProportionalMillionths: number
    cltvExpiryDelta: number
    htlcMinimumMsat?: number
    htlcMaximumMsat?: number
}

export interface SendPaymentRequest {
    prepareResponse: PrepareSendResponse
    useAssetFees?: boolean
    payerNote?: string
}

export interface SendPaymentResponse {
    payment: Payment
}

export interface SignMessageRequest {
    message: string
}

export interface SignMessageResponse {
    signature: string
}

export interface SymbolType {
    grapheme?: string
    template?: string
    rtl?: boolean
    position?: number
}

export interface UrlSuccessActionData {
    description: string
    url: string
    matchesCallbackDomain: boolean
}

export interface WalletInfo {
    balanceSat: number
    pendingSendSat: number
    pendingReceiveSat: number
    fingerprint: string
    pubkey: string
    assetBalances: AssetBalance[]
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

export enum AmountVariant {
    BITCOIN = "bitcoin",
    CURRENCY = "currency"
}

export type Amount = {
    type: AmountVariant.BITCOIN,
    amountMsat: number
} | {
    type: AmountVariant.CURRENCY,
    iso4217Code: string
    fractionalAmount: number
}

export enum BlockchainExplorerVariant {
    ELECTRUM = "electrum",
    ESPLORA = "esplora"
}

export type BlockchainExplorer = {
    type: BlockchainExplorerVariant.ELECTRUM,
    url: string
} | {
    type: BlockchainExplorerVariant.ESPLORA,
    url: string
    useWaterfalls: boolean
}

export enum BuyBitcoinProvider {
    MOONPAY = "moonpay"
}

export enum GetPaymentRequestVariant {
    PAYMENT_HASH = "paymentHash",
    SWAP_ID = "swapId"
}

export type GetPaymentRequest = {
    type: GetPaymentRequestVariant.PAYMENT_HASH,
    paymentHash: string
} | {
    type: GetPaymentRequestVariant.SWAP_ID,
    swapId: string
}

export enum InputTypeVariant {
    BITCOIN_ADDRESS = "bitcoinAddress",
    LIQUID_ADDRESS = "liquidAddress",
    BOLT11 = "bolt11",
    BOLT12_OFFER = "bolt12Offer",
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
    type: InputTypeVariant.LIQUID_ADDRESS,
    address: LiquidAddressData
} | {
    type: InputTypeVariant.BOLT11,
    invoice: LnInvoice
} | {
    type: InputTypeVariant.BOLT12_OFFER,
    offer: LnOffer
    bip353Address?: string
} | {
    type: InputTypeVariant.NODE_ID,
    nodeId: string
} | {
    type: InputTypeVariant.URL,
    url: string
} | {
    type: InputTypeVariant.LN_URL_PAY,
    data: LnUrlPayRequestData
    bip353Address?: string
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
    TESTNET = "testnet",
    REGTEST = "regtest"
}

export enum ListPaymentDetailsVariant {
    LIQUID = "liquid",
    BITCOIN = "bitcoin"
}

export type ListPaymentDetails = {
    type: ListPaymentDetailsVariant.LIQUID,
    assetId?: string
    destination?: string
} | {
    type: ListPaymentDetailsVariant.BITCOIN,
    address?: string
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

export enum PayAmountVariant {
    BITCOIN = "bitcoin",
    ASSET = "asset",
    DRAIN = "drain"
}

export type PayAmount = {
    type: PayAmountVariant.BITCOIN,
    receiverAmountSat: number
} | {
    type: PayAmountVariant.ASSET,
    toAsset: string
    receiverAmount: number
    estimateAssetFees?: boolean
    fromAsset?: string
} | {
    type: PayAmountVariant.DRAIN
}

export enum PaymentDetailsVariant {
    LIGHTNING = "lightning",
    LIQUID = "liquid",
    BITCOIN = "bitcoin"
}

export type PaymentDetails = {
    type: PaymentDetailsVariant.LIGHTNING,
    swapId: string
    description: string
    liquidExpirationBlockheight: number
    preimage?: string
    invoice?: string
    bolt12Offer?: string
    paymentHash?: string
    destinationPubkey?: string
    lnurlInfo?: LnUrlInfo
    bip353Address?: string
    payerNote?: string
    claimTxId?: string
    refundTxId?: string
    refundTxAmountSat?: number
} | {
    type: PaymentDetailsVariant.LIQUID,
    assetId: string
    destination: string
    description: string
    assetInfo?: AssetInfo
    lnurlInfo?: LnUrlInfo
    bip353Address?: string
    payerNote?: string
} | {
    type: PaymentDetailsVariant.BITCOIN,
    swapId: string
    bitcoinAddress: string
    description: string
    autoAcceptedFees: boolean
    bitcoinExpirationBlockheight: number
    liquidExpirationBlockheight: number
    lockupTxId?: string
    claimTxId?: string
    refundTxId?: string
    refundTxAmountSat?: number
}

export enum PaymentMethod {
    LIGHTNING = "lightning",
    BOLT11_INVOICE = "bolt11Invoice",
    BOLT12_OFFER = "bolt12Offer",
    BITCOIN_ADDRESS = "bitcoinAddress",
    LIQUID_ADDRESS = "liquidAddress"
}

export enum PaymentState {
    CREATED = "created",
    PENDING = "pending",
    COMPLETE = "complete",
    FAILED = "failed",
    TIMED_OUT = "timedOut",
    REFUNDABLE = "refundable",
    REFUND_PENDING = "refundPending",
    WAITING_FEE_ACCEPTANCE = "waitingFeeAcceptance"
}

export enum PaymentType {
    RECEIVE = "receive",
    SEND = "send"
}

export enum ReceiveAmountVariant {
    BITCOIN = "bitcoin",
    ASSET = "asset"
}

export type ReceiveAmount = {
    type: ReceiveAmountVariant.BITCOIN,
    payerAmountSat: number
} | {
    type: ReceiveAmountVariant.ASSET,
    assetId: string
    payerAmount?: number
}

export enum SdkEventVariant {
    PAYMENT_FAILED = "paymentFailed",
    PAYMENT_PENDING = "paymentPending",
    PAYMENT_REFUNDABLE = "paymentRefundable",
    PAYMENT_REFUNDED = "paymentRefunded",
    PAYMENT_REFUND_PENDING = "paymentRefundPending",
    PAYMENT_SUCCEEDED = "paymentSucceeded",
    PAYMENT_WAITING_CONFIRMATION = "paymentWaitingConfirmation",
    PAYMENT_WAITING_FEE_ACCEPTANCE = "paymentWaitingFeeAcceptance",
    SYNCED = "synced",
    DATA_SYNCED = "dataSynced"
}

export type SdkEvent = {
    type: SdkEventVariant.PAYMENT_FAILED,
    details: Payment
} | {
    type: SdkEventVariant.PAYMENT_PENDING,
    details: Payment
} | {
    type: SdkEventVariant.PAYMENT_REFUNDABLE,
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
    type: SdkEventVariant.PAYMENT_WAITING_FEE_ACCEPTANCE,
    details: Payment
} | {
    type: SdkEventVariant.SYNCED
} | {
    type: SdkEventVariant.DATA_SYNCED,
    didPullNewRecords: boolean
}

export enum SendDestinationVariant {
    LIQUID_ADDRESS = "liquidAddress",
    BOLT11 = "bolt11",
    BOLT12 = "bolt12"
}

export type SendDestination = {
    type: SendDestinationVariant.LIQUID_ADDRESS,
    addressData: LiquidAddressData
    bip353Address?: string
} | {
    type: SendDestinationVariant.BOLT11,
    invoice: LnInvoice
    bip353Address?: string
} | {
    type: SendDestinationVariant.BOLT12,
    offer: LnOffer
    receiverAmountSat: number
    bip353Address?: string
}

export enum SuccessActionVariant {
    AES = "aes",
    MESSAGE = "message",
    URL = "url"
}

export type SuccessAction = {
    type: SuccessActionVariant.AES,
    data: AesSuccessActionData
} | {
    type: SuccessActionVariant.MESSAGE,
    data: MessageSuccessActionData
} | {
    type: SuccessActionVariant.URL,
    data: UrlSuccessActionData
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

export const defaultConfig = async (network: LiquidNetwork, breezApiKey: string = ""): Promise<Config> => {
    const response = await BreezSDKLiquid.defaultConfig(network, breezApiKey)
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

export const signMessage = async (req: SignMessageRequest): Promise<SignMessageResponse> => {
    const response = await BreezSDKLiquid.signMessage(req)
    return response
}

export const checkMessage = async (req: CheckMessageRequest): Promise<CheckMessageResponse> => {
    const response = await BreezSDKLiquid.checkMessage(req)
    return response
}

export const parse = async (input: string): Promise<InputType> => {
    const response = await BreezSDKLiquid.parse(input)
    return response
}

export const prepareSendPayment = async (req: PrepareSendRequest): Promise<PrepareSendResponse> => {
    const response = await BreezSDKLiquid.prepareSendPayment(req)
    return response
}

export const sendPayment = async (req: SendPaymentRequest): Promise<SendPaymentResponse> => {
    const response = await BreezSDKLiquid.sendPayment(req)
    return response
}

export const prepareReceivePayment = async (req: PrepareReceiveRequest): Promise<PrepareReceiveResponse> => {
    const response = await BreezSDKLiquid.prepareReceivePayment(req)
    return response
}

export const receivePayment = async (req: ReceivePaymentRequest): Promise<ReceivePaymentResponse> => {
    const response = await BreezSDKLiquid.receivePayment(req)
    return response
}

export const createBolt12Invoice = async (req: CreateBolt12InvoiceRequest): Promise<CreateBolt12InvoiceResponse> => {
    const response = await BreezSDKLiquid.createBolt12Invoice(req)
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

export const getPayment = async (req: GetPaymentRequest): Promise<Payment | null> => {
    const response = await BreezSDKLiquid.getPayment(req)
    return response
}

export const fetchPaymentProposedFees = async (req: FetchPaymentProposedFeesRequest): Promise<FetchPaymentProposedFeesResponse> => {
    const response = await BreezSDKLiquid.fetchPaymentProposedFees(req)
    return response
}

export const acceptPaymentProposedFees = async (req: AcceptPaymentProposedFeesRequest): Promise<void> => {
    await BreezSDKLiquid.acceptPaymentProposedFees(req)
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

export const prepareLnurlPay = async (req: PrepareLnUrlPayRequest): Promise<PrepareLnUrlPayResponse> => {
    const response = await BreezSDKLiquid.prepareLnurlPay(req)
    return response
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

export const registerWebhook = async (webhookUrl: string): Promise<void> => {
    await BreezSDKLiquid.registerWebhook(webhookUrl)
}

export const unregisterWebhook = async (): Promise<void> => {
    await BreezSDKLiquid.unregisterWebhook()
}

export const fetchFiatRates = async (): Promise<Rate[]> => {
    const response = await BreezSDKLiquid.fetchFiatRates()
    return response
}

export const listFiatCurrencies = async (): Promise<FiatCurrency[]> => {
    const response = await BreezSDKLiquid.listFiatCurrencies()
    return response
}
