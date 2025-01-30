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

/**
 * An argument when calling {@link acceptPaymentProposedFees}.
 */
export interface AcceptPaymentProposedFeesRequest {
    response: FetchPaymentProposedFeesResponse
}

/**
 * Payload of the AES success action, as received from the LNURL endpoint
 *
 * See {@link AesSuccessActionDataDecrypted} for a similar wrapper containing the decrypted payload
 */
export interface AesSuccessActionData {
	/**
	 * Contents description, up to 144 characters
	 */
    description: string
	/**
	 * Base64, AES-encrypted data where encryption key is payment preimage, up to 4kb of characters
	 */
    ciphertext: string
	/**
	 * Base64, initialization vector, exactly 24 characters
	 */
    iv: string
}

/**
 * Wrapper for the decrypted {@link AesSuccessActionDataResult.DECRYPTED} payload
 */
export interface AesSuccessActionDataDecrypted {
	/**
	 * Contents description, up to 144 characters
	 */
    description: string
	/**
	 * Decrypted content
	 */
    plaintext: string
}

/**
 * An asset balance to denote the balance for each asset.
 */
export interface AssetBalance {
    assetId: string
    balanceSat: number
    name?: string
    ticker?: string
    balance?: number
}

/**
 * Represents the Liquid payment asset info. The asset info is derived from
 * the available {@link AssetMetadata} that is set in the {@link Config}.
 */
export interface AssetInfo {
	/**
	 * The name of the asset
	 */
    name: string
	/**
	 * The ticker of the asset
	 */
    ticker: string
	/**
	 * The amount calculated from the satoshi amount of the transaction, having its
	 * decimal shifted to the left by the {@link AssetMetadata}'s `precision`
	 */
    amount: number
}

/**
 * Configuration for asset metadata. Each asset metadata item represents an entry in the
 * Liquid Asset Registry <https://docs.liquid.net/docs/blockstream-liquid-asset-registry>.
 * An example Liquid Asset in the registry would be Tether USD <https://assets.blockstream.info/ce091c998b83c78bb71a632313ba3760f1763d9cfcffae02258ffa9865a37bd2.json>.
 */
export interface AssetMetadata {
	/**
	 * The asset id of the registered asset
	 */
    assetId: string
	/**
	 * The name of the asset
	 */
    name: string
	/**
	 * The ticker of the asset
	 */
    ticker: string
	/**
	 * The precision used to display the asset amount.
	 * For example, precision of 2 shifts the decimal 2 places left from the satoshi amount.
	 */
    precision: number
}

/**
 * An argument when calling {@link backup}.
 */
export interface BackupRequest {
	/**
	 * Path to the backup.
	 *
	 * If not set, it defaults to 'backup.sql' for mainnet and 'backup-testnet.sql' for testnet.
	 * The file will be saved in {@link ConnectRequest}'s `dataDir`.
	 */
    backupPath?: string
}

/**
 * Wrapped in a {@link InputType.BITCOIN_ADDRESS}, this is the result of [parse] when given a plain or BIP-21 BTC address.
 */
export interface BitcoinAddressData {
    address: string
    network: Network
    amountSat?: number
    label?: string
    message?: string
}

export interface BlockchainInfo {
	/**
	 * The block height of the Liquid chain tip
	 */
    liquidTip: number
	/**
	 * The block height of the Bitcoin chain tip
	 */
    bitcoinTip: number
}

/**
 * An argument when calling {@link buyBitcoin}.
 */
export interface BuyBitcoinRequest {
    prepareResponse: PrepareBuyBitcoinResponse
	/**
	 * The optional URL to redirect to after completing the buy.
	 *
	 * For Moonpay, see <https://dev.moonpay.com/docs/on-ramp-configure-user-journey-params>
	 */
    redirectUrl?: string
}

/**
 * An argument when calling {@link checkMessage}.
 */
export interface CheckMessageRequest {
	/**
	 * The message that was signed.
	 */
    message: string
	/**
	 * The public key of the node that signed the message.
	 */
    pubkey: string
	/**
	 * The zbase encoded signature to verify.
	 */
    signature: string
}

/**
 * Returned when calling {@link checkMessage}.
 */
export interface CheckMessageResponse {
	/**
	 * Boolean value indicating whether the signature covers the message and
	 * was signed by the given pubkey.
	 */
    isValid: boolean
}

/**
 * Configuration for the Liquid SDK
 */
export interface Config {
    liquidElectrumUrl: string
    bitcoinElectrumUrl: string
	/**
	 * The mempool.space API URL, has to be in the format: https://mempool.space/api
	 */
    mempoolspaceUrl: string
	/**
	 * Directory in which all SDK files (DB, log, cache) are stored.
	 *
	 * Prefix can be a relative or absolute path to this directory.
	 */
    workingDir: string
    network: LiquidNetwork
	/**
	 * Send payment timeout. See {@link sendPayment}
	 */
    paymentTimeoutSec: number
	/**
	 * Zero-conf minimum accepted fee-rate in millisatoshis per vbyte
	 */
    zeroConfMinFeeRateMsat: number
	/**
	 * The url of the real-time sync service.
	 * Setting this field to `none` will disable the service
	 */
    syncServiceUrl?: string
	/**
	 * The Breez API key used for making requests to their mempool service
	 */
    breezApiKey?: string
	/**
	 * Directory in which the Liquid wallet cache is stored. Defaults to `workingDir`
	 */
    cacheDir?: string
	/**
	 * Maximum amount in satoshi to accept zero-conf payments with
	 * Defaults to [DEFAULT_ZERO_CONF_MAX_SAT]
	 */
    zeroConfMaxAmountSat?: number
	/**
	 * The SDK includes some default external input parsers
	 * Set this to false in order to prevent their use.
	 */
    useDefaultExternalInputParsers: boolean
	/**
	 * A set of external input parsers that are used by [crate::sdk::LiquidSdk::parse] when the input
	 * is not recognized. See {@link ExternalInputParser} for more details on how to configure
	 * external parsing.
	 */
    externalInputParsers?: ExternalInputParser[]
	/**
	 * For payments where the onchain fees can only be estimated on creation, this can be used
	 * in order to automatically allow slightly more expensive fees. If the actual fee rate ends up
	 * being above the sum of the initial estimate and this leeway, the payment will require
	 * user fee acceptance. See {@link PaymentState.WAITING_FEE_ACCEPTANCE}.
	 *
	 * Defaults to zero.
	 */
    onchainFeeRateLeewaySatPerVbyte?: number
	/**
	 * A set of asset metadata used by [crate::sdk::LiquidSdk::parse] when the input is a
	 * {@link LiquidAddressData} and the {@link LiquidAddressData.assetId} differs from the Liquid Bitcoin asset.
	 * See {@link AssetMetadata} for more details on how define asset metadata.
	 * By default the asset metadata for Liquid Bitcoin and Tether USD are included.
	 */
    assetMetadata?: AssetMetadata[]
}

/**
 * An argument when calling {@link connect}.
 */
export interface ConnectRequest {
    config: Config
    mnemonic: string
}

export interface ConnectWithSignerRequest {
    config: Config
}

/**
 * Details about a supported currency in the fiat rate feed
 */
