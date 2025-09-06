package breez_sdk_liquid_notification.job

import android.content.Context
import breez_sdk_liquid.BindingLiquidSdk
import breez_sdk_liquid.SdkEvent
import breez_sdk_liquid_notification.SdkForegroundService
import breez_sdk_liquid_notification.ServiceLogger
import breez_sdk_liquid_notification.NotificationHelper.Companion.notifyChannel
import breez_sdk_liquid_notification.ResourceHelper.Companion.getString
import breez_sdk_liquid_notification.Constants.NWC_EVENT_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.NWC_EVENT_NOTIFICATION_FAILURE_TITLE
import breez_sdk_liquid_notification.Constants.NWC_EVENT_NOTIFICATION_TEXT
import breez_sdk_liquid_notification.Constants.NWC_EVENT_NOTIFICATION_FAILURE_TEXT
import breez_sdk_liquid_notification.Constants.DEFAULT_NWC_EVENT_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.DEFAULT_NWC_EVENT_NOTIFICATION_FAILURE_TITLE
import breez_sdk_liquid_notification.Constants.DEFAULT_NWC_EVENT_NOTIFICATION_TEXT
import breez_sdk_liquid_notification.Constants.DEFAULT_NWC_EVENT_NOTIFICATION_FAILURE_TEXT
import breez_sdk_liquid_notification.Constants.NOTIFICATION_CHANNEL_DISMISSIBLE
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

@Serializable
data class NwcEventRequest(
  @SerialName("event_id") val eventId: String,
)

class NwcEventJob(
  private val context: Context,
  private val fgService: SdkForegroundService,
  private val payload: String,
  private val logger: ServiceLogger,
  private val scope: CoroutineScope = CoroutineScope(Dispatchers.Default)
) : Job {
  companion object {
    private const val TAG = "NwcEventJob"
  }

  private var request: NwcEventRequest? = null
  private var notified: Boolean = false

  override fun start(liquidSDK: BindingLiquidSdk) {
    try {
      val decoder = Json { ignoreUnknownKeys = true }
      request = decoder.decodeFromString(NwcEventRequest.serializer(), payload)
      
      logger.log(TAG, "Starting SDK for NWC event with ID: ${request!!.eventId}", "INFO")
    } catch (e: Exception) {
      logger.log(TAG, "Failed to decode NWC event payload: ${e.message}", "ERROR")
      fgService.onFinished(this)
    }
  }

  override fun onEvent(e: SdkEvent) {
    when (e) {
      is SdkEvent.Nwc -> {
        val nwcEvent = e.details
        val eventId = e.event_id

        if (eventId == request?.eventId) {
          logger.log(TAG, "Received matching NWC event with ID: $eventId", "INFO")
          notifySuccess(nwcEvent)
        }
      }
      else -> {
        logger.log(TAG, "Received event: $e", "TRACE")
      }
    }
  }

  override fun onShutdown() {
    if (!notified) {
      notifyFailure()
    }
  }

  private fun notifySuccess(nwcEvent: breez_sdk_liquid.NwcEvent) {
    if (!this.notified) {
      logger.log(TAG, "NWC event processing successful for ID: ${request?.eventId}", "INFO")

      val notificationTitle = getString(
        context,
        NWC_EVENT_NOTIFICATION_TITLE,
        DEFAULT_NWC_EVENT_NOTIFICATION_TITLE
      )
      val notificationBody = getString(
        context,
        NWC_EVENT_NOTIFICATION_TEXT,
        DEFAULT_NWC_EVENT_NOTIFICATION_TEXT
      )

      notifyChannel(
        context,
        NOTIFICATION_CHANNEL_DISMISSIBLE,
        notificationTitle,
        notificationBody
      )

      this.notified = true
      fgService.onFinished(this)
    }
  }

  private fun notifyFailure() {
    request?.eventId?.let { eventId ->
      logger.log(TAG, "NWC event $eventId processing failed", "INFO")

      val notificationTitle = getString(
        context,
        NWC_EVENT_NOTIFICATION_FAILURE_TITLE,
        DEFAULT_NWC_EVENT_NOTIFICATION_FAILURE_TITLE
      )
      val notificationBody = getString(
        context,
        NWC_EVENT_NOTIFICATION_FAILURE_TEXT,
        DEFAULT_NWC_EVENT_NOTIFICATION_FAILURE_TEXT
      )

      notifyChannel(
        context,
        NOTIFICATION_CHANNEL_DISMISSIBLE,
        notificationTitle,
        notificationBody
      )
    }
  }
}
