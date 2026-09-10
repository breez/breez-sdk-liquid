package com.breezsdkliquid_notifications

import android.content.Intent
import androidx.core.content.ContextCompat
import breez_sdk_liquid_notification.Constants
import breez_sdk_liquid_notification.Message
import breez_sdk_liquid_notification.MessagingService
import com.google.firebase.messaging.RemoteMessage
import expo.modules.notifications.service.ExpoFirebaseMessagingService

/**
 * Receives Breez data messages and hands them to the foreground service.
 *
 * This extends Expo's messaging service rather than listening for its internal
 * NOTIFICATION_EVENT broadcast. Expo registers its own service at `android:priority="-1"`
 * precisely so that an app can take over, and only one service ever receives a given FCM
 * message — so anything that isn't ours is passed to `super` and expo-notifications carries
 * on handling it exactly as before.
 *
 * Taking the message here, rather than downstream of Expo's notification pipeline, also
 * matters on Android 12+: a foreground service may only be started from the background under
 * an exemption, and FCM grants that exemption for the duration of `onMessageReceived`.
 */
class BreezFirebaseMessagingService :
    ExpoFirebaseMessagingService(),
    MessagingService {
    override fun onMessageReceived(remoteMessage: RemoteMessage) {
        val type = remoteMessage.data[Constants.MESSAGE_DATA_TYPE]
        val payload = remoteMessage.data[Constants.MESSAGE_DATA_PAYLOAD]

        if (type == null || payload == null) {
            super.onMessageReceived(remoteMessage)
            return
        }

        // Lets the SDK decide whether the service is actually needed — it skips messages that
        // the app can handle itself while it's in the foreground.
        startServiceIfNeeded(this, Message(type, payload))
    }

    override fun startForegroundService(message: Message) {
        val intent =
            Intent(this, ForegroundService::class.java)
                .putExtra(Constants.EXTRA_REMOTE_MESSAGE, message)

        ContextCompat.startForegroundService(this, intent)
    }
}
