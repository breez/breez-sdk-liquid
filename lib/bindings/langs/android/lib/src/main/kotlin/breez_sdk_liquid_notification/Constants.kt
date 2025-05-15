package breez_sdk_liquid_notification

object Constants {
    const val SERVICE_TIMEOUT_MS = 3 * 60 * 1000L
    const val SHUTDOWN_DELAY_MS = 60 * 1000L

    // Cache Control
    const val CACHE_CONTROL_MAX_AGE_DAY = 60 * 60 * 24
    const val CACHE_CONTROL_MAX_AGE_WEEK = 60 * 60 * 24 * 7

    // Notification Channels
    const val NOTIFICATION_CHANNEL_DISMISSIBLE = "DISMISSIBLE"
    const val NOTIFICATION_CHANNEL_FOREGROUND_SERVICE = "FOREGROUND_SERVICE"
    const val NOTIFICATION_CHANNEL_REPLACEABLE = "REPLACEABLE"

    // Notification Ids
    const val NOTIFICATION_ID_FOREGROUND_SERVICE = 100
    const val NOTIFICATION_ID_REPLACEABLE = 1001

    // Intent Extras
    const val EXTRA_REMOTE_MESSAGE = "remote_message"

    // Message Data
    @Suppress("unused")
    const val MESSAGE_DATA_TYPE = "notification_type"

    @Suppress("unused")
    const val MESSAGE_DATA_PAYLOAD = "notification_payload"

    const val MESSAGE_TYPE_INVOICE_REQUEST = "invoice_request"
    const val MESSAGE_TYPE_LNURL_PAY_INFO = "lnurlpay_info"
    const val MESSAGE_TYPE_LNURL_PAY_INVOICE = "lnurlpay_invoice"
    const val MESSAGE_TYPE_LNURL_PAY_VERIFY = "lnurlpay_verify"
    const val MESSAGE_TYPE_SWAP_UPDATED = "swap_updated"

    // Resource Identifiers
    const val DISMISSIBLE_NOTIFICATION_CHANNEL_DESCRIPTION =
        "dismissible_notification_channel_description"
    const val DISMISSIBLE_NOTIFICATION_CHANNEL_NAME =
        "dismissible_notification_channel_name"
    const val DISMISSIBLE_WORKGROUP_ID = "dismissible"
    const val DISMISSIBLE_WORKGROUP_DESCRIPTION =
        "dismissible_work_group_description"
    const val DISMISSIBLE_WORKGROUP_NAME = "dismissible_work_group_name"
    const val FOREGROUND_SERVICE_NOTIFICATION_CHANNEL_DESCRIPTION =
        "foreground_service_notification_channel_description"
    const val FOREGROUND_SERVICE_NOTIFICATION_CHANNEL_NAME =
        "foreground_service_notification_channel_name"
    const val FOREGROUND_SERVICE_NOTIFICATION_TITLE =
        "foreground_service_notification_title"
    const val INVOICE_REQUEST_NOTIFICATION_TITLE =
        "invoice_request_notification_title"
    const val INVOICE_REQUEST_NOTIFICATION_FAILURE_TITLE =
        "invoice_request_notification_failure_title"
    const val LNURL_PAY_INFO_NOTIFICATION_TITLE =
        "lnurl_pay_info_notification_title"
    const val LNURL_PAY_INVOICE_NOTIFICATION_TITLE =
        "lnurl_pay_invoice_notification_title"
    const val LNURL_PAY_VERIFY_NOTIFICATION_TITLE =
        "lnurl_pay_verify_notification_title"
    const val LNURL_PAY_VERIFY_NOTIFICATION_FAILURE_TITLE =
        "lnurl_pay_verify_notification_failure_title"
    const val LNURL_PAY_METADATA_PLAIN_TEXT =
        "lnurl_pay_metadata_plain_text"
    const val LNURL_PAY_NOTIFICATION_FAILURE_TITLE =
        "lnurl_pay_notification_failure_title"
    const val NOTIFICATION_COLOR = "default_notification_color"
    const val NOTIFICATION_ICON = "ic_stat_ic_notification"
    const val PAYMENT_RECEIVED_NOTIFICATION_TEXT =
        "payment_received_notification_text"
    const val PAYMENT_RECEIVED_NOTIFICATION_TITLE =
        "payment_received_notification_title"
    const val PAYMENT_SENT_NOTIFICATION_TEXT =
        "payment_sent_notification_text"
    const val PAYMENT_SENT_NOTIFICATION_TITLE =
        "payment_sent_notification_title"
    const val PAYMENT_WAITING_FEE_ACCEPTANCE_TITLE =
        "payment_waiting_fee_acceptance_notification_title"
    const val PAYMENT_WAITING_FEE_ACCEPTANCE_TEXT =
        "payment_waiting_fee_acceptance_text"
    const val REPLACEABLE_NOTIFICATION_CHANNEL_DESCRIPTION =
        "replaceable_notification_channel_description"
    const val REPLACEABLE_NOTIFICATION_CHANNEL_NAME =
        "replaceable_notification_channel_name"
    const val REPLACEABLE_WORKGROUP_ID = "replaceable"
    const val REPLACEABLE_WORKGROUP_DESCRIPTION = "replaceable_work_group_description"
    const val REPLACEABLE_WORKGROUP_NAME = "replaceable_work_group_name"
    const val SWAP_CONFIRMED_NOTIFICATION_FAILURE_TEXT =
        "swap_confirmed_notification_failure_text"
    const val SWAP_CONFIRMED_NOTIFICATION_FAILURE_TITLE =
        "swap_confirmed_notification_failure_title"
    const val SWAP_CONFIRMED_NOTIFICATION_TITLE =
        "swap_confirmed_notification_title"

