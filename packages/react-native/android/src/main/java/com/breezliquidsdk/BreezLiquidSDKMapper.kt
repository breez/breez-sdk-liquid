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

fun readableMapOf(backupRequest: BackupRequest): ReadableMap {
    return readableMapOf(
        "backupPath" to backupRequest.backupPath,
    )
}

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

fun asConnectRequest(connectRequest: ReadableMap): ConnectRequest? {
    if (!validateMandatoryFields(
            connectRequest,
            arrayOf(
                "mnemonic",
                "network",
            ),
        )
    ) {
        return null
    }
    val mnemonic = connectRequest.getString("mnemonic")!!
    val network = connectRequest.getString("network")?.let { asNetwork(it) }!!
    val dataDir = if (hasNonNullKey(connectRequest, "dataDir")) connectRequest.getString("dataDir") else null
    return ConnectRequest(
        mnemonic,
        network,
        dataDir,
    )
}

fun readableMapOf(connectRequest: ConnectRequest): ReadableMap {
    return readableMapOf(
        "mnemonic" to connectRequest.mnemonic,
        "network" to connectRequest.network.name.lowercase(),
        "dataDir" to connectRequest.dataDir,
    )
}

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

fun asGetInfoRequest(getInfoRequest: ReadableMap): GetInfoRequest? {
    if (!validateMandatoryFields(
            getInfoRequest,
            arrayOf(
                "withScan",
            ),
        )
    ) {
        return null
    }
    val withScan = getInfoRequest.getBoolean("withScan")
    return GetInfoRequest(
        withScan,
    )
}

fun readableMapOf(getInfoRequest: GetInfoRequest): ReadableMap {
    return readableMapOf(
        "withScan" to getInfoRequest.withScan,
    )
}

fun asGetInfoRequestList(arr: ReadableArray): List<GetInfoRequest> {
    val list = ArrayList<GetInfoRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asGetInfoRequest(value)!!)
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

fun readableMapOf(getInfoResponse: GetInfoResponse): ReadableMap {
    return readableMapOf(
        "balanceSat" to getInfoResponse.balanceSat,
        "pendingSendSat" to getInfoResponse.pendingSendSat,
        "pendingReceiveSat" to getInfoResponse.pendingReceiveSat,
        "pubkey" to getInfoResponse.pubkey,
    )
}

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
    val amountMsat = if (hasNonNullKey(lnInvoice, "amountMsat")) lnInvoice.getDouble("amountMsat").toULong() else null
    val timestamp = lnInvoice.getDouble("timestamp").toULong()
    val expiry = lnInvoice.getDouble("expiry").toULong()
    return LnInvoice(
        bolt11,
        network,
        payeePubkey,
        paymentHash,
        description,
        amountMsat,
        timestamp,
        expiry,
    )
}

fun readableMapOf(lnInvoice: LnInvoice): ReadableMap {
    return readableMapOf(
        "bolt11" to lnInvoice.bolt11,
        "network" to lnInvoice.network.name.lowercase(),
        "payeePubkey" to lnInvoice.payeePubkey,
        "paymentHash" to lnInvoice.paymentHash,
        "description" to lnInvoice.description,
        "amountMsat" to lnInvoice.amountMsat,
        "timestamp" to lnInvoice.timestamp,
        "expiry" to lnInvoice.expiry,
    )
}

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

fun asPayment(payment: ReadableMap): Payment? {
    if (!validateMandatoryFields(
            payment,
            arrayOf(
                "txId",
                "timestamp",
                "amountSat",
                "paymentType",
                "status",
            ),
        )
    ) {
        return null
    }
    val txId = payment.getString("txId")!!
    val swapId = if (hasNonNullKey(payment, "swapId")) payment.getString("swapId") else null
    val timestamp = payment.getInt("timestamp").toUInt()
    val amountSat = payment.getDouble("amountSat").toULong()
    val feesSat = if (hasNonNullKey(payment, "feesSat")) payment.getDouble("feesSat").toULong() else null
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

fun readableMapOf(payment: Payment): ReadableMap {
    return readableMapOf(
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
}

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

fun readableMapOf(prepareReceiveRequest: PrepareReceiveRequest): ReadableMap {
    return readableMapOf(
        "payerAmountSat" to prepareReceiveRequest.payerAmountSat,
    )
}

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

fun readableMapOf(prepareReceiveResponse: PrepareReceiveResponse): ReadableMap {
    return readableMapOf(
        "payerAmountSat" to prepareReceiveResponse.payerAmountSat,
        "feesSat" to prepareReceiveResponse.feesSat,
    )
}

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

fun readableMapOf(prepareSendRequest: PrepareSendRequest): ReadableMap {
    return readableMapOf(
        "invoice" to prepareSendRequest.invoice,
    )
}

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

fun readableMapOf(prepareSendResponse: PrepareSendResponse): ReadableMap {
    return readableMapOf(
        "invoice" to prepareSendResponse.invoice,
        "feesSat" to prepareSendResponse.feesSat,
    )
}

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

fun readableMapOf(receivePaymentResponse: ReceivePaymentResponse): ReadableMap {
    return readableMapOf(
        "id" to receivePaymentResponse.id,
        "invoice" to receivePaymentResponse.invoice,
    )
}

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

fun readableMapOf(restoreRequest: RestoreRequest): ReadableMap {
    return readableMapOf(
        "backupPath" to restoreRequest.backupPath,
    )
}

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

fun asSendPaymentResponse(sendPaymentResponse: ReadableMap): SendPaymentResponse? {
    if (!validateMandatoryFields(
            sendPaymentResponse,
            arrayOf(
                "txid",
            ),
        )
    ) {
        return null
    }
    val txid = sendPaymentResponse.getString("txid")!!
    return SendPaymentResponse(
        txid,
    )
}

fun readableMapOf(sendPaymentResponse: SendPaymentResponse): ReadableMap {
    return readableMapOf(
        "txid" to sendPaymentResponse.txid,
    )
}

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
    if (type == "paymentSucceed") {
        return LiquidSdkEvent.PaymentSucceed(liquidSdkEvent.getMap("details")?.let { asPayment(it) }!!)
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
        is LiquidSdkEvent.PaymentSucceed -> {
            pushToMap(map, "type", "paymentSucceed")
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

fun asNetwork(type: String): Network {
    return Network.valueOf(camelToUpperSnakeCase(type))
}

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

fun asPaymentState(type: String): PaymentState {
    return PaymentState.valueOf(camelToUpperSnakeCase(type))
}

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

fun asPaymentType(type: String): PaymentType {
    return PaymentType.valueOf(camelToUpperSnakeCase(type))
}

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
): Boolean {
    return map.hasKey(key) && !map.isNull(key)
}

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
): String {
    return "Missing mandatory field $fieldName for type $typeName"
}

fun errUnexpectedType(typeName: String): String {
    return "Unexpected type $typeName"
}

fun errUnexpectedValue(fieldName: String): String {
    return "Unexpected value for optional field $fieldName"
}

fun camelToUpperSnakeCase(str: String): String {
    val pattern = "(?<=.)[A-Z]".toRegex()
    return str.replace(pattern, "_$0").uppercase()
}
