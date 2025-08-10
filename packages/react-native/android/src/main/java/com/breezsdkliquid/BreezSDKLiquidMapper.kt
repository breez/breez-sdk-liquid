package com.breezsdkliquid
import breez_sdk_liquid.*
import com.facebook.react.bridge.*
import com.facebook.react.modules.core.DeviceEventManagerModule.RCTDeviceEventEmitter
import java.io.File
import java.util.*
import java.util.concurrent.ExecutorService
import java.util.concurrent.Executors

fun asAcceptPaymentProposedFeesRequest(acceptPaymentProposedFeesRequest: ReadableMap): AcceptPaymentProposedFeesRequest? {
    if (!validateMandatoryFields(
            acceptPaymentProposedFeesRequest,
            arrayOf(
                "response",
            ),
        )
    ) {
        return null
    }
    val response = acceptPaymentProposedFeesRequest.getMap("response")?.let { asFetchPaymentProposedFeesResponse(it) }!!
    return AcceptPaymentProposedFeesRequest(response)
}

fun readableMapOf(acceptPaymentProposedFeesRequest: AcceptPaymentProposedFeesRequest): ReadableMap =
    readableMapOf(
        "response" to readableMapOf(acceptPaymentProposedFeesRequest.response),
    )

fun asAcceptPaymentProposedFeesRequestList(arr: ReadableArray): List<AcceptPaymentProposedFeesRequest> {
    val list = ArrayList<AcceptPaymentProposedFeesRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asAcceptPaymentProposedFeesRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asAesSuccessActionData(aesSuccessActionData: ReadableMap): AesSuccessActionData? {
    if (!validateMandatoryFields(
            aesSuccessActionData,
            arrayOf(
                "description",
                "ciphertext",
                "iv",
            ),
        )
    ) {
        return null
    }
    val description = aesSuccessActionData.getString("description")!!
    val ciphertext = aesSuccessActionData.getString("ciphertext")!!
    val iv = aesSuccessActionData.getString("iv")!!
    return AesSuccessActionData(description, ciphertext, iv)
}

fun readableMapOf(aesSuccessActionData: AesSuccessActionData): ReadableMap =
    readableMapOf(
        "description" to aesSuccessActionData.description,
        "ciphertext" to aesSuccessActionData.ciphertext,
        "iv" to aesSuccessActionData.iv,
    )

fun asAesSuccessActionDataList(arr: ReadableArray): List<AesSuccessActionData> {
    val list = ArrayList<AesSuccessActionData>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asAesSuccessActionData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asAesSuccessActionDataDecrypted(aesSuccessActionDataDecrypted: ReadableMap): AesSuccessActionDataDecrypted? {
    if (!validateMandatoryFields(
            aesSuccessActionDataDecrypted,
            arrayOf(
                "description",
                "plaintext",
            ),
        )
    ) {
        return null
    }
    val description = aesSuccessActionDataDecrypted.getString("description")!!
    val plaintext = aesSuccessActionDataDecrypted.getString("plaintext")!!
    return AesSuccessActionDataDecrypted(description, plaintext)
}

fun readableMapOf(aesSuccessActionDataDecrypted: AesSuccessActionDataDecrypted): ReadableMap =
    readableMapOf(
        "description" to aesSuccessActionDataDecrypted.description,
        "plaintext" to aesSuccessActionDataDecrypted.plaintext,
    )

fun asAesSuccessActionDataDecryptedList(arr: ReadableArray): List<AesSuccessActionDataDecrypted> {
    val list = ArrayList<AesSuccessActionDataDecrypted>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asAesSuccessActionDataDecrypted(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asAssetBalance(assetBalance: ReadableMap): AssetBalance? {
    if (!validateMandatoryFields(
            assetBalance,
            arrayOf(
                "assetId",
                "balanceSat",
            ),
        )
    ) {
        return null
    }
    val assetId = assetBalance.getString("assetId")!!
    val balanceSat = assetBalance.getDouble("balanceSat").toULong()
    val name = if (hasNonNullKey(assetBalance, "name")) assetBalance.getString("name") else null
    val ticker = if (hasNonNullKey(assetBalance, "ticker")) assetBalance.getString("ticker") else null
    val balance = if (hasNonNullKey(assetBalance, "balance")) assetBalance.getDouble("balance") else null
    return AssetBalance(assetId, balanceSat, name, ticker, balance)
}

fun readableMapOf(assetBalance: AssetBalance): ReadableMap =
    readableMapOf(
        "assetId" to assetBalance.assetId,
        "balanceSat" to assetBalance.balanceSat,
        "name" to assetBalance.name,
        "ticker" to assetBalance.ticker,
        "balance" to assetBalance.balance,
    )

fun asAssetBalanceList(arr: ReadableArray): List<AssetBalance> {
    val list = ArrayList<AssetBalance>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asAssetBalance(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asAssetInfo(assetInfo: ReadableMap): AssetInfo? {
    if (!validateMandatoryFields(
            assetInfo,
            arrayOf(
                "name",
                "ticker",
                "amount",
            ),
        )
    ) {
        return null
    }
    val name = assetInfo.getString("name")!!
    val ticker = assetInfo.getString("ticker")!!
    val amount = assetInfo.getDouble("amount")
    val fees = if (hasNonNullKey(assetInfo, "fees")) assetInfo.getDouble("fees") else null
    return AssetInfo(name, ticker, amount, fees)
}

fun readableMapOf(assetInfo: AssetInfo): ReadableMap =
    readableMapOf(
        "name" to assetInfo.name,
        "ticker" to assetInfo.ticker,
        "amount" to assetInfo.amount,
        "fees" to assetInfo.fees,
    )

fun asAssetInfoList(arr: ReadableArray): List<AssetInfo> {
    val list = ArrayList<AssetInfo>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asAssetInfo(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asAssetMetadata(assetMetadata: ReadableMap): AssetMetadata? {
    if (!validateMandatoryFields(
            assetMetadata,
            arrayOf(
                "assetId",
                "name",
                "ticker",
                "precision",
            ),
        )
    ) {
        return null
    }
    val assetId = assetMetadata.getString("assetId")!!
    val name = assetMetadata.getString("name")!!
    val ticker = assetMetadata.getString("ticker")!!
    val precision = assetMetadata.getInt("precision").toUByte()
    val fiatId = if (hasNonNullKey(assetMetadata, "fiatId")) assetMetadata.getString("fiatId") else null
    return AssetMetadata(assetId, name, ticker, precision, fiatId)
}

fun readableMapOf(assetMetadata: AssetMetadata): ReadableMap =
    readableMapOf(
        "assetId" to assetMetadata.assetId,
        "name" to assetMetadata.name,
        "ticker" to assetMetadata.ticker,
        "precision" to assetMetadata.precision,
        "fiatId" to assetMetadata.fiatId,
    )

fun asAssetMetadataList(arr: ReadableArray): List<AssetMetadata> {
    val list = ArrayList<AssetMetadata>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asAssetMetadata(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asBackupRequest(backupRequest: ReadableMap): BackupRequest? {
    if (!validateMandatoryFields(
            backupRequest,
            arrayOf(),
        )
    ) {
        return null
    }
    val backupPath = if (hasNonNullKey(backupRequest, "backupPath")) backupRequest.getString("backupPath") else null
    return BackupRequest(backupPath)
}

fun readableMapOf(backupRequest: BackupRequest): ReadableMap =
    readableMapOf(
        "backupPath" to backupRequest.backupPath,
    )

fun asBackupRequestList(arr: ReadableArray): List<BackupRequest> {
    val list = ArrayList<BackupRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asBackupRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asBitcoinAddressData(bitcoinAddressData: ReadableMap): BitcoinAddressData? {
    if (!validateMandatoryFields(
            bitcoinAddressData,
            arrayOf(
                "address",
                "network",
            ),
        )
    ) {
        return null
    }
    val address = bitcoinAddressData.getString("address")!!
    val network = bitcoinAddressData.getString("network")?.let { asNetwork(it) }!!
    val amountSat = if (hasNonNullKey(bitcoinAddressData, "amountSat")) bitcoinAddressData.getDouble("amountSat").toULong() else null
    val label = if (hasNonNullKey(bitcoinAddressData, "label")) bitcoinAddressData.getString("label") else null
    val message = if (hasNonNullKey(bitcoinAddressData, "message")) bitcoinAddressData.getString("message") else null
    return BitcoinAddressData(address, network, amountSat, label, message)
}

fun readableMapOf(bitcoinAddressData: BitcoinAddressData): ReadableMap =
    readableMapOf(
        "address" to bitcoinAddressData.address,
        "network" to snakeToLowerCamelCase(bitcoinAddressData.network.name),
        "amountSat" to bitcoinAddressData.amountSat,
        "label" to bitcoinAddressData.label,
        "message" to bitcoinAddressData.message,
    )

fun asBitcoinAddressDataList(arr: ReadableArray): List<BitcoinAddressData> {
    val list = ArrayList<BitcoinAddressData>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asBitcoinAddressData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asBlockchainInfo(blockchainInfo: ReadableMap): BlockchainInfo? {
    if (!validateMandatoryFields(
            blockchainInfo,
            arrayOf(
                "liquidTip",
                "bitcoinTip",
            ),
        )
    ) {
        return null
    }
    val liquidTip = blockchainInfo.getInt("liquidTip").toUInt()
    val bitcoinTip = blockchainInfo.getInt("bitcoinTip").toUInt()
    return BlockchainInfo(liquidTip, bitcoinTip)
}

fun readableMapOf(blockchainInfo: BlockchainInfo): ReadableMap =
    readableMapOf(
        "liquidTip" to blockchainInfo.liquidTip,
        "bitcoinTip" to blockchainInfo.bitcoinTip,
    )

fun asBlockchainInfoList(arr: ReadableArray): List<BlockchainInfo> {
    val list = ArrayList<BlockchainInfo>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asBlockchainInfo(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asBuyBitcoinRequest(buyBitcoinRequest: ReadableMap): BuyBitcoinRequest? {
    if (!validateMandatoryFields(
            buyBitcoinRequest,
            arrayOf(
                "prepareResponse",
            ),
        )
    ) {
        return null
    }
    val prepareResponse = buyBitcoinRequest.getMap("prepareResponse")?.let { asPrepareBuyBitcoinResponse(it) }!!
    val redirectUrl = if (hasNonNullKey(buyBitcoinRequest, "redirectUrl")) buyBitcoinRequest.getString("redirectUrl") else null
    return BuyBitcoinRequest(prepareResponse, redirectUrl)
}

fun readableMapOf(buyBitcoinRequest: BuyBitcoinRequest): ReadableMap =
    readableMapOf(
        "prepareResponse" to readableMapOf(buyBitcoinRequest.prepareResponse),
        "redirectUrl" to buyBitcoinRequest.redirectUrl,
    )

fun asBuyBitcoinRequestList(arr: ReadableArray): List<BuyBitcoinRequest> {
    val list = ArrayList<BuyBitcoinRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asBuyBitcoinRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asCheckMessageRequest(checkMessageRequest: ReadableMap): CheckMessageRequest? {
    if (!validateMandatoryFields(
            checkMessageRequest,
            arrayOf(
                "message",
                "pubkey",
                "signature",
            ),
        )
    ) {
        return null
    }
    val message = checkMessageRequest.getString("message")!!
    val pubkey = checkMessageRequest.getString("pubkey")!!
    val signature = checkMessageRequest.getString("signature")!!
    return CheckMessageRequest(message, pubkey, signature)
}

fun readableMapOf(checkMessageRequest: CheckMessageRequest): ReadableMap =
    readableMapOf(
        "message" to checkMessageRequest.message,
        "pubkey" to checkMessageRequest.pubkey,
        "signature" to checkMessageRequest.signature,
    )

fun asCheckMessageRequestList(arr: ReadableArray): List<CheckMessageRequest> {
    val list = ArrayList<CheckMessageRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asCheckMessageRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asCheckMessageResponse(checkMessageResponse: ReadableMap): CheckMessageResponse? {
    if (!validateMandatoryFields(
            checkMessageResponse,
            arrayOf(
                "isValid",
            ),
        )
    ) {
        return null
    }
    val isValid = checkMessageResponse.getBoolean("isValid")
    return CheckMessageResponse(isValid)
}

fun readableMapOf(checkMessageResponse: CheckMessageResponse): ReadableMap =
    readableMapOf(
        "isValid" to checkMessageResponse.isValid,
    )

fun asCheckMessageResponseList(arr: ReadableArray): List<CheckMessageResponse> {
    val list = ArrayList<CheckMessageResponse>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asCheckMessageResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asConfig(config: ReadableMap): Config? {
    if (!validateMandatoryFields(
            config,
            arrayOf(
                "liquidExplorer",
                "bitcoinExplorer",
                "workingDir",
                "network",
                "paymentTimeoutSec",
                "useDefaultExternalInputParsers",
                "useMagicRoutingHints",
            ),
        )
    ) {
        return null
    }
    val liquidExplorer = config.getMap("liquidExplorer")?.let { asBlockchainExplorer(it) }!!
    val bitcoinExplorer = config.getMap("bitcoinExplorer")?.let { asBlockchainExplorer(it) }!!
    val workingDir = config.getString("workingDir")!!
    val network = config.getString("network")?.let { asLiquidNetwork(it) }!!
    val paymentTimeoutSec = config.getDouble("paymentTimeoutSec").toULong()
    val syncServiceUrl = if (hasNonNullKey(config, "syncServiceUrl")) config.getString("syncServiceUrl") else null
    val breezApiKey = if (hasNonNullKey(config, "breezApiKey")) config.getString("breezApiKey") else null
    val zeroConfMaxAmountSat =
        if (hasNonNullKey(
                config,
                "zeroConfMaxAmountSat",
            )
        ) {
            config.getDouble("zeroConfMaxAmountSat").toULong()
        } else {
            null
        }
    val useDefaultExternalInputParsers = config.getBoolean("useDefaultExternalInputParsers")
    val useMagicRoutingHints = config.getBoolean("useMagicRoutingHints")
    val externalInputParsers =
        if (hasNonNullKey(config, "externalInputParsers")) {
            config.getArray("externalInputParsers")?.let {
                asExternalInputParserList(it)
            }
        } else {
            null
        }
    val onchainFeeRateLeewaySat =
        if (hasNonNullKey(
                config,
                "onchainFeeRateLeewaySat",
            )
        ) {
            config.getDouble("onchainFeeRateLeewaySat").toULong()
        } else {
            null
        }
    val assetMetadata =
        if (hasNonNullKey(
                config,
                "assetMetadata",
            )
        ) {
            config.getArray("assetMetadata")?.let { asAssetMetadataList(it) }
        } else {
            null
        }
    val sideswapApiKey = if (hasNonNullKey(config, "sideswapApiKey")) config.getString("sideswapApiKey") else null
    return Config(
        liquidExplorer,
        bitcoinExplorer,
        workingDir,
        network,
        paymentTimeoutSec,
        syncServiceUrl,
        breezApiKey,
        zeroConfMaxAmountSat,
        useDefaultExternalInputParsers,
        useMagicRoutingHints,
        externalInputParsers,
        onchainFeeRateLeewaySat,
        assetMetadata,
        sideswapApiKey,
    )
}

fun readableMapOf(config: Config): ReadableMap =
    readableMapOf(
        "liquidExplorer" to readableMapOf(config.liquidExplorer),
        "bitcoinExplorer" to readableMapOf(config.bitcoinExplorer),
        "workingDir" to config.workingDir,
        "network" to snakeToLowerCamelCase(config.network.name),
        "paymentTimeoutSec" to config.paymentTimeoutSec,
        "syncServiceUrl" to config.syncServiceUrl,
        "breezApiKey" to config.breezApiKey,
        "zeroConfMaxAmountSat" to config.zeroConfMaxAmountSat,
        "useDefaultExternalInputParsers" to config.useDefaultExternalInputParsers,
        "useMagicRoutingHints" to config.useMagicRoutingHints,
        "externalInputParsers" to config.externalInputParsers?.let { readableArrayOf(it) },
        "onchainFeeRateLeewaySat" to config.onchainFeeRateLeewaySat,
        "assetMetadata" to config.assetMetadata?.let { readableArrayOf(it) },
        "sideswapApiKey" to config.sideswapApiKey,
    )

fun asConfigList(arr: ReadableArray): List<Config> {
    val list = ArrayList<Config>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asConfig(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asConnectRequest(connectRequest: ReadableMap): ConnectRequest? {
    if (!validateMandatoryFields(
            connectRequest,
            arrayOf(
                "config",
            ),
        )
    ) {
        return null
    }
    val config = connectRequest.getMap("config")?.let { asConfig(it) }!!
    val mnemonic = if (hasNonNullKey(connectRequest, "mnemonic")) connectRequest.getString("mnemonic") else null
    val passphrase = if (hasNonNullKey(connectRequest, "passphrase")) connectRequest.getString("passphrase") else null
    val seed = if (hasNonNullKey(connectRequest, "seed")) connectRequest.getArray("seed")?.let { asUByteList(it) } else null
    return ConnectRequest(config, mnemonic, passphrase, seed)
}

fun readableMapOf(connectRequest: ConnectRequest): ReadableMap =
    readableMapOf(
        "config" to readableMapOf(connectRequest.config),
        "mnemonic" to connectRequest.mnemonic,
        "passphrase" to connectRequest.passphrase,
        "seed" to connectRequest.seed?.let { readableArrayOf(it) },
    )

fun asConnectRequestList(arr: ReadableArray): List<ConnectRequest> {
    val list = ArrayList<ConnectRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asConnectRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asConnectWithSignerRequest(connectWithSignerRequest: ReadableMap): ConnectWithSignerRequest? {
    if (!validateMandatoryFields(
            connectWithSignerRequest,
            arrayOf(
                "config",
            ),
        )
    ) {
        return null
    }
    val config = connectWithSignerRequest.getMap("config")?.let { asConfig(it) }!!
    return ConnectWithSignerRequest(config)
}

fun readableMapOf(connectWithSignerRequest: ConnectWithSignerRequest): ReadableMap =
    readableMapOf(
        "config" to readableMapOf(connectWithSignerRequest.config),
    )

fun asConnectWithSignerRequestList(arr: ReadableArray): List<ConnectWithSignerRequest> {
    val list = ArrayList<ConnectWithSignerRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asConnectWithSignerRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asCreateBolt12InvoiceRequest(createBolt12InvoiceRequest: ReadableMap): CreateBolt12InvoiceRequest? {
    if (!validateMandatoryFields(
            createBolt12InvoiceRequest,
            arrayOf(
                "offer",
                "invoiceRequest",
            ),
        )
    ) {
        return null
    }
    val offer = createBolt12InvoiceRequest.getString("offer")!!
    val invoiceRequest = createBolt12InvoiceRequest.getString("invoiceRequest")!!
    return CreateBolt12InvoiceRequest(offer, invoiceRequest)
}

fun readableMapOf(createBolt12InvoiceRequest: CreateBolt12InvoiceRequest): ReadableMap =
    readableMapOf(
        "offer" to createBolt12InvoiceRequest.offer,
        "invoiceRequest" to createBolt12InvoiceRequest.invoiceRequest,
    )

fun asCreateBolt12InvoiceRequestList(arr: ReadableArray): List<CreateBolt12InvoiceRequest> {
    val list = ArrayList<CreateBolt12InvoiceRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asCreateBolt12InvoiceRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asCreateBolt12InvoiceResponse(createBolt12InvoiceResponse: ReadableMap): CreateBolt12InvoiceResponse? {
    if (!validateMandatoryFields(
            createBolt12InvoiceResponse,
            arrayOf(
                "invoice",
            ),
        )
    ) {
        return null
    }
    val invoice = createBolt12InvoiceResponse.getString("invoice")!!
    return CreateBolt12InvoiceResponse(invoice)
}

fun readableMapOf(createBolt12InvoiceResponse: CreateBolt12InvoiceResponse): ReadableMap =
    readableMapOf(
        "invoice" to createBolt12InvoiceResponse.invoice,
    )

fun asCreateBolt12InvoiceResponseList(arr: ReadableArray): List<CreateBolt12InvoiceResponse> {
    val list = ArrayList<CreateBolt12InvoiceResponse>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asCreateBolt12InvoiceResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asCurrencyInfo(currencyInfo: ReadableMap): CurrencyInfo? {
    if (!validateMandatoryFields(
            currencyInfo,
            arrayOf(
                "name",
                "fractionSize",
                "localizedName",
                "localeOverrides",
            ),
        )
    ) {
        return null
    }
    val name = currencyInfo.getString("name")!!
    val fractionSize = currencyInfo.getInt("fractionSize").toUInt()
    val spacing = if (hasNonNullKey(currencyInfo, "spacing")) currencyInfo.getInt("spacing").toUInt() else null
    val symbol = if (hasNonNullKey(currencyInfo, "symbol")) currencyInfo.getMap("symbol")?.let { asSymbol(it) } else null
    val uniqSymbol = if (hasNonNullKey(currencyInfo, "uniqSymbol")) currencyInfo.getMap("uniqSymbol")?.let { asSymbol(it) } else null
    val localizedName = currencyInfo.getArray("localizedName")?.let { asLocalizedNameList(it) }!!
    val localeOverrides = currencyInfo.getArray("localeOverrides")?.let { asLocaleOverridesList(it) }!!
    return CurrencyInfo(name, fractionSize, spacing, symbol, uniqSymbol, localizedName, localeOverrides)
}

fun readableMapOf(currencyInfo: CurrencyInfo): ReadableMap =
    readableMapOf(
        "name" to currencyInfo.name,
        "fractionSize" to currencyInfo.fractionSize,
        "spacing" to currencyInfo.spacing,
        "symbol" to currencyInfo.symbol?.let { readableMapOf(it) },
        "uniqSymbol" to currencyInfo.uniqSymbol?.let { readableMapOf(it) },
        "localizedName" to readableArrayOf(currencyInfo.localizedName),
        "localeOverrides" to readableArrayOf(currencyInfo.localeOverrides),
    )

fun asCurrencyInfoList(arr: ReadableArray): List<CurrencyInfo> {
    val list = ArrayList<CurrencyInfo>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asCurrencyInfo(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asExternalInputParser(externalInputParser: ReadableMap): ExternalInputParser? {
    if (!validateMandatoryFields(
            externalInputParser,
            arrayOf(
                "providerId",
                "inputRegex",
                "parserUrl",
            ),
        )
    ) {
        return null
    }
    val providerId = externalInputParser.getString("providerId")!!
    val inputRegex = externalInputParser.getString("inputRegex")!!
    val parserUrl = externalInputParser.getString("parserUrl")!!
    return ExternalInputParser(providerId, inputRegex, parserUrl)
}

fun readableMapOf(externalInputParser: ExternalInputParser): ReadableMap =
    readableMapOf(
        "providerId" to externalInputParser.providerId,
        "inputRegex" to externalInputParser.inputRegex,
        "parserUrl" to externalInputParser.parserUrl,
    )

fun asExternalInputParserList(arr: ReadableArray): List<ExternalInputParser> {
    val list = ArrayList<ExternalInputParser>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asExternalInputParser(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asFetchPaymentProposedFeesRequest(fetchPaymentProposedFeesRequest: ReadableMap): FetchPaymentProposedFeesRequest? {
    if (!validateMandatoryFields(
            fetchPaymentProposedFeesRequest,
            arrayOf(
                "swapId",
            ),
        )
    ) {
        return null
    }
    val swapId = fetchPaymentProposedFeesRequest.getString("swapId")!!
    return FetchPaymentProposedFeesRequest(swapId)
}

fun readableMapOf(fetchPaymentProposedFeesRequest: FetchPaymentProposedFeesRequest): ReadableMap =
    readableMapOf(
        "swapId" to fetchPaymentProposedFeesRequest.swapId,
    )

fun asFetchPaymentProposedFeesRequestList(arr: ReadableArray): List<FetchPaymentProposedFeesRequest> {
    val list = ArrayList<FetchPaymentProposedFeesRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asFetchPaymentProposedFeesRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asFetchPaymentProposedFeesResponse(fetchPaymentProposedFeesResponse: ReadableMap): FetchPaymentProposedFeesResponse? {
    if (!validateMandatoryFields(
            fetchPaymentProposedFeesResponse,
            arrayOf(
                "swapId",
                "feesSat",
                "payerAmountSat",
                "receiverAmountSat",
            ),
        )
    ) {
        return null
    }
    val swapId = fetchPaymentProposedFeesResponse.getString("swapId")!!
    val feesSat = fetchPaymentProposedFeesResponse.getDouble("feesSat").toULong()
    val payerAmountSat = fetchPaymentProposedFeesResponse.getDouble("payerAmountSat").toULong()
    val receiverAmountSat = fetchPaymentProposedFeesResponse.getDouble("receiverAmountSat").toULong()
    return FetchPaymentProposedFeesResponse(swapId, feesSat, payerAmountSat, receiverAmountSat)
}

fun readableMapOf(fetchPaymentProposedFeesResponse: FetchPaymentProposedFeesResponse): ReadableMap =
    readableMapOf(
        "swapId" to fetchPaymentProposedFeesResponse.swapId,
        "feesSat" to fetchPaymentProposedFeesResponse.feesSat,
        "payerAmountSat" to fetchPaymentProposedFeesResponse.payerAmountSat,
        "receiverAmountSat" to fetchPaymentProposedFeesResponse.receiverAmountSat,
    )

fun asFetchPaymentProposedFeesResponseList(arr: ReadableArray): List<FetchPaymentProposedFeesResponse> {
    val list = ArrayList<FetchPaymentProposedFeesResponse>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asFetchPaymentProposedFeesResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asFiatCurrency(fiatCurrency: ReadableMap): FiatCurrency? {
    if (!validateMandatoryFields(
            fiatCurrency,
            arrayOf(
                "id",
                "info",
            ),
        )
    ) {
        return null
    }
    val id = fiatCurrency.getString("id")!!
    val info = fiatCurrency.getMap("info")?.let { asCurrencyInfo(it) }!!
    return FiatCurrency(id, info)
}

fun readableMapOf(fiatCurrency: FiatCurrency): ReadableMap =
    readableMapOf(
        "id" to fiatCurrency.id,
        "info" to readableMapOf(fiatCurrency.info),
    )

fun asFiatCurrencyList(arr: ReadableArray): List<FiatCurrency> {
    val list = ArrayList<FiatCurrency>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asFiatCurrency(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asGetInfoResponse(getInfoResponse: ReadableMap): GetInfoResponse? {
    if (!validateMandatoryFields(
            getInfoResponse,
            arrayOf(
                "walletInfo",
                "blockchainInfo",
            ),
        )
    ) {
        return null
    }
    val walletInfo = getInfoResponse.getMap("walletInfo")?.let { asWalletInfo(it) }!!
    val blockchainInfo = getInfoResponse.getMap("blockchainInfo")?.let { asBlockchainInfo(it) }!!
    return GetInfoResponse(walletInfo, blockchainInfo)
}

fun readableMapOf(getInfoResponse: GetInfoResponse): ReadableMap =
    readableMapOf(
        "walletInfo" to readableMapOf(getInfoResponse.walletInfo),
        "blockchainInfo" to readableMapOf(getInfoResponse.blockchainInfo),
    )

fun asGetInfoResponseList(arr: ReadableArray): List<GetInfoResponse> {
    val list = ArrayList<GetInfoResponse>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asGetInfoResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLnInvoice(lnInvoice: ReadableMap): LnInvoice? {
    if (!validateMandatoryFields(
            lnInvoice,
            arrayOf(
                "bolt11",
                "network",
                "payeePubkey",
                "paymentHash",
                "timestamp",
                "expiry",
                "routingHints",
                "paymentSecret",
                "minFinalCltvExpiryDelta",
            ),
        )
    ) {
        return null
    }
    val bolt11 = lnInvoice.getString("bolt11")!!
    val network = lnInvoice.getString("network")?.let { asNetwork(it) }!!
    val payeePubkey = lnInvoice.getString("payeePubkey")!!
    val paymentHash = lnInvoice.getString("paymentHash")!!
    val description = if (hasNonNullKey(lnInvoice, "description")) lnInvoice.getString("description") else null
    val descriptionHash = if (hasNonNullKey(lnInvoice, "descriptionHash")) lnInvoice.getString("descriptionHash") else null
    val amountMsat = if (hasNonNullKey(lnInvoice, "amountMsat")) lnInvoice.getDouble("amountMsat").toULong() else null
    val timestamp = lnInvoice.getDouble("timestamp").toULong()
    val expiry = lnInvoice.getDouble("expiry").toULong()
    val routingHints = lnInvoice.getArray("routingHints")?.let { asRouteHintList(it) }!!
    val paymentSecret = lnInvoice.getArray("paymentSecret")?.let { asUByteList(it) }!!
    val minFinalCltvExpiryDelta = lnInvoice.getDouble("minFinalCltvExpiryDelta").toULong()
    return LnInvoice(
        bolt11,
        network,
        payeePubkey,
        paymentHash,
        description,
        descriptionHash,
        amountMsat,
        timestamp,
        expiry,
        routingHints,
        paymentSecret,
        minFinalCltvExpiryDelta,
    )
}

fun readableMapOf(lnInvoice: LnInvoice): ReadableMap =
    readableMapOf(
        "bolt11" to lnInvoice.bolt11,
        "network" to snakeToLowerCamelCase(lnInvoice.network.name),
        "payeePubkey" to lnInvoice.payeePubkey,
        "paymentHash" to lnInvoice.paymentHash,
        "description" to lnInvoice.description,
        "descriptionHash" to lnInvoice.descriptionHash,
        "amountMsat" to lnInvoice.amountMsat,
        "timestamp" to lnInvoice.timestamp,
        "expiry" to lnInvoice.expiry,
        "routingHints" to readableArrayOf(lnInvoice.routingHints),
        "paymentSecret" to readableArrayOf(lnInvoice.paymentSecret),
        "minFinalCltvExpiryDelta" to lnInvoice.minFinalCltvExpiryDelta,
    )

fun asLnInvoiceList(arr: ReadableArray): List<LnInvoice> {
    val list = ArrayList<LnInvoice>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asLnInvoice(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLnOffer(lnOffer: ReadableMap): LnOffer? {
    if (!validateMandatoryFields(
            lnOffer,
            arrayOf(
                "offer",
                "chains",
                "paths",
            ),
        )
    ) {
        return null
    }
    val offer = lnOffer.getString("offer")!!
    val chains = lnOffer.getArray("chains")?.let { asStringList(it) }!!
    val paths = lnOffer.getArray("paths")?.let { asLnOfferBlindedPathList(it) }!!
    val description = if (hasNonNullKey(lnOffer, "description")) lnOffer.getString("description") else null
    val signingPubkey = if (hasNonNullKey(lnOffer, "signingPubkey")) lnOffer.getString("signingPubkey") else null
    val minAmount = if (hasNonNullKey(lnOffer, "minAmount")) lnOffer.getMap("minAmount")?.let { asAmount(it) } else null
    val absoluteExpiry = if (hasNonNullKey(lnOffer, "absoluteExpiry")) lnOffer.getDouble("absoluteExpiry").toULong() else null
    val issuer = if (hasNonNullKey(lnOffer, "issuer")) lnOffer.getString("issuer") else null
    return LnOffer(offer, chains, paths, description, signingPubkey, minAmount, absoluteExpiry, issuer)
}

fun readableMapOf(lnOffer: LnOffer): ReadableMap =
    readableMapOf(
        "offer" to lnOffer.offer,
        "chains" to readableArrayOf(lnOffer.chains),
        "paths" to readableArrayOf(lnOffer.paths),
        "description" to lnOffer.description,
        "signingPubkey" to lnOffer.signingPubkey,
        "minAmount" to lnOffer.minAmount?.let { readableMapOf(it) },
        "absoluteExpiry" to lnOffer.absoluteExpiry,
        "issuer" to lnOffer.issuer,
    )

fun asLnOfferList(arr: ReadableArray): List<LnOffer> {
    val list = ArrayList<LnOffer>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asLnOffer(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLightningPaymentLimitsResponse(lightningPaymentLimitsResponse: ReadableMap): LightningPaymentLimitsResponse? {
    if (!validateMandatoryFields(
            lightningPaymentLimitsResponse,
            arrayOf(
                "send",
                "receive",
            ),
        )
    ) {
        return null
    }
    val send = lightningPaymentLimitsResponse.getMap("send")?.let { asLimits(it) }!!
    val receive = lightningPaymentLimitsResponse.getMap("receive")?.let { asLimits(it) }!!
    return LightningPaymentLimitsResponse(send, receive)
}

fun readableMapOf(lightningPaymentLimitsResponse: LightningPaymentLimitsResponse): ReadableMap =
    readableMapOf(
        "send" to readableMapOf(lightningPaymentLimitsResponse.send),
        "receive" to readableMapOf(lightningPaymentLimitsResponse.receive),
    )

fun asLightningPaymentLimitsResponseList(arr: ReadableArray): List<LightningPaymentLimitsResponse> {
    val list = ArrayList<LightningPaymentLimitsResponse>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asLightningPaymentLimitsResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLimits(limits: ReadableMap): Limits? {
    if (!validateMandatoryFields(
            limits,
            arrayOf(
                "minSat",
                "maxSat",
                "maxZeroConfSat",
            ),
        )
    ) {
        return null
    }
    val minSat = limits.getDouble("minSat").toULong()
    val maxSat = limits.getDouble("maxSat").toULong()
    val maxZeroConfSat = limits.getDouble("maxZeroConfSat").toULong()
    return Limits(minSat, maxSat, maxZeroConfSat)
}

fun readableMapOf(limits: Limits): ReadableMap =
    readableMapOf(
        "minSat" to limits.minSat,
        "maxSat" to limits.maxSat,
        "maxZeroConfSat" to limits.maxZeroConfSat,
    )

fun asLimitsList(arr: ReadableArray): List<Limits> {
    val list = ArrayList<Limits>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asLimits(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLiquidAddressData(liquidAddressData: ReadableMap): LiquidAddressData? {
    if (!validateMandatoryFields(
            liquidAddressData,
            arrayOf(
                "address",
                "network",
            ),
        )
    ) {
        return null
    }
    val address = liquidAddressData.getString("address")!!
    val network = liquidAddressData.getString("network")?.let { asNetwork(it) }!!
    val assetId = if (hasNonNullKey(liquidAddressData, "assetId")) liquidAddressData.getString("assetId") else null
    val amount = if (hasNonNullKey(liquidAddressData, "amount")) liquidAddressData.getDouble("amount") else null
    val amountSat = if (hasNonNullKey(liquidAddressData, "amountSat")) liquidAddressData.getDouble("amountSat").toULong() else null
    val label = if (hasNonNullKey(liquidAddressData, "label")) liquidAddressData.getString("label") else null
    val message = if (hasNonNullKey(liquidAddressData, "message")) liquidAddressData.getString("message") else null
    return LiquidAddressData(address, network, assetId, amount, amountSat, label, message)
}

fun readableMapOf(liquidAddressData: LiquidAddressData): ReadableMap =
    readableMapOf(
        "address" to liquidAddressData.address,
        "network" to snakeToLowerCamelCase(liquidAddressData.network.name),
        "assetId" to liquidAddressData.assetId,
        "amount" to liquidAddressData.amount,
        "amountSat" to liquidAddressData.amountSat,
        "label" to liquidAddressData.label,
        "message" to liquidAddressData.message,
    )

fun asLiquidAddressDataList(arr: ReadableArray): List<LiquidAddressData> {
    val list = ArrayList<LiquidAddressData>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asLiquidAddressData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asListPaymentsRequest(listPaymentsRequest: ReadableMap): ListPaymentsRequest? {
    if (!validateMandatoryFields(
            listPaymentsRequest,
            arrayOf(),
        )
    ) {
        return null
    }
    val filters =
        if (hasNonNullKey(listPaymentsRequest, "filters")) {
            listPaymentsRequest.getArray("filters")?.let {
                asPaymentTypeList(it)
            }
        } else {
            null
        }
    val states =
        if (hasNonNullKey(listPaymentsRequest, "states")) {
            listPaymentsRequest.getArray("states")?.let {
                asPaymentStateList(it)
            }
        } else {
            null
        }
    val fromTimestamp =
        if (hasNonNullKey(
                listPaymentsRequest,
                "fromTimestamp",
            )
        ) {
            listPaymentsRequest.getDouble("fromTimestamp").toLong()
        } else {
            null
        }
    val toTimestamp = if (hasNonNullKey(listPaymentsRequest, "toTimestamp")) listPaymentsRequest.getDouble("toTimestamp").toLong() else null
    val offset = if (hasNonNullKey(listPaymentsRequest, "offset")) listPaymentsRequest.getInt("offset").toUInt() else null
    val limit = if (hasNonNullKey(listPaymentsRequest, "limit")) listPaymentsRequest.getInt("limit").toUInt() else null
    val details =
        if (hasNonNullKey(listPaymentsRequest, "details")) {
            listPaymentsRequest.getMap("details")?.let {
                asListPaymentDetails(it)
            }
        } else {
            null
        }
    val sortAscending = if (hasNonNullKey(listPaymentsRequest, "sortAscending")) listPaymentsRequest.getBoolean("sortAscending") else null
    return ListPaymentsRequest(filters, states, fromTimestamp, toTimestamp, offset, limit, details, sortAscending)
}

fun readableMapOf(listPaymentsRequest: ListPaymentsRequest): ReadableMap =
    readableMapOf(
        "filters" to listPaymentsRequest.filters?.let { readableArrayOf(it) },
        "states" to listPaymentsRequest.states?.let { readableArrayOf(it) },
        "fromTimestamp" to listPaymentsRequest.fromTimestamp,
        "toTimestamp" to listPaymentsRequest.toTimestamp,
        "offset" to listPaymentsRequest.offset,
        "limit" to listPaymentsRequest.limit,
        "details" to listPaymentsRequest.details?.let { readableMapOf(it) },
        "sortAscending" to listPaymentsRequest.sortAscending,
    )

fun asListPaymentsRequestList(arr: ReadableArray): List<ListPaymentsRequest> {
    val list = ArrayList<ListPaymentsRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asListPaymentsRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLnOfferBlindedPath(lnOfferBlindedPath: ReadableMap): LnOfferBlindedPath? {
    if (!validateMandatoryFields(
            lnOfferBlindedPath,
            arrayOf(
                "blindedHops",
            ),
        )
    ) {
        return null
    }
    val blindedHops = lnOfferBlindedPath.getArray("blindedHops")?.let { asStringList(it) }!!
    return LnOfferBlindedPath(blindedHops)
}

fun readableMapOf(lnOfferBlindedPath: LnOfferBlindedPath): ReadableMap =
    readableMapOf(
        "blindedHops" to readableArrayOf(lnOfferBlindedPath.blindedHops),
    )

fun asLnOfferBlindedPathList(arr: ReadableArray): List<LnOfferBlindedPath> {
    val list = ArrayList<LnOfferBlindedPath>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asLnOfferBlindedPath(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLnUrlAuthRequestData(lnUrlAuthRequestData: ReadableMap): LnUrlAuthRequestData? {
    if (!validateMandatoryFields(
            lnUrlAuthRequestData,
            arrayOf(
                "k1",
                "domain",
                "url",
            ),
        )
    ) {
        return null
    }
    val k1 = lnUrlAuthRequestData.getString("k1")!!
    val domain = lnUrlAuthRequestData.getString("domain")!!
    val url = lnUrlAuthRequestData.getString("url")!!
    val action = if (hasNonNullKey(lnUrlAuthRequestData, "action")) lnUrlAuthRequestData.getString("action") else null
    return LnUrlAuthRequestData(k1, domain, url, action)
}

fun readableMapOf(lnUrlAuthRequestData: LnUrlAuthRequestData): ReadableMap =
    readableMapOf(
        "k1" to lnUrlAuthRequestData.k1,
        "domain" to lnUrlAuthRequestData.domain,
        "url" to lnUrlAuthRequestData.url,
        "action" to lnUrlAuthRequestData.action,
    )

fun asLnUrlAuthRequestDataList(arr: ReadableArray): List<LnUrlAuthRequestData> {
    val list = ArrayList<LnUrlAuthRequestData>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlAuthRequestData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLnUrlErrorData(lnUrlErrorData: ReadableMap): LnUrlErrorData? {
    if (!validateMandatoryFields(
            lnUrlErrorData,
            arrayOf(
                "reason",
            ),
        )
    ) {
        return null
    }
    val reason = lnUrlErrorData.getString("reason")!!
    return LnUrlErrorData(reason)
}

fun readableMapOf(lnUrlErrorData: LnUrlErrorData): ReadableMap =
    readableMapOf(
        "reason" to lnUrlErrorData.reason,
    )

fun asLnUrlErrorDataList(arr: ReadableArray): List<LnUrlErrorData> {
    val list = ArrayList<LnUrlErrorData>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlErrorData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLnUrlInfo(lnUrlInfo: ReadableMap): LnUrlInfo? {
    if (!validateMandatoryFields(
            lnUrlInfo,
            arrayOf(),
        )
    ) {
        return null
    }
    val lnAddress = if (hasNonNullKey(lnUrlInfo, "lnAddress")) lnUrlInfo.getString("lnAddress") else null
    val lnurlPayComment = if (hasNonNullKey(lnUrlInfo, "lnurlPayComment")) lnUrlInfo.getString("lnurlPayComment") else null
    val lnurlPayDomain = if (hasNonNullKey(lnUrlInfo, "lnurlPayDomain")) lnUrlInfo.getString("lnurlPayDomain") else null
    val lnurlPayMetadata = if (hasNonNullKey(lnUrlInfo, "lnurlPayMetadata")) lnUrlInfo.getString("lnurlPayMetadata") else null
    val lnurlPaySuccessAction =
        if (hasNonNullKey(lnUrlInfo, "lnurlPaySuccessAction")) {
            lnUrlInfo.getMap("lnurlPaySuccessAction")?.let {
                asSuccessActionProcessed(it)
            }
        } else {
            null
        }
    val lnurlPayUnprocessedSuccessAction =
        if (hasNonNullKey(
                lnUrlInfo,
                "lnurlPayUnprocessedSuccessAction",
            )
        ) {
            lnUrlInfo.getMap("lnurlPayUnprocessedSuccessAction")?.let {
                asSuccessAction(it)
            }
        } else {
            null
        }
    val lnurlWithdrawEndpoint =
        if (hasNonNullKey(
                lnUrlInfo,
                "lnurlWithdrawEndpoint",
            )
        ) {
            lnUrlInfo.getString("lnurlWithdrawEndpoint")
        } else {
            null
        }
    return LnUrlInfo(
        lnAddress,
        lnurlPayComment,
        lnurlPayDomain,
        lnurlPayMetadata,
        lnurlPaySuccessAction,
        lnurlPayUnprocessedSuccessAction,
        lnurlWithdrawEndpoint,
    )
}

fun readableMapOf(lnUrlInfo: LnUrlInfo): ReadableMap =
    readableMapOf(
        "lnAddress" to lnUrlInfo.lnAddress,
        "lnurlPayComment" to lnUrlInfo.lnurlPayComment,
        "lnurlPayDomain" to lnUrlInfo.lnurlPayDomain,
        "lnurlPayMetadata" to lnUrlInfo.lnurlPayMetadata,
        "lnurlPaySuccessAction" to lnUrlInfo.lnurlPaySuccessAction?.let { readableMapOf(it) },
        "lnurlPayUnprocessedSuccessAction" to lnUrlInfo.lnurlPayUnprocessedSuccessAction?.let { readableMapOf(it) },
        "lnurlWithdrawEndpoint" to lnUrlInfo.lnurlWithdrawEndpoint,
    )

fun asLnUrlInfoList(arr: ReadableArray): List<LnUrlInfo> {
    val list = ArrayList<LnUrlInfo>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlInfo(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLnUrlPayErrorData(lnUrlPayErrorData: ReadableMap): LnUrlPayErrorData? {
    if (!validateMandatoryFields(
            lnUrlPayErrorData,
            arrayOf(
                "paymentHash",
                "reason",
            ),
        )
    ) {
        return null
    }
    val paymentHash = lnUrlPayErrorData.getString("paymentHash")!!
    val reason = lnUrlPayErrorData.getString("reason")!!
    return LnUrlPayErrorData(paymentHash, reason)
}

fun readableMapOf(lnUrlPayErrorData: LnUrlPayErrorData): ReadableMap =
    readableMapOf(
        "paymentHash" to lnUrlPayErrorData.paymentHash,
        "reason" to lnUrlPayErrorData.reason,
    )

fun asLnUrlPayErrorDataList(arr: ReadableArray): List<LnUrlPayErrorData> {
    val list = ArrayList<LnUrlPayErrorData>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlPayErrorData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLnUrlPayRequest(lnUrlPayRequest: ReadableMap): LnUrlPayRequest? {
    if (!validateMandatoryFields(
            lnUrlPayRequest,
            arrayOf(
                "prepareResponse",
            ),
        )
    ) {
        return null
    }
    val prepareResponse = lnUrlPayRequest.getMap("prepareResponse")?.let { asPrepareLnUrlPayResponse(it) }!!
    return LnUrlPayRequest(prepareResponse)
}

fun readableMapOf(lnUrlPayRequest: LnUrlPayRequest): ReadableMap =
    readableMapOf(
        "prepareResponse" to readableMapOf(lnUrlPayRequest.prepareResponse),
    )

fun asLnUrlPayRequestList(arr: ReadableArray): List<LnUrlPayRequest> {
    val list = ArrayList<LnUrlPayRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlPayRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLnUrlPayRequestData(lnUrlPayRequestData: ReadableMap): LnUrlPayRequestData? {
    if (!validateMandatoryFields(
            lnUrlPayRequestData,
            arrayOf(
                "callback",
                "minSendable",
                "maxSendable",
                "metadataStr",
                "commentAllowed",
                "domain",
                "allowsNostr",
            ),
        )
    ) {
        return null
    }
    val callback = lnUrlPayRequestData.getString("callback")!!
    val minSendable = lnUrlPayRequestData.getDouble("minSendable").toULong()
    val maxSendable = lnUrlPayRequestData.getDouble("maxSendable").toULong()
    val metadataStr = lnUrlPayRequestData.getString("metadataStr")!!
    val commentAllowed = lnUrlPayRequestData.getInt("commentAllowed").toUShort()
    val domain = lnUrlPayRequestData.getString("domain")!!
    val allowsNostr = lnUrlPayRequestData.getBoolean("allowsNostr")
    val nostrPubkey = if (hasNonNullKey(lnUrlPayRequestData, "nostrPubkey")) lnUrlPayRequestData.getString("nostrPubkey") else null
    val lnAddress = if (hasNonNullKey(lnUrlPayRequestData, "lnAddress")) lnUrlPayRequestData.getString("lnAddress") else null
    return LnUrlPayRequestData(
        callback,
        minSendable,
        maxSendable,
        metadataStr,
        commentAllowed,
        domain,
        allowsNostr,
        nostrPubkey,
        lnAddress,
    )
}

fun readableMapOf(lnUrlPayRequestData: LnUrlPayRequestData): ReadableMap =
    readableMapOf(
        "callback" to lnUrlPayRequestData.callback,
        "minSendable" to lnUrlPayRequestData.minSendable,
        "maxSendable" to lnUrlPayRequestData.maxSendable,
        "metadataStr" to lnUrlPayRequestData.metadataStr,
        "commentAllowed" to lnUrlPayRequestData.commentAllowed,
        "domain" to lnUrlPayRequestData.domain,
        "allowsNostr" to lnUrlPayRequestData.allowsNostr,
        "nostrPubkey" to lnUrlPayRequestData.nostrPubkey,
        "lnAddress" to lnUrlPayRequestData.lnAddress,
    )

fun asLnUrlPayRequestDataList(arr: ReadableArray): List<LnUrlPayRequestData> {
    val list = ArrayList<LnUrlPayRequestData>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlPayRequestData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLnUrlPaySuccessData(lnUrlPaySuccessData: ReadableMap): LnUrlPaySuccessData? {
    if (!validateMandatoryFields(
            lnUrlPaySuccessData,
            arrayOf(
                "payment",
            ),
        )
    ) {
        return null
    }
    val successAction =
        if (hasNonNullKey(lnUrlPaySuccessData, "successAction")) {
            lnUrlPaySuccessData.getMap("successAction")?.let {
                asSuccessActionProcessed(it)
            }
        } else {
            null
        }
    val payment = lnUrlPaySuccessData.getMap("payment")?.let { asPayment(it) }!!
    return LnUrlPaySuccessData(successAction, payment)
}

fun readableMapOf(lnUrlPaySuccessData: LnUrlPaySuccessData): ReadableMap =
    readableMapOf(
        "successAction" to lnUrlPaySuccessData.successAction?.let { readableMapOf(it) },
        "payment" to readableMapOf(lnUrlPaySuccessData.payment),
    )

fun asLnUrlPaySuccessDataList(arr: ReadableArray): List<LnUrlPaySuccessData> {
    val list = ArrayList<LnUrlPaySuccessData>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlPaySuccessData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLnUrlWithdrawRequest(lnUrlWithdrawRequest: ReadableMap): LnUrlWithdrawRequest? {
    if (!validateMandatoryFields(
            lnUrlWithdrawRequest,
            arrayOf(
                "data",
                "amountMsat",
            ),
        )
    ) {
        return null
    }
    val data = lnUrlWithdrawRequest.getMap("data")?.let { asLnUrlWithdrawRequestData(it) }!!
    val amountMsat = lnUrlWithdrawRequest.getDouble("amountMsat").toULong()
    val description = if (hasNonNullKey(lnUrlWithdrawRequest, "description")) lnUrlWithdrawRequest.getString("description") else null
    return LnUrlWithdrawRequest(data, amountMsat, description)
}

fun readableMapOf(lnUrlWithdrawRequest: LnUrlWithdrawRequest): ReadableMap =
    readableMapOf(
        "data" to readableMapOf(lnUrlWithdrawRequest.data),
        "amountMsat" to lnUrlWithdrawRequest.amountMsat,
        "description" to lnUrlWithdrawRequest.description,
    )

fun asLnUrlWithdrawRequestList(arr: ReadableArray): List<LnUrlWithdrawRequest> {
    val list = ArrayList<LnUrlWithdrawRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlWithdrawRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLnUrlWithdrawRequestData(lnUrlWithdrawRequestData: ReadableMap): LnUrlWithdrawRequestData? {
    if (!validateMandatoryFields(
            lnUrlWithdrawRequestData,
            arrayOf(
                "callback",
                "k1",
                "defaultDescription",
                "minWithdrawable",
                "maxWithdrawable",
            ),
        )
    ) {
        return null
    }
    val callback = lnUrlWithdrawRequestData.getString("callback")!!
    val k1 = lnUrlWithdrawRequestData.getString("k1")!!
    val defaultDescription = lnUrlWithdrawRequestData.getString("defaultDescription")!!
    val minWithdrawable = lnUrlWithdrawRequestData.getDouble("minWithdrawable").toULong()
    val maxWithdrawable = lnUrlWithdrawRequestData.getDouble("maxWithdrawable").toULong()
    return LnUrlWithdrawRequestData(callback, k1, defaultDescription, minWithdrawable, maxWithdrawable)
}

fun readableMapOf(lnUrlWithdrawRequestData: LnUrlWithdrawRequestData): ReadableMap =
    readableMapOf(
        "callback" to lnUrlWithdrawRequestData.callback,
        "k1" to lnUrlWithdrawRequestData.k1,
        "defaultDescription" to lnUrlWithdrawRequestData.defaultDescription,
        "minWithdrawable" to lnUrlWithdrawRequestData.minWithdrawable,
        "maxWithdrawable" to lnUrlWithdrawRequestData.maxWithdrawable,
    )

fun asLnUrlWithdrawRequestDataList(arr: ReadableArray): List<LnUrlWithdrawRequestData> {
    val list = ArrayList<LnUrlWithdrawRequestData>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlWithdrawRequestData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLnUrlWithdrawSuccessData(lnUrlWithdrawSuccessData: ReadableMap): LnUrlWithdrawSuccessData? {
    if (!validateMandatoryFields(
            lnUrlWithdrawSuccessData,
            arrayOf(
                "invoice",
            ),
        )
    ) {
        return null
    }
    val invoice = lnUrlWithdrawSuccessData.getMap("invoice")?.let { asLnInvoice(it) }!!
    return LnUrlWithdrawSuccessData(invoice)
}

fun readableMapOf(lnUrlWithdrawSuccessData: LnUrlWithdrawSuccessData): ReadableMap =
    readableMapOf(
        "invoice" to readableMapOf(lnUrlWithdrawSuccessData.invoice),
    )

fun asLnUrlWithdrawSuccessDataList(arr: ReadableArray): List<LnUrlWithdrawSuccessData> {
    val list = ArrayList<LnUrlWithdrawSuccessData>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlWithdrawSuccessData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLocaleOverrides(localeOverrides: ReadableMap): LocaleOverrides? {
    if (!validateMandatoryFields(
            localeOverrides,
            arrayOf(
                "locale",
                "symbol",
            ),
        )
    ) {
        return null
    }
    val locale = localeOverrides.getString("locale")!!
    val spacing = if (hasNonNullKey(localeOverrides, "spacing")) localeOverrides.getInt("spacing").toUInt() else null
    val symbol = localeOverrides.getMap("symbol")?.let { asSymbol(it) }!!
    return LocaleOverrides(locale, spacing, symbol)
}

fun readableMapOf(localeOverrides: LocaleOverrides): ReadableMap =
    readableMapOf(
        "locale" to localeOverrides.locale,
        "spacing" to localeOverrides.spacing,
        "symbol" to readableMapOf(localeOverrides.symbol),
    )

fun asLocaleOverridesList(arr: ReadableArray): List<LocaleOverrides> {
    val list = ArrayList<LocaleOverrides>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asLocaleOverrides(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLocalizedName(localizedName: ReadableMap): LocalizedName? {
    if (!validateMandatoryFields(
            localizedName,
            arrayOf(
                "locale",
                "name",
            ),
        )
    ) {
        return null
    }
    val locale = localizedName.getString("locale")!!
    val name = localizedName.getString("name")!!
    return LocalizedName(locale, name)
}

fun readableMapOf(localizedName: LocalizedName): ReadableMap =
    readableMapOf(
        "locale" to localizedName.locale,
        "name" to localizedName.name,
    )

fun asLocalizedNameList(arr: ReadableArray): List<LocalizedName> {
    val list = ArrayList<LocalizedName>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asLocalizedName(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLogEntry(logEntry: ReadableMap): LogEntry? {
    if (!validateMandatoryFields(
            logEntry,
            arrayOf(
                "line",
                "level",
            ),
        )
    ) {
        return null
    }
    val line = logEntry.getString("line")!!
    val level = logEntry.getString("level")!!
    return LogEntry(line, level)
}

fun readableMapOf(logEntry: LogEntry): ReadableMap =
    readableMapOf(
        "line" to logEntry.line,
        "level" to logEntry.level,
    )

fun asLogEntryList(arr: ReadableArray): List<LogEntry> {
    val list = ArrayList<LogEntry>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asLogEntry(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asMessageSuccessActionData(messageSuccessActionData: ReadableMap): MessageSuccessActionData? {
    if (!validateMandatoryFields(
            messageSuccessActionData,
            arrayOf(
                "message",
            ),
        )
    ) {
        return null
    }
    val message = messageSuccessActionData.getString("message")!!
    return MessageSuccessActionData(message)
}

fun readableMapOf(messageSuccessActionData: MessageSuccessActionData): ReadableMap =
    readableMapOf(
        "message" to messageSuccessActionData.message,
    )

fun asMessageSuccessActionDataList(arr: ReadableArray): List<MessageSuccessActionData> {
    val list = ArrayList<MessageSuccessActionData>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asMessageSuccessActionData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asOnchainPaymentLimitsResponse(onchainPaymentLimitsResponse: ReadableMap): OnchainPaymentLimitsResponse? {
    if (!validateMandatoryFields(
            onchainPaymentLimitsResponse,
            arrayOf(
                "send",
                "receive",
            ),
        )
    ) {
        return null
    }
    val send = onchainPaymentLimitsResponse.getMap("send")?.let { asLimits(it) }!!
    val receive = onchainPaymentLimitsResponse.getMap("receive")?.let { asLimits(it) }!!
    return OnchainPaymentLimitsResponse(send, receive)
}

fun readableMapOf(onchainPaymentLimitsResponse: OnchainPaymentLimitsResponse): ReadableMap =
    readableMapOf(
        "send" to readableMapOf(onchainPaymentLimitsResponse.send),
        "receive" to readableMapOf(onchainPaymentLimitsResponse.receive),
    )

fun asOnchainPaymentLimitsResponseList(arr: ReadableArray): List<OnchainPaymentLimitsResponse> {
    val list = ArrayList<OnchainPaymentLimitsResponse>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asOnchainPaymentLimitsResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asPayOnchainRequest(payOnchainRequest: ReadableMap): PayOnchainRequest? {
    if (!validateMandatoryFields(
            payOnchainRequest,
            arrayOf(
                "address",
                "prepareResponse",
            ),
        )
    ) {
        return null
    }
    val address = payOnchainRequest.getString("address")!!
    val prepareResponse = payOnchainRequest.getMap("prepareResponse")?.let { asPreparePayOnchainResponse(it) }!!
    return PayOnchainRequest(address, prepareResponse)
}

fun readableMapOf(payOnchainRequest: PayOnchainRequest): ReadableMap =
    readableMapOf(
        "address" to payOnchainRequest.address,
        "prepareResponse" to readableMapOf(payOnchainRequest.prepareResponse),
    )

fun asPayOnchainRequestList(arr: ReadableArray): List<PayOnchainRequest> {
    val list = ArrayList<PayOnchainRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asPayOnchainRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asPayment(payment: ReadableMap): Payment? {
    if (!validateMandatoryFields(
            payment,
            arrayOf(
                "timestamp",
                "amountSat",
                "feesSat",
                "paymentType",
                "status",
                "details",
            ),
        )
    ) {
        return null
    }
    val timestamp = payment.getInt("timestamp").toUInt()
    val amountSat = payment.getDouble("amountSat").toULong()
    val feesSat = payment.getDouble("feesSat").toULong()
    val paymentType = payment.getString("paymentType")?.let { asPaymentType(it) }!!
    val status = payment.getString("status")?.let { asPaymentState(it) }!!
    val details = payment.getMap("details")?.let { asPaymentDetails(it) }!!
    val swapperFeesSat = if (hasNonNullKey(payment, "swapperFeesSat")) payment.getDouble("swapperFeesSat").toULong() else null
    val destination = if (hasNonNullKey(payment, "destination")) payment.getString("destination") else null
    val txId = if (hasNonNullKey(payment, "txId")) payment.getString("txId") else null
    val unblindingData = if (hasNonNullKey(payment, "unblindingData")) payment.getString("unblindingData") else null
    return Payment(timestamp, amountSat, feesSat, paymentType, status, details, swapperFeesSat, destination, txId, unblindingData)
}

fun readableMapOf(payment: Payment): ReadableMap =
    readableMapOf(
        "timestamp" to payment.timestamp,
        "amountSat" to payment.amountSat,
        "feesSat" to payment.feesSat,
        "paymentType" to snakeToLowerCamelCase(payment.paymentType.name),
        "status" to snakeToLowerCamelCase(payment.status.name),
        "details" to readableMapOf(payment.details),
        "swapperFeesSat" to payment.swapperFeesSat,
        "destination" to payment.destination,
        "txId" to payment.txId,
        "unblindingData" to payment.unblindingData,
    )

fun asPaymentList(arr: ReadableArray): List<Payment> {
    val list = ArrayList<Payment>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asPayment(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asPrepareBuyBitcoinRequest(prepareBuyBitcoinRequest: ReadableMap): PrepareBuyBitcoinRequest? {
    if (!validateMandatoryFields(
            prepareBuyBitcoinRequest,
            arrayOf(
                "provider",
                "amountSat",
            ),
        )
    ) {
        return null
    }
    val provider = prepareBuyBitcoinRequest.getString("provider")?.let { asBuyBitcoinProvider(it) }!!
    val amountSat = prepareBuyBitcoinRequest.getDouble("amountSat").toULong()
    return PrepareBuyBitcoinRequest(provider, amountSat)
}

fun readableMapOf(prepareBuyBitcoinRequest: PrepareBuyBitcoinRequest): ReadableMap =
    readableMapOf(
        "provider" to snakeToLowerCamelCase(prepareBuyBitcoinRequest.provider.name),
        "amountSat" to prepareBuyBitcoinRequest.amountSat,
    )

fun asPrepareBuyBitcoinRequestList(arr: ReadableArray): List<PrepareBuyBitcoinRequest> {
    val list = ArrayList<PrepareBuyBitcoinRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareBuyBitcoinRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asPrepareBuyBitcoinResponse(prepareBuyBitcoinResponse: ReadableMap): PrepareBuyBitcoinResponse? {
    if (!validateMandatoryFields(
            prepareBuyBitcoinResponse,
            arrayOf(
                "provider",
                "amountSat",
                "feesSat",
            ),
        )
    ) {
        return null
    }
    val provider = prepareBuyBitcoinResponse.getString("provider")?.let { asBuyBitcoinProvider(it) }!!
    val amountSat = prepareBuyBitcoinResponse.getDouble("amountSat").toULong()
    val feesSat = prepareBuyBitcoinResponse.getDouble("feesSat").toULong()
    return PrepareBuyBitcoinResponse(provider, amountSat, feesSat)
}

fun readableMapOf(prepareBuyBitcoinResponse: PrepareBuyBitcoinResponse): ReadableMap =
    readableMapOf(
        "provider" to snakeToLowerCamelCase(prepareBuyBitcoinResponse.provider.name),
        "amountSat" to prepareBuyBitcoinResponse.amountSat,
        "feesSat" to prepareBuyBitcoinResponse.feesSat,
    )

fun asPrepareBuyBitcoinResponseList(arr: ReadableArray): List<PrepareBuyBitcoinResponse> {
    val list = ArrayList<PrepareBuyBitcoinResponse>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareBuyBitcoinResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asPrepareLnUrlPayRequest(prepareLnUrlPayRequest: ReadableMap): PrepareLnUrlPayRequest? {
    if (!validateMandatoryFields(
            prepareLnUrlPayRequest,
            arrayOf(
                "data",
                "amount",
            ),
        )
    ) {
        return null
    }
    val data = prepareLnUrlPayRequest.getMap("data")?.let { asLnUrlPayRequestData(it) }!!
    val amount = prepareLnUrlPayRequest.getMap("amount")?.let { asPayAmount(it) }!!
    val bip353Address =
        if (hasNonNullKey(
                prepareLnUrlPayRequest,
                "bip353Address",
            )
        ) {
            prepareLnUrlPayRequest.getString("bip353Address")
        } else {
            null
        }
    val comment = if (hasNonNullKey(prepareLnUrlPayRequest, "comment")) prepareLnUrlPayRequest.getString("comment") else null
    val validateSuccessActionUrl =
        if (hasNonNullKey(
                prepareLnUrlPayRequest,
                "validateSuccessActionUrl",
            )
        ) {
            prepareLnUrlPayRequest.getBoolean("validateSuccessActionUrl")
        } else {
            null
        }
    return PrepareLnUrlPayRequest(data, amount, bip353Address, comment, validateSuccessActionUrl)
}

fun readableMapOf(prepareLnUrlPayRequest: PrepareLnUrlPayRequest): ReadableMap =
    readableMapOf(
        "data" to readableMapOf(prepareLnUrlPayRequest.data),
        "amount" to readableMapOf(prepareLnUrlPayRequest.amount),
        "bip353Address" to prepareLnUrlPayRequest.bip353Address,
        "comment" to prepareLnUrlPayRequest.comment,
        "validateSuccessActionUrl" to prepareLnUrlPayRequest.validateSuccessActionUrl,
    )

fun asPrepareLnUrlPayRequestList(arr: ReadableArray): List<PrepareLnUrlPayRequest> {
    val list = ArrayList<PrepareLnUrlPayRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareLnUrlPayRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asPrepareLnUrlPayResponse(prepareLnUrlPayResponse: ReadableMap): PrepareLnUrlPayResponse? {
    if (!validateMandatoryFields(
            prepareLnUrlPayResponse,
            arrayOf(
                "destination",
                "feesSat",
                "data",
                "amount",
            ),
        )
    ) {
        return null
    }
    val destination = prepareLnUrlPayResponse.getMap("destination")?.let { asSendDestination(it) }!!
    val feesSat = prepareLnUrlPayResponse.getDouble("feesSat").toULong()
    val data = prepareLnUrlPayResponse.getMap("data")?.let { asLnUrlPayRequestData(it) }!!
    val amount = prepareLnUrlPayResponse.getMap("amount")?.let { asPayAmount(it) }!!
    val comment = if (hasNonNullKey(prepareLnUrlPayResponse, "comment")) prepareLnUrlPayResponse.getString("comment") else null
    val successAction =
        if (hasNonNullKey(prepareLnUrlPayResponse, "successAction")) {
            prepareLnUrlPayResponse.getMap("successAction")?.let {
                asSuccessAction(it)
            }
        } else {
            null
        }
    return PrepareLnUrlPayResponse(destination, feesSat, data, amount, comment, successAction)
}

fun readableMapOf(prepareLnUrlPayResponse: PrepareLnUrlPayResponse): ReadableMap =
    readableMapOf(
        "destination" to readableMapOf(prepareLnUrlPayResponse.destination),
        "feesSat" to prepareLnUrlPayResponse.feesSat,
        "data" to readableMapOf(prepareLnUrlPayResponse.data),
        "amount" to readableMapOf(prepareLnUrlPayResponse.amount),
        "comment" to prepareLnUrlPayResponse.comment,
        "successAction" to prepareLnUrlPayResponse.successAction?.let { readableMapOf(it) },
    )

fun asPrepareLnUrlPayResponseList(arr: ReadableArray): List<PrepareLnUrlPayResponse> {
    val list = ArrayList<PrepareLnUrlPayResponse>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareLnUrlPayResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asPreparePayOnchainRequest(preparePayOnchainRequest: ReadableMap): PreparePayOnchainRequest? {
    if (!validateMandatoryFields(
            preparePayOnchainRequest,
            arrayOf(
                "amount",
            ),
        )
    ) {
        return null
    }
    val amount = preparePayOnchainRequest.getMap("amount")?.let { asPayAmount(it) }!!
    val feeRateSatPerVbyte =
        if (hasNonNullKey(
                preparePayOnchainRequest,
                "feeRateSatPerVbyte",
            )
        ) {
            preparePayOnchainRequest.getInt("feeRateSatPerVbyte").toUInt()
        } else {
            null
        }
    return PreparePayOnchainRequest(amount, feeRateSatPerVbyte)
}

fun readableMapOf(preparePayOnchainRequest: PreparePayOnchainRequest): ReadableMap =
    readableMapOf(
        "amount" to readableMapOf(preparePayOnchainRequest.amount),
        "feeRateSatPerVbyte" to preparePayOnchainRequest.feeRateSatPerVbyte,
    )

fun asPreparePayOnchainRequestList(arr: ReadableArray): List<PreparePayOnchainRequest> {
    val list = ArrayList<PreparePayOnchainRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asPreparePayOnchainRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asPreparePayOnchainResponse(preparePayOnchainResponse: ReadableMap): PreparePayOnchainResponse? {
    if (!validateMandatoryFields(
            preparePayOnchainResponse,
            arrayOf(
                "receiverAmountSat",
                "claimFeesSat",
                "totalFeesSat",
            ),
        )
    ) {
        return null
    }
    val receiverAmountSat = preparePayOnchainResponse.getDouble("receiverAmountSat").toULong()
    val claimFeesSat = preparePayOnchainResponse.getDouble("claimFeesSat").toULong()
    val totalFeesSat = preparePayOnchainResponse.getDouble("totalFeesSat").toULong()
    return PreparePayOnchainResponse(receiverAmountSat, claimFeesSat, totalFeesSat)
}

fun readableMapOf(preparePayOnchainResponse: PreparePayOnchainResponse): ReadableMap =
    readableMapOf(
        "receiverAmountSat" to preparePayOnchainResponse.receiverAmountSat,
        "claimFeesSat" to preparePayOnchainResponse.claimFeesSat,
        "totalFeesSat" to preparePayOnchainResponse.totalFeesSat,
    )

fun asPreparePayOnchainResponseList(arr: ReadableArray): List<PreparePayOnchainResponse> {
    val list = ArrayList<PreparePayOnchainResponse>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asPreparePayOnchainResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asPrepareReceiveRequest(prepareReceiveRequest: ReadableMap): PrepareReceiveRequest? {
    if (!validateMandatoryFields(
            prepareReceiveRequest,
            arrayOf(
                "paymentMethod",
            ),
        )
    ) {
        return null
    }
    val paymentMethod = prepareReceiveRequest.getString("paymentMethod")?.let { asPaymentMethod(it) }!!
    val amount =
        if (hasNonNullKey(prepareReceiveRequest, "amount")) {
            prepareReceiveRequest.getMap("amount")?.let {
                asReceiveAmount(it)
            }
        } else {
            null
        }
    return PrepareReceiveRequest(paymentMethod, amount)
}

fun readableMapOf(prepareReceiveRequest: PrepareReceiveRequest): ReadableMap =
    readableMapOf(
        "paymentMethod" to snakeToLowerCamelCase(prepareReceiveRequest.paymentMethod.name),
        "amount" to prepareReceiveRequest.amount?.let { readableMapOf(it) },
    )

fun asPrepareReceiveRequestList(arr: ReadableArray): List<PrepareReceiveRequest> {
    val list = ArrayList<PrepareReceiveRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareReceiveRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asPrepareReceiveResponse(prepareReceiveResponse: ReadableMap): PrepareReceiveResponse? {
    if (!validateMandatoryFields(
            prepareReceiveResponse,
            arrayOf(
                "paymentMethod",
                "feesSat",
            ),
        )
    ) {
        return null
    }
    val paymentMethod = prepareReceiveResponse.getString("paymentMethod")?.let { asPaymentMethod(it) }!!
    val feesSat = prepareReceiveResponse.getDouble("feesSat").toULong()
    val amount =
        if (hasNonNullKey(prepareReceiveResponse, "amount")) {
            prepareReceiveResponse.getMap("amount")?.let {
                asReceiveAmount(it)
            }
        } else {
            null
        }
    val minPayerAmountSat =
        if (hasNonNullKey(
                prepareReceiveResponse,
                "minPayerAmountSat",
            )
        ) {
            prepareReceiveResponse.getDouble("minPayerAmountSat").toULong()
        } else {
            null
        }
    val maxPayerAmountSat =
        if (hasNonNullKey(
                prepareReceiveResponse,
                "maxPayerAmountSat",
            )
        ) {
            prepareReceiveResponse.getDouble("maxPayerAmountSat").toULong()
        } else {
            null
        }
    val swapperFeerate =
        if (hasNonNullKey(
                prepareReceiveResponse,
                "swapperFeerate",
            )
        ) {
            prepareReceiveResponse.getDouble("swapperFeerate")
        } else {
            null
        }
    return PrepareReceiveResponse(paymentMethod, feesSat, amount, minPayerAmountSat, maxPayerAmountSat, swapperFeerate)
}

fun readableMapOf(prepareReceiveResponse: PrepareReceiveResponse): ReadableMap =
    readableMapOf(
        "paymentMethod" to snakeToLowerCamelCase(prepareReceiveResponse.paymentMethod.name),
        "feesSat" to prepareReceiveResponse.feesSat,
        "amount" to prepareReceiveResponse.amount?.let { readableMapOf(it) },
        "minPayerAmountSat" to prepareReceiveResponse.minPayerAmountSat,
        "maxPayerAmountSat" to prepareReceiveResponse.maxPayerAmountSat,
        "swapperFeerate" to prepareReceiveResponse.swapperFeerate,
    )

fun asPrepareReceiveResponseList(arr: ReadableArray): List<PrepareReceiveResponse> {
    val list = ArrayList<PrepareReceiveResponse>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareReceiveResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asPrepareRefundRequest(prepareRefundRequest: ReadableMap): PrepareRefundRequest? {
    if (!validateMandatoryFields(
            prepareRefundRequest,
            arrayOf(
                "swapAddress",
                "refundAddress",
                "feeRateSatPerVbyte",
            ),
        )
    ) {
        return null
    }
    val swapAddress = prepareRefundRequest.getString("swapAddress")!!
    val refundAddress = prepareRefundRequest.getString("refundAddress")!!
    val feeRateSatPerVbyte = prepareRefundRequest.getInt("feeRateSatPerVbyte").toUInt()
    return PrepareRefundRequest(swapAddress, refundAddress, feeRateSatPerVbyte)
}

fun readableMapOf(prepareRefundRequest: PrepareRefundRequest): ReadableMap =
    readableMapOf(
        "swapAddress" to prepareRefundRequest.swapAddress,
        "refundAddress" to prepareRefundRequest.refundAddress,
        "feeRateSatPerVbyte" to prepareRefundRequest.feeRateSatPerVbyte,
    )

fun asPrepareRefundRequestList(arr: ReadableArray): List<PrepareRefundRequest> {
    val list = ArrayList<PrepareRefundRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareRefundRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asPrepareRefundResponse(prepareRefundResponse: ReadableMap): PrepareRefundResponse? {
    if (!validateMandatoryFields(
            prepareRefundResponse,
            arrayOf(
                "txVsize",
                "txFeeSat",
            ),
        )
    ) {
        return null
    }
    val txVsize = prepareRefundResponse.getInt("txVsize").toUInt()
    val txFeeSat = prepareRefundResponse.getDouble("txFeeSat").toULong()
    val lastRefundTxId =
        if (hasNonNullKey(
                prepareRefundResponse,
                "lastRefundTxId",
            )
        ) {
            prepareRefundResponse.getString("lastRefundTxId")
        } else {
            null
        }
    return PrepareRefundResponse(txVsize, txFeeSat, lastRefundTxId)
}

fun readableMapOf(prepareRefundResponse: PrepareRefundResponse): ReadableMap =
    readableMapOf(
        "txVsize" to prepareRefundResponse.txVsize,
        "txFeeSat" to prepareRefundResponse.txFeeSat,
        "lastRefundTxId" to prepareRefundResponse.lastRefundTxId,
    )

fun asPrepareRefundResponseList(arr: ReadableArray): List<PrepareRefundResponse> {
    val list = ArrayList<PrepareRefundResponse>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareRefundResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asPrepareSendRequest(prepareSendRequest: ReadableMap): PrepareSendRequest? {
    if (!validateMandatoryFields(
            prepareSendRequest,
            arrayOf(
                "destination",
            ),
        )
    ) {
        return null
    }
    val destination = prepareSendRequest.getString("destination")!!
    val amount = if (hasNonNullKey(prepareSendRequest, "amount")) prepareSendRequest.getMap("amount")?.let { asPayAmount(it) } else null
    return PrepareSendRequest(destination, amount)
}

fun readableMapOf(prepareSendRequest: PrepareSendRequest): ReadableMap =
    readableMapOf(
        "destination" to prepareSendRequest.destination,
        "amount" to prepareSendRequest.amount?.let { readableMapOf(it) },
    )

fun asPrepareSendRequestList(arr: ReadableArray): List<PrepareSendRequest> {
    val list = ArrayList<PrepareSendRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareSendRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asPrepareSendResponse(prepareSendResponse: ReadableMap): PrepareSendResponse? {
    if (!validateMandatoryFields(
            prepareSendResponse,
            arrayOf(
                "destination",
            ),
        )
    ) {
        return null
    }
    val destination = prepareSendResponse.getMap("destination")?.let { asSendDestination(it) }!!
    val amount = if (hasNonNullKey(prepareSendResponse, "amount")) prepareSendResponse.getMap("amount")?.let { asPayAmount(it) } else null
    val feesSat = if (hasNonNullKey(prepareSendResponse, "feesSat")) prepareSendResponse.getDouble("feesSat").toULong() else null
    val estimatedAssetFees =
        if (hasNonNullKey(
                prepareSendResponse,
                "estimatedAssetFees",
            )
        ) {
            prepareSendResponse.getDouble("estimatedAssetFees")
        } else {
            null
        }
    val exchangeAmountSat =
        if (hasNonNullKey(
                prepareSendResponse,
                "exchangeAmountSat",
            )
        ) {
            prepareSendResponse.getDouble("exchangeAmountSat").toULong()
        } else {
            null
        }
    return PrepareSendResponse(destination, amount, feesSat, estimatedAssetFees, exchangeAmountSat)
}

fun readableMapOf(prepareSendResponse: PrepareSendResponse): ReadableMap =
    readableMapOf(
        "destination" to readableMapOf(prepareSendResponse.destination),
        "amount" to prepareSendResponse.amount?.let { readableMapOf(it) },
        "feesSat" to prepareSendResponse.feesSat,
        "estimatedAssetFees" to prepareSendResponse.estimatedAssetFees,
        "exchangeAmountSat" to prepareSendResponse.exchangeAmountSat,
    )

fun asPrepareSendResponseList(arr: ReadableArray): List<PrepareSendResponse> {
    val list = ArrayList<PrepareSendResponse>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareSendResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asRate(rate: ReadableMap): Rate? {
    if (!validateMandatoryFields(
            rate,
            arrayOf(
                "coin",
                "value",
            ),
        )
    ) {
        return null
    }
    val coin = rate.getString("coin")!!
    val value = rate.getDouble("value")
    return Rate(coin, value)
}

fun readableMapOf(rate: Rate): ReadableMap =
    readableMapOf(
        "coin" to rate.coin,
        "value" to rate.value,
    )

fun asRateList(arr: ReadableArray): List<Rate> {
    val list = ArrayList<Rate>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asRate(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asReceivePaymentRequest(receivePaymentRequest: ReadableMap): ReceivePaymentRequest? {
    if (!validateMandatoryFields(
            receivePaymentRequest,
            arrayOf(
                "prepareResponse",
            ),
        )
    ) {
        return null
    }
    val prepareResponse = receivePaymentRequest.getMap("prepareResponse")?.let { asPrepareReceiveResponse(it) }!!
    val description = if (hasNonNullKey(receivePaymentRequest, "description")) receivePaymentRequest.getString("description") else null
    val useDescriptionHash =
        if (hasNonNullKey(
                receivePaymentRequest,
                "useDescriptionHash",
            )
        ) {
            receivePaymentRequest.getBoolean("useDescriptionHash")
        } else {
            null
        }
    val payerNote = if (hasNonNullKey(receivePaymentRequest, "payerNote")) receivePaymentRequest.getString("payerNote") else null
    return ReceivePaymentRequest(prepareResponse, description, useDescriptionHash, payerNote)
}

fun readableMapOf(receivePaymentRequest: ReceivePaymentRequest): ReadableMap =
    readableMapOf(
        "prepareResponse" to readableMapOf(receivePaymentRequest.prepareResponse),
        "description" to receivePaymentRequest.description,
        "useDescriptionHash" to receivePaymentRequest.useDescriptionHash,
        "payerNote" to receivePaymentRequest.payerNote,
    )

fun asReceivePaymentRequestList(arr: ReadableArray): List<ReceivePaymentRequest> {
    val list = ArrayList<ReceivePaymentRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asReceivePaymentRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asReceivePaymentResponse(receivePaymentResponse: ReadableMap): ReceivePaymentResponse? {
    if (!validateMandatoryFields(
            receivePaymentResponse,
            arrayOf(
                "destination",
            ),
        )
    ) {
        return null
    }
    val destination = receivePaymentResponse.getString("destination")!!
    val liquidExpirationBlockheight =
        if (hasNonNullKey(
                receivePaymentResponse,
                "liquidExpirationBlockheight",
            )
        ) {
            receivePaymentResponse.getInt("liquidExpirationBlockheight").toUInt()
        } else {
            null
        }
    val bitcoinExpirationBlockheight =
        if (hasNonNullKey(
                receivePaymentResponse,
                "bitcoinExpirationBlockheight",
            )
        ) {
            receivePaymentResponse.getInt("bitcoinExpirationBlockheight").toUInt()
        } else {
            null
        }
    return ReceivePaymentResponse(destination, liquidExpirationBlockheight, bitcoinExpirationBlockheight)
}

fun readableMapOf(receivePaymentResponse: ReceivePaymentResponse): ReadableMap =
    readableMapOf(
        "destination" to receivePaymentResponse.destination,
        "liquidExpirationBlockheight" to receivePaymentResponse.liquidExpirationBlockheight,
        "bitcoinExpirationBlockheight" to receivePaymentResponse.bitcoinExpirationBlockheight,
    )

fun asReceivePaymentResponseList(arr: ReadableArray): List<ReceivePaymentResponse> {
    val list = ArrayList<ReceivePaymentResponse>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asReceivePaymentResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asRecommendedFees(recommendedFees: ReadableMap): RecommendedFees? {
    if (!validateMandatoryFields(
            recommendedFees,
            arrayOf(
                "fastestFee",
                "halfHourFee",
                "hourFee",
                "economyFee",
                "minimumFee",
            ),
        )
    ) {
        return null
    }
    val fastestFee = recommendedFees.getDouble("fastestFee").toULong()
    val halfHourFee = recommendedFees.getDouble("halfHourFee").toULong()
    val hourFee = recommendedFees.getDouble("hourFee").toULong()
    val economyFee = recommendedFees.getDouble("economyFee").toULong()
    val minimumFee = recommendedFees.getDouble("minimumFee").toULong()
    return RecommendedFees(fastestFee, halfHourFee, hourFee, economyFee, minimumFee)
}

fun readableMapOf(recommendedFees: RecommendedFees): ReadableMap =
    readableMapOf(
        "fastestFee" to recommendedFees.fastestFee,
        "halfHourFee" to recommendedFees.halfHourFee,
        "hourFee" to recommendedFees.hourFee,
        "economyFee" to recommendedFees.economyFee,
        "minimumFee" to recommendedFees.minimumFee,
    )

fun asRecommendedFeesList(arr: ReadableArray): List<RecommendedFees> {
    val list = ArrayList<RecommendedFees>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asRecommendedFees(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asRefundRequest(refundRequest: ReadableMap): RefundRequest? {
    if (!validateMandatoryFields(
            refundRequest,
            arrayOf(
                "swapAddress",
                "refundAddress",
                "feeRateSatPerVbyte",
            ),
        )
    ) {
        return null
    }
    val swapAddress = refundRequest.getString("swapAddress")!!
    val refundAddress = refundRequest.getString("refundAddress")!!
    val feeRateSatPerVbyte = refundRequest.getInt("feeRateSatPerVbyte").toUInt()
    return RefundRequest(swapAddress, refundAddress, feeRateSatPerVbyte)
}

fun readableMapOf(refundRequest: RefundRequest): ReadableMap =
    readableMapOf(
        "swapAddress" to refundRequest.swapAddress,
        "refundAddress" to refundRequest.refundAddress,
        "feeRateSatPerVbyte" to refundRequest.feeRateSatPerVbyte,
    )

fun asRefundRequestList(arr: ReadableArray): List<RefundRequest> {
    val list = ArrayList<RefundRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asRefundRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asRefundResponse(refundResponse: ReadableMap): RefundResponse? {
    if (!validateMandatoryFields(
            refundResponse,
            arrayOf(
                "refundTxId",
            ),
        )
    ) {
        return null
    }
    val refundTxId = refundResponse.getString("refundTxId")!!
    return RefundResponse(refundTxId)
}

fun readableMapOf(refundResponse: RefundResponse): ReadableMap =
    readableMapOf(
        "refundTxId" to refundResponse.refundTxId,
    )

fun asRefundResponseList(arr: ReadableArray): List<RefundResponse> {
    val list = ArrayList<RefundResponse>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asRefundResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asRefundableSwap(refundableSwap: ReadableMap): RefundableSwap? {
    if (!validateMandatoryFields(
            refundableSwap,
            arrayOf(
                "swapAddress",
                "timestamp",
                "amountSat",
            ),
        )
    ) {
        return null
    }
    val swapAddress = refundableSwap.getString("swapAddress")!!
    val timestamp = refundableSwap.getInt("timestamp").toUInt()
    val amountSat = refundableSwap.getDouble("amountSat").toULong()
    val lastRefundTxId = if (hasNonNullKey(refundableSwap, "lastRefundTxId")) refundableSwap.getString("lastRefundTxId") else null
    return RefundableSwap(swapAddress, timestamp, amountSat, lastRefundTxId)
}

fun readableMapOf(refundableSwap: RefundableSwap): ReadableMap =
    readableMapOf(
        "swapAddress" to refundableSwap.swapAddress,
        "timestamp" to refundableSwap.timestamp,
        "amountSat" to refundableSwap.amountSat,
        "lastRefundTxId" to refundableSwap.lastRefundTxId,
    )

fun asRefundableSwapList(arr: ReadableArray): List<RefundableSwap> {
    val list = ArrayList<RefundableSwap>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asRefundableSwap(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asRestoreRequest(restoreRequest: ReadableMap): RestoreRequest? {
    if (!validateMandatoryFields(
            restoreRequest,
            arrayOf(),
        )
    ) {
        return null
    }
    val backupPath = if (hasNonNullKey(restoreRequest, "backupPath")) restoreRequest.getString("backupPath") else null
    return RestoreRequest(backupPath)
}

fun readableMapOf(restoreRequest: RestoreRequest): ReadableMap =
    readableMapOf(
        "backupPath" to restoreRequest.backupPath,
    )

fun asRestoreRequestList(arr: ReadableArray): List<RestoreRequest> {
    val list = ArrayList<RestoreRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asRestoreRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asRouteHint(routeHint: ReadableMap): RouteHint? {
    if (!validateMandatoryFields(
            routeHint,
            arrayOf(
                "hops",
            ),
        )
    ) {
        return null
    }
    val hops = routeHint.getArray("hops")?.let { asRouteHintHopList(it) }!!
    return RouteHint(hops)
}

fun readableMapOf(routeHint: RouteHint): ReadableMap =
    readableMapOf(
        "hops" to readableArrayOf(routeHint.hops),
    )

fun asRouteHintList(arr: ReadableArray): List<RouteHint> {
    val list = ArrayList<RouteHint>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asRouteHint(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asRouteHintHop(routeHintHop: ReadableMap): RouteHintHop? {
    if (!validateMandatoryFields(
            routeHintHop,
            arrayOf(
                "srcNodeId",
                "shortChannelId",
                "feesBaseMsat",
                "feesProportionalMillionths",
                "cltvExpiryDelta",
            ),
        )
    ) {
        return null
    }
    val srcNodeId = routeHintHop.getString("srcNodeId")!!
    val shortChannelId = routeHintHop.getString("shortChannelId")!!
    val feesBaseMsat = routeHintHop.getInt("feesBaseMsat").toUInt()
    val feesProportionalMillionths = routeHintHop.getInt("feesProportionalMillionths").toUInt()
    val cltvExpiryDelta = routeHintHop.getDouble("cltvExpiryDelta").toULong()
    val htlcMinimumMsat = if (hasNonNullKey(routeHintHop, "htlcMinimumMsat")) routeHintHop.getDouble("htlcMinimumMsat").toULong() else null
    val htlcMaximumMsat = if (hasNonNullKey(routeHintHop, "htlcMaximumMsat")) routeHintHop.getDouble("htlcMaximumMsat").toULong() else null
    return RouteHintHop(
        srcNodeId,
        shortChannelId,
        feesBaseMsat,
        feesProportionalMillionths,
        cltvExpiryDelta,
        htlcMinimumMsat,
        htlcMaximumMsat,
    )
}

fun readableMapOf(routeHintHop: RouteHintHop): ReadableMap =
    readableMapOf(
        "srcNodeId" to routeHintHop.srcNodeId,
        "shortChannelId" to routeHintHop.shortChannelId,
        "feesBaseMsat" to routeHintHop.feesBaseMsat,
        "feesProportionalMillionths" to routeHintHop.feesProportionalMillionths,
        "cltvExpiryDelta" to routeHintHop.cltvExpiryDelta,
        "htlcMinimumMsat" to routeHintHop.htlcMinimumMsat,
        "htlcMaximumMsat" to routeHintHop.htlcMaximumMsat,
    )

fun asRouteHintHopList(arr: ReadableArray): List<RouteHintHop> {
    val list = ArrayList<RouteHintHop>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asRouteHintHop(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asSendPaymentRequest(sendPaymentRequest: ReadableMap): SendPaymentRequest? {
    if (!validateMandatoryFields(
            sendPaymentRequest,
            arrayOf(
                "prepareResponse",
            ),
        )
    ) {
        return null
    }
    val prepareResponse = sendPaymentRequest.getMap("prepareResponse")?.let { asPrepareSendResponse(it) }!!
    val useAssetFees = if (hasNonNullKey(sendPaymentRequest, "useAssetFees")) sendPaymentRequest.getBoolean("useAssetFees") else null
    val payerNote = if (hasNonNullKey(sendPaymentRequest, "payerNote")) sendPaymentRequest.getString("payerNote") else null
    return SendPaymentRequest(prepareResponse, useAssetFees, payerNote)
}

fun readableMapOf(sendPaymentRequest: SendPaymentRequest): ReadableMap =
    readableMapOf(
        "prepareResponse" to readableMapOf(sendPaymentRequest.prepareResponse),
        "useAssetFees" to sendPaymentRequest.useAssetFees,
        "payerNote" to sendPaymentRequest.payerNote,
    )

fun asSendPaymentRequestList(arr: ReadableArray): List<SendPaymentRequest> {
    val list = ArrayList<SendPaymentRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asSendPaymentRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asSendPaymentResponse(sendPaymentResponse: ReadableMap): SendPaymentResponse? {
    if (!validateMandatoryFields(
            sendPaymentResponse,
            arrayOf(
                "payment",
            ),
        )
    ) {
        return null
    }
    val payment = sendPaymentResponse.getMap("payment")?.let { asPayment(it) }!!
    return SendPaymentResponse(payment)
}

fun readableMapOf(sendPaymentResponse: SendPaymentResponse): ReadableMap =
    readableMapOf(
        "payment" to readableMapOf(sendPaymentResponse.payment),
    )

fun asSendPaymentResponseList(arr: ReadableArray): List<SendPaymentResponse> {
    val list = ArrayList<SendPaymentResponse>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asSendPaymentResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asSignMessageRequest(signMessageRequest: ReadableMap): SignMessageRequest? {
    if (!validateMandatoryFields(
            signMessageRequest,
            arrayOf(
                "message",
            ),
        )
    ) {
        return null
    }
    val message = signMessageRequest.getString("message")!!
    return SignMessageRequest(message)
}

fun readableMapOf(signMessageRequest: SignMessageRequest): ReadableMap =
    readableMapOf(
        "message" to signMessageRequest.message,
    )

fun asSignMessageRequestList(arr: ReadableArray): List<SignMessageRequest> {
    val list = ArrayList<SignMessageRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asSignMessageRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asSignMessageResponse(signMessageResponse: ReadableMap): SignMessageResponse? {
    if (!validateMandatoryFields(
            signMessageResponse,
            arrayOf(
                "signature",
            ),
        )
    ) {
        return null
    }
    val signature = signMessageResponse.getString("signature")!!
    return SignMessageResponse(signature)
}

fun readableMapOf(signMessageResponse: SignMessageResponse): ReadableMap =
    readableMapOf(
        "signature" to signMessageResponse.signature,
    )

fun asSignMessageResponseList(arr: ReadableArray): List<SignMessageResponse> {
    val list = ArrayList<SignMessageResponse>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asSignMessageResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asSymbol(symbol: ReadableMap): Symbol? {
    if (!validateMandatoryFields(
            symbol,
            arrayOf(),
        )
    ) {
        return null
    }
    val grapheme = if (hasNonNullKey(symbol, "grapheme")) symbol.getString("grapheme") else null
    val template = if (hasNonNullKey(symbol, "template")) symbol.getString("template") else null
    val rtl = if (hasNonNullKey(symbol, "rtl")) symbol.getBoolean("rtl") else null
    val position = if (hasNonNullKey(symbol, "position")) symbol.getInt("position").toUInt() else null
    return Symbol(grapheme, template, rtl, position)
}

fun readableMapOf(symbol: Symbol): ReadableMap =
    readableMapOf(
        "grapheme" to symbol.grapheme,
        "template" to symbol.template,
        "rtl" to symbol.rtl,
        "position" to symbol.position,
    )

fun asSymbolList(arr: ReadableArray): List<Symbol> {
    val list = ArrayList<Symbol>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asSymbol(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asUrlSuccessActionData(urlSuccessActionData: ReadableMap): UrlSuccessActionData? {
    if (!validateMandatoryFields(
            urlSuccessActionData,
            arrayOf(
                "description",
                "url",
                "matchesCallbackDomain",
            ),
        )
    ) {
        return null
    }
    val description = urlSuccessActionData.getString("description")!!
    val url = urlSuccessActionData.getString("url")!!
    val matchesCallbackDomain = urlSuccessActionData.getBoolean("matchesCallbackDomain")
    return UrlSuccessActionData(description, url, matchesCallbackDomain)
}

fun readableMapOf(urlSuccessActionData: UrlSuccessActionData): ReadableMap =
    readableMapOf(
        "description" to urlSuccessActionData.description,
        "url" to urlSuccessActionData.url,
        "matchesCallbackDomain" to urlSuccessActionData.matchesCallbackDomain,
    )

fun asUrlSuccessActionDataList(arr: ReadableArray): List<UrlSuccessActionData> {
    val list = ArrayList<UrlSuccessActionData>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asUrlSuccessActionData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asWalletInfo(walletInfo: ReadableMap): WalletInfo? {
    if (!validateMandatoryFields(
            walletInfo,
            arrayOf(
                "balanceSat",
                "pendingSendSat",
                "pendingReceiveSat",
                "fingerprint",
                "pubkey",
                "assetBalances",
            ),
        )
    ) {
        return null
    }
    val balanceSat = walletInfo.getDouble("balanceSat").toULong()
    val pendingSendSat = walletInfo.getDouble("pendingSendSat").toULong()
    val pendingReceiveSat = walletInfo.getDouble("pendingReceiveSat").toULong()
    val fingerprint = walletInfo.getString("fingerprint")!!
    val pubkey = walletInfo.getString("pubkey")!!
    val assetBalances = walletInfo.getArray("assetBalances")?.let { asAssetBalanceList(it) }!!
    return WalletInfo(balanceSat, pendingSendSat, pendingReceiveSat, fingerprint, pubkey, assetBalances)
}

fun readableMapOf(walletInfo: WalletInfo): ReadableMap =
    readableMapOf(
        "balanceSat" to walletInfo.balanceSat,
        "pendingSendSat" to walletInfo.pendingSendSat,
        "pendingReceiveSat" to walletInfo.pendingReceiveSat,
        "fingerprint" to walletInfo.fingerprint,
        "pubkey" to walletInfo.pubkey,
        "assetBalances" to readableArrayOf(walletInfo.assetBalances),
    )

fun asWalletInfoList(arr: ReadableArray): List<WalletInfo> {
    val list = ArrayList<WalletInfo>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asWalletInfo(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asAesSuccessActionDataResult(aesSuccessActionDataResult: ReadableMap): AesSuccessActionDataResult? {
    val type = aesSuccessActionDataResult.getString("type")

    if (type == "decrypted") {
        val data = aesSuccessActionDataResult.getMap("data")?.let { asAesSuccessActionDataDecrypted(it) }!!
        return AesSuccessActionDataResult.Decrypted(data)
    }
    if (type == "errorStatus") {
        val reason = aesSuccessActionDataResult.getString("reason")!!
        return AesSuccessActionDataResult.ErrorStatus(reason)
    }
    return null
}

fun readableMapOf(aesSuccessActionDataResult: AesSuccessActionDataResult): ReadableMap? {
    val map = Arguments.createMap()
    when (aesSuccessActionDataResult) {
        is AesSuccessActionDataResult.Decrypted -> {
            pushToMap(map, "type", "decrypted")
            pushToMap(map, "data", readableMapOf(aesSuccessActionDataResult.data))
        }
        is AesSuccessActionDataResult.ErrorStatus -> {
            pushToMap(map, "type", "errorStatus")
            pushToMap(map, "reason", aesSuccessActionDataResult.reason)
        }
    }
    return map
}

fun asAesSuccessActionDataResultList(arr: ReadableArray): List<AesSuccessActionDataResult> {
    val list = ArrayList<AesSuccessActionDataResult>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asAesSuccessActionDataResult(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asAmount(amount: ReadableMap): Amount? {
    val type = amount.getString("type")

    if (type == "bitcoin") {
        val amountMsat = amount.getDouble("amountMsat").toULong()
        return Amount.Bitcoin(amountMsat)
    }
    if (type == "currency") {
        val iso4217Code = amount.getString("iso4217Code")!!
        val fractionalAmount = amount.getDouble("fractionalAmount").toULong()
        return Amount.Currency(iso4217Code, fractionalAmount)
    }
    return null
}

fun readableMapOf(amount: Amount): ReadableMap? {
    val map = Arguments.createMap()
    when (amount) {
        is Amount.Bitcoin -> {
            pushToMap(map, "type", "bitcoin")
            pushToMap(map, "amountMsat", amount.amountMsat)
        }
        is Amount.Currency -> {
            pushToMap(map, "type", "currency")
            pushToMap(map, "iso4217Code", amount.iso4217Code)
            pushToMap(map, "fractionalAmount", amount.fractionalAmount)
        }
    }
    return map
}

fun asAmountList(arr: ReadableArray): List<Amount> {
    val list = ArrayList<Amount>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asAmount(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asBlockchainExplorer(blockchainExplorer: ReadableMap): BlockchainExplorer? {
    val type = blockchainExplorer.getString("type")

    if (type == "electrum") {
        val url = blockchainExplorer.getString("url")!!
        return BlockchainExplorer.Electrum(url)
    }
    if (type == "esplora") {
        val url = blockchainExplorer.getString("url")!!
        val useWaterfalls = blockchainExplorer.getBoolean("useWaterfalls")
        return BlockchainExplorer.Esplora(url, useWaterfalls)
    }
    return null
}

fun readableMapOf(blockchainExplorer: BlockchainExplorer): ReadableMap? {
    val map = Arguments.createMap()
    when (blockchainExplorer) {
        is BlockchainExplorer.Electrum -> {
            pushToMap(map, "type", "electrum")
            pushToMap(map, "url", blockchainExplorer.url)
        }
        is BlockchainExplorer.Esplora -> {
            pushToMap(map, "type", "esplora")
            pushToMap(map, "url", blockchainExplorer.url)
            pushToMap(map, "useWaterfalls", blockchainExplorer.useWaterfalls)
        }
    }
    return map
}

fun asBlockchainExplorerList(arr: ReadableArray): List<BlockchainExplorer> {
    val list = ArrayList<BlockchainExplorer>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asBlockchainExplorer(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asBuyBitcoinProvider(type: String): BuyBitcoinProvider = BuyBitcoinProvider.valueOf(camelToUpperSnakeCase(type))

fun asBuyBitcoinProviderList(arr: ReadableArray): List<BuyBitcoinProvider> {
    val list = ArrayList<BuyBitcoinProvider>()
    for (value in arr.toList()) {
        when (value) {
            is String -> list.add(asBuyBitcoinProvider(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asGetPaymentRequest(getPaymentRequest: ReadableMap): GetPaymentRequest? {
    val type = getPaymentRequest.getString("type")

    if (type == "paymentHash") {
        val paymentHash = getPaymentRequest.getString("paymentHash")!!
        return GetPaymentRequest.PaymentHash(paymentHash)
    }
    if (type == "swapId") {
        val swapId = getPaymentRequest.getString("swapId")!!
        return GetPaymentRequest.SwapId(swapId)
    }
    return null
}

fun readableMapOf(getPaymentRequest: GetPaymentRequest): ReadableMap? {
    val map = Arguments.createMap()
    when (getPaymentRequest) {
        is GetPaymentRequest.PaymentHash -> {
            pushToMap(map, "type", "paymentHash")
            pushToMap(map, "paymentHash", getPaymentRequest.paymentHash)
        }
        is GetPaymentRequest.SwapId -> {
            pushToMap(map, "type", "swapId")
            pushToMap(map, "swapId", getPaymentRequest.swapId)
        }
    }
    return map
}

fun asGetPaymentRequestList(arr: ReadableArray): List<GetPaymentRequest> {
    val list = ArrayList<GetPaymentRequest>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asGetPaymentRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asInputType(inputType: ReadableMap): InputType? {
    val type = inputType.getString("type")

    if (type == "bitcoinAddress") {
        val address = inputType.getMap("address")?.let { asBitcoinAddressData(it) }!!
        return InputType.BitcoinAddress(address)
    }
    if (type == "liquidAddress") {
        val address = inputType.getMap("address")?.let { asLiquidAddressData(it) }!!
        return InputType.LiquidAddress(address)
    }
    if (type == "bolt11") {
        val invoice = inputType.getMap("invoice")?.let { asLnInvoice(it) }!!
        return InputType.Bolt11(invoice)
    }
    if (type == "bolt12Offer") {
        val offer = inputType.getMap("offer")?.let { asLnOffer(it) }!!
        val bip353Address = if (hasNonNullKey(inputType, "bip353Address")) inputType.getString("bip353Address") else null
        return InputType.Bolt12Offer(offer, bip353Address)
    }
    if (type == "nodeId") {
        val nodeId = inputType.getString("nodeId")!!
        return InputType.NodeId(nodeId)
    }
    if (type == "url") {
        val url = inputType.getString("url")!!
        return InputType.Url(url)
    }
    if (type == "lnUrlPay") {
        val data = inputType.getMap("data")?.let { asLnUrlPayRequestData(it) }!!
        val bip353Address = if (hasNonNullKey(inputType, "bip353Address")) inputType.getString("bip353Address") else null
        return InputType.LnUrlPay(data, bip353Address)
    }
    if (type == "lnUrlWithdraw") {
        val data = inputType.getMap("data")?.let { asLnUrlWithdrawRequestData(it) }!!
        return InputType.LnUrlWithdraw(data)
    }
    if (type == "lnUrlAuth") {
        val data = inputType.getMap("data")?.let { asLnUrlAuthRequestData(it) }!!
        return InputType.LnUrlAuth(data)
    }
    if (type == "lnUrlError") {
        val data = inputType.getMap("data")?.let { asLnUrlErrorData(it) }!!
        return InputType.LnUrlError(data)
    }
    return null
}

fun readableMapOf(inputType: InputType): ReadableMap? {
    val map = Arguments.createMap()
    when (inputType) {
        is InputType.BitcoinAddress -> {
            pushToMap(map, "type", "bitcoinAddress")
            pushToMap(map, "address", readableMapOf(inputType.address))
        }
        is InputType.LiquidAddress -> {
            pushToMap(map, "type", "liquidAddress")
            pushToMap(map, "address", readableMapOf(inputType.address))
        }
        is InputType.Bolt11 -> {
            pushToMap(map, "type", "bolt11")
            pushToMap(map, "invoice", readableMapOf(inputType.invoice))
        }
        is InputType.Bolt12Offer -> {
            pushToMap(map, "type", "bolt12Offer")
            pushToMap(map, "offer", readableMapOf(inputType.offer))
            pushToMap(map, "bip353Address", inputType.bip353Address)
        }
        is InputType.NodeId -> {
            pushToMap(map, "type", "nodeId")
            pushToMap(map, "nodeId", inputType.nodeId)
        }
        is InputType.Url -> {
            pushToMap(map, "type", "url")
            pushToMap(map, "url", inputType.url)
        }
        is InputType.LnUrlPay -> {
            pushToMap(map, "type", "lnUrlPay")
            pushToMap(map, "data", readableMapOf(inputType.data))
            pushToMap(map, "bip353Address", inputType.bip353Address)
        }
        is InputType.LnUrlWithdraw -> {
            pushToMap(map, "type", "lnUrlWithdraw")
            pushToMap(map, "data", readableMapOf(inputType.data))
        }
        is InputType.LnUrlAuth -> {
            pushToMap(map, "type", "lnUrlAuth")
            pushToMap(map, "data", readableMapOf(inputType.data))
        }
        is InputType.LnUrlError -> {
            pushToMap(map, "type", "lnUrlError")
            pushToMap(map, "data", readableMapOf(inputType.data))
        }
    }
    return map
}

fun asInputTypeList(arr: ReadableArray): List<InputType> {
    val list = ArrayList<InputType>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asInputType(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLiquidNetwork(type: String): LiquidNetwork = LiquidNetwork.valueOf(camelToUpperSnakeCase(type))

fun asLiquidNetworkList(arr: ReadableArray): List<LiquidNetwork> {
    val list = ArrayList<LiquidNetwork>()
    for (value in arr.toList()) {
        when (value) {
            is String -> list.add(asLiquidNetwork(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asListPaymentDetails(listPaymentDetails: ReadableMap): ListPaymentDetails? {
    val type = listPaymentDetails.getString("type")

    if (type == "liquid") {
        val assetId = if (hasNonNullKey(listPaymentDetails, "assetId")) listPaymentDetails.getString("assetId") else null
        val destination = if (hasNonNullKey(listPaymentDetails, "destination")) listPaymentDetails.getString("destination") else null
        return ListPaymentDetails.Liquid(assetId, destination)
    }
    if (type == "bitcoin") {
        val address = if (hasNonNullKey(listPaymentDetails, "address")) listPaymentDetails.getString("address") else null
        return ListPaymentDetails.Bitcoin(address)
    }
    return null
}

fun readableMapOf(listPaymentDetails: ListPaymentDetails): ReadableMap? {
    val map = Arguments.createMap()
    when (listPaymentDetails) {
        is ListPaymentDetails.Liquid -> {
            pushToMap(map, "type", "liquid")
            pushToMap(map, "assetId", listPaymentDetails.assetId)
            pushToMap(map, "destination", listPaymentDetails.destination)
        }
        is ListPaymentDetails.Bitcoin -> {
            pushToMap(map, "type", "bitcoin")
            pushToMap(map, "address", listPaymentDetails.address)
        }
    }
    return map
}

fun asListPaymentDetailsList(arr: ReadableArray): List<ListPaymentDetails> {
    val list = ArrayList<ListPaymentDetails>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asListPaymentDetails(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLnUrlCallbackStatus(lnUrlCallbackStatus: ReadableMap): LnUrlCallbackStatus? {
    val type = lnUrlCallbackStatus.getString("type")

    if (type == "ok") {
        return LnUrlCallbackStatus.Ok
    }
    if (type == "errorStatus") {
        val data = lnUrlCallbackStatus.getMap("data")?.let { asLnUrlErrorData(it) }!!
        return LnUrlCallbackStatus.ErrorStatus(data)
    }
    return null
}

fun readableMapOf(lnUrlCallbackStatus: LnUrlCallbackStatus): ReadableMap? {
    val map = Arguments.createMap()
    when (lnUrlCallbackStatus) {
        is LnUrlCallbackStatus.Ok -> {
            pushToMap(map, "type", "ok")
        }
        is LnUrlCallbackStatus.ErrorStatus -> {
            pushToMap(map, "type", "errorStatus")
            pushToMap(map, "data", readableMapOf(lnUrlCallbackStatus.data))
        }
    }
    return map
}

fun asLnUrlCallbackStatusList(arr: ReadableArray): List<LnUrlCallbackStatus> {
    val list = ArrayList<LnUrlCallbackStatus>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlCallbackStatus(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLnUrlPayResult(lnUrlPayResult: ReadableMap): LnUrlPayResult? {
    val type = lnUrlPayResult.getString("type")

    if (type == "endpointSuccess") {
        val data = lnUrlPayResult.getMap("data")?.let { asLnUrlPaySuccessData(it) }!!
        return LnUrlPayResult.EndpointSuccess(data)
    }
    if (type == "endpointError") {
        val data = lnUrlPayResult.getMap("data")?.let { asLnUrlErrorData(it) }!!
        return LnUrlPayResult.EndpointError(data)
    }
    if (type == "payError") {
        val data = lnUrlPayResult.getMap("data")?.let { asLnUrlPayErrorData(it) }!!
        return LnUrlPayResult.PayError(data)
    }
    return null
}

fun readableMapOf(lnUrlPayResult: LnUrlPayResult): ReadableMap? {
    val map = Arguments.createMap()
    when (lnUrlPayResult) {
        is LnUrlPayResult.EndpointSuccess -> {
            pushToMap(map, "type", "endpointSuccess")
            pushToMap(map, "data", readableMapOf(lnUrlPayResult.data))
        }
        is LnUrlPayResult.EndpointError -> {
            pushToMap(map, "type", "endpointError")
            pushToMap(map, "data", readableMapOf(lnUrlPayResult.data))
        }
        is LnUrlPayResult.PayError -> {
            pushToMap(map, "type", "payError")
            pushToMap(map, "data", readableMapOf(lnUrlPayResult.data))
        }
    }
    return map
}

fun asLnUrlPayResultList(arr: ReadableArray): List<LnUrlPayResult> {
    val list = ArrayList<LnUrlPayResult>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlPayResult(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asLnUrlWithdrawResult(lnUrlWithdrawResult: ReadableMap): LnUrlWithdrawResult? {
    val type = lnUrlWithdrawResult.getString("type")

    if (type == "ok") {
        val data = lnUrlWithdrawResult.getMap("data")?.let { asLnUrlWithdrawSuccessData(it) }!!
        return LnUrlWithdrawResult.Ok(data)
    }
    if (type == "timeout") {
        val data = lnUrlWithdrawResult.getMap("data")?.let { asLnUrlWithdrawSuccessData(it) }!!
        return LnUrlWithdrawResult.Timeout(data)
    }
    if (type == "errorStatus") {
        val data = lnUrlWithdrawResult.getMap("data")?.let { asLnUrlErrorData(it) }!!
        return LnUrlWithdrawResult.ErrorStatus(data)
    }
    return null
}

fun readableMapOf(lnUrlWithdrawResult: LnUrlWithdrawResult): ReadableMap? {
    val map = Arguments.createMap()
    when (lnUrlWithdrawResult) {
        is LnUrlWithdrawResult.Ok -> {
            pushToMap(map, "type", "ok")
            pushToMap(map, "data", readableMapOf(lnUrlWithdrawResult.data))
        }
        is LnUrlWithdrawResult.Timeout -> {
            pushToMap(map, "type", "timeout")
            pushToMap(map, "data", readableMapOf(lnUrlWithdrawResult.data))
        }
        is LnUrlWithdrawResult.ErrorStatus -> {
            pushToMap(map, "type", "errorStatus")
            pushToMap(map, "data", readableMapOf(lnUrlWithdrawResult.data))
        }
    }
    return map
}

fun asLnUrlWithdrawResultList(arr: ReadableArray): List<LnUrlWithdrawResult> {
    val list = ArrayList<LnUrlWithdrawResult>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlWithdrawResult(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asNetwork(type: String): Network = Network.valueOf(camelToUpperSnakeCase(type))

fun asNetworkList(arr: ReadableArray): List<Network> {
    val list = ArrayList<Network>()
    for (value in arr.toList()) {
        when (value) {
            is String -> list.add(asNetwork(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asPayAmount(payAmount: ReadableMap): PayAmount? {
    val type = payAmount.getString("type")

    if (type == "bitcoin") {
        val receiverAmountSat = payAmount.getDouble("receiverAmountSat").toULong()
        return PayAmount.Bitcoin(receiverAmountSat)
    }
    if (type == "asset") {
        val toAsset = payAmount.getString("toAsset")!!
        val receiverAmount = payAmount.getDouble("receiverAmount")
        val estimateAssetFees = if (hasNonNullKey(payAmount, "estimateAssetFees")) payAmount.getBoolean("estimateAssetFees") else null
        val fromAsset = if (hasNonNullKey(payAmount, "fromAsset")) payAmount.getString("fromAsset") else null
        return PayAmount.Asset(toAsset, receiverAmount, estimateAssetFees, fromAsset)
    }
    if (type == "drain") {
        return PayAmount.Drain
    }
    return null
}

fun readableMapOf(payAmount: PayAmount): ReadableMap? {
    val map = Arguments.createMap()
    when (payAmount) {
        is PayAmount.Bitcoin -> {
            pushToMap(map, "type", "bitcoin")
            pushToMap(map, "receiverAmountSat", payAmount.receiverAmountSat)
        }
        is PayAmount.Asset -> {
            pushToMap(map, "type", "asset")
            pushToMap(map, "toAsset", payAmount.toAsset)
            pushToMap(map, "receiverAmount", payAmount.receiverAmount)
            pushToMap(map, "estimateAssetFees", payAmount.estimateAssetFees)
            pushToMap(map, "fromAsset", payAmount.fromAsset)
        }
        is PayAmount.Drain -> {
            pushToMap(map, "type", "drain")
        }
    }
    return map
}

fun asPayAmountList(arr: ReadableArray): List<PayAmount> {
    val list = ArrayList<PayAmount>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asPayAmount(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asPaymentDetails(paymentDetails: ReadableMap): PaymentDetails? {
    val type = paymentDetails.getString("type")

    if (type == "lightning") {
        val swapId = paymentDetails.getString("swapId")!!
        val description = paymentDetails.getString("description")!!
        val liquidExpirationBlockheight = paymentDetails.getInt("liquidExpirationBlockheight").toUInt()
        val preimage = if (hasNonNullKey(paymentDetails, "preimage")) paymentDetails.getString("preimage") else null
        val invoice = if (hasNonNullKey(paymentDetails, "invoice")) paymentDetails.getString("invoice") else null
        val bolt12Offer = if (hasNonNullKey(paymentDetails, "bolt12Offer")) paymentDetails.getString("bolt12Offer") else null
        val paymentHash = if (hasNonNullKey(paymentDetails, "paymentHash")) paymentDetails.getString("paymentHash") else null
        val destinationPubkey =
            if (hasNonNullKey(
                    paymentDetails,
                    "destinationPubkey",
                )
            ) {
                paymentDetails.getString("destinationPubkey")
            } else {
                null
            }
        val lnurlInfo =
            if (hasNonNullKey(
                    paymentDetails,
                    "lnurlInfo",
                )
            ) {
                paymentDetails.getMap("lnurlInfo")?.let { asLnUrlInfo(it) }
            } else {
                null
            }
        val bip353Address = if (hasNonNullKey(paymentDetails, "bip353Address")) paymentDetails.getString("bip353Address") else null
        val payerNote = if (hasNonNullKey(paymentDetails, "payerNote")) paymentDetails.getString("payerNote") else null
        val claimTxId = if (hasNonNullKey(paymentDetails, "claimTxId")) paymentDetails.getString("claimTxId") else null
        val refundTxId = if (hasNonNullKey(paymentDetails, "refundTxId")) paymentDetails.getString("refundTxId") else null
        val refundTxAmountSat =
            if (hasNonNullKey(
                    paymentDetails,
                    "refundTxAmountSat",
                )
            ) {
                paymentDetails.getDouble("refundTxAmountSat").toULong()
            } else {
                null
            }
        return PaymentDetails.Lightning(
            swapId,
            description,
            liquidExpirationBlockheight,
            preimage,
            invoice,
            bolt12Offer,
            paymentHash,
            destinationPubkey,
            lnurlInfo,
            bip353Address,
            payerNote,
            claimTxId,
            refundTxId,
            refundTxAmountSat,
        )
    }
    if (type == "liquid") {
        val assetId = paymentDetails.getString("assetId")!!
        val destination = paymentDetails.getString("destination")!!
        val description = paymentDetails.getString("description")!!
        val assetInfo =
            if (hasNonNullKey(
                    paymentDetails,
                    "assetInfo",
                )
            ) {
                paymentDetails.getMap("assetInfo")?.let { asAssetInfo(it) }
            } else {
                null
            }
        val lnurlInfo =
            if (hasNonNullKey(
                    paymentDetails,
                    "lnurlInfo",
                )
            ) {
                paymentDetails.getMap("lnurlInfo")?.let { asLnUrlInfo(it) }
            } else {
                null
            }
        val bip353Address = if (hasNonNullKey(paymentDetails, "bip353Address")) paymentDetails.getString("bip353Address") else null
        val payerNote = if (hasNonNullKey(paymentDetails, "payerNote")) paymentDetails.getString("payerNote") else null
        return PaymentDetails.Liquid(assetId, destination, description, assetInfo, lnurlInfo, bip353Address, payerNote)
    }
    if (type == "bitcoin") {
        val swapId = paymentDetails.getString("swapId")!!
        val bitcoinAddress = paymentDetails.getString("bitcoinAddress")!!
        val description = paymentDetails.getString("description")!!
        val autoAcceptedFees = paymentDetails.getBoolean("autoAcceptedFees")
        val bitcoinExpirationBlockheight = paymentDetails.getInt("bitcoinExpirationBlockheight").toUInt()
        val liquidExpirationBlockheight = paymentDetails.getInt("liquidExpirationBlockheight").toUInt()
        val lockupTxId = if (hasNonNullKey(paymentDetails, "lockupTxId")) paymentDetails.getString("lockupTxId") else null
        val claimTxId = if (hasNonNullKey(paymentDetails, "claimTxId")) paymentDetails.getString("claimTxId") else null
        val refundTxId = if (hasNonNullKey(paymentDetails, "refundTxId")) paymentDetails.getString("refundTxId") else null
        val refundTxAmountSat =
            if (hasNonNullKey(
                    paymentDetails,
                    "refundTxAmountSat",
                )
            ) {
                paymentDetails.getDouble("refundTxAmountSat").toULong()
            } else {
                null
            }
        return PaymentDetails.Bitcoin(
            swapId,
            bitcoinAddress,
            description,
            autoAcceptedFees,
            bitcoinExpirationBlockheight,
            liquidExpirationBlockheight,
            lockupTxId,
            claimTxId,
            refundTxId,
            refundTxAmountSat,
        )
    }
    return null
}

fun readableMapOf(paymentDetails: PaymentDetails): ReadableMap? {
    val map = Arguments.createMap()
    when (paymentDetails) {
        is PaymentDetails.Lightning -> {
            pushToMap(map, "type", "lightning")
            pushToMap(map, "swapId", paymentDetails.swapId)
            pushToMap(map, "description", paymentDetails.description)
            pushToMap(map, "liquidExpirationBlockheight", paymentDetails.liquidExpirationBlockheight)
            pushToMap(map, "preimage", paymentDetails.preimage)
            pushToMap(map, "invoice", paymentDetails.invoice)
            pushToMap(map, "bolt12Offer", paymentDetails.bolt12Offer)
            pushToMap(map, "paymentHash", paymentDetails.paymentHash)
            pushToMap(map, "destinationPubkey", paymentDetails.destinationPubkey)
            pushToMap(map, "lnurlInfo", paymentDetails.lnurlInfo?.let { readableMapOf(it) })
            pushToMap(map, "bip353Address", paymentDetails.bip353Address)
            pushToMap(map, "payerNote", paymentDetails.payerNote)
            pushToMap(map, "claimTxId", paymentDetails.claimTxId)
            pushToMap(map, "refundTxId", paymentDetails.refundTxId)
            pushToMap(map, "refundTxAmountSat", paymentDetails.refundTxAmountSat)
        }
        is PaymentDetails.Liquid -> {
            pushToMap(map, "type", "liquid")
            pushToMap(map, "assetId", paymentDetails.assetId)
            pushToMap(map, "destination", paymentDetails.destination)
            pushToMap(map, "description", paymentDetails.description)
            pushToMap(map, "assetInfo", paymentDetails.assetInfo?.let { readableMapOf(it) })
            pushToMap(map, "lnurlInfo", paymentDetails.lnurlInfo?.let { readableMapOf(it) })
            pushToMap(map, "bip353Address", paymentDetails.bip353Address)
            pushToMap(map, "payerNote", paymentDetails.payerNote)
        }
        is PaymentDetails.Bitcoin -> {
            pushToMap(map, "type", "bitcoin")
            pushToMap(map, "swapId", paymentDetails.swapId)
            pushToMap(map, "bitcoinAddress", paymentDetails.bitcoinAddress)
            pushToMap(map, "description", paymentDetails.description)
            pushToMap(map, "autoAcceptedFees", paymentDetails.autoAcceptedFees)
            pushToMap(map, "bitcoinExpirationBlockheight", paymentDetails.bitcoinExpirationBlockheight)
            pushToMap(map, "liquidExpirationBlockheight", paymentDetails.liquidExpirationBlockheight)
            pushToMap(map, "lockupTxId", paymentDetails.lockupTxId)
            pushToMap(map, "claimTxId", paymentDetails.claimTxId)
            pushToMap(map, "refundTxId", paymentDetails.refundTxId)
            pushToMap(map, "refundTxAmountSat", paymentDetails.refundTxAmountSat)
        }
    }
    return map
}

fun asPaymentDetailsList(arr: ReadableArray): List<PaymentDetails> {
    val list = ArrayList<PaymentDetails>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asPaymentDetails(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asPaymentMethod(type: String): PaymentMethod = PaymentMethod.valueOf(camelToUpperSnakeCase(type))

fun asPaymentMethodList(arr: ReadableArray): List<PaymentMethod> {
    val list = ArrayList<PaymentMethod>()
    for (value in arr.toList()) {
        when (value) {
            is String -> list.add(asPaymentMethod(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asPaymentState(type: String): PaymentState = PaymentState.valueOf(camelToUpperSnakeCase(type))

fun asPaymentStateList(arr: ReadableArray): List<PaymentState> {
    val list = ArrayList<PaymentState>()
    for (value in arr.toList()) {
        when (value) {
            is String -> list.add(asPaymentState(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asPaymentType(type: String): PaymentType = PaymentType.valueOf(camelToUpperSnakeCase(type))

fun asPaymentTypeList(arr: ReadableArray): List<PaymentType> {
    val list = ArrayList<PaymentType>()
    for (value in arr.toList()) {
        when (value) {
            is String -> list.add(asPaymentType(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asReceiveAmount(receiveAmount: ReadableMap): ReceiveAmount? {
    val type = receiveAmount.getString("type")

    if (type == "bitcoin") {
        val payerAmountSat = receiveAmount.getDouble("payerAmountSat").toULong()
        return ReceiveAmount.Bitcoin(payerAmountSat)
    }
    if (type == "asset") {
        val assetId = receiveAmount.getString("assetId")!!
        val payerAmount = if (hasNonNullKey(receiveAmount, "payerAmount")) receiveAmount.getDouble("payerAmount") else null
        return ReceiveAmount.Asset(assetId, payerAmount)
    }
    return null
}

fun readableMapOf(receiveAmount: ReceiveAmount): ReadableMap? {
    val map = Arguments.createMap()
    when (receiveAmount) {
        is ReceiveAmount.Bitcoin -> {
            pushToMap(map, "type", "bitcoin")
            pushToMap(map, "payerAmountSat", receiveAmount.payerAmountSat)
        }
        is ReceiveAmount.Asset -> {
            pushToMap(map, "type", "asset")
            pushToMap(map, "assetId", receiveAmount.assetId)
            pushToMap(map, "payerAmount", receiveAmount.payerAmount)
        }
    }
    return map
}

fun asReceiveAmountList(arr: ReadableArray): List<ReceiveAmount> {
    val list = ArrayList<ReceiveAmount>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asReceiveAmount(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asSdkEvent(sdkEvent: ReadableMap): SdkEvent? {
    val type = sdkEvent.getString("type")

    if (type == "paymentFailed") {
        val details = sdkEvent.getMap("details")?.let { asPayment(it) }!!
        return SdkEvent.PaymentFailed(details)
    }
    if (type == "paymentPending") {
        val details = sdkEvent.getMap("details")?.let { asPayment(it) }!!
        return SdkEvent.PaymentPending(details)
    }
    if (type == "paymentRefundable") {
        val details = sdkEvent.getMap("details")?.let { asPayment(it) }!!
        return SdkEvent.PaymentRefundable(details)
    }
    if (type == "paymentRefunded") {
        val details = sdkEvent.getMap("details")?.let { asPayment(it) }!!
        return SdkEvent.PaymentRefunded(details)
    }
    if (type == "paymentRefundPending") {
        val details = sdkEvent.getMap("details")?.let { asPayment(it) }!!
        return SdkEvent.PaymentRefundPending(details)
    }
    if (type == "paymentSucceeded") {
        val details = sdkEvent.getMap("details")?.let { asPayment(it) }!!
        return SdkEvent.PaymentSucceeded(details)
    }
    if (type == "paymentWaitingConfirmation") {
        val details = sdkEvent.getMap("details")?.let { asPayment(it) }!!
        return SdkEvent.PaymentWaitingConfirmation(details)
    }
    if (type == "paymentWaitingFeeAcceptance") {
        val details = sdkEvent.getMap("details")?.let { asPayment(it) }!!
        return SdkEvent.PaymentWaitingFeeAcceptance(details)
    }
    if (type == "synced") {
        return SdkEvent.Synced
    }
    if (type == "dataSynced") {
        val didPullNewRecords = sdkEvent.getBoolean("didPullNewRecords")
        return SdkEvent.DataSynced(didPullNewRecords)
    }
    return null
}

fun readableMapOf(sdkEvent: SdkEvent): ReadableMap? {
    val map = Arguments.createMap()
    when (sdkEvent) {
        is SdkEvent.PaymentFailed -> {
            pushToMap(map, "type", "paymentFailed")
            pushToMap(map, "details", readableMapOf(sdkEvent.details))
        }
        is SdkEvent.PaymentPending -> {
            pushToMap(map, "type", "paymentPending")
            pushToMap(map, "details", readableMapOf(sdkEvent.details))
        }
        is SdkEvent.PaymentRefundable -> {
            pushToMap(map, "type", "paymentRefundable")
            pushToMap(map, "details", readableMapOf(sdkEvent.details))
        }
        is SdkEvent.PaymentRefunded -> {
            pushToMap(map, "type", "paymentRefunded")
            pushToMap(map, "details", readableMapOf(sdkEvent.details))
        }
        is SdkEvent.PaymentRefundPending -> {
            pushToMap(map, "type", "paymentRefundPending")
            pushToMap(map, "details", readableMapOf(sdkEvent.details))
        }
        is SdkEvent.PaymentSucceeded -> {
            pushToMap(map, "type", "paymentSucceeded")
            pushToMap(map, "details", readableMapOf(sdkEvent.details))
        }
        is SdkEvent.PaymentWaitingConfirmation -> {
            pushToMap(map, "type", "paymentWaitingConfirmation")
            pushToMap(map, "details", readableMapOf(sdkEvent.details))
        }
        is SdkEvent.PaymentWaitingFeeAcceptance -> {
            pushToMap(map, "type", "paymentWaitingFeeAcceptance")
            pushToMap(map, "details", readableMapOf(sdkEvent.details))
        }
        is SdkEvent.Synced -> {
            pushToMap(map, "type", "synced")
        }
        is SdkEvent.DataSynced -> {
            pushToMap(map, "type", "dataSynced")
            pushToMap(map, "didPullNewRecords", sdkEvent.didPullNewRecords)
        }
    }
    return map
}

fun asSdkEventList(arr: ReadableArray): List<SdkEvent> {
    val list = ArrayList<SdkEvent>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asSdkEvent(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asSendDestination(sendDestination: ReadableMap): SendDestination? {
    val type = sendDestination.getString("type")

    if (type == "liquidAddress") {
        val addressData = sendDestination.getMap("addressData")?.let { asLiquidAddressData(it) }!!
        val bip353Address = if (hasNonNullKey(sendDestination, "bip353Address")) sendDestination.getString("bip353Address") else null
        return SendDestination.LiquidAddress(addressData, bip353Address)
    }
    if (type == "bolt11") {
        val invoice = sendDestination.getMap("invoice")?.let { asLnInvoice(it) }!!
        val bip353Address = if (hasNonNullKey(sendDestination, "bip353Address")) sendDestination.getString("bip353Address") else null
        return SendDestination.Bolt11(invoice, bip353Address)
    }
    if (type == "bolt12") {
        val offer = sendDestination.getMap("offer")?.let { asLnOffer(it) }!!
        val receiverAmountSat = sendDestination.getDouble("receiverAmountSat").toULong()
        val bip353Address = if (hasNonNullKey(sendDestination, "bip353Address")) sendDestination.getString("bip353Address") else null
        return SendDestination.Bolt12(offer, receiverAmountSat, bip353Address)
    }
    return null
}

fun readableMapOf(sendDestination: SendDestination): ReadableMap? {
    val map = Arguments.createMap()
    when (sendDestination) {
        is SendDestination.LiquidAddress -> {
            pushToMap(map, "type", "liquidAddress")
            pushToMap(map, "addressData", readableMapOf(sendDestination.addressData))
            pushToMap(map, "bip353Address", sendDestination.bip353Address)
        }
        is SendDestination.Bolt11 -> {
            pushToMap(map, "type", "bolt11")
            pushToMap(map, "invoice", readableMapOf(sendDestination.invoice))
            pushToMap(map, "bip353Address", sendDestination.bip353Address)
        }
        is SendDestination.Bolt12 -> {
            pushToMap(map, "type", "bolt12")
            pushToMap(map, "offer", readableMapOf(sendDestination.offer))
            pushToMap(map, "receiverAmountSat", sendDestination.receiverAmountSat)
            pushToMap(map, "bip353Address", sendDestination.bip353Address)
        }
    }
    return map
}

fun asSendDestinationList(arr: ReadableArray): List<SendDestination> {
    val list = ArrayList<SendDestination>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asSendDestination(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asSuccessAction(successAction: ReadableMap): SuccessAction? {
    val type = successAction.getString("type")

    if (type == "aes") {
        val data = successAction.getMap("data")?.let { asAesSuccessActionData(it) }!!
        return SuccessAction.Aes(data)
    }
    if (type == "message") {
        val data = successAction.getMap("data")?.let { asMessageSuccessActionData(it) }!!
        return SuccessAction.Message(data)
    }
    if (type == "url") {
        val data = successAction.getMap("data")?.let { asUrlSuccessActionData(it) }!!
        return SuccessAction.Url(data)
    }
    return null
}

fun readableMapOf(successAction: SuccessAction): ReadableMap? {
    val map = Arguments.createMap()
    when (successAction) {
        is SuccessAction.Aes -> {
            pushToMap(map, "type", "aes")
            pushToMap(map, "data", readableMapOf(successAction.data))
        }
        is SuccessAction.Message -> {
            pushToMap(map, "type", "message")
            pushToMap(map, "data", readableMapOf(successAction.data))
        }
        is SuccessAction.Url -> {
            pushToMap(map, "type", "url")
            pushToMap(map, "data", readableMapOf(successAction.data))
        }
    }
    return map
}

fun asSuccessActionList(arr: ReadableArray): List<SuccessAction> {
    val list = ArrayList<SuccessAction>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asSuccessAction(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asSuccessActionProcessed(successActionProcessed: ReadableMap): SuccessActionProcessed? {
    val type = successActionProcessed.getString("type")

    if (type == "aes") {
        val result = successActionProcessed.getMap("result")?.let { asAesSuccessActionDataResult(it) }!!
        return SuccessActionProcessed.Aes(result)
    }
    if (type == "message") {
        val data = successActionProcessed.getMap("data")?.let { asMessageSuccessActionData(it) }!!
        return SuccessActionProcessed.Message(data)
    }
    if (type == "url") {
        val data = successActionProcessed.getMap("data")?.let { asUrlSuccessActionData(it) }!!
        return SuccessActionProcessed.Url(data)
    }
    return null
}

fun readableMapOf(successActionProcessed: SuccessActionProcessed): ReadableMap? {
    val map = Arguments.createMap()
    when (successActionProcessed) {
        is SuccessActionProcessed.Aes -> {
            pushToMap(map, "type", "aes")
            pushToMap(map, "result", readableMapOf(successActionProcessed.result))
        }
        is SuccessActionProcessed.Message -> {
            pushToMap(map, "type", "message")
            pushToMap(map, "data", readableMapOf(successActionProcessed.data))
        }
        is SuccessActionProcessed.Url -> {
            pushToMap(map, "type", "url")
            pushToMap(map, "data", readableMapOf(successActionProcessed.data))
        }
    }
    return map
}

fun asSuccessActionProcessedList(arr: ReadableArray): List<SuccessActionProcessed> {
    val list = ArrayList<SuccessActionProcessed>()
    for (value in arr.toList()) {
        when (value) {
            is ReadableMap -> list.add(asSuccessActionProcessed(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun readableMapOf(vararg values: Pair<String, *>): ReadableMap {
    val map = Arguments.createMap()
    for ((key, value) in values) {
        pushToMap(map, key, value)
    }
    return map
}

fun hasNonNullKey(
    map: ReadableMap,
    key: String,
): Boolean = map.hasKey(key) && !map.isNull(key)

fun validateMandatoryFields(
    map: ReadableMap,
    keys: Array<String>,
): Boolean {
    for (k in keys) {
        if (!hasNonNullKey(map, k)) return false
    }

    return true
}

fun pushToArray(
    array: WritableArray,
    value: Any?,
) {
    when (value) {
        null -> array.pushNull()
        is AssetBalance -> array.pushMap(readableMapOf(value))
        is AssetMetadata -> array.pushMap(readableMapOf(value))
        is ExternalInputParser -> array.pushMap(readableMapOf(value))
        is FiatCurrency -> array.pushMap(readableMapOf(value))
        is LnOfferBlindedPath -> array.pushMap(readableMapOf(value))
        is LocaleOverrides -> array.pushMap(readableMapOf(value))
        is LocalizedName -> array.pushMap(readableMapOf(value))
        is Payment -> array.pushMap(readableMapOf(value))
        is PaymentState -> array.pushString(snakeToLowerCamelCase(value.name))
        is PaymentType -> array.pushString(snakeToLowerCamelCase(value.name))
        is Rate -> array.pushMap(readableMapOf(value))
        is RefundableSwap -> array.pushMap(readableMapOf(value))
        is RouteHint -> array.pushMap(readableMapOf(value))
        is RouteHintHop -> array.pushMap(readableMapOf(value))
        is String -> array.pushString(value)
        is UByte -> array.pushInt(value.toInt())
        is Array<*> -> array.pushArray(readableArrayOf(value.asIterable()))
        is List<*> -> array.pushArray(readableArrayOf(value))
        else -> throw SdkException.Generic(errUnexpectedType(value))
    }
}

fun pushToMap(
    map: WritableMap,
    key: String,
    value: Any?,
) {
    when (value) {
        null -> map.putNull(key)
        is Boolean -> map.putBoolean(key, value)
        is Byte -> map.putInt(key, value.toInt())
        is Double -> map.putDouble(key, value)
        is Float -> map.putDouble(key, value.toDouble())
        is Int -> map.putInt(key, value)
        is Long -> map.putDouble(key, value.toDouble())
        is ReadableArray -> map.putArray(key, value)
        is ReadableMap -> map.putMap(key, value)
        is String -> map.putString(key, value)
        is UByte -> map.putInt(key, value.toInt())
        is UInt -> map.putInt(key, value.toInt())
        is UShort -> map.putInt(key, value.toInt())
        is ULong -> map.putDouble(key, value.toDouble())
        is Array<*> -> map.putArray(key, readableArrayOf(value.asIterable()))
        is List<*> -> map.putArray(key, readableArrayOf(value))
        else -> throw SdkException.Generic("Unexpected type ${value::class.java.name} for key [$key]")
    }
}

fun readableArrayOf(values: Iterable<*>?): ReadableArray {
    val array = Arguments.createArray()
    if (values != null) {
        for (value in values) {
            pushToArray(array, value)
        }
    }

    return array
}

fun asUByteList(arr: ReadableArray): List<UByte> {
    val list = ArrayList<UByte>()
    for (value in arr.toArrayList()) {
        when (value) {
            is Double -> list.add(value.toInt().toUByte())
            is Int -> list.add(value.toUByte())
            is UByte -> list.add(value)
            else -> throw SdkException.Generic(errUnexpectedType(value))
        }
    }
    return list
}

fun asStringList(arr: ReadableArray): List<String> {
    val list = ArrayList<String>()
    for (value in arr.toArrayList()) {
        list.add(value.toString())
    }
    return list
}

fun errMissingMandatoryField(
    fieldName: String,
    typeName: String,
): String = "Missing mandatory field $fieldName for type $typeName"

fun errUnexpectedType(type: Any?): String {
    val typeName = if (type != null) type::class.java.name else "null"
    return "Unexpected type $typeName"
}

fun errUnexpectedValue(fieldName: String): String = "Unexpected value for optional field $fieldName"

fun camelToUpperSnakeCase(str: String): String =
    "(?<=.)[A-Z]"
        .toRegex()
        .replace(str) {
            "_${it.value}"
        }.uppercase()

fun snakeToLowerCamelCase(str: String): String =
    "_[a-zA-Z]".toRegex().replace(str.lowercase()) {
        it.value.replace("_", "").uppercase()
    }

internal fun ReadableArray.toList(): List<*> {
    val arrayList = mutableListOf<Any?>()
    for (i in 0 until size()) {
        when (getType(i)) {
            ReadableType.Null -> arrayList.add(null)
            ReadableType.Boolean -> arrayList.add(getBoolean(i))
            ReadableType.Number -> arrayList.add(getDouble(i))
            ReadableType.String -> arrayList.add(getString(i))
            ReadableType.Map -> arrayList.add(getMap(i))
            ReadableType.Array -> arrayList.add(getArray(i))
            else -> throw SdkException.Generic("Could not convert object at index: $i")
        }
    }
    return arrayList
}
