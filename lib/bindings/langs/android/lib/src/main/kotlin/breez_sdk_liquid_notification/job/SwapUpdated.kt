package breez_sdk_liquid_notification.job

import android.content.Context
import breez_sdk_liquid.BindingLiquidSdk
import breez_sdk_liquid.Payment
import breez_sdk_liquid.PaymentDetails
import breez_sdk_liquid.PaymentType
import breez_sdk_liquid.SdkEvent
import breez_sdk_liquid_notification.Constants.DEFAULT_PAYMENT_RECEIVED_NOTIFICATION_TEXT
import breez_sdk_liquid_notification.Constants.DEFAULT_PAYMENT_RECEIVED_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.DEFAULT_PAYMENT_SENT_NOTIFICATION_TEXT
import breez_sdk_liquid_notification.Constants.DEFAULT_PAYMENT_SENT_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.DEFAULT_SWAP_CONFIRMED_NOTIFICATION_FAILURE_TEXT
import breez_sdk_liquid_notification.Constants.DEFAULT_SWAP_CONFIRMED_NOTIFICATION_FAILURE_TITLE
import breez_sdk_liquid_notification.Constants.NOTIFICATION_CHANNEL_SWAP_UPDATED
import breez_sdk_liquid_notification.Constants.PAYMENT_RECEIVED_NOTIFICATION_TEXT
import breez_sdk_liquid_notification.Constants.PAYMENT_RECEIVED_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.PAYMENT_SENT_NOTIFICATION_TEXT
import breez_sdk_liquid_notification.Constants.PAYMENT_SENT_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.SWAP_CONFIRMED_NOTIFICATION_FAILURE_TEXT
import breez_sdk_liquid_notification.Constants.SWAP_CONFIRMED_NOTIFICATION_FAILURE_TITLE
import breez_sdk_liquid_notification.NotificationHelper.Companion.notifyChannel
import breez_sdk_liquid_notification.ResourceHelper.Companion.getString
import breez_sdk_liquid_notification.SdkForegroundService
import breez_sdk_liquid_notification.ServiceLogger
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import java.security.MessageDigest

@Serializable
data class SwapUpdatedRequest(
    val id: String,
    val status: String,
)

class SwapUpdatedJob(
    private val context: Context,
    private val fgService: SdkForegroundService,
    private val payload: String,
    private val logger: ServiceLogger,
) : Job {
    private var swapIdHash: String? = null
    private var notified: Boolean = false

    companion object {
        private const val TAG = "SwapUpdatedJob"
    }

    override fun start(liquidSDK: BindingLiquidSdk) {
        try {
            val request = Json.decodeFromString(SwapUpdatedRequest.serializer(), payload)
            this.swapIdHash = request.id
        } catch (e: Exception) {
            logger.log(TAG, "Failed to decode payload: ${e.message}", "WARN")
        }
    }

    override fun onEvent(e: SdkEvent) {
        when (e) {
            is SdkEvent.PaymentWaitingConfirmation -> handlePaymentEvent(e.details)
            is SdkEvent.PaymentSucceeded -> handlePaymentEvent(e.details)

            else -> {
                logger.log(TAG, "Received event: ${e}", "TRACE")
            }
        }
    }

    override fun onShutdown() {
        notifyFailure()
    }

    private fun hashId(id: String): String =
        MessageDigest.getInstance("SHA-256")
            .digest(id.toByteArray())
            .fold(StringBuilder()) { sb, it -> sb.append("%02x".format(it)) }
            .toString()

    private fun handlePaymentEvent(payment: Payment) {
        val swapId = when (val details = payment.details) {
            is PaymentDetails.Bitcoin -> details.swapId
            is PaymentDetails.Lightning -> details.swapId
            else -> null
        }

        swapId?.let {
            if (this.swapIdHash == hashId(it)) {
                logger.log(
                    TAG,
                    "Received payment event: ${this.swapIdHash} ${payment.status}",
                    "TRACE"
                )
                notifySuccess(payment)
            }
        }
    }

    private fun notifySuccess(payment: Payment) {
        if (!this.notified) {
            logger.log(TAG, "Payment ${payment.txId} processing successful", "INFO")
            val received = payment.paymentType == PaymentType.RECEIVE
            notifyChannel(
                context,
                NOTIFICATION_CHANNEL_SWAP_UPDATED,
                getString(
                    context,
                    if (received) PAYMENT_RECEIVED_NOTIFICATION_TITLE else PAYMENT_SENT_NOTIFICATION_TITLE,
                    if (received) DEFAULT_PAYMENT_RECEIVED_NOTIFICATION_TITLE else DEFAULT_PAYMENT_SENT_NOTIFICATION_TITLE
                ),
                String.format(
                    getString(
                        context,
                        if (received) PAYMENT_RECEIVED_NOTIFICATION_TEXT else PAYMENT_SENT_NOTIFICATION_TEXT,
                        "%d",
                        if (received) DEFAULT_PAYMENT_RECEIVED_NOTIFICATION_TEXT else DEFAULT_PAYMENT_SENT_NOTIFICATION_TEXT
                    ), payment.amountSat.toLong()
                )
            )
            this.notified = true
            fgService.onFinished(this)
        }
    }

    private fun notifyFailure() {
        this.swapIdHash?.let { swapIdHash ->
            logger.log(TAG, "Swap $swapIdHash processing failed", "INFO")
            notifyChannel(
                context,
                NOTIFICATION_CHANNEL_SWAP_UPDATED,
                getString(
                    context,
                    SWAP_CONFIRMED_NOTIFICATION_FAILURE_TITLE,
                    DEFAULT_SWAP_CONFIRMED_NOTIFICATION_FAILURE_TITLE
                ),
                getString(
                    context,
                    SWAP_CONFIRMED_NOTIFICATION_FAILURE_TEXT,
                    DEFAULT_SWAP_CONFIRMED_NOTIFICATION_FAILURE_TEXT
                ),
            )
        }
    }
}