export interface CurrencyInfo {
    name: string
    fractionSize: number
    spacing?: number
    symbol?: SymbolType
    uniqSymbol?: SymbolType
    localizedName: LocalizedName[]
    localeOverrides: LocaleOverrides[]
}

/**
 * Configuration for an external input parser
 */
export interface ExternalInputParser {
	/**
	 * An arbitrary parser provider id
	 */
    providerId: string
	/**
	 * The external parser will be used when an input conforms to this regex
	 */
    inputRegex: string
	/**
	 * The URL of the parser containing a placeholder `<input>` that will be replaced with the
	 * input to be parsed. The input is sanitized using percent encoding.
	 */
    parserUrl: string
}

/**
 * An argument when calling {@link fetchPaymentProposedFees}.
 */
export interface FetchPaymentProposedFeesRequest {
    swapId: string
}

/**
 * Returned when calling {@link fetchPaymentProposedFees}.
 */
export interface FetchPaymentProposedFeesResponse {
    swapId: string
    feesSat: number
	/**
	 * Amount sent by the swap payer
	 */
    payerAmountSat: number
	/**
	 * Amount that will be received if these fees are accepted
	 */
    receiverAmountSat: number
}

export interface FiatCurrency {
    id: string
    info: CurrencyInfo
}

/**
 * Returned when calling {@link getInfo}.
 */
export interface GetInfoResponse {
	/**
	 * The wallet information, such as the balance, fingerprint and public key
	 */
    walletInfo: WalletInfo
	/**
	 * The latest synced blockchain information, such as the Liquid/Bitcoin tips
	 */
    blockchainInfo: BlockchainInfo
}

/**
 * Wrapper for a BOLT11 LN invoice
 */
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

/**
 * Wrapped in a {@link InputType.BOLT12_OFFER}, this is the result of [parse] when given a BOLT12 Offer.
 */
export interface LnOffer {
	/**
	 * String representation of the Bolt12 offer
	 */
    offer: string
    chains: string[]
    paths: LnOfferBlindedPath[]
    description?: string
	/**
	 * The public key used by the recipient to sign invoices
	 */
    signingPubkey?: string
	/**
	 * If set, it represents the minimum amount that an invoice must have to be valid for this offer
	 */
    minAmount?: Amount
	/**
	 * Epoch time from which an invoice should no longer be requested. If None, the offer does not expire
	 */
    absoluteExpiry?: number
    issuer?: string
}

/**
 * Returned when calling {@link fetchLightningLimits}.
 */
export interface LightningPaymentLimitsResponse {
	/**
	 * Amount limits for a Send Payment to be valid
	 */
    send: Limits
	/**
	 * Amount limits for a Receive Payment to be valid
	 */
    receive: Limits
}

/**
 * The minimum and maximum in satoshis of a Lightning or onchain payment.
 */
export interface Limits {
    minSat: number
    maxSat: number
    maxZeroConfSat: number
}

/**
 * Wrapped in a {@link InputType.LIQUID_ADDRESS}, this is the result of [parse] when given a plain or BIP-21 Liquid address.
 */
export interface LiquidAddressData {
    address: string
    network: Network
    assetId?: string
    amount?: number
    amountSat?: number
    label?: string
    message?: string
}

/**
 * An argument when calling {@link listPayments}.
 */
export interface ListPaymentsRequest {
    filters?: PaymentType[]
    states?: PaymentState[]
	/**
	 * Epoch time, in seconds
	 */
    fromTimestamp?: number
	/**
	 * Epoch time, in seconds
	 */
    toTimestamp?: number
    offset?: number
    limit?: number
    details?: ListPaymentDetails
    sortAscending?: boolean
}

export interface LnOfferBlindedPath {
	/**
	 * For each blinded hop, we store the node ID (pubkey as hex).
	 */
    blindedHops: string[]
}

/**
 * Wrapped in a {@link InputType.LN_URL_AUTH}, this is the result of [parse] when given a LNURL-auth endpoint.
 *
 * It represents the endpoint's parameters for the LNURL workflow.
 *
 * See <https://github.com/lnurl/luds/blob/luds/04.md>
 */
export interface LnUrlAuthRequestData {
	/**
	 * Hex encoded 32 bytes of challenge
	 */
    k1: string
	/**
	 * Indicates the domain of the LNURL-auth service, to be shown to the user when asking for
	 * auth confirmation, as per LUD-04 spec.
	 */
    domain: string
	/**
	 * Indicates the URL of the LNURL-auth service, including the query arguments. This will be
	 * extended with the signed challenge and the linking key, then called in the second step of the workflow.
	 */
    url: string
	/**
	 * When available, one of: register, login, link, auth
	 */
    action?: string
}

/**
 * Wrapped in a {@link InputType.LN_URL_ERROR}, this represents a LNURL-endpoint error.
 */
export interface LnUrlErrorData {
    reason: string
}

/**
 * Represents the payment LNURL info
 */
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

/**
 * An argument when calling [crate::sdk::LiquidSdk::lnurl_pay].
 */
export interface LnUrlPayRequest {
	/**
	 * The response from calling [crate::sdk::LiquidSdk::prepare_lnurl_pay]
	 */
    prepareResponse: PrepareLnUrlPayResponse
}

/**
 * Wrapped in a {@link InputType.LN_URL_PAY}, this is the result of [parse] when given a LNURL-pay endpoint.
 *
 * It represents the endpoint's parameters for the LNURL workflow.
 *
 * See <https://github.com/lnurl/luds/blob/luds/06.md>
 */
export interface LnUrlPayRequestData {
    callback: string
	/**
	 * The minimum amount, in millisats, that this LNURL-pay endpoint accepts
	 */
    minSendable: number
	/**
	 * The maximum amount, in millisats, that this LNURL-pay endpoint accepts
	 */
    maxSendable: number
	/**
	 * As per LUD-06, `metadata` is a raw string (e.g. a json representation of the inner map)
	 */
    metadataStr: string
	/**
	 * The comment length accepted by this endpoint
	 *
	 * See <https://github.com/lnurl/luds/blob/luds/12.md>
	 */
    commentAllowed: number
	/**
	 * Indicates the domain of the LNURL-pay service, to be shown to the user when asking for
	 * payment input, as per LUD-06 spec.
	 *
	 * Note: this is not the domain of the callback, but the domain of the LNURL-pay endpoint.
	 */
    domain: string
	/**
	 * Value indicating whether the recipient supports Nostr Zaps through NIP-57.
	 *
	 * See <https://github.com/nostr-protocol/nips/blob/master/57.md>
	 */
    allowsNostr: boolean
	/**
	 * Optional recipient's lnurl provider's Nostr pubkey for NIP-57. If it exists it should be a
	 * valid BIP 340 public key in hex.
	 *
	 * See <https://github.com/nostr-protocol/nips/blob/master/57.md>
	 * See <https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki>
	 */
    nostrPubkey?: string
	/**
	 * If sending to a LN Address, this will be filled.
	 */
    lnAddress?: string
}

export interface LnUrlPaySuccessData {
    successAction?: SuccessActionProcessed
    payment: Payment
}

export interface LnUrlWithdrawRequest {
	/**
	 * Request data containing information on how to call the lnurl withdraw
	 * endpoint. Typically retrieved by calling [parse] on a lnurl withdraw
	 * input.
	 */
    data: LnUrlWithdrawRequestData
	/**
	 * The amount to withdraw from the lnurl withdraw endpoint. Must be between
	 * `minWithdrawable` and `maxWithdrawable`.
	 */
    amountMsat: number
	/**
	 * Optional description that will be put in the payment request for the
	 * lnurl withdraw endpoint.
	 */
    description?: string
}

