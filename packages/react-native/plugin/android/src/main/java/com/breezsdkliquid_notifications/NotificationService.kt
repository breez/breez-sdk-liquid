package com.breezsdkliquid_notifications

import android.content.Intent
import android.content.BroadcastReceiver
import android.content.Context
import androidx.core.content.ContextCompat
import expo.modules.notifications.notifications.model.Notification
import breez_sdk_liquid_notification.Constants
import breez_sdk_liquid_notification.Message

const val NOTIFICATION_KEY = "notification"

class NotificationService: BroadcastReceiver() {

    override fun onReceive(context: Context?, intent: Intent?) {

        if(context === null || intent === null) {
            return
        }

        intent.getParcelableExtra<Notification>(NOTIFICATION_KEY)?.let { notification ->
            notification.notificationRequest.content.body?.let { body ->
                body[Constants.MESSAGE_DATA_TYPE]?.let { dataType ->
                    if(dataType is String) {
                        val message = Message(dataType, body[Constants.MESSAGE_DATA_PAYLOAD] as String)
                        val serviceIntent = Intent(context, ForegroundService::class.java)
                        serviceIntent.putExtra(Constants.EXTRA_REMOTE_MESSAGE, message)
                        ContextCompat.startForegroundService(context, serviceIntent)
                    }
                }
            }
        }
    }
}
