package breez_sdk_liquid_notification.job

import android.content.Context
import breez_sdk_liquid.BindingLiquidSdk
import breez_sdk_liquid_notification.Constants.DEFAULT_LNURL_PAY_INFO_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.DEFAULT_LNURL_PAY_METADATA_PLAIN_TEXT
import breez_sdk_liquid_notification.Constants.DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE
import breez_sdk_liquid_notification.Constants.LNURL_PAY_INFO_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.LNURL_PAY_METADATA_PLAIN_TEXT
import breez_sdk_liquid_notification.Constants.LNURL_PAY_NOTIFICATION_FAILURE_TITLE
import breez_sdk_liquid_notification.Constants.NOTIFICATION_CHANNEL_LNURL_PAY
import breez_sdk_liquid_notification.NotificationHelper.Companion.notifyChannel
import breez_sdk_liquid_notification.ResourceHelper.Companion.getString
import breez_sdk_liquid_notification.SdkForegroundService
import breez_sdk_liquid_notification.ServiceLogger
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

@Serializable
data class LnurlInfoRequest(
    @SerialName("callback_url") val callbackURL: String,
    @SerialName("reply_url") val replyURL: String,
)

// Serialize the response according to to LUD-06 payRequest base specification:
// https://github.com/lnurl/luds/blob/luds/06.md
@Serializable
data class LnurlPayInfoResponse(
    val callback: String,
    val maxSendable: ULong,
    val minSendable: ULong,
    val metadata: String,
    val tag: String,
)

class LnurlPayInfoJob(
    private val context: Context,
    private val fgService: SdkForegroundService,
    private val payload: String,
    private val logger: ServiceLogger,
) : LnurlPayJob {
    companion object {
        private const val TAG = "LnurlPayInfoJob"
    }

    override fun start(liquidSDK: BindingLiquidSdk) {
        var request: LnurlInfoRequest? = null
        try {
            request = Json.decodeFromString(LnurlInfoRequest.serializer(), payload)
            // Get the lightning limits
            val limits = liquidSDK.fetchLightningLimits()
            // Max millisatoshi amount LN SERVICE is willing to receive
            val maxSendableMsat = limits.receive.maxSat * 1000UL
            // Min millisatoshi amount LN SERVICE is willing to receive, can not be less than 1 or more than `maxSendableMsat`
            val minSendableMsat = limits.receive.minSat * 1000UL
            if (minSendableMsat < 1UL || minSendableMsat > maxSendableMsat) {
                throw InvalidLnurlPayException("Minimum sendable amount is invalid")
            }
            // Format the response
            val plainTextMetadata = getString(
                context, LNURL_PAY_METADATA_PLAIN_TEXT, DEFAULT_LNURL_PAY_METADATA_PLAIN_TEXT
            )
            val response = LnurlPayInfoResponse(
                request.callbackURL,
                maxSendableMsat,
                minSendableMsat,
                "[[\"text/plain\",\"$plainTextMetadata\"]]",
                "payRequest",
            )
            val success = replyServer(Json.encodeToString(response), request.replyURL)
            notifyChannel(
                context,
                NOTIFICATION_CHANNEL_LNURL_PAY,
                getString(
                    context,
                    if (success) LNURL_PAY_INFO_NOTIFICATION_TITLE else LNURL_PAY_NOTIFICATION_FAILURE_TITLE,
                    if (success) DEFAULT_LNURL_PAY_INFO_NOTIFICATION_TITLE else DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE
                ),
            )
        } catch (e: Exception) {
            logger.log(TAG, "Failed to process lnurl: ${e.message}", "WARN")
            if (request != null) {
                fail(e.message, request.replyURL)
            }
            notifyChannel(
                context,
                NOTIFICATION_CHANNEL_LNURL_PAY,
                getString(
                    context,
                    LNURL_PAY_NOTIFICATION_FAILURE_TITLE,
                    DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE
                ),
            )
        }

        fgService.onFinished(this)
    }

    override fun onShutdown() {}
}