/**
 * Wrapped in a {@link InputType.LN_URL_WITHDRAW}, this is the result of [parse] when given a LNURL-withdraw endpoint.
 *
 * It represents the endpoint's parameters for the LNURL workflow.
 *
 * See <https://github.com/lnurl/luds/blob/luds/03.md>
 */
export interface LnUrlWithdrawRequestData {
    callback: string
    k1: string
    defaultDescription: string
	/**
	 * The minimum amount, in millisats, that this LNURL-withdraw endpoint accepts
	 */
    minWithdrawable: number
	/**
	 * The maximum amount, in millisats, that this LNURL-withdraw endpoint accepts
	 */
    maxWithdrawable: number
}

export interface LnUrlWithdrawSuccessData {
    invoice: LnInvoice
}

/**
 * Locale-specific settings for the representation of a currency
 */
export interface LocaleOverrides {
    locale: string
    spacing?: number
    symbol: SymbolType
}

/**
 * Localized name of a currency
 */
export interface LocalizedName {
    locale: string
    name: string
}

export interface LogEntry {
    line: string
    level: string
}

/**
 * Wrapper for the {@link SuccessActionProcessed.MESSAGE} payload
 */
export interface MessageSuccessActionData {
    message: string
}

/**
 * Returned when calling {@link fetchOnchainLimits}.
 */
export interface OnchainPaymentLimitsResponse {
	/**
	 * Amount limits for a Send Onchain Payment to be valid
	 */
    send: Limits
	/**
	 * Amount limits for a Receive Onchain Payment to be valid
	 */
    receive: Limits
}

/**
 * An argument when calling {@link payOnchain}.
 */
export interface PayOnchainRequest {
    address: string
    prepareResponse: PreparePayOnchainResponse
}

/**
 * Represents an SDK payment.
 *
 * By default, this is an onchain tx. It may represent a swap, if swap metadata is available.
 */
export interface Payment {
	/**
	 * Composite timestamp that can be used for sorting or displaying the payment.
	 *
	 * If this payment has an associated swap, it is the swap creation time. Otherwise, the point
	 * in time when the underlying tx was included in a block. If there is no associated swap
	 * available and the underlying tx is not yet confirmed, the value is 'now()'.
	 */
    timestamp: number
	/**
	 * The payment amount, which corresponds to the onchain tx amount.
	 *
	 * In case of an outbound payment (Send), this is the payer amount. Otherwise it's the receiver amount.
	 */
    amountSat: number
	/**
	 * Represents the fees paid by this wallet for this payment.
	 *
	 * ### Swaps
	 * If there is an associated Send Swap, these fees represent the total fees paid by this wallet
	 * (the sender). It is the difference between the amount that was sent and the amount received.
	 *
	 * If there is an associated Receive Swap, these fees represent the total fees paid by this wallet
	 * (the receiver). It is also the difference between the amount that was sent and the amount received.
	 *
	 * ### Pure onchain txs
	 * If no swap is associated with this payment:
	 * - for Send payments, this is the onchain tx fee
	 * - for Receive payments, this is zero
	 */
    feesSat: number
	/**
	 * If it is a {@link PaymentType.SEND} or {@link PaymentType.RECEIVE} payment
	 */
    paymentType: PaymentType
	/**
	 * Composite status representing the overall status of the payment.
	 *
	 * If the tx has no associated swap, this reflects the onchain tx status (confirmed or not).
	 *
	 * If the tx has an associated swap, this is determined by the swap status (pending or complete).
	 */
    status: PaymentState
	/**
	 * The details of a payment, depending on its {@link Payment}'s `destination` and
	 * {@link Payment}'s `type`.
	 */
    details: PaymentDetails
	/**
	 * Service fees paid to the swapper service. This is only set for swaps (i.e. doesn't apply to
	 * direct Liquid payments).
	 */
    swapperFeesSat?: number
	/**
	 * The destination associated with the payment, if it was created via our SDK.
	 * Can be either a Liquid/Bitcoin address, a Liquid BIP21 URI or an invoice
	 */
    destination?: string
    txId?: string
	/**
	 * Data to use in the "blinded" param when unblinding the transaction in an explorer.
	 * See: <https://docs.liquid.net/docs/unblinding-transactions>
	 */
    unblindingData?: string
}

/**
 * An argument when calling {@link prepareBuyBitcoin}.
 */
export interface PrepareBuyBitcoinRequest {
    provider: BuyBitcoinProvider
    amountSat: number
}

/**
 * Returned when calling {@link prepareBuyBitcoin}.
 */
export interface PrepareBuyBitcoinResponse {
    provider: BuyBitcoinProvider
    amountSat: number
    feesSat: number
}

/**
 * An argument when calling [crate::sdk::LiquidSdk::prepare_lnurl_pay].
 */
export interface PrepareLnUrlPayRequest {
	/**
	 * The {@link LnUrlPayRequestData} returned by [crate::input_parser::parse]
	 */
    data: LnUrlPayRequestData
	/**
	 * The amount to send
	 */
    amount: PayAmount
	/**
	 * An optional comment for this payment
	 */
    comment?: string
	/**
	 * Validates that, if there is a URL success action, the URL domain matches
	 * the LNURL callback domain. Defaults to true
	 */
    validateSuccessActionUrl?: boolean
}

/**
 * Returned when calling [crate::sdk::LiquidSdk::prepare_lnurl_pay].
 */
export interface PrepareLnUrlPayResponse {
	/**
	 * The destination of the payment
	 */
    destination: SendDestination
	/**
	 * The fees in satoshis to send the payment
	 */
    feesSat: number
	/**
	 * The {@link LnUrlPayRequestData} returned by [crate::input_parser::parse]
	 */
    data: LnUrlPayRequestData
	/**
	 * An optional comment for this payment
	 */
    comment?: string
	/**
	 * The unprocessed LUD-09 success action. This will be processed and decrypted if
	 * needed after calling [crate::sdk::LiquidSdk::lnurl_pay]
	 */
    successAction?: SuccessAction
}

/**
 * An argument when calling {@link preparePayOnchain}.
 */
export interface PreparePayOnchainRequest {
	/**
	 * The amount to send
	 */
    amount: PayAmount
	/**
	 * The optional fee rate of the Bitcoin claim transaction in sat/vB. Defaults to the swapper estimated claim fee.
	 */
    feeRateSatPerVbyte?: number
}

/**
 * Returned when calling {@link preparePayOnchain}.
 */
export interface PreparePayOnchainResponse {
    receiverAmountSat: number
    claimFeesSat: number
    totalFeesSat: number
}

/**
 * An argument when calling {@link prepareReceivePayment}.
 */
export interface PrepareReceiveRequest {
    paymentMethod: PaymentMethod
    amount?: ReceiveAmount
}

/**
 * Returned when calling {@link prepareReceivePayment}.
 */
