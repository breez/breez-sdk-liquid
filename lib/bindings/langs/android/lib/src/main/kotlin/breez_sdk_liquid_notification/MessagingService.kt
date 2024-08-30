package breez_sdk_liquid_notification

import android.app.ActivityManager
import android.app.ActivityManager.RunningAppProcessInfo.IMPORTANCE_FOREGROUND
import android.app.ActivityManager.RunningAppProcessInfo.IMPORTANCE_VISIBLE
import android.content.Context
import android.util.Log
import breez_sdk_liquid_notification.Constants.MESSAGE_TYPE_SWAP_UPDATED
import breez_sdk_liquid_notification.NotificationHelper.Companion.getNotificationManager

@Suppress("unused")
interface MessagingService {
    companion object {
        private const val TAG = "MessagingService"
    }

    /** To be implemented by the application messaging service.
     *  The implemented function should start the foreground with
     *  the provided Message in an Intent. */
    fun startForegroundService(message: Message)

    /** Check if the foreground service is needed depending on the
     *  message type and foreground state of the application. */
    fun startServiceIfNeeded(context: Context, message: Message) {
        val notificationManager = getNotificationManager(context)
        val isServiceNeeded = when (message.type) {
            MESSAGE_TYPE_SWAP_UPDATED -> !isAppForeground(context)
            else -> true
        }
        if (notificationManager != null && isServiceNeeded) startForegroundService(message)
        else Log.w(TAG, "Ignoring message ${message.type}: ${message.payload}")
    }

    /** Basic implementation to check if the application is in the foreground */
    fun isAppForeground(context: Context): Boolean {
        val appProcessInfo = ActivityManager.RunningAppProcessInfo()
        ActivityManager.getMyMemoryState(appProcessInfo)

        return (appProcessInfo.importance == IMPORTANCE_FOREGROUND || appProcessInfo.importance == IMPORTANCE_VISIBLE)
    }
}
