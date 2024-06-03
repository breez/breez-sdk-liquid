package com.breezliquidsdk
import breez_liquid_sdk.*
import com.facebook.react.bridge.*
import java.util.*

fun asBackupRequest(backupRequest: ReadableMap): BackupRequest? {
    if (!validateMandatoryFields(
            backupRequest,
            arrayOf(),
        )
    ) {
        return null
    }
    val backupPath = if (hasNonNullKey(backupRequest, "backupPath")) backupRequest.getString("backupPath") else null
    return BackupRequest(
        backupPath,
    )
}

fun readableMapOf(backupRequest: BackupRequest): ReadableMap =
    readableMapOf(
        "backupPath" to backupRequest.backupPath,
    )

fun asBackupRequestList(arr: ReadableArray): List<BackupRequest> {
    val list = ArrayList<BackupRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asBackupRequest(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asConfig(config: ReadableMap): Config? {
    if (!validateMandatoryFields(
            config,
            arrayOf(
                "boltzUrl",
                "electrumUrl",
                "workingDir",
                "network",
                "paymentTimeoutSec",
                "zeroConfMinFeeRate",
            ),
        )
    ) {
        return null
    }
    val boltzUrl = config.getString("boltzUrl")!!
    val electrumUrl = config.getString("electrumUrl")!!
    val workingDir = config.getString("workingDir")!!
    val network = config.getString("network")?.let { asNetwork(it) }!!
    val paymentTimeoutSec = config.getDouble("paymentTimeoutSec").toULong()
    val zeroConfMinFeeRate = config.getDouble("zeroConfMinFeeRate")
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
    return Config(
        boltzUrl,
        electrumUrl,
        workingDir,
        network,
        paymentTimeoutSec,
        zeroConfMinFeeRate,
        zeroConfMaxAmountSat,
    )
}

fun readableMapOf(config: Config): ReadableMap =
    readableMapOf(
        "boltzUrl" to config.boltzUrl,
        "electrumUrl" to config.electrumUrl,
        "workingDir" to config.workingDir,
        "network" to config.network.name.lowercase(),
        "paymentTimeoutSec" to config.paymentTimeoutSec,
        "zeroConfMinFeeRate" to config.zeroConfMinFeeRate,
        "zeroConfMaxAmountSat" to config.zeroConfMaxAmountSat,
    )

fun asConfigList(arr: ReadableArray): List<Config> {
    val list = ArrayList<Config>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asConfig(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asConnectRequest(connectRequest: ReadableMap): ConnectRequest? {
    if (!validateMandatoryFields(
            connectRequest,
            arrayOf(
                "config",
                "mnemonic",
            ),
        )
    ) {
        return null
    }
    val config = connectRequest.getMap("config")?.let { asConfig(it) }!!
    val mnemonic = connectRequest.getString("mnemonic")!!
    return ConnectRequest(
        config,
        mnemonic,
    )
}

fun readableMapOf(connectRequest: ConnectRequest): ReadableMap =
    readableMapOf(
        "config" to readableMapOf(connectRequest.config),
        "mnemonic" to connectRequest.mnemonic,
    )

fun asConnectRequestList(arr: ReadableArray): List<ConnectRequest> {
    val list = ArrayList<ConnectRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asConnectRequest(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asGetInfoResponse(getInfoResponse: ReadableMap): GetInfoResponse? {
    if (!validateMandatoryFields(
            getInfoResponse,
            arrayOf(
                "balanceSat",
                "pendingSendSat",
                "pendingReceiveSat",
                "pubkey",
            ),
        )
    ) {
        return null
    }
    val balanceSat = getInfoResponse.getDouble("balanceSat").toULong()
    val pendingSendSat = getInfoResponse.getDouble("pendingSendSat").toULong()
    val pendingReceiveSat = getInfoResponse.getDouble("pendingReceiveSat").toULong()
    val pubkey = getInfoResponse.getString("pubkey")!!
    return GetInfoResponse(
        balanceSat,
        pendingSendSat,
        pendingReceiveSat,
        pubkey,
    )
}

fun readableMapOf(getInfoResponse: GetInfoResponse): ReadableMap =
    readableMapOf(
        "balanceSat" to getInfoResponse.balanceSat,
        "pendingSendSat" to getInfoResponse.pendingSendSat,
        "pendingReceiveSat" to getInfoResponse.pendingReceiveSat,
        "pubkey" to getInfoResponse.pubkey,
    )

fun asGetInfoResponseList(arr: ReadableArray): List<GetInfoResponse> {
    val list = ArrayList<GetInfoResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asGetInfoResponse(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
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
        "network" to lnInvoice.network.name.lowercase(),
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
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLnInvoice(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
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
    return LogEntry(
        line,
        level,
    )
}

fun readableMapOf(logEntry: LogEntry): ReadableMap =
    readableMapOf(
        "line" to logEntry.line,
        "level" to logEntry.level,
    )

fun asLogEntryList(arr: ReadableArray): List<LogEntry> {
    val list = ArrayList<LogEntry>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLogEntry(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
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
            ),
        )
    ) {
        return null
    }
    val txId = if (hasNonNullKey(payment, "txId")) payment.getString("txId") else null
    val swapId = if (hasNonNullKey(payment, "swapId")) payment.getString("swapId") else null
    val timestamp = payment.getInt("timestamp").toUInt()
    val amountSat = payment.getDouble("amountSat").toULong()
    val feesSat = payment.getDouble("feesSat").toULong()
    val preimage = if (hasNonNullKey(payment, "preimage")) payment.getString("preimage") else null
    val refundTxId = if (hasNonNullKey(payment, "refundTxId")) payment.getString("refundTxId") else null
    val refundTxAmountSat = if (hasNonNullKey(payment, "refundTxAmountSat")) payment.getDouble("refundTxAmountSat").toULong() else null
    val paymentType = payment.getString("paymentType")?.let { asPaymentType(it) }!!
    val status = payment.getString("status")?.let { asPaymentState(it) }!!
    return Payment(
        txId,
        swapId,
        timestamp,
        amountSat,
        feesSat,
        preimage,
        refundTxId,
        refundTxAmountSat,
        paymentType,
        status,
    )
}

fun readableMapOf(payment: Payment): ReadableMap =
    readableMapOf(
        "txId" to payment.txId,
        "swapId" to payment.swapId,
        "timestamp" to payment.timestamp,
        "amountSat" to payment.amountSat,
        "feesSat" to payment.feesSat,
        "preimage" to payment.preimage,
        "refundTxId" to payment.refundTxId,
        "refundTxAmountSat" to payment.refundTxAmountSat,
        "paymentType" to payment.paymentType.name.lowercase(),
        "status" to payment.status.name.lowercase(),
    )

fun asPaymentList(arr: ReadableArray): List<Payment> {
    val list = ArrayList<Payment>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPayment(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPrepareReceiveRequest(prepareReceiveRequest: ReadableMap): PrepareReceiveRequest? {
    if (!validateMandatoryFields(
            prepareReceiveRequest,
            arrayOf(
                "payerAmountSat",
            ),
        )
    ) {
        return null
    }
    val payerAmountSat = prepareReceiveRequest.getDouble("payerAmountSat").toULong()
    return PrepareReceiveRequest(
        payerAmountSat,
    )
}

fun readableMapOf(prepareReceiveRequest: PrepareReceiveRequest): ReadableMap =
    readableMapOf(
        "payerAmountSat" to prepareReceiveRequest.payerAmountSat,
    )

fun asPrepareReceiveRequestList(arr: ReadableArray): List<PrepareReceiveRequest> {
    val list = ArrayList<PrepareReceiveRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareReceiveRequest(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPrepareReceiveResponse(prepareReceiveResponse: ReadableMap): PrepareReceiveResponse? {
    if (!validateMandatoryFields(
            prepareReceiveResponse,
            arrayOf(
                "payerAmountSat",
                "feesSat",
            ),
        )
    ) {
        return null
    }
    val payerAmountSat = prepareReceiveResponse.getDouble("payerAmountSat").toULong()
    val feesSat = prepareReceiveResponse.getDouble("feesSat").toULong()
    return PrepareReceiveResponse(
        payerAmountSat,
        feesSat,
    )
}

fun readableMapOf(prepareReceiveResponse: PrepareReceiveResponse): ReadableMap =
    readableMapOf(
        "payerAmountSat" to prepareReceiveResponse.payerAmountSat,
        "feesSat" to prepareReceiveResponse.feesSat,
    )

fun asPrepareReceiveResponseList(arr: ReadableArray): List<PrepareReceiveResponse> {
    val list = ArrayList<PrepareReceiveResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareReceiveResponse(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPrepareSendRequest(prepareSendRequest: ReadableMap): PrepareSendRequest? {
    if (!validateMandatoryFields(
            prepareSendRequest,
            arrayOf(
                "invoice",
            ),
        )
    ) {
        return null
    }
    val invoice = prepareSendRequest.getString("invoice")!!
    return PrepareSendRequest(
        invoice,
    )
}

fun readableMapOf(prepareSendRequest: PrepareSendRequest): ReadableMap =
    readableMapOf(
        "invoice" to prepareSendRequest.invoice,
    )

fun asPrepareSendRequestList(arr: ReadableArray): List<PrepareSendRequest> {
    val list = ArrayList<PrepareSendRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareSendRequest(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPrepareSendResponse(prepareSendResponse: ReadableMap): PrepareSendResponse? {
    if (!validateMandatoryFields(
            prepareSendResponse,
            arrayOf(
                "invoice",
                "feesSat",
            ),
        )
    ) {
        return null
    }
    val invoice = prepareSendResponse.getString("invoice")!!
    val feesSat = prepareSendResponse.getDouble("feesSat").toULong()
    return PrepareSendResponse(
        invoice,
        feesSat,
    )
}

fun readableMapOf(prepareSendResponse: PrepareSendResponse): ReadableMap =
    readableMapOf(
        "invoice" to prepareSendResponse.invoice,
        "feesSat" to prepareSendResponse.feesSat,
    )

fun asPrepareSendResponseList(arr: ReadableArray): List<PrepareSendResponse> {
    val list = ArrayList<PrepareSendResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareSendResponse(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asReceivePaymentResponse(receivePaymentResponse: ReadableMap): ReceivePaymentResponse? {
    if (!validateMandatoryFields(
            receivePaymentResponse,
            arrayOf(
                "id",
                "invoice",
            ),
        )
    ) {
        return null
    }
    val id = receivePaymentResponse.getString("id")!!
    val invoice = receivePaymentResponse.getString("invoice")!!
    return ReceivePaymentResponse(
        id,
        invoice,
    )
}

fun readableMapOf(receivePaymentResponse: ReceivePaymentResponse): ReadableMap =
    readableMapOf(
        "id" to receivePaymentResponse.id,
        "invoice" to receivePaymentResponse.invoice,
    )

fun asReceivePaymentResponseList(arr: ReadableArray): List<ReceivePaymentResponse> {
    val list = ArrayList<ReceivePaymentResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asReceivePaymentResponse(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
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
    return RestoreRequest(
        backupPath,
    )
}

fun readableMapOf(restoreRequest: RestoreRequest): ReadableMap =
    readableMapOf(
        "backupPath" to restoreRequest.backupPath,
    )

fun asRestoreRequestList(arr: ReadableArray): List<RestoreRequest> {
    val list = ArrayList<RestoreRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asRestoreRequest(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
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
    return RouteHint(
        hops,
    )
}

fun readableMapOf(routeHint: RouteHint): ReadableMap =
    readableMapOf(
        "hops" to readableArrayOf(routeHint.hops),
    )

fun asRouteHintList(arr: ReadableArray): List<RouteHint> {
    val list = ArrayList<RouteHint>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asRouteHint(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
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
    val shortChannelId = routeHintHop.getDouble("shortChannelId").toULong()
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
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asRouteHintHop(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
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
    return SendPaymentResponse(
        payment,
    )
}

fun readableMapOf(sendPaymentResponse: SendPaymentResponse): ReadableMap =
    readableMapOf(
        "payment" to readableMapOf(sendPaymentResponse.payment),
    )

fun asSendPaymentResponseList(arr: ReadableArray): List<SendPaymentResponse> {
    val list = ArrayList<SendPaymentResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asSendPaymentResponse(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asLiquidSdkEvent(liquidSdkEvent: ReadableMap): LiquidSdkEvent? {
    val type = liquidSdkEvent.getString("type")

    if (type == "paymentFailed") {
        return LiquidSdkEvent.PaymentFailed(liquidSdkEvent.getMap("details")?.let { asPayment(it) }!!)
    }
    if (type == "paymentPending") {
        return LiquidSdkEvent.PaymentPending(liquidSdkEvent.getMap("details")?.let { asPayment(it) }!!)
    }
    if (type == "paymentRefunded") {
        return LiquidSdkEvent.PaymentRefunded(liquidSdkEvent.getMap("details")?.let { asPayment(it) }!!)
    }
    if (type == "paymentRefundPending") {
        return LiquidSdkEvent.PaymentRefundPending(liquidSdkEvent.getMap("details")?.let { asPayment(it) }!!)
    }
    if (type == "paymentSucceeded") {
        return LiquidSdkEvent.PaymentSucceeded(liquidSdkEvent.getMap("details")?.let { asPayment(it) }!!)
    }
    if (type == "paymentWaitingConfirmation") {
        return LiquidSdkEvent.PaymentWaitingConfirmation(liquidSdkEvent.getMap("details")?.let { asPayment(it) }!!)
    }
    if (type == "synced") {
        return LiquidSdkEvent.Synced
    }
    return null
}

fun readableMapOf(liquidSdkEvent: LiquidSdkEvent): ReadableMap? {
    val map = Arguments.createMap()
    when (liquidSdkEvent) {
        is LiquidSdkEvent.PaymentFailed -> {
            pushToMap(map, "type", "paymentFailed")
            pushToMap(map, "details", readableMapOf(liquidSdkEvent.details))
        }
        is LiquidSdkEvent.PaymentPending -> {
            pushToMap(map, "type", "paymentPending")
            pushToMap(map, "details", readableMapOf(liquidSdkEvent.details))
        }
        is LiquidSdkEvent.PaymentRefunded -> {
            pushToMap(map, "type", "paymentRefunded")
            pushToMap(map, "details", readableMapOf(liquidSdkEvent.details))
        }
        is LiquidSdkEvent.PaymentRefundPending -> {
            pushToMap(map, "type", "paymentRefundPending")
            pushToMap(map, "details", readableMapOf(liquidSdkEvent.details))
        }
        is LiquidSdkEvent.PaymentSucceeded -> {
            pushToMap(map, "type", "paymentSucceeded")
            pushToMap(map, "details", readableMapOf(liquidSdkEvent.details))
        }
        is LiquidSdkEvent.PaymentWaitingConfirmation -> {
            pushToMap(map, "type", "paymentWaitingConfirmation")
            pushToMap(map, "details", readableMapOf(liquidSdkEvent.details))
        }
        is LiquidSdkEvent.Synced -> {
            pushToMap(map, "type", "synced")
        }
    }
    return map
}

fun asLiquidSdkEventList(arr: ReadableArray): List<LiquidSdkEvent> {
    val list = ArrayList<LiquidSdkEvent>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLiquidSdkEvent(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asNetwork(type: String): Network = Network.valueOf(camelToUpperSnakeCase(type))

fun asNetworkList(arr: ReadableArray): List<Network> {
    val list = ArrayList<Network>()
    for (value in arr.toArrayList()) {
        when (value) {
            is String -> list.add(asNetwork(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPaymentState(type: String): PaymentState = PaymentState.valueOf(camelToUpperSnakeCase(type))

fun asPaymentStateList(arr: ReadableArray): List<PaymentState> {
    val list = ArrayList<PaymentState>()
    for (value in arr.toArrayList()) {
        when (value) {
            is String -> list.add(asPaymentState(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPaymentType(type: String): PaymentType = PaymentType.valueOf(camelToUpperSnakeCase(type))

fun asPaymentTypeList(arr: ReadableArray): List<PaymentType> {
    val list = ArrayList<PaymentType>()
    for (value in arr.toArrayList()) {
        when (value) {
            is String -> list.add(asPaymentType(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
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
        is Payment -> array.pushMap(readableMapOf(value))
        is RouteHint -> array.pushMap(readableMapOf(value))
        is RouteHintHop -> array.pushMap(readableMapOf(value))
        is UByte -> array.pushInt(value.toInt())
        is Array<*> -> array.pushArray(readableArrayOf(value.asIterable()))
        is List<*> -> array.pushArray(readableArrayOf(value))
        else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
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
        else -> throw LiquidSdkException.Generic("Unexpected type ${value::class.java.name} for key [$key]")
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
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
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

fun errUnexpectedType(typeName: String): String = "Unexpected type $typeName"

fun errUnexpectedValue(fieldName: String): String = "Unexpected value for optional field $fieldName"

fun camelToUpperSnakeCase(str: String): String {
    val pattern = "(?<=.)[A-Z]".toRegex()
    return str.replace(pattern, "_$0").uppercase()
}