export interface PrepareReceiveResponse {
    paymentMethod: PaymentMethod
	/**
	 * Generally represents the total fees that would be paid to send or receive this payment.
	 *
	 * In case of Zero-Amount Receive Chain swaps, the swapper service fee (`swapperFeerate` times
	 * the amount) is paid in addition to `feesSat`. The swapper service feerate is already known
	 * in the beginning, but the exact swapper service fee will only be known when the
	 * `payerAmountSat` is known.
	 *
	 * In all other types of swaps, the swapper service fee is included in `feesSat`.
	 */
    feesSat: number
    amount?: ReceiveAmount
	/**
	 * The minimum amount the payer can send for this swap to succeed.
	 *
	 * When the method is {@link PaymentMethod.LIQUID_ADDRESS}, this is empty.
	 */
    minPayerAmountSat?: number
	/**
	 * The maximum amount the payer can send for this swap to succeed.
	 *
	 * When the method is {@link PaymentMethod.LIQUID_ADDRESS}, this is empty.
	 */
    maxPayerAmountSat?: number
	/**
	 * The percentage of the sent amount that will count towards the service fee.
	 *
	 * When the method is {@link PaymentMethod.LIQUID_ADDRESS}, this is empty.
	 */
    swapperFeerate?: number
}

/**
 * An argument when calling {@link prepareRefund}.
 */
export interface PrepareRefundRequest {
	/**
	 * The address where the swap funds are locked up
	 */
    swapAddress: string
	/**
	 * The address to refund the swap funds to
	 */
    refundAddress: string
	/**
	 * The fee rate in sat/vB for the refund transaction
	 */
    feeRateSatPerVbyte: number
}

/**
 * Returned when calling {@link prepareRefund}.
 */
export interface PrepareRefundResponse {
    txVsize: number
    txFeeSat: number
    lastRefundTxId?: string
}

/**
 * An argument when calling {@link prepareSendPayment}.
 */
export interface PrepareSendRequest {
	/**
	 * The destination we intend to pay to.
	 * Supports BIP21 URIs, BOLT11 invoices and Liquid addresses
	 */
    destination: string
	/**
	 * Should only be set when paying directly onchain or to a BIP21 URI
	 * where no amount is specified, or when the caller wishes to drain
	 */
    amount?: PayAmount
}

/**
 * Returned when calling {@link prepareSendPayment}.
 */
export interface PrepareSendResponse {
    destination: SendDestination
    feesSat: number
}

/**
 * Denominator in an exchange rate
 */
export interface Rate {
    coin: string
    value: number
}

/**
 * An argument when calling {@link receivePayment}.
 */
export interface ReceivePaymentRequest {
    prepareResponse: PrepareReceiveResponse
	/**
	 * The description for this payment request.
	 */
    description?: string
	/**
	 * If set to true, then the hash of the description will be used.
	 */
    useDescriptionHash?: boolean
}

/**
 * Returned when calling {@link receivePayment}.
 */
export interface ReceivePaymentResponse {
	/**
	 * Either a BIP21 URI (Liquid or Bitcoin), a Liquid address
	 * or an invoice, depending on the {@link PrepareReceiveResponse} parameters
	 */
    destination: string
}

/**
 * Returned when calling {@link recommendedFees}.
 */
export interface RecommendedFees {
    fastestFee: number
    halfHourFee: number
    hourFee: number
    economyFee: number
    minimumFee: number
}

/**
 * An argument when calling {@link refund}.
 */
export interface RefundRequest {
	/**
	 * The address where the swap funds are locked up
	 */
    swapAddress: string
	/**
	 * The address to refund the swap funds to
	 */
    refundAddress: string
	/**
	 * The fee rate in sat/vB for the refund transaction
	 */
    feeRateSatPerVbyte: number
}

/**
 * Returned when calling {@link refund}.
 */
export interface RefundResponse {
    refundTxId: string
}

/**
 * Returned when calling {@link listRefundables}.
 */
export interface RefundableSwap {
    swapAddress: string
    timestamp: number
	/**
	 * Amount that is refundable, from all UTXOs
	 */
    amountSat: number
	/**
	 * The txid of the last broadcasted refund tx, if any
	 */
    lastRefundTxId?: string
}

/**
 * An argument when calling {@link restore}.
 */
export interface RestoreRequest {
    backupPath?: string
}

/**
 * A route hint for a LN payment
 */
export interface RouteHint {
    hops: RouteHintHop[]
}

/**
 * Details of a specific hop in a larger route hint
 */
export interface RouteHintHop {
	/**
	 * The node id of the non-target end of the route
	 */
    srcNodeId: string
	/**
	 * The short channel id of this channel
	 */
    shortChannelId: string
	/**
	 * The fees which must be paid to use this channel
	 */
    feesBaseMsat: number
    feesProportionalMillionths: number
	/**
	 * The difference in CLTV values between this node and the next node
	 */
    cltvExpiryDelta: number
	/**
	 * The minimum value, in msat, which must be relayed to the next hop
	 */
    htlcMinimumMsat?: number
	/**
	 * The maximum value in msat available for routing with a single HTLC
	 */
    htlcMaximumMsat?: number
}

/**
 * An argument when calling {@link sendPayment}.
 */
export interface SendPaymentRequest {
    prepareResponse: PrepareSendResponse
}

/**
 * Returned when calling {@link sendPayment}.
 */
export interface SendPaymentResponse {
    payment: Payment
}

/**
 * An argument when calling {@link signMessage}.
 */
export interface SignMessageRequest {
    message: string
}

/**
 * Returned when calling {@link signMessage}.
 */
export interface SignMessageResponse {
    signature: string
}

/**
 * Settings for the symbol representation of a currency
 */
export interface SymbolType {
    grapheme?: string
    template?: string
    rtl?: boolean
    position?: number
}

/**
 * Wrapper for the {@link SuccessActionProcessed.URL} payload
 */
export interface UrlSuccessActionData {
	/**
	 * Contents description, up to 144 characters
	 */
    description: string
	/**
	 * URL of the success action
	 */
    url: string
	/**
	 * Indicates the success URL domain matches the LNURL callback domain.
	 *
	 * See <https://github.com/lnurl/luds/blob/luds/09.md>
	 */
    matchesCallbackDomain: boolean
}

export interface WalletInfo {
	/**
	 * Usable balance. This is the confirmed onchain balance minus `pendingSendSat`.
	 */
    balanceSat: number
	/**
	 * Amount that is being used for ongoing Send swaps
	 */
    pendingSendSat: number
	/**
	 * Incoming amount that is pending from ongoing Receive swaps
	 */
    pendingReceiveSat: number
    fingerprint: string
    pubkey: string
    assetBalances: AssetBalance[]
}

export enum AesSuccessActionDataResultVariant {
    DECRYPTED = "decrypted",
    ERROR_STATUS = "errorStatus"
}

/**
 * Result of decryption of {@link SuccessActionProcessed.AES} payload
 */
export type AesSuccessActionDataResult = {
    type: AesSuccessActionDataResultVariant.DECRYPTED,
    data: AesSuccessActionDataDecrypted
} | {
    type: AesSuccessActionDataResultVariant.ERROR_STATUS,
    reason: string
}

export enum AmountVariant {
    BITCOIN = "bitcoin",
	/**
	 * An amount of currency specified using ISO 4712
	 */
    CURRENCY = "currency"
}

/**
 * Different kinds of inputs supported by [parse], including any relevant details extracted from the input.
 */
