package breez_sdk_liquid_notification.job

import android.content.Context
import android.os.Handler
import android.os.Looper
import breez_sdk_liquid.BindingLiquidSdk
import breez_sdk_liquid.InputType
import breez_sdk_liquid.PaymentMethod
import breez_sdk_liquid.PrepareReceiveRequest
import breez_sdk_liquid.ReceiveAmount
import breez_sdk_liquid.DescriptionHash
import breez_sdk_liquid.ReceivePaymentRequest
import breez_sdk_liquid.NwcEvent
import breez_sdk_liquid.NwcEventListener
import breez_sdk_liquid.NwcEventDetails
import breez_sdk_liquid_notification.Constants.DEFAULT_LNURL_PAY_INVOICE_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.DEFAULT_LNURL_PAY_METADATA_PLAIN_TEXT
import breez_sdk_liquid_notification.Constants.DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE
import breez_sdk_liquid_notification.Constants.LNURL_PAY_COMMENT_MAX_LENGTH
import breez_sdk_liquid_notification.Constants.LNURL_PAY_INVOICE_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.LNURL_PAY_METADATA_PLAIN_TEXT
import breez_sdk_liquid_notification.Constants.LNURL_PAY_NOTIFICATION_FAILURE_TITLE
import breez_sdk_liquid_notification.Constants.NOTIFICATION_CHANNEL_REPLACEABLE
import breez_sdk_liquid_notification.NotificationHelper.Companion.notifyChannel
import breez_sdk_liquid_notification.ResourceHelper.Companion.getString
import breez_sdk_liquid_notification.SdkForegroundService
import breez_sdk_liquid_notification.ServiceLogger
import breez_sdk_liquid_notification.PluginConfigs
import breez_sdk_liquid_notification.PluginManager
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

@Serializable
data class LnurlInvoiceRequest(
    @SerialName("amount") val amount: ULong,
    @SerialName("comment") val comment: String? = null,
    @SerialName("reply_url") val replyURL: String,
    @SerialName("verify_url") val verifyURL: String? = null,
    @SerialName("nostr") val nostr: String? = null,
)

// Serialize the response according to:
// - LUD-06: https://github.com/lnurl/luds/blob/luds/06.md
// - LUD-21: https://github.com/lnurl/luds/blob/luds/21.md
@Serializable
data class LnurlPayInvoiceResponse(
    val pr: String,
    val routes: List<String>,
    val verify: String?,
)

