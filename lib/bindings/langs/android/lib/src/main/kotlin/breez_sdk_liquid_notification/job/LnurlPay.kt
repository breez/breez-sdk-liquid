package breez_sdk_liquid_notification.job

import breez_sdk_liquid.SdkEvent
import breez_sdk_liquid_notification.ServiceLogger
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import java.io.DataOutputStream
import java.net.HttpURLConnection
import java.net.URL

@Serializable
data class LnurlErrorResponse(
    @SerialName("status") val status: String,
    @SerialName("reason") val reason: String? = null,
)

interface LnurlPayJob : Job {
    companion object {
        private const val TAG = "LnurlPayJob"
    }

    override fun onEvent(e: SdkEvent) {}

    fun fail(
        withError: String?,
        replyURL: String,
        logger: ServiceLogger,
    ): Boolean {
        val url = URL(replyURL)
        val payload = Json.encodeToString(LnurlErrorResponse("ERROR", withError))
        logger.log(TAG, "Send to: $replyURL, fail response: $payload", "WARN")
        val response = payload.toByteArray()

        with(url.openConnection() as HttpURLConnection) {
            requestMethod = "POST"
            doOutput = true
            useCaches = false
            setRequestProperty("Content-Type", "application/json")
            setRequestProperty("Content-Length", response.size.toString())
            DataOutputStream(outputStream).use { it.write(response, 0, response.size) }

            return responseCode == 200
        }
    }
}

class InvalidLnurlPayException(
    message: String,
) : Exception(message)
