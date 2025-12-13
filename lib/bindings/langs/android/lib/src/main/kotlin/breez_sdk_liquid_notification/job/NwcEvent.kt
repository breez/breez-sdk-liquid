package breez_sdk_liquid_notification.job

import android.content.Context
import breez_sdk_liquid.BindingLiquidSdk
import breez_sdk_liquid.SdkEvent
import breez_sdk_liquid.NwcEvent
import breez_sdk_liquid.NwcEventListener
import breez_sdk_liquid.NwcEventDetails
import breez_sdk_liquid_notification.SdkForegroundService
import breez_sdk_liquid_notification.ServiceLogger
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import breez_sdk_liquid_notification.ResourceHelper.Companion.getString
import breez_sdk_liquid_notification.NotificationHelper.Companion.notifyChannel
import breez_sdk_liquid_notification.Constants.NOTIFICATION_CHANNEL_DISMISSIBLE
import breez_sdk_liquid_notification.Constants.NWC_SUCCESS_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.DEFAULT_NWC_SUCCESS_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.NWC_FAILURE_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.DEFAULT_NWC_FAILURE_NOTIFICATION_TITLE
import PluginConfigs

@Serializable
data class NwcEventNotification(
        @SerialName("event_id") val eventId: String,
)

class NwcEventJob(
        private val context: Context,
        private val fgService: SdkForegroundService,
        private val payload: String,
        private val logger: ServiceLogger,
) : Job, NwcEventListener {
    private var eventId: String? = null
    companion object {
        private const val TAG = "NwcEventJob"
    }

    override fun start(liquidSDK: BindingLiquidSdk, pluginConfigs: PluginConfigs) {
        val nwcService = PluginManager.nwc(liquidSDK, pluginConfigs, logger)
        if (nwcService == null) return;
        nwcService.addEventListener(this)
        try {
            val decoder = Json { ignoreUnknownKeys = true }
            var notification = decoder.decodeFromString(NwcEventNotification.serializer(), payload)
            nwcService.handleEvent(notification.eventId)
            eventId = notification.eventId
        } catch (e: Exception) {
            logger.log(TAG, "Failed to process NWC event: ${e.message}", "WARN")
            notifyChannel(
                context,
                NOTIFICATION_CHANNEL_DISMISSIBLE,
                getString(
                    context,
                    NWC_FAILURE_NOTIFICATION_TITLE,
                    DEFAULT_NWC_FAILURE_NOTIFICATION_TITLE,
                ),
            )
        }
    }

    override fun onEvent(e: SdkEvent) {}

    override fun onEvent(event: NwcEvent) {
        if (eventId == null || eventId != event.eventId) {
            return
        }
        var eventName: String
        when(event.details) {
            is NwcEventDetails.GetBalance -> eventName = "Get Balance"
            is NwcEventDetails.ListTransactions -> eventName = "List Transactions"
            is NwcEventDetails.PayInvoice -> eventName = "Pay Invoice"
            else -> return
        }
        notifyChannel(
            context,
            NOTIFICATION_CHANNEL_DISMISSIBLE,
            String.format(
                getString(
                    context,
                    NWC_SUCCESS_NOTIFICATION_TITLE,
                    "%s",
                    DEFAULT_NWC_SUCCESS_NOTIFICATION_TITLE,
                ),
                eventName,
            )
        )
        fgService.onFinished(this)
    }

    override fun onShutdown() {}

}