class LnurlPayInvoiceJob(
    private val context: Context,
    private val fgService: SdkForegroundService,
    private val payload: String,
    private val logger: ServiceLogger,
) : LnurlPayJob, NwcEventListener {
    companion object {
        private const val TAG = "LnurlPayInvoiceJob"
        private const val ZAP_TRACKING_TIMEOUT_MS = 120_000L
    }
    private val handler = Handler(Looper.getMainLooper())
    private var zapTrackingInvoice: String? = null
    private val zapReceiptTimeout = Runnable {
        logger.log(TAG, "Zap tracking timeout reached for invoice: $zapTrackingInvoice", "WARN")
        zapTrackingInvoice = null
        fgService.onFinished(this)
    }

    override fun start(liquidSDK: BindingLiquidSdk, pluginConfigs: PluginConfigs) {
        var request: LnurlInvoiceRequest? = null
        try {
            val decoder = Json { ignoreUnknownKeys = true }
            request = decoder.decodeFromString(LnurlInvoiceRequest.serializer(), payload)
            // Get the lightning limits
            val limits = liquidSDK.fetchLightningLimits()
            // Check amount is within limits
            val amountSat = request.amount / 1000UL
            if (amountSat < limits.receive.minSat || amountSat > limits.receive.maxSat) {
                throw InvalidLnurlPayException("Invalid amount requested ${request.amount}")
            }
            // Check comment length
            if ((request.comment?.length ?: 0) > LNURL_PAY_COMMENT_MAX_LENGTH) {
                throw InvalidLnurlPayException("Comment is too long")
            }
            val plainTextMetadata =
                getString(
                    context,
                    LNURL_PAY_METADATA_PLAIN_TEXT,
                    DEFAULT_LNURL_PAY_METADATA_PLAIN_TEXT,
                )
            val prepareReceivePaymentRes =
                liquidSDK.prepareReceivePayment(
                    PrepareReceiveRequest(PaymentMethod.BOLT11_INVOICE, ReceiveAmount.Bitcoin(amountSat)),
                )
            val receivePaymentResponse =
                liquidSDK.receivePayment(
                    ReceivePaymentRequest(
                        prepareReceivePaymentRes,
                        description = "[[\"text/plain\",\"$plainTextMetadata\"]]",
                        descriptionHash = DescriptionHash.UseDescription,
                        payerNote = request.comment,
                    ),
                )
            // Add the verify URL
            var verify: String? = null
            if (request.verifyURL != null) {
                try {
                    val inputType = liquidSDK.parse(receivePaymentResponse.destination)
                    if (inputType is InputType.Bolt11) {
                        verify = request.verifyURL?.replace("{payment_hash}", inputType.invoice.paymentHash)
                    }
                } catch (e: Exception) {
                    logger.log(TAG, "Failed to parse destination: ${e.message}", "WARN")
                }
            }
            val response =
                LnurlPayInvoiceResponse(
                    receivePaymentResponse.destination,
                    listOf(),
                    verify,
                )
            val success = replyServer(Json.encodeToString(response), request.replyURL)
            notifyChannel(
                context,
                NOTIFICATION_CHANNEL_REPLACEABLE,
                getString(
                    context,
                    if (success) LNURL_PAY_INVOICE_NOTIFICATION_TITLE else LNURL_PAY_NOTIFICATION_FAILURE_TITLE,
                    if (success) DEFAULT_LNURL_PAY_INVOICE_NOTIFICATION_TITLE else DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE,
                ),
            )
            val nwcService = PluginManager.nwc(liquidSDK, pluginConfigs, logger)
            if (request.nostr != null && nwcService != null) {
                try {
                    nwcService.addEventListener(this)
                    zapTrackingInvoice = receivePaymentResponse.destination
                    logger.log(TAG, "Tracking zap for invoice: ${zapTrackingInvoice}", "INFO")
                    nwcService.trackZap(zapTrackingInvoice as String, request.nostr!!)
                    handler.postDelayed(zapReceiptTimeout, ZAP_TRACKING_TIMEOUT_MS)
                } catch (e: Exception) {
                    logger.log(TAG, "Failed to track zap: ${e.message}", "WARN")
                    fgService.onFinished(this)
                }
            } else {
                fgService.onFinished(this)
            }
        } catch (e: Exception) {
            logger.log(TAG, "Failed to process lnurl: ${e.message}", "WARN")
            if (request != null) {
                fail(e.message, request.replyURL, logger)
            }
            notifyChannel(
                context,
                NOTIFICATION_CHANNEL_REPLACEABLE,
                getString(
                    context,
                    LNURL_PAY_NOTIFICATION_FAILURE_TITLE,
                    DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE,
                ),
            )
        }
    }

    override fun onShutdown() {
        handler.removeCallbacks(zapReceiptTimeout)
    }

    override fun onEvent(event: NwcEvent) {
        if (zapTrackingInvoice == null) return
        when(event.details) {
            is NwcEventDetails.ZapReceived -> {
                if (zapTrackingInvoice != (event.details as NwcEventDetails.ZapReceived).invoice) {
                    return
                }
                handler.removeCallbacks(zapReceiptTimeout)
                logger.log(TAG, "Successfully received zap for invoice: ${zapTrackingInvoice}", "INFO")
                fgService.onFinished(this)
            }
            else -> return
        }
    }
}