export type Amount = {
    type: AmountVariant.BITCOIN,
    amountMsat: number
} | {
    type: AmountVariant.CURRENCY,
    iso4217Code: string
    fractionalAmount: number
}

/**
 * An argument of {@link PrepareBuyBitcoinRequest} when calling {@link prepareBuyBitcoin}.
 */
export enum BuyBitcoinProvider {
    MOONPAY = "moonpay"
}

export enum GetPaymentRequestVariant {
	/**
	 * The Lightning payment hash of the payment
	 */
    LIGHTNING = "lightning"
}

export interface GetPaymentRequest {
    type: GetPaymentRequestVariant.LIGHTNING,
    paymentHash: string
}

export enum InputTypeVariant {
	/**
	 * # Supported standards
	 *
	 * - plain on-chain BTC address
	 * - BIP21
	 */
    BITCOIN_ADDRESS = "bitcoinAddress",
	/**
	 * # Supported standards
	 *
	 * - plain on-chain liquid address
	 * - BIP21 on liquid/liquidtestnet
	 */
    LIQUID_ADDRESS = "liquidAddress",
    BOLT11 = "bolt11",
    BOLT12_OFFER = "bolt12Offer",
    NODE_ID = "nodeId",
    URL = "url",
	/**
	 * # Supported standards
	 *
	 * - LUD-01 LNURL bech32 encoding
	 * - LUD-06 `payRequest` spec
	 * - LUD-16 LN Address
	 * - LUD-17 Support for lnurlp prefix with non-bech32-encoded LNURL URLs
	 */
    LN_URL_PAY = "lnUrlPay",
	/**
	 * # Supported standards
	 *
	 * - LUD-01 LNURL bech32 encoding
	 * - LUD-03 `withdrawRequest` spec
	 * - LUD-17 Support for lnurlw prefix with non-bech32-encoded LNURL URLs
	 *
	 * # Not supported (yet)
	 *
	 * - LUD-14 `balanceCheck`: reusable `withdrawRequest`s
	 * - LUD-19 Pay link discoverable from withdraw link
	 */
    LN_URL_WITHDRAW = "lnUrlWithdraw",
	/**
	 * # Supported standards
	 *
	 * - LUD-01 LNURL bech32 encoding
	 * - LUD-04 `auth` base spec
	 * - LUD-17 Support for keyauth prefix with non-bech32-encoded LNURL URLs
	 */
    LN_URL_AUTH = "lnUrlAuth",
	/**
	 * Error returned by the LNURL endpoint
	 */
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

/**
 * Network chosen for this Liquid SDK instance. Note that it represents both the Liquid and the
 * Bitcoin network used.
 */
export enum LiquidNetwork {
	/**
	 * Mainnet Bitcoin and Liquid chains
	 */
    MAINNET = "mainnet",
	/**
	 * Testnet Bitcoin and Liquid chains
	 */
    TESTNET = "testnet"
}

export enum ListPaymentDetailsVariant {
	/**
	 * A Liquid payment
	 */
    LIQUID = "liquid",
	/**
	 * A Bitcoin payment
	 */
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

/**
 * Contains the result of the entire LNURL interaction, as reported by the LNURL endpoint.
 *
 * * {@link LnUrlCallbackStatus.OK} indicates the interaction with the endpoint was valid, and the endpoint
 *  - started to pay the invoice asynchronously in the case of LNURL-withdraw,
 *  - verified the client signature in the case of LNURL-auth
 * * {@link LnUrlCallbackStatus.ERROR_STATUS} indicates a generic issue the LNURL endpoint encountered, including a freetext
 *    description of the reason.
 *
 * Both cases are described in LUD-03 <https://github.com/lnurl/luds/blob/luds/03.md> & LUD-04: <https://github.com/lnurl/luds/blob/luds/04.md>
 */
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

/**
 * Contains the result of the entire LNURL-pay interaction, as reported by the LNURL endpoint.
 *
 * * {@link LnUrlPayResult.ENDPOINT_SUCCESS} indicates the payment is complete. The endpoint may return a {@link SuccessActionProcessed},
 *   in which case, the wallet has to present it to the user as described in
 *   <https://github.com/lnurl/luds/blob/luds/09.md>
 *
 * * {@link LnUrlPayResult.ENDPOINT_ERROR} indicates a generic issue the LNURL endpoint encountered, including a freetext
 *   field with the reason.
 *
 * * {@link LnUrlPayResult.PAY_ERROR} indicates that an error occurred while trying to pay the invoice from the LNURL endpoint.
 *   This includes the payment hash of the failed invoice and the failure reason.
 */
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

/**
 * {@link LnUrlCallbackStatus} specific to LNURL-withdraw, where the success case contains the invoice.
 */
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

/**
 * The different supported bitcoin networks
 */
export enum Network {
	/**
	 * Mainnet
	 */
    BITCOIN = "bitcoin",
    TESTNET = "testnet",
    SIGNET = "signet",
    REGTEST = "regtest"
}

export enum PayAmountVariant {
	/**
	 * The amount in satoshi that will be received
	 */
    BITCOIN = "bitcoin",
	/**
	 * The amount of an asset that will be received
	 */
    ASSET = "asset",
	/**
	 * Indicates that all available Bitcoin funds should be sent
	 */
    DRAIN = "drain"
}

/**
 * Used to specify the amount to sent or to send all funds.
 */
export type PayAmount = {
    type: PayAmountVariant.BITCOIN,
    receiverAmountSat: number
} | {
    type: PayAmountVariant.ASSET,
    assetId: string
    receiverAmount: number
} | {
    type: PayAmountVariant.DRAIN
}

export enum PaymentDetailsVariant {
	/**
	 * Swapping to or from Lightning
	 */
    LIGHTNING = "lightning",
	/**
	 * Direct onchain payment to a Liquid address
	 */
    LIQUID = "liquid",
	/**
	 * Swapping to or from the Bitcoin chain
	 */
    BITCOIN = "bitcoin"
}

/**
 * The specific details of a payment, depending on its type
 */
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
    refundTxId?: string
    refundTxAmountSat?: number
} | {
    type: PaymentDetailsVariant.LIQUID,
    assetId: string
    destination: string
    description: string
    assetInfo?: AssetInfo
} | {
    type: PaymentDetailsVariant.BITCOIN,
    swapId: string
    description: string
    autoAcceptedFees: boolean
    bitcoinExpirationBlockheight?: number
    liquidExpirationBlockheight?: number
    refundTxId?: string
    refundTxAmountSat?: number
}

/**
 * The send/receive methods supported by the SDK
 */
export enum PaymentMethod {
    LIGHTNING = "lightning",
    BITCOIN_ADDRESS = "bitcoinAddress",
    LIQUID_ADDRESS = "liquidAddress"
}

/**
 * The payment state of an individual payment.
 */
export enum PaymentState {
    CREATED = "created",
	/**
	 * ## Receive Swaps
	 *
	 * Covers the cases when
	 * - the lockup tx is seen in the mempool or
	 * - our claim tx is broadcast
	 *
	 * When the claim tx is broadcast, `claimTxId` is set in the swap.
	 *
	 * ## Send Swaps
	 *
	 * This is the status when our lockup tx was broadcast
	 *
	 * ## Chain Swaps
	 *
	 * This is the status when the user lockup tx was broadcast
	 *
	 * ## No swap data available
	 *
	 * If no associated swap is found, this indicates the underlying tx is not confirmed yet.
	 */
    PENDING = "pending",
	/**
	 * ## Receive Swaps
	 *
	 * Covers the case when the claim tx is confirmed.
	 *
	 * ## Send and Chain Swaps
	 *
	 * This is the status when the claim tx is broadcast and we see it in the mempool.
	 *
	 * ## No swap data available
	 *
	 * If no associated swap is found, this indicates the underlying tx is confirmed.
	 */
    COMPLETE = "complete",
	/**
	 * ## Receive Swaps
	 *
	 * This is the status when the swap failed for any reason and the Receive could not complete.
	 *
	 * ## Send and Chain Swaps
	 *
	 * This is the status when a swap refund was initiated and the refund tx is confirmed.
	 */
    FAILED = "failed",
	/**
	 * ## Send and Outgoing Chain Swaps
	 *
	 * This covers the case when the swap state is still Created and the swap fails to reach the
	 * Pending state in time. The TimedOut state indicates the lockup tx should never be broadcast.
	 */
    TIMED_OUT = "timedOut",
	/**
	 * ## Incoming Chain Swaps
	 *
	 * This covers the case when the swap failed for any reason and there is a user lockup tx.
	 * The swap in this case has to be manually refunded with a provided Bitcoin address
	 */
    REFUNDABLE = "refundable",
	/**
	 * ## Send and Chain Swaps
	 *
	 * This is the status when a refund was initiated and/or our refund tx was broadcast
	 *
	 * When the refund tx is broadcast, `refundTxId` is set in the swap.
	 */
    REFUND_PENDING = "refundPending",
	/**
	 * ## Chain Swaps
	 *
	 * This is the state when the user needs to accept new fees before the payment can proceed.
	 *
	 * Use {@link fetchPaymentProposedFees} to find out the current fees and
	 * {@link acceptPaymentProposedFees} to accept them, allowing the payment to proceed.
	 *
	 * Otherwise, this payment can be immediately refunded using
	 * {@link prepareRefund}/{@link refund}.
	 */
    WAITING_FEE_ACCEPTANCE = "waitingFeeAcceptance"
}

export enum PaymentType {
    RECEIVE = "receive",
    SEND = "send"
}

export enum ReceiveAmountVariant {
	/**
	 * The amount in satoshi that should be paid
	 */
    BITCOIN = "bitcoin",
	/**
	 * The amount of an asset that should be paid
	 */
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
    SYNCED = "synced"
}

/**
 * Event emitted by the SDK. Add an {@link EventListener} by calling {@link addEventListener}
 * to listen for emitted events.
 */
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
}

