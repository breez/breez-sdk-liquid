package breez_sdk_liquid_notification.job

import android.content.Context
import breez_sdk_liquid.BindingLiquidSdk
import breez_sdk_liquid.BindingNwcService
import breez_sdk_liquid.GetPaymentRequest
import breez_sdk_liquid.NwcEvent
import breez_sdk_liquid.NwcEventDetails
import breez_sdk_liquid.NwcEventListener
import breez_sdk_liquid.Payment
import breez_sdk_liquid.PaymentDetails
import breez_sdk_liquid.PaymentState
import breez_sdk_liquid.PaymentType
import breez_sdk_liquid.SdkEvent
import breez_sdk_liquid_notification.Constants.DEFAULT_PAYMENT_RECEIVED_NOTIFICATION_TEXT
import breez_sdk_liquid_notification.Constants.DEFAULT_PAYMENT_RECEIVED_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.DEFAULT_PAYMENT_SENT_NOTIFICATION_TEXT
import breez_sdk_liquid_notification.Constants.DEFAULT_PAYMENT_SENT_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.DEFAULT_PAYMENT_WAITING_FEE_ACCEPTANCE_TEXT
import breez_sdk_liquid_notification.Constants.DEFAULT_PAYMENT_WAITING_FEE_ACCEPTANCE_TITLE
import breez_sdk_liquid_notification.Constants.DEFAULT_SWAP_CONFIRMED_NOTIFICATION_FAILURE_TEXT
import breez_sdk_liquid_notification.Constants.DEFAULT_SWAP_CONFIRMED_NOTIFICATION_FAILURE_TITLE
import breez_sdk_liquid_notification.Constants.NOTIFICATION_CHANNEL_DISMISSIBLE
import breez_sdk_liquid_notification.Constants.PAYMENT_RECEIVED_NOTIFICATION_TEXT
import breez_sdk_liquid_notification.Constants.PAYMENT_RECEIVED_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.PAYMENT_SENT_NOTIFICATION_TEXT
import breez_sdk_liquid_notification.Constants.PAYMENT_SENT_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.PAYMENT_WAITING_FEE_ACCEPTANCE_TEXT
import breez_sdk_liquid_notification.Constants.PAYMENT_WAITING_FEE_ACCEPTANCE_TITLE
import breez_sdk_liquid_notification.Constants.SWAP_CONFIRMED_NOTIFICATION_FAILURE_TEXT
import breez_sdk_liquid_notification.Constants.SWAP_CONFIRMED_NOTIFICATION_FAILURE_TITLE
import breez_sdk_liquid_notification.NotificationHelper.Companion.notifyChannel
import breez_sdk_liquid_notification.ResourceHelper.Companion.getString
import breez_sdk_liquid_notification.SdkForegroundService
import breez_sdk_liquid_notification.ServiceLogger
import breez_sdk_liquid_notification.PluginConfigs
import breez_sdk_liquid_notification.PluginManager
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.isActive
import kotlinx.coroutines.launch
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
    private val scope: CoroutineScope = CoroutineScope(Dispatchers.Default)
) : Job, NwcEventListener {
    private var swapIdHash: String? = null
    private var zapInvoice: String? = null
    private var isZapPublished: Boolean = false
    private var notified: Boolean = false
    private var pollingJob: kotlinx.coroutines.Job? = null
    private val pollingInterval: Long = 5000
    private var nwcService: BindingNwcService? = null

    companion object {
        private const val TAG = "SwapUpdatedJob"
    }

    override fun start(liquidSDK: BindingLiquidSdk, pluginConfigs: PluginConfigs) {
        this.nwcService = PluginManager.nwc(liquidSDK, pluginConfigs, logger)
        nwcService?.addEventListener(this)
        try {
            val decoder = Json { ignoreUnknownKeys = true }
            val request = decoder.decodeFromString(SwapUpdatedRequest.serializer(), payload)
            this.swapIdHash = request.id
            startPolling(liquidSDK)
        } catch (e: Exception) {
            logger.log(TAG, "Failed to decode payload: ${e.message}", "WARN")
        }
    }

    private fun startPolling(liquidSDK: BindingLiquidSdk) {
        pollingJob = scope.launch {
            while (isActive) {
                try {
                    if (swapIdHash == null) {
                        stopPolling(Exception("Missing swap ID"))
                        return@launch
                    }

                    liquidSDK.getPayment(GetPaymentRequest.SwapId(swapIdHash!!))?.let { payment ->
                        when (payment.status) {
                            PaymentState.COMPLETE -> {
                                onEvent(SdkEvent.PaymentSucceeded(payment))
                                stopPolling()
                                return@launch
                            }
                            PaymentState.WAITING_FEE_ACCEPTANCE -> {
                                onEvent(SdkEvent.PaymentWaitingFeeAcceptance(payment))
                                stopPolling()
                                return@launch
                            }
                            PaymentState.PENDING -> {
                                if (paymentClaimIsBroadcasted(payment.details)) {
                                    onEvent(SdkEvent.PaymentWaitingConfirmation(payment))
                                    stopPolling()
                                    return@launch
                                }
                            }
                            else -> { }
                        }
                    }
                    delay(pollingInterval)
                } catch (e: Exception) {
                    stopPolling(e)
                    return@launch
                }
            }
        }
    }

    private fun stopPolling(error: Exception? = null) {
        pollingJob?.cancel()
        pollingJob = null

        error?.let {
            logger.log(TAG, "Polling stopped with error: ${it.message}", "ERROR")
            onShutdown()
        }
    }

    override fun onEvent(e: SdkEvent) {
        when (e) {
            is SdkEvent.PaymentWaitingConfirmation -> handlePaymentSuccess(e.details)
            is SdkEvent.PaymentSucceeded -> handlePaymentSuccess(e.details)
            is SdkEvent.PaymentWaitingFeeAcceptance -> handlePaymentWaitingFeeAcceptance(e.details)

            else -> {
                logger.log(TAG, "Received event: $e", "TRACE")
            }
        }
    }

    override fun onShutdown() {
        notifyFailure()
    }

    private fun hashId(id: String): String =
        MessageDigest
            .getInstance("SHA-256")
            .digest(id.toByteArray())
            .fold(StringBuilder()) { sb, it -> sb.append("%02x".format(it)) }
            .toString()

    private fun getSwapId(details: PaymentDetails?): String? =
        when (details) {
            is PaymentDetails.Bitcoin -> details.swapId
            is PaymentDetails.Lightning -> details.swapId
            else -> null
        }

    private fun getInvoice(details: PaymentDetails?): String? =
        when (details) {
            is PaymentDetails.Lightning -> details.invoice
            else -> null
        }

    private fun paymentClaimIsBroadcasted(details: PaymentDetails?): Boolean {
        return when (details) {
            is PaymentDetails.Bitcoin -> details.claimTxId != null
            is PaymentDetails.Lightning -> details.claimTxId != null
            else -> false
        }
    }

    private fun handlePaymentSuccess(payment: Payment) {
        val swapId = getSwapId(payment.details)

        swapId?.let {
            if (this.swapIdHash == hashId(it)) {
                logger.log(
                    TAG,
                    "Received payment event: ${this.swapIdHash} ${payment.status}",
                    "TRACE",
                )
                notifySuccess(payment)
                // Run off the callback thread to avoid JNA reentrancy crash
                // when isZap() calls back into Rust
                scope.launch {
                    checkAndTrackZap(payment)
                }
            }
        }
    }

    private fun handlePaymentWaitingFeeAcceptance(payment: Payment) {
        val swapId = getSwapId(payment.details)

        swapId?.let {
            if (this.swapIdHash == hashId(it)) {
                logger.log(
                    TAG,
                    "Payment waiting fee acceptance: ${this.swapIdHash}",
                    "TRACE",
                )
                notifyPaymentWaitingFeeAcceptance(payment)
            }
        }
    }

    private fun notifySuccess(payment: Payment) {
        if (!this.notified) {
            logger.log(TAG, "Payment ${payment.txId} processing successful", "INFO")
            val received = payment.paymentType == PaymentType.RECEIVE
            notifyChannel(
                context,
                NOTIFICATION_CHANNEL_DISMISSIBLE,
                getString(
                    context,
                    if (received) PAYMENT_RECEIVED_NOTIFICATION_TITLE else PAYMENT_SENT_NOTIFICATION_TITLE,
                    if (received) DEFAULT_PAYMENT_RECEIVED_NOTIFICATION_TITLE else DEFAULT_PAYMENT_SENT_NOTIFICATION_TITLE,
                ),
                String.format(
                    getString(
                        context,
                        if (received) PAYMENT_RECEIVED_NOTIFICATION_TEXT else PAYMENT_SENT_NOTIFICATION_TEXT,
                        "%d",
                        if (received) DEFAULT_PAYMENT_RECEIVED_NOTIFICATION_TEXT else DEFAULT_PAYMENT_SENT_NOTIFICATION_TEXT,
                    ),
                    payment.amountSat.toLong(),
                ),
            )
            this.notified = true
        }
    }

    private fun notifyPaymentWaitingFeeAcceptance(payment: Payment) {
        if (!this.notified) {
            logger.log(TAG, "Payment with swap ID ${getSwapId(payment.details)} requires fee acceptance", "INFO")
            notifyChannel(
                context,
                NOTIFICATION_CHANNEL_DISMISSIBLE,
                getString(
                    context,
                    PAYMENT_WAITING_FEE_ACCEPTANCE_TITLE,
                    DEFAULT_PAYMENT_WAITING_FEE_ACCEPTANCE_TITLE,
                ),
                getString(
                    context,
                    PAYMENT_WAITING_FEE_ACCEPTANCE_TEXT,
                    DEFAULT_PAYMENT_WAITING_FEE_ACCEPTANCE_TEXT,
                ),
            )
            this.notified = true
        }
        fgService.onFinished(this)
    }

    private fun checkAndTrackZap(payment: Payment) {
        val invoice = getInvoice(payment.details)
        if (invoice != null && nwcService != null) {
            try {
                if (nwcService!!.isZap(invoice)) {
                    logger.log(TAG, "Waiting for zap receipt", "INFO")
                    zapInvoice = invoice
                    return
                }
            } catch (e: Exception) {
                logger.log(TAG, "Failed to check zap status: ${e.message}", "WARN")
            }
        }
        // If not a zap or error checking, finish immediately
        fgService.onFinished(this)
    }

    override fun onEvent(event: NwcEvent) {
        if (zapInvoice == null) return
        when (event.details) {
            is NwcEventDetails.ZapReceived -> {
                val receivedInvoice = (event.details as NwcEventDetails.ZapReceived).invoice
                if (zapInvoice == receivedInvoice) {
                    logger.log(TAG, "Zap receipt published for invoice: $zapInvoice", "INFO")
                    isZapPublished = true
                    fgService.onFinished(this)
                }
            }
            else -> { }
        }
    }

    private fun notifyFailure() {
        this.swapIdHash?.let { swapIdHash ->
            logger.log(TAG, "Swap $swapIdHash processing failed", "INFO")
            notifyChannel(
                context,
                NOTIFICATION_CHANNEL_DISMISSIBLE,
                getString(
                    context,
                    SWAP_CONFIRMED_NOTIFICATION_FAILURE_TITLE,
                    DEFAULT_SWAP_CONFIRMED_NOTIFICATION_FAILURE_TITLE,
                ),
                getString(
                    context,
                    SWAP_CONFIRMED_NOTIFICATION_FAILURE_TEXT,
                    DEFAULT_SWAP_CONFIRMED_NOTIFICATION_FAILURE_TEXT,
                ),
            )
        }
    }
}