    // Resource Identifier Defaults
    const val DEFAULT_DISMISSIBLE_NOTIFICATION_CHANNEL_DESCRIPTION =
        "Channel for dismissible notifications when the application is in the background"
    const val DEFAULT_DISMISSIBLE_NOTIFICATION_CHANNEL_NAME = "Dismissable Notifications"
    const val DEFAULT_DISMISSIBLE_WORKGROUP_DESCRIPTION =
        "Required to handle dismissible notifications when the application is in the background"
    const val DEFAULT_DISMISSIBLE_WORKGROUP_NAME = "Dismissable Notifications"
    const val DEFAULT_FOREGROUND_SERVICE_NOTIFICATION_CHANNEL_DESCRIPTION =
        "Shown when the application is in the background"
    const val DEFAULT_FOREGROUND_SERVICE_NOTIFICATION_CHANNEL_NAME =
        "Foreground Service"
    const val DEFAULT_FOREGROUND_SERVICE_NOTIFICATION_TITLE =
        "Running in the background"
    const val DEFAULT_LNURL_PAY_INFO_NOTIFICATION_TITLE =
        "Retrieving Payment Information"
    const val DEFAULT_INVOICE_REQUEST_NOTIFICATION_TITLE =
        "Fetching Invoice"
    const val DEFAULT_INVOICE_REQUEST_NOTIFICATION_FAILURE_TITLE =
        "Invoice Request Failed"
    const val DEFAULT_LNURL_PAY_INVOICE_NOTIFICATION_TITLE =
        "Fetching Invoice"
    const val DEFAULT_LNURL_PAY_VERIFY_NOTIFICATION_TITLE =
        "Verifying Payment"
    const val DEFAULT_LNURL_PAY_VERIFY_NOTIFICATION_FAILURE_TITLE =
        "Payment Verification Failed"
    const val DEFAULT_LNURL_PAY_METADATA_PLAIN_TEXT =
        "Pay with LNURL"
    const val DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE =
        "Receive Payment Failed"
    const val DEFAULT_NOTIFICATION_COLOR = "#0089F9"
    const val DEFAULT_PAYMENT_RECEIVED_NOTIFICATION_TEXT =
        "Received %d sats"
    const val DEFAULT_PAYMENT_RECEIVED_NOTIFICATION_TITLE =
        "Payment Received"
    const val DEFAULT_PAYMENT_SENT_NOTIFICATION_TEXT =
        "Sent %d sats"
    const val DEFAULT_PAYMENT_SENT_NOTIFICATION_TITLE =
        "Payment Sent"
    const val DEFAULT_PAYMENT_WAITING_FEE_ACCEPTANCE_TITLE =
        "Payment requires fee acceptance"
    const val DEFAULT_PAYMENT_WAITING_FEE_ACCEPTANCE_TEXT =
        "Tap to review updated fees"
    const val DEFAULT_REPLACEABLE_WORKGROUP_DESCRIPTION =
        "Required to handle replaceable notifications when the application is in the background"
    const val DEFAULT_REPLACEABLE_WORKGROUP_NAME = "Replaceable Notifications"
    const val DEFAULT_REPLACEABLE_NOTIFICATION_CHANNEL_DESCRIPTION =
        "Channel for replaceable notifications when the application is in the background"
    const val DEFAULT_REPLACEABLE_NOTIFICATION_CHANNEL_NAME =
        "Replaceable Notifications"
    const val DEFAULT_SWAP_CONFIRMED_NOTIFICATION_FAILURE_TEXT =
        "Tap to complete payment"
    const val DEFAULT_SWAP_CONFIRMED_NOTIFICATION_FAILURE_TITLE =
        "Payment Pending"
}