export enum SendDestinationVariant {
    LIQUID_ADDRESS = "liquidAddress",
    BOLT11 = "bolt11",
    BOLT12 = "bolt12"
}

/**
 * Specifies the supported destinations which can be payed by the SDK
 */
export type SendDestination = {
    type: SendDestinationVariant.LIQUID_ADDRESS,
    addressData: LiquidAddressData
} | {
    type: SendDestinationVariant.BOLT11,
    invoice: LnInvoice
} | {
    type: SendDestinationVariant.BOLT12,
    offer: LnOffer
    receiverAmountSat: number
}

export enum SuccessActionVariant {
	/**
	 * AES type, described in LUD-10
	 */
    AES = "aes",
	/**
	 * Message type, described in LUD-09
	 */
    MESSAGE = "message",
	/**
	 * URL type, described in LUD-09
	 */
    URL = "url"
}

/**
 * Supported success action types
 *
 * Receiving any other (unsupported) success action type will result in a failed parsing,
 * which will abort the LNURL-pay workflow, as per LUD-09.
 */
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
	/**
	 * See {@link SuccessAction.AES} for received payload
	 *
	 * See {@link AesSuccessActionDataDecrypted} for decrypted payload
	 */
    AES = "aes",
	/**
	 * See {@link SuccessAction.MESSAGE}
	 */
    MESSAGE = "message",
	/**
	 * See {@link SuccessAction.URL}
	 */
    URL = "url"
}

/**
 * {@link SuccessAction} where contents are ready to be consumed by the caller
 *
 * Contents are identical to {@link SuccessAction}, except for AES where the ciphertext is decrypted.
 */
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

/**
 * Interface that can be used to receive {@link SdkEvent}s emitted by the SDK.
 */
export type EventListener = (e: SdkEvent) => void

/**
 * Interface that can be used to receive {@link LogEntry}s emitted by the SDK.
 */
export type Logger = (logEntry: LogEntry) => void

/**
 * Initializes the SDK services and starts the background tasks.
 * This must be called to create the {@link BindingLiquidSdk} instance.
 *
 * # Arguments
 *
 * * `req` - the {@link ConnectRequest} containing:
 *     * `mnemonic` - the Liquid wallet mnemonic
 *     * `config` - the SDK {@link Config}
 */
export const connect = async (req: ConnectRequest): Promise<void> => {
    const response = await BreezSDKLiquid.connect(req)
    return response
}

/**
 * Adds an event listener to the [LiquidSdk] instance, where all {@link SdkEvent}'s will be emitted to.
 * The event listener can be removed be calling {@link removeEventListener}.
 *
 * # Arguments
 *
 * * `listener` - The listener which is an implementation of the {@link EventListener} trait
 */
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

/**
 * Get the full default {@link Config} for specific {@link LiquidNetwork}.
 */
export const defaultConfig = async (network: LiquidNetwork, breezApiKey: string = ""): Promise<Config> => {
    const response = await BreezSDKLiquid.defaultConfig(network, breezApiKey)
    return response
}

/**
 * Parses a string into an {@link LnInvoice}.
 */
export const parseInvoice = async (input: string): Promise<LnInvoice> => {
    const response = await BreezSDKLiquid.parseInvoice(input)
    return response
}


/**
 * Accepts proposed fees for a {@link Payment} that is [WaitingFeeAcceptance].
 *
 * Use {@link fetchPaymentProposedFees} to get an up-to-date fees proposal.
 */
export const acceptPaymentProposedFees = async (req: AcceptPaymentProposedFeesRequest): Promise<void> => {
    await BreezSDKLiquid.acceptPaymentProposedFees(req)
}

/**
 * Backup the local state to the provided backup path.
 *
 * # Arguments
 *
 * * `req` - the {@link BackupRequest} containing:
 *     * `backupPath` - the optional backup path. Defaults to {@link Config.workingDir}
 */
export const backup = async (req: BackupRequest): Promise<void> => {
    await BreezSDKLiquid.backup(req)
}

/**
 * Generate a URL to a third party provider used to buy Bitcoin via a chain swap.
 *
 * # Arguments
 *
 * * `req` - the {@link BuyBitcoinRequest} containing:
 *     * `prepareResponse` - the {@link PrepareBuyBitcoinResponse} from calling {@link prepareBuyBitcoin}
 *     * `redirectUrl` - the optional redirect URL the provider should redirect to after purchase
 */
export const buyBitcoin = async (req: BuyBitcoinRequest): Promise<string> => {
    const response = await BreezSDKLiquid.buyBitcoin(req)
    return response
}

/**
 * Check whether given message was signed by the given
 * pubkey and the signature (zbase encoded) is valid.
 */
