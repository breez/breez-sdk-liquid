package breez_sdk_liquid_notification.job

import android.content.Context
import breez_sdk_liquid.SdkEvent
import breez_sdk_liquid.BindingLiquidSdk
import breez_sdk_liquid.PaymentMethod
import breez_sdk_liquid.PrepareReceiveRequest
import breez_sdk_liquid.ReceivePaymentRequest
import breez_sdk_liquid_notification.Constants.DEFAULT_INVOICE_REQUEST_NOTIFICATION_FAILURE_TITLE
import breez_sdk_liquid_notification.Constants.DEFAULT_INVOICE_REQUEST_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.INVOICE_REQUEST_NOTIFICATION_FAILURE_TITLE
import breez_sdk_liquid_notification.Constants.INVOICE_REQUEST_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.NOTIFICATION_CHANNEL_REPLACEABLE
import breez_sdk_liquid_notification.NotificationHelper.Companion.notifyChannel
import breez_sdk_liquid_notification.ResourceHelper.Companion.getString
import breez_sdk_liquid_notification.SdkForegroundService
import breez_sdk_liquid_notification.ServiceLogger
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

@Serializable
data class InvoiceRequestRequest(
    @SerialName("offer") val offer: String,
    @SerialName("invoice_request") val invoiceRequest: String,
    @SerialName("reply_url") val replyURL: String,
)

@Serializable
data class InvoiceRequestResponse(
    val invoice: String,
)

class InvoiceRequestJob(
    private val context: Context,
    private val fgService: SdkForegroundService,
    private val payload: String,
    private val logger: ServiceLogger,
) : Job {
    companion object {
        private const val TAG = "InvoiceRequestJob"
    }

    override fun start(liquidSDK: BindingLiquidSdk) {
        var request: InvoiceRequestRequest? = null
        try {
            request = Json.decodeFromString(InvoiceRequestRequest.serializer(), payload)
            val prepareReceivePaymentRes =
                liquidSDK.prepareReceivePayment(
                    PrepareReceiveRequest(
                        PaymentMethod.BOLT12_INVOICE, 
                        offer = request.offer, 
                        invoiceRequest = request.invoiceRequest
                    ),
                )
            val receivePaymentResponse =
                liquidSDK.receivePayment(
                    ReceivePaymentRequest(prepareReceivePaymentRes),
                )
            val response = InvoiceRequestResponse(receivePaymentResponse.destination)
            val success = replyServer(Json.encodeToString(response), request.replyURL)
            notifyChannel(
                context,
                NOTIFICATION_CHANNEL_REPLACEABLE,
                getString(
                    context,
                    if (success) INVOICE_REQUEST_NOTIFICATION_TITLE else INVOICE_REQUEST_NOTIFICATION_FAILURE_TITLE,
                    if (success) DEFAULT_INVOICE_REQUEST_NOTIFICATION_TITLE else DEFAULT_INVOICE_REQUEST_NOTIFICATION_FAILURE_TITLE,
                ),
            )
        } catch (e: Exception) {
            logger.log(TAG, "Failed to process invoice request: ${e.message}", "WARN")
            notifyChannel(
                context,
                NOTIFICATION_CHANNEL_REPLACEABLE,
                getString(
                    context,
                    INVOICE_REQUEST_NOTIFICATION_FAILURE_TITLE,
                    DEFAULT_INVOICE_REQUEST_NOTIFICATION_FAILURE_TITLE,
                ),
            )
        }

        fgService.onFinished(this)
    }

    override fun onEvent(e: SdkEvent) {}

    override fun onShutdown() {}
}
