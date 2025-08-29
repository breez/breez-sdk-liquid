package breez_sdk_liquid_notification.job

import android.content.Context
import breez_sdk_liquid.BindingLiquidSdk
import breez_sdk_liquid_notification.SdkForegroundService
import breez_sdk_liquid_notification.ServiceLogger
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
) : Job {
  companion object {
    private const val TAG = "NwcEventJob"
  }

  private var request: NwcEventRequest? = null

  override fun start(liquidSDK: BindingLiquidSdk) {
    try {
      val decoder = Json { ignoreUnknownKeys = true }
      request = decoder.decodeFromString(NwcEventRequest.serializer(), payload)
      
      logger.log(TAG, "Starting SDK for NWC event with ID: ${request!!.eventId}", "INFO")
      fgService.onFinished(this)
    } catch (e: Exception) {
      logger.log(TAG, "Failed to decode NWC event payload: ${e.message}", "ERROR")
      fgService.onFinished(this)
    }
  }
}
