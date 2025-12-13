package breez_sdk_liquid_notification.job

import breez_sdk_liquid.BindingLiquidSdk
import breez_sdk_liquid.EventListener
import java.io.DataOutputStream
import java.net.HttpURLConnection
import java.net.URL
import PluginConfigs


interface Job : EventListener {
    /** When the notification service is connected to the Breez Liquid SDK
     *  it calls `start` to initiate the job.
     */
    fun start(liquidSDK: BindingLiquidSdk, pluginConfigs: PluginConfigs)

    /** When the short service timeout is reached it calls `onShutdown`
     *  to cleanup the job.
     */
    fun onShutdown()

    fun replyServer(
        payload: String,
        replyURL: String,
        maxAge: Int = 0,
    ): Boolean {
        val url = URL(replyURL)
        val response = payload.toByteArray()

        with(url.openConnection() as HttpURLConnection) {
            requestMethod = "POST"
            doOutput = true
            useCaches = false
            setRequestProperty("Content-Type", "application/json")
            setRequestProperty("Content-Length", response.size.toString())
            if (maxAge > 0) {
                setRequestProperty("Cache-Control", "max-age=$maxAge")
            }
            DataOutputStream(outputStream).use { it.write(response, 0, response.size) }

            return responseCode == 200
        }
    }
}