export const checkMessage = async (req: CheckMessageRequest): Promise<CheckMessageResponse> => {
    const response = await BreezSDKLiquid.checkMessage(req)
    return response
}

/**
 * Disconnects the {@link BindingLiquidSdk} instance and stops the background tasks.
 */
export const disconnect = async (): Promise<void> => {
    await BreezSDKLiquid.disconnect()
}

/**
 * Fetch live rates of fiat currencies, sorted by name.
 */
export const fetchFiatRates = async (): Promise<Rate[]> => {
    const response = await BreezSDKLiquid.fetchFiatRates()
    return response
}

/**
 * Fetch the current payment limits for {@link sendPayment} and {@link receivePayment}.
 */
export const fetchLightningLimits = async (): Promise<LightningPaymentLimitsResponse> => {
    const response = await BreezSDKLiquid.fetchLightningLimits()
    return response
}

/**
 * Fetch the current payment limits for {@link payOnchain} and {@link receiveOnchain}.
 */
export const fetchOnchainLimits = async (): Promise<OnchainPaymentLimitsResponse> => {
    const response = await BreezSDKLiquid.fetchOnchainLimits()
    return response
}

/**
 * Fetches an up-to-date fees proposal for a {@link Payment} that is [WaitingFeeAcceptance].
 *
 * Use {@link acceptPaymentProposedFees} to accept the proposed fees and proceed
 * with the payment.
 */
export const fetchPaymentProposedFees = async (req: FetchPaymentProposedFeesRequest): Promise<FetchPaymentProposedFeesResponse> => {
    const response = await BreezSDKLiquid.fetchPaymentProposedFees(req)
    return response
}

/**
 * Get the wallet info, calculating the current pending and confirmed balances.
 */
export const getInfo = async (): Promise<GetInfoResponse> => {
    const response = await BreezSDKLiquid.getInfo()
    return response
}

/**
 * Retrieves a payment.
 *
 * # Arguments
 *
 * * `req` - the {@link GetPaymentRequest} containing:
 *     * {@link GetPaymentRequest.LIGHTNING} - the `paymentHash` of the lightning invoice
 *
 * # Returns
 *
 * Returns an `Option<Payment>` if found, or `none` if no payment matches the given request.
 */
export const getPayment = async (req: GetPaymentRequest): Promise<Payment | null> => {
    const response = await BreezSDKLiquid.getPayment(req)
    return response
}

/**
 * List all supported fiat currencies for which there is a known exchange rate.
 * List is sorted by the canonical name of the currency.
 */
export const listFiatCurrencies = async (): Promise<FiatCurrency[]> => {
    const response = await BreezSDKLiquid.listFiatCurrencies()
    return response
}

/**
 * Lists the SDK payments in reverse chronological order, from newest to oldest.
 * The payments are determined based on onchain transactions and swaps.
 */
export const listPayments = async (req: ListPaymentsRequest): Promise<Payment[]> => {
    const response = await BreezSDKLiquid.listPayments(req)
    return response
}

/**
 * List all failed chain swaps that need to be refunded.
 * They can be refunded by calling {@link prepareRefund} then {@link refund}.
 */
export const listRefundables = async (): Promise<RefundableSwap[]> => {
    const response = await BreezSDKLiquid.listRefundables()
    return response
}

/**
 * Third and last step of LNURL-auth. The first step is [parse], which also validates the LNURL destination
 * and generates the {@link LnUrlAuthRequestData} payload needed here. The second step is user approval of auth action.
 *
 * This call will sign the {@link LnUrlAuthRequestData.k1} of the `reqData` using the derived linking private key and DER-encodes the signature.
 * If they match the endpoint requirements, the LNURL auth request is made. A successful result here means the client signature is verified.
 */
export const lnurlAuth = async (reqData: LnUrlAuthRequestData): Promise<LnUrlCallbackStatus> => {
    const response = await BreezSDKLiquid.lnurlAuth(reqData)
    return response
}

export const lnurlPay = async (req: LnUrlPayRequest): Promise<LnUrlPayResult> => {
    const response = await BreezSDKLiquid.lnurlPay(req)
    return response
}

/**
 * Second step of LNURL-withdraw. The first step is [parse], which also validates the LNURL destination
 * and generates the {@link LnUrlWithdrawRequest} payload needed here.
 *
 * This call will validate the given `amountMsat` against the parameters
 * of the {@link LnUrlWithdrawRequestData.data}. If they match the endpoint requirements, the LNURL withdraw
 * request is made. A successful result here means the endpoint started the payment.
 */
export const lnurlWithdraw = async (req: LnUrlWithdrawRequest): Promise<LnUrlWithdrawResult> => {
    const response = await BreezSDKLiquid.lnurlWithdraw(req)
    return response
}

/**
 * Parses a string into an {@link InputType}.
 */
export const parse = async (input: string): Promise<InputType> => {
    const response = await BreezSDKLiquid.parse(input)
    return response
}

/**
 * Pays to a Bitcoin address via a chain swap.
 *
 * Depending on {@link Config}'s `paymentTimeoutSec`, this function will return:
 * * {@link PaymentState.PENDING} payment - if the payment could be initiated but didn't yet
 *   complete in this time
 * * {@link PaymentState.COMPLETE} payment - if the payment was successfully completed in this time
 *
 * # Arguments
 *
 * * `req` - the {@link PayOnchainRequest} containing:
 *     * `address` - the Bitcoin address to pay to
 *     * `prepareResponse` - the {@link PreparePayOnchainResponse} from calling {@link preparePayOnchain}
 *
 * # Errors
 *
 * * {@link PaymentError.PAYMENT_TIMEOUT} - if the payment could not be initiated in this time
 */
export const payOnchain = async (req: PayOnchainRequest): Promise<SendPaymentResponse> => {
    const response = await BreezSDKLiquid.payOnchain(req)
    return response
}

/**
 * Prepares to buy Bitcoin via a chain swap.
 *
 * # Arguments
 *
 * * `req` - the {@link PrepareBuyBitcoinRequest} containing:
 *     * `provider` - the {@link BuyBitcoinProvider} to use
 *     * `amountSat` - the amount in satoshis to buy from the provider
 */
export const prepareBuyBitcoin = async (req: PrepareBuyBitcoinRequest): Promise<PrepareBuyBitcoinResponse> => {
    const response = await BreezSDKLiquid.prepareBuyBitcoin(req)
    return response
}

/**
 * Second step of LNURL-pay. The first step is [parse], which also validates the LNURL destination
 * and generates the {@link LnUrlPayRequest} payload needed here.
 *
 * This call will validate the `amountMsat` and `comment` parameters of `req` against the parameters
 * of the {@link LnUrlPayRequestData.reqData}. If they match the endpoint requirements, the LNURL payment
 * is made.
 */
export const prepareLnurlPay = async (req: PrepareLnUrlPayRequest): Promise<PrepareLnUrlPayResponse> => {
    const response = await BreezSDKLiquid.prepareLnurlPay(req)
    return response
}

/**
 * Prepares to pay to a Bitcoin address via a chain swap.
 *
 * # Arguments
 *
 * * `req` - the {@link PreparePayOnchainRequest} containing:
 *     * `amount` - which can be of two types: {@link PayAmount.DRAIN}, which uses all funds,
 *        and {@link PayAmount.BITCOIN}, which sets the amount the receiver should receive
 *     * `feeRateSatPerVbyte` - the optional fee rate of the Bitcoin claim transaction. Defaults to the swapper estimated claim fee
 */
