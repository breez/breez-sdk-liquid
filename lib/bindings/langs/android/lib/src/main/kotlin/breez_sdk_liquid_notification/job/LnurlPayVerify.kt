package breez_sdk_liquid_notification.job

import android.content.Context
import breez_sdk_liquid.BindingLiquidSdk
import breez_sdk_liquid.GetPaymentRequest
import breez_sdk_liquid.PaymentDetails
import breez_sdk_liquid.PaymentState
import breez_sdk_liquid_notification.Constants.CACHE_CONTROL_MAX_AGE_THREE_SEC
import breez_sdk_liquid_notification.Constants.CACHE_CONTROL_MAX_AGE_WEEK
import breez_sdk_liquid_notification.Constants.DEFAULT_LNURL_PAY_VERIFY_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.DEFAULT_LNURL_PAY_VERIFY_NOTIFICATION_FAILURE_TITLE
import breez_sdk_liquid_notification.Constants.LNURL_PAY_VERIFY_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.LNURL_PAY_VERIFY_NOTIFICATION_FAILURE_TITLE
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
data class LnurlVerifyRequest(
    @SerialName("payment_hash") val paymentHash: String,
    @SerialName("reply_url") val replyURL: String,
)

// Serialize the response according to to LUD-21 verify specification:
// https://github.com/lnurl/luds/blob/luds/21.md
@Serializable
data class LnurlPayVerifyResponse(
    val status: String,
    val settled: Boolean,
    val preimage: String?,
    val pr: String,
) {
    constructor(settled: Boolean, preimage: String?, pr: String) : this("OK", settled, preimage, pr)
}

class LnurlPayVerifyJob(
    private val context: Context,
    private val fgService: SdkForegroundService,
    private val payload: String,
    private val logger: ServiceLogger,
) : LnurlPayJob {
    companion object {
        private const val TAG = "LnurlPayVerifyJob"
    }

    override fun start(liquidSDK: BindingLiquidSdk) {
        var request: LnurlVerifyRequest? = null
        try {
            val decoder = Json { ignoreUnknownKeys = true }
            request = decoder.decodeFromString(LnurlVerifyRequest.serializer(), payload)
            // Get the payment by payment hash
            val getPaymentReq = GetPaymentRequest.PaymentHash(request.paymentHash)
            val payment = liquidSDK.getPayment(getPaymentReq) ?: run {
                throw InvalidLnurlPayException("Not found")
            }
            val response = when (val details = payment.details) {
                is PaymentDetails.Lightning -> {
                    // In the case of a Lightning payment, if it's paid via Lightning or MRH,
                    // we can release the preimage
                    val settled = when (payment.status) {
                        // If the payment is pending, we need to check if it's paid via Lightning or MRH
                        PaymentState.PENDING -> details.claimTxId != null
                        PaymentState.COMPLETE -> true
                        else -> false
                    }
                    LnurlPayVerifyResponse(
                        settled,
                        if (settled) details.preimage else null,
                        details.invoice!!,
                    )
                }
                else -> null
            } ?: run {
                throw InvalidLnurlPayException("Not found")
            }

            val maxAge = if (response.settled) CACHE_CONTROL_MAX_AGE_WEEK else CACHE_CONTROL_MAX_AGE_THREE_SEC
            val success = replyServer(Json.encodeToString(response), request.replyURL, maxAge)
            notifyChannel(
                context,
                NOTIFICATION_CHANNEL_REPLACEABLE,
                getString(
                    context,
                    if (success) LNURL_PAY_VERIFY_NOTIFICATION_TITLE else LNURL_PAY_VERIFY_NOTIFICATION_FAILURE_TITLE,
                    if (success) DEFAULT_LNURL_PAY_VERIFY_NOTIFICATION_TITLE else DEFAULT_LNURL_PAY_VERIFY_NOTIFICATION_FAILURE_TITLE,
                ),
            )
        } catch (e: Exception) {
            logger.log(TAG, "Failed to process lnurl verify: ${e.message}", "WARN")
            if (request != null) {
                fail(e.message, request.replyURL, logger)
            }
            notifyChannel(
                context,
                NOTIFICATION_CHANNEL_REPLACEABLE,
                getString(
                    context,
                    LNURL_PAY_VERIFY_NOTIFICATION_FAILURE_TITLE,
                    DEFAULT_LNURL_PAY_VERIFY_NOTIFICATION_FAILURE_TITLE,
                ),
            )
        }

        fgService.onFinished(this)
    }

    override fun onShutdown() {}
}