export const preparePayOnchain = async (req: PreparePayOnchainRequest): Promise<PreparePayOnchainResponse> => {
    const response = await BreezSDKLiquid.preparePayOnchain(req)
    return response
}

/**
 * Prepares to receive a Lightning payment via a reverse submarine swap.
 *
 * # Arguments
 *
 * * `req` - the {@link PrepareReceiveRequest} containing:
 *     * `payerAmountSat` - the amount in satoshis to be paid by the payer
 *     * `paymentMethod` - the supported payment methods; either an invoice, a Liquid address or a Bitcoin address
 */
export const prepareReceivePayment = async (req: PrepareReceiveRequest): Promise<PrepareReceiveResponse> => {
    const response = await BreezSDKLiquid.prepareReceivePayment(req)
    return response
}

/**
 * Prepares to refund a failed chain swap by calculating the refund transaction size and absolute fee.
 *
 * # Arguments
 *
 * * `req` - the {@link PrepareRefundRequest} containing:
 *     * `swapAddress` - the swap address to refund from {@link RefundableSwap.swapAddress}
 *     * `refundAddress` - the Bitcoin address to refund to
 *     * `feeRateSatPerVbyte` - the fee rate at which to broadcast the refund transaction
 */
export const prepareRefund = async (req: PrepareRefundRequest): Promise<PrepareRefundResponse> => {
    const response = await BreezSDKLiquid.prepareRefund(req)
    return response
}

/**
 * Prepares to pay a Lightning invoice via a submarine swap.
 *
 * # Arguments
 *
 * * `req` - the {@link PrepareSendRequest} containing:
 *     * `destination` - Either a Liquid BIP21 URI/address or a BOLT11 invoice
 *     * `amountSat` - Should only be specified when paying directly onchain or via amount-less BIP21
 *
 * # Returns
 * Returns a {@link PrepareSendResponse} containing:
 *     * `destination` - the parsed destination, of type {@link SendDestination}
 *     * `feesSat` - the additional fees which will be paid by the sender
 */
export const prepareSendPayment = async (req: PrepareSendRequest): Promise<PrepareSendResponse> => {
    const response = await BreezSDKLiquid.prepareSendPayment(req)
    return response
}

/**
 * Receive a Lightning payment via a reverse submarine swap, a chain swap or via direct Liquid
 * payment.
 *
 * # Arguments
 *
 * * `req` - the {@link ReceivePaymentRequest} containing:
 *     * `prepareResponse` - the {@link PrepareReceiveResponse} from calling {@link prepareReceivePayment}
 *     * `description` - the optional payment description
 *     * `useDescriptionHash` - optional if true uses the hash of the description
 *
 * # Returns
 *
 * * A {@link ReceivePaymentResponse} containing:
 *     * `destination` - the final destination to be paid by the payer, either a BIP21 URI (Liquid or Bitcoin), a Liquid address or an invoice
 */
export const receivePayment = async (req: ReceivePaymentRequest): Promise<ReceivePaymentResponse> => {
    const response = await BreezSDKLiquid.receivePayment(req)
    return response
}

/**
 * Get the recommended Bitcoin fees based on the configured mempool.space instance.
 */
export const recommendedFees = async (): Promise<RecommendedFees> => {
    const response = await BreezSDKLiquid.recommendedFees()
    return response
}

/**
 * Refund a failed chain swap.
 *
 * # Arguments
 *
 * * `req` - the {@link RefundRequest} containing:
 *     * `swapAddress` - the swap address to refund from {@link RefundableSwap.swapAddress}
 *     * `refundAddress` - the Bitcoin address to refund to
 *     * `feeRateSatPerVbyte` - the fee rate at which to broadcast the refund transaction
 */
export const refund = async (req: RefundRequest): Promise<RefundResponse> => {
    const response = await BreezSDKLiquid.refund(req)
    return response
}

/**
 * Register for webhook callbacks at the given `webhookUrl`. Each created swap after registering the
 * webhook will include the `webhookUrl`.
 *
 * This method should be called every time the application is started and when the `webhookUrl` changes.
 * For example, if the `webhookUrl` contains a push notification token and the token changes after
 * the application was started, then this method should be called to register for callbacks at
 * the new correct `webhookUrl`. To unregister a webhook call {@link unregisterWebhook}.
 */
export const registerWebhook = async (webhookUrl: string): Promise<void> => {
    await BreezSDKLiquid.registerWebhook(webhookUrl)
}

/**
 * Removes an event listener from the {@link BindingLiquidSdk} instance.
 *
 * # Arguments
 *
 * * `id` - the event listener id returned by {@link addEventListener}
 */
export const removeEventListener = async (id: string): Promise<void> => {
    await BreezSDKLiquid.removeEventListener(id)
}

/**
 * Rescans all expired chain swaps created from calling {@link receiveOnchain} within
 * the monitoring period to check if there are any confirmed funds available to refund.
 */
export const rescanOnchainSwaps = async (): Promise<void> => {
    await BreezSDKLiquid.rescanOnchainSwaps()
}

/**
 * Restores the local state from the provided backup path.
 *
 * # Arguments
 *
 * * `req` - the {@link RestoreRequest} containing:
 *     * `backupPath` - the optional backup path. Defaults to {@link Config.workingDir}
 */
export const restore = async (req: RestoreRequest): Promise<void> => {
    await BreezSDKLiquid.restore(req)
}

/**
 * Either pays a Lightning invoice via a submarine swap or sends funds directly to an address.
 *
 * Depending on {@link Config}'s `paymentTimeoutSec`, this function will return:
 * * {@link PaymentState.PENDING} payment - if the payment could be initiated but didn't yet
 *   complete in this time
 * * {@link PaymentState.COMPLETE} payment - if the payment was successfully completed in this time
 *
 * # Arguments
 *
 * * `req` - A {@link SendPaymentRequest}, containing:
 *     * `prepareResponse` - the {@link PrepareSendResponse} returned by {@link prepareSendPayment}
 *
 * # Errors
 *
 * * {@link PaymentError.PAYMENT_TIMEOUT} - if the payment could not be initiated in this time
 */
export const sendPayment = async (req: SendPaymentRequest): Promise<SendPaymentResponse> => {
    const response = await BreezSDKLiquid.sendPayment(req)
    return response
}

/**
 * Sign given message with the private key. Returns a zbase encoded signature.
 */
export const signMessage = async (req: SignMessageRequest): Promise<SignMessageResponse> => {
    const response = await BreezSDKLiquid.signMessage(req)
    return response
}

/**
 * Synchronizes the local state with the mempool and onchain data.
 */
export const sync = async (): Promise<void> => {
    await BreezSDKLiquid.sync()
}

/**
 * Unregister webhook callbacks. Each swap already created will continue to use the registered
 * `webhookUrl` until complete.
 *
 * This can be called when callbacks are no longer needed or the `webhookUrl`
 * has changed such that it needs unregistering. For example, the token is valid but the locale changes.
 * To register a webhook call {@link registerWebhook}.
 */
export const unregisterWebhook = async (): Promise<void> => {
    await BreezSDKLiquid.unregisterWebhook()
}

