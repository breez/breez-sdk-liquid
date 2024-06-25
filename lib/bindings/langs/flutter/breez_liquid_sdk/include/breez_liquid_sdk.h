#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
// EXTRA BEGIN
typedef struct DartCObject *WireSyncRust2DartDco;
typedef struct WireSyncRust2DartSse {
  uint8_t *ptr;
  int32_t len;
} WireSyncRust2DartSse;

typedef int64_t DartPort;
typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);
void store_dart_post_cobject(DartPostCObjectFnType ptr);
// EXTRA END
typedef struct _Dart_Handle* Dart_Handle;

#define LOWBALL_FEE_RATE_SAT_PER_VBYTE 0.01

/**
 * The minimum acceptable fee rate when claiming using zero-conf
 */
#define DEFAULT_ZERO_CONF_MIN_FEE_RATE_TESTNET 0.1

#define DEFAULT_ZERO_CONF_MIN_FEE_RATE_MAINNET 0.01

/**
 * The maximum acceptable amount in satoshi when claiming using zero-conf
 */
#define DEFAULT_ZERO_CONF_MAX_SAT 100000

typedef struct wire_cst_list_prim_u_8_strict {
  uint8_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_8_strict;

typedef struct wire_cst_backup_request {
  struct wire_cst_list_prim_u_8_strict *backup_path;
} wire_cst_backup_request;

typedef struct wire_cst_ln_url_auth_request_data {
  struct wire_cst_list_prim_u_8_strict *k1;
  struct wire_cst_list_prim_u_8_strict *action;
  struct wire_cst_list_prim_u_8_strict *domain;
  struct wire_cst_list_prim_u_8_strict *url;
} wire_cst_ln_url_auth_request_data;

typedef struct wire_cst_ln_url_pay_request_data {
  struct wire_cst_list_prim_u_8_strict *callback;
  uint64_t min_sendable;
  uint64_t max_sendable;
  struct wire_cst_list_prim_u_8_strict *metadata_str;
  uint16_t comment_allowed;
  struct wire_cst_list_prim_u_8_strict *domain;
  bool allows_nostr;
  struct wire_cst_list_prim_u_8_strict *nostr_pubkey;
  struct wire_cst_list_prim_u_8_strict *ln_address;
} wire_cst_ln_url_pay_request_data;

typedef struct wire_cst_ln_url_pay_request {
  struct wire_cst_ln_url_pay_request_data data;
  uint64_t amount_msat;
  struct wire_cst_list_prim_u_8_strict *comment;
  struct wire_cst_list_prim_u_8_strict *payment_label;
} wire_cst_ln_url_pay_request;

typedef struct wire_cst_ln_url_withdraw_request_data {
  struct wire_cst_list_prim_u_8_strict *callback;
  struct wire_cst_list_prim_u_8_strict *k1;
  struct wire_cst_list_prim_u_8_strict *default_description;
  uint64_t min_withdrawable;
  uint64_t max_withdrawable;
} wire_cst_ln_url_withdraw_request_data;

typedef struct wire_cst_ln_url_withdraw_request {
  struct wire_cst_ln_url_withdraw_request_data data;
  uint64_t amount_msat;
  struct wire_cst_list_prim_u_8_strict *description;
} wire_cst_ln_url_withdraw_request;

typedef struct wire_cst_prepare_pay_onchain_response {
  uint64_t amount_sat;
  uint64_t fees_sat;
} wire_cst_prepare_pay_onchain_response;

typedef struct wire_cst_pay_onchain_request {
  struct wire_cst_list_prim_u_8_strict *address;
  struct wire_cst_prepare_pay_onchain_response prepare_res;
} wire_cst_pay_onchain_request;

typedef struct wire_cst_prepare_pay_onchain_request {
  uint64_t amount_sat;
} wire_cst_prepare_pay_onchain_request;

typedef struct wire_cst_prepare_receive_onchain_request {
  uint64_t amount_sat;
} wire_cst_prepare_receive_onchain_request;

typedef struct wire_cst_prepare_receive_request {
  uint64_t payer_amount_sat;
} wire_cst_prepare_receive_request;

typedef struct wire_cst_prepare_refund_request {
  struct wire_cst_list_prim_u_8_strict *swap_address;
  struct wire_cst_list_prim_u_8_strict *refund_address;
  uint32_t sat_per_vbyte;
} wire_cst_prepare_refund_request;

typedef struct wire_cst_prepare_send_request {
  struct wire_cst_list_prim_u_8_strict *invoice;
} wire_cst_prepare_send_request;

typedef struct wire_cst_prepare_receive_onchain_response {
  uint64_t amount_sat;
  uint64_t fees_sat;
} wire_cst_prepare_receive_onchain_response;

typedef struct wire_cst_receive_onchain_request {
  struct wire_cst_prepare_receive_onchain_response prepare_res;
} wire_cst_receive_onchain_request;

typedef struct wire_cst_prepare_receive_response {
  uint64_t payer_amount_sat;
  uint64_t fees_sat;
} wire_cst_prepare_receive_response;

typedef struct wire_cst_refund_request {
  struct wire_cst_list_prim_u_8_strict *swap_address;
  struct wire_cst_list_prim_u_8_strict *refund_address;
  uint32_t sat_per_vbyte;
} wire_cst_refund_request;

typedef struct wire_cst_restore_request {
  struct wire_cst_list_prim_u_8_strict *backup_path;
} wire_cst_restore_request;

typedef struct wire_cst_prepare_send_response {
  struct wire_cst_list_prim_u_8_strict *invoice;
  uint64_t fees_sat;
} wire_cst_prepare_send_response;

typedef struct wire_cst_binding_event_listener {
  struct wire_cst_list_prim_u_8_strict *stream;
} wire_cst_binding_event_listener;

typedef struct wire_cst_payment {
  struct wire_cst_list_prim_u_8_strict *tx_id;
  struct wire_cst_list_prim_u_8_strict *swap_id;
  uint32_t timestamp;
  uint64_t amount_sat;
  uint64_t fees_sat;
  struct wire_cst_list_prim_u_8_strict *preimage;
  struct wire_cst_list_prim_u_8_strict *bolt11;
  struct wire_cst_list_prim_u_8_strict *refund_tx_id;
  uint64_t *refund_tx_amount_sat;
  int32_t payment_type;
  int32_t status;
} wire_cst_payment;

typedef struct wire_cst_LiquidSdkEvent_PaymentFailed {
  struct wire_cst_payment *details;
} wire_cst_LiquidSdkEvent_PaymentFailed;

typedef struct wire_cst_LiquidSdkEvent_PaymentPending {
  struct wire_cst_payment *details;
} wire_cst_LiquidSdkEvent_PaymentPending;

typedef struct wire_cst_LiquidSdkEvent_PaymentRefunded {
  struct wire_cst_payment *details;
} wire_cst_LiquidSdkEvent_PaymentRefunded;

typedef struct wire_cst_LiquidSdkEvent_PaymentRefundPending {
  struct wire_cst_payment *details;
} wire_cst_LiquidSdkEvent_PaymentRefundPending;

typedef struct wire_cst_LiquidSdkEvent_PaymentSucceeded {
  struct wire_cst_payment *details;
} wire_cst_LiquidSdkEvent_PaymentSucceeded;

typedef struct wire_cst_LiquidSdkEvent_PaymentWaitingConfirmation {
  struct wire_cst_payment *details;
} wire_cst_LiquidSdkEvent_PaymentWaitingConfirmation;

typedef union LiquidSdkEventKind {
  struct wire_cst_LiquidSdkEvent_PaymentFailed PaymentFailed;
  struct wire_cst_LiquidSdkEvent_PaymentPending PaymentPending;
  struct wire_cst_LiquidSdkEvent_PaymentRefunded PaymentRefunded;
  struct wire_cst_LiquidSdkEvent_PaymentRefundPending PaymentRefundPending;
  struct wire_cst_LiquidSdkEvent_PaymentSucceeded PaymentSucceeded;
  struct wire_cst_LiquidSdkEvent_PaymentWaitingConfirmation PaymentWaitingConfirmation;
} LiquidSdkEventKind;

typedef struct wire_cst_liquid_sdk_event {
  int32_t tag;
  union LiquidSdkEventKind kind;
} wire_cst_liquid_sdk_event;

typedef struct wire_cst_config {
  struct wire_cst_list_prim_u_8_strict *liquid_electrum_url;
  struct wire_cst_list_prim_u_8_strict *bitcoin_electrum_url;
  struct wire_cst_list_prim_u_8_strict *working_dir;
  int32_t network;
  uint64_t payment_timeout_sec;
  float zero_conf_min_fee_rate;
  uint64_t *zero_conf_max_amount_sat;
} wire_cst_config;

typedef struct wire_cst_connect_request {
  struct wire_cst_list_prim_u_8_strict *mnemonic;
  struct wire_cst_config config;
} wire_cst_connect_request;

typedef struct wire_cst_aes_success_action_data_decrypted {
  struct wire_cst_list_prim_u_8_strict *description;
  struct wire_cst_list_prim_u_8_strict *plaintext;
} wire_cst_aes_success_action_data_decrypted;

typedef struct wire_cst_AesSuccessActionDataResult_Decrypted {
  struct wire_cst_aes_success_action_data_decrypted *data;
} wire_cst_AesSuccessActionDataResult_Decrypted;

typedef struct wire_cst_AesSuccessActionDataResult_ErrorStatus {
  struct wire_cst_list_prim_u_8_strict *reason;
} wire_cst_AesSuccessActionDataResult_ErrorStatus;

typedef union AesSuccessActionDataResultKind {
  struct wire_cst_AesSuccessActionDataResult_Decrypted Decrypted;
  struct wire_cst_AesSuccessActionDataResult_ErrorStatus ErrorStatus;
} AesSuccessActionDataResultKind;

typedef struct wire_cst_aes_success_action_data_result {
  int32_t tag;
  union AesSuccessActionDataResultKind kind;
} wire_cst_aes_success_action_data_result;

typedef struct wire_cst_bitcoin_address_data {
  struct wire_cst_list_prim_u_8_strict *address;
  int32_t network;
  uint64_t *amount_sat;
  struct wire_cst_list_prim_u_8_strict *label;
  struct wire_cst_list_prim_u_8_strict *message;
} wire_cst_bitcoin_address_data;

typedef struct wire_cst_route_hint_hop {
  struct wire_cst_list_prim_u_8_strict *src_node_id;
  uint64_t short_channel_id;
  uint32_t fees_base_msat;
  uint32_t fees_proportional_millionths;
  uint64_t cltv_expiry_delta;
  uint64_t *htlc_minimum_msat;
  uint64_t *htlc_maximum_msat;
} wire_cst_route_hint_hop;

typedef struct wire_cst_list_route_hint_hop {
  struct wire_cst_route_hint_hop *ptr;
  int32_t len;
} wire_cst_list_route_hint_hop;

typedef struct wire_cst_route_hint {
  struct wire_cst_list_route_hint_hop *hops;
} wire_cst_route_hint;

typedef struct wire_cst_list_route_hint {
  struct wire_cst_route_hint *ptr;
  int32_t len;
} wire_cst_list_route_hint;

typedef struct wire_cst_ln_invoice {
  struct wire_cst_list_prim_u_8_strict *bolt11;
  int32_t network;
  struct wire_cst_list_prim_u_8_strict *payee_pubkey;
  struct wire_cst_list_prim_u_8_strict *payment_hash;
  struct wire_cst_list_prim_u_8_strict *description;
  struct wire_cst_list_prim_u_8_strict *description_hash;
  uint64_t *amount_msat;
  uint64_t timestamp;
  uint64_t expiry;
  struct wire_cst_list_route_hint *routing_hints;
  struct wire_cst_list_prim_u_8_strict *payment_secret;
  uint64_t min_final_cltv_expiry_delta;
} wire_cst_ln_invoice;

typedef struct wire_cst_ln_url_error_data {
  struct wire_cst_list_prim_u_8_strict *reason;
} wire_cst_ln_url_error_data;

typedef struct wire_cst_ln_url_pay_error_data {
  struct wire_cst_list_prim_u_8_strict *payment_hash;
  struct wire_cst_list_prim_u_8_strict *reason;
} wire_cst_ln_url_pay_error_data;

typedef struct wire_cst_SuccessActionProcessed_Aes {
  struct wire_cst_aes_success_action_data_result *result;
} wire_cst_SuccessActionProcessed_Aes;

typedef struct wire_cst_message_success_action_data {
  struct wire_cst_list_prim_u_8_strict *message;
} wire_cst_message_success_action_data;

typedef struct wire_cst_SuccessActionProcessed_Message {
  struct wire_cst_message_success_action_data *data;
} wire_cst_SuccessActionProcessed_Message;

typedef struct wire_cst_url_success_action_data {
  struct wire_cst_list_prim_u_8_strict *description;
  struct wire_cst_list_prim_u_8_strict *url;
} wire_cst_url_success_action_data;

typedef struct wire_cst_SuccessActionProcessed_Url {
  struct wire_cst_url_success_action_data *data;
} wire_cst_SuccessActionProcessed_Url;

typedef union SuccessActionProcessedKind {
  struct wire_cst_SuccessActionProcessed_Aes Aes;
  struct wire_cst_SuccessActionProcessed_Message Message;
  struct wire_cst_SuccessActionProcessed_Url Url;
} SuccessActionProcessedKind;

typedef struct wire_cst_success_action_processed {
  int32_t tag;
  union SuccessActionProcessedKind kind;
} wire_cst_success_action_processed;

typedef struct wire_cst_ln_url_pay_success_data {
  struct wire_cst_payment payment;
  struct wire_cst_success_action_processed *success_action;
} wire_cst_ln_url_pay_success_data;

typedef struct wire_cst_ln_url_withdraw_success_data {
  struct wire_cst_ln_invoice invoice;
} wire_cst_ln_url_withdraw_success_data;

typedef struct wire_cst_list_payment {
  struct wire_cst_payment *ptr;
  int32_t len;
} wire_cst_list_payment;

typedef struct wire_cst_refundable_swap {
  struct wire_cst_list_prim_u_8_strict *swap_address;
  uint32_t timestamp;
  uint64_t amount_sat;
} wire_cst_refundable_swap;

typedef struct wire_cst_list_refundable_swap {
  struct wire_cst_refundable_swap *ptr;
  int32_t len;
} wire_cst_list_refundable_swap;

typedef struct wire_cst_get_info_response {
  uint64_t balance_sat;
  uint64_t pending_send_sat;
  uint64_t pending_receive_sat;
  struct wire_cst_list_prim_u_8_strict *pubkey;
} wire_cst_get_info_response;

typedef struct wire_cst_InputType_BitcoinAddress {
  struct wire_cst_bitcoin_address_data *address;
} wire_cst_InputType_BitcoinAddress;

typedef struct wire_cst_InputType_Bolt11 {
  struct wire_cst_ln_invoice *invoice;
} wire_cst_InputType_Bolt11;

typedef struct wire_cst_InputType_NodeId {
  struct wire_cst_list_prim_u_8_strict *node_id;
} wire_cst_InputType_NodeId;

typedef struct wire_cst_InputType_Url {
  struct wire_cst_list_prim_u_8_strict *url;
} wire_cst_InputType_Url;

typedef struct wire_cst_InputType_LnUrlPay {
  struct wire_cst_ln_url_pay_request_data *data;
} wire_cst_InputType_LnUrlPay;

typedef struct wire_cst_InputType_LnUrlWithdraw {
  struct wire_cst_ln_url_withdraw_request_data *data;
} wire_cst_InputType_LnUrlWithdraw;

typedef struct wire_cst_InputType_LnUrlAuth {
  struct wire_cst_ln_url_auth_request_data *data;
} wire_cst_InputType_LnUrlAuth;

typedef struct wire_cst_InputType_LnUrlError {
  struct wire_cst_ln_url_error_data *data;
} wire_cst_InputType_LnUrlError;

typedef union InputTypeKind {
  struct wire_cst_InputType_BitcoinAddress BitcoinAddress;
  struct wire_cst_InputType_Bolt11 Bolt11;
  struct wire_cst_InputType_NodeId NodeId;
  struct wire_cst_InputType_Url Url;
  struct wire_cst_InputType_LnUrlPay LnUrlPay;
  struct wire_cst_InputType_LnUrlWithdraw LnUrlWithdraw;
  struct wire_cst_InputType_LnUrlAuth LnUrlAuth;
  struct wire_cst_InputType_LnUrlError LnUrlError;
} InputTypeKind;

typedef struct wire_cst_input_type {
  int32_t tag;
  union InputTypeKind kind;
} wire_cst_input_type;

typedef struct wire_cst_LiquidSdkError_Generic {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_LiquidSdkError_Generic;

typedef struct wire_cst_LiquidSdkError_ServiceConnectivity {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_LiquidSdkError_ServiceConnectivity;

typedef union LiquidSdkErrorKind {
  struct wire_cst_LiquidSdkError_Generic Generic;
  struct wire_cst_LiquidSdkError_ServiceConnectivity ServiceConnectivity;
} LiquidSdkErrorKind;

typedef struct wire_cst_liquid_sdk_error {
  int32_t tag;
  union LiquidSdkErrorKind kind;
} wire_cst_liquid_sdk_error;

typedef struct wire_cst_LnUrlAuthError_Generic {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_LnUrlAuthError_Generic;

typedef struct wire_cst_LnUrlAuthError_InvalidUri {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_LnUrlAuthError_InvalidUri;

typedef struct wire_cst_LnUrlAuthError_ServiceConnectivity {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_LnUrlAuthError_ServiceConnectivity;

typedef union LnUrlAuthErrorKind {
  struct wire_cst_LnUrlAuthError_Generic Generic;
  struct wire_cst_LnUrlAuthError_InvalidUri InvalidUri;
  struct wire_cst_LnUrlAuthError_ServiceConnectivity ServiceConnectivity;
} LnUrlAuthErrorKind;

typedef struct wire_cst_ln_url_auth_error {
  int32_t tag;
  union LnUrlAuthErrorKind kind;
} wire_cst_ln_url_auth_error;

typedef struct wire_cst_LnUrlCallbackStatus_ErrorStatus {
  struct wire_cst_ln_url_error_data *data;
} wire_cst_LnUrlCallbackStatus_ErrorStatus;

typedef union LnUrlCallbackStatusKind {
  struct wire_cst_LnUrlCallbackStatus_ErrorStatus ErrorStatus;
} LnUrlCallbackStatusKind;

typedef struct wire_cst_ln_url_callback_status {
  int32_t tag;
  union LnUrlCallbackStatusKind kind;
} wire_cst_ln_url_callback_status;

typedef struct wire_cst_LnUrlPayError_Generic {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_LnUrlPayError_Generic;

typedef struct wire_cst_LnUrlPayError_InvalidAmount {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_LnUrlPayError_InvalidAmount;

typedef struct wire_cst_LnUrlPayError_InvalidInvoice {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_LnUrlPayError_InvalidInvoice;

typedef struct wire_cst_LnUrlPayError_InvalidNetwork {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_LnUrlPayError_InvalidNetwork;

typedef struct wire_cst_LnUrlPayError_InvalidUri {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_LnUrlPayError_InvalidUri;

typedef struct wire_cst_LnUrlPayError_InvoiceExpired {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_LnUrlPayError_InvoiceExpired;

typedef struct wire_cst_LnUrlPayError_PaymentFailed {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_LnUrlPayError_PaymentFailed;

typedef struct wire_cst_LnUrlPayError_PaymentTimeout {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_LnUrlPayError_PaymentTimeout;

typedef struct wire_cst_LnUrlPayError_RouteNotFound {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_LnUrlPayError_RouteNotFound;

typedef struct wire_cst_LnUrlPayError_RouteTooExpensive {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_LnUrlPayError_RouteTooExpensive;

typedef struct wire_cst_LnUrlPayError_ServiceConnectivity {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_LnUrlPayError_ServiceConnectivity;

typedef union LnUrlPayErrorKind {
  struct wire_cst_LnUrlPayError_Generic Generic;
  struct wire_cst_LnUrlPayError_InvalidAmount InvalidAmount;
  struct wire_cst_LnUrlPayError_InvalidInvoice InvalidInvoice;
  struct wire_cst_LnUrlPayError_InvalidNetwork InvalidNetwork;
  struct wire_cst_LnUrlPayError_InvalidUri InvalidUri;
  struct wire_cst_LnUrlPayError_InvoiceExpired InvoiceExpired;
  struct wire_cst_LnUrlPayError_PaymentFailed PaymentFailed;
  struct wire_cst_LnUrlPayError_PaymentTimeout PaymentTimeout;
  struct wire_cst_LnUrlPayError_RouteNotFound RouteNotFound;
  struct wire_cst_LnUrlPayError_RouteTooExpensive RouteTooExpensive;
  struct wire_cst_LnUrlPayError_ServiceConnectivity ServiceConnectivity;
} LnUrlPayErrorKind;

typedef struct wire_cst_ln_url_pay_error {
  int32_t tag;
  union LnUrlPayErrorKind kind;
} wire_cst_ln_url_pay_error;

typedef struct wire_cst_LnUrlPayResult_EndpointSuccess {
  struct wire_cst_ln_url_pay_success_data *data;
} wire_cst_LnUrlPayResult_EndpointSuccess;

typedef struct wire_cst_LnUrlPayResult_EndpointError {
  struct wire_cst_ln_url_error_data *data;
} wire_cst_LnUrlPayResult_EndpointError;

typedef struct wire_cst_LnUrlPayResult_PayError {
  struct wire_cst_ln_url_pay_error_data *data;
} wire_cst_LnUrlPayResult_PayError;

typedef union LnUrlPayResultKind {
  struct wire_cst_LnUrlPayResult_EndpointSuccess EndpointSuccess;
  struct wire_cst_LnUrlPayResult_EndpointError EndpointError;
  struct wire_cst_LnUrlPayResult_PayError PayError;
} LnUrlPayResultKind;

typedef struct wire_cst_ln_url_pay_result {
  int32_t tag;
  union LnUrlPayResultKind kind;
} wire_cst_ln_url_pay_result;

typedef struct wire_cst_LnUrlWithdrawError_Generic {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_LnUrlWithdrawError_Generic;

typedef struct wire_cst_LnUrlWithdrawError_InvalidAmount {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_LnUrlWithdrawError_InvalidAmount;

typedef struct wire_cst_LnUrlWithdrawError_InvalidInvoice {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_LnUrlWithdrawError_InvalidInvoice;

typedef struct wire_cst_LnUrlWithdrawError_InvalidUri {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_LnUrlWithdrawError_InvalidUri;

typedef struct wire_cst_LnUrlWithdrawError_InvoiceNoRoutingHints {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_LnUrlWithdrawError_InvoiceNoRoutingHints;

typedef struct wire_cst_LnUrlWithdrawError_ServiceConnectivity {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_LnUrlWithdrawError_ServiceConnectivity;

typedef union LnUrlWithdrawErrorKind {
  struct wire_cst_LnUrlWithdrawError_Generic Generic;
  struct wire_cst_LnUrlWithdrawError_InvalidAmount InvalidAmount;
  struct wire_cst_LnUrlWithdrawError_InvalidInvoice InvalidInvoice;
  struct wire_cst_LnUrlWithdrawError_InvalidUri InvalidUri;
  struct wire_cst_LnUrlWithdrawError_InvoiceNoRoutingHints InvoiceNoRoutingHints;
  struct wire_cst_LnUrlWithdrawError_ServiceConnectivity ServiceConnectivity;
} LnUrlWithdrawErrorKind;

typedef struct wire_cst_ln_url_withdraw_error {
  int32_t tag;
  union LnUrlWithdrawErrorKind kind;
} wire_cst_ln_url_withdraw_error;

typedef struct wire_cst_LnUrlWithdrawResult_Ok {
  struct wire_cst_ln_url_withdraw_success_data *data;
} wire_cst_LnUrlWithdrawResult_Ok;

typedef struct wire_cst_LnUrlWithdrawResult_ErrorStatus {
  struct wire_cst_ln_url_error_data *data;
} wire_cst_LnUrlWithdrawResult_ErrorStatus;

typedef union LnUrlWithdrawResultKind {
  struct wire_cst_LnUrlWithdrawResult_Ok Ok;
  struct wire_cst_LnUrlWithdrawResult_ErrorStatus ErrorStatus;
} LnUrlWithdrawResultKind;

typedef struct wire_cst_ln_url_withdraw_result {
  int32_t tag;
  union LnUrlWithdrawResultKind kind;
} wire_cst_ln_url_withdraw_result;

typedef struct wire_cst_log_entry {
  struct wire_cst_list_prim_u_8_strict *line;
  struct wire_cst_list_prim_u_8_strict *level;
} wire_cst_log_entry;

typedef struct wire_cst_PaymentError_Generic {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_PaymentError_Generic;

typedef struct wire_cst_PaymentError_InvalidInvoice {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_PaymentError_InvalidInvoice;

typedef struct wire_cst_PaymentError_LwkError {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_PaymentError_LwkError;

typedef struct wire_cst_PaymentError_ReceiveError {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_PaymentError_ReceiveError;

typedef struct wire_cst_PaymentError_Refunded {
  struct wire_cst_list_prim_u_8_strict *err;
  struct wire_cst_list_prim_u_8_strict *refund_tx_id;
} wire_cst_PaymentError_Refunded;

typedef struct wire_cst_PaymentError_SendError {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_PaymentError_SendError;

typedef struct wire_cst_PaymentError_SignerError {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_PaymentError_SignerError;

typedef union PaymentErrorKind {
  struct wire_cst_PaymentError_Generic Generic;
  struct wire_cst_PaymentError_InvalidInvoice InvalidInvoice;
  struct wire_cst_PaymentError_LwkError LwkError;
  struct wire_cst_PaymentError_ReceiveError ReceiveError;
  struct wire_cst_PaymentError_Refunded Refunded;
  struct wire_cst_PaymentError_SendError SendError;
  struct wire_cst_PaymentError_SignerError SignerError;
} PaymentErrorKind;

typedef struct wire_cst_payment_error {
  int32_t tag;
  union PaymentErrorKind kind;
} wire_cst_payment_error;

typedef struct wire_cst_prepare_refund_response {
  uint32_t refund_tx_vsize;
  uint64_t refund_tx_fee_sat;
} wire_cst_prepare_refund_response;

typedef struct wire_cst_receive_onchain_response {
  struct wire_cst_list_prim_u_8_strict *address;
  struct wire_cst_list_prim_u_8_strict *bip21;
} wire_cst_receive_onchain_response;

typedef struct wire_cst_receive_payment_response {
  struct wire_cst_list_prim_u_8_strict *id;
  struct wire_cst_list_prim_u_8_strict *invoice;
} wire_cst_receive_payment_response;

typedef struct wire_cst_refund_response {
  struct wire_cst_list_prim_u_8_strict *refund_tx_id;
} wire_cst_refund_response;

typedef struct wire_cst_send_payment_response {
  struct wire_cst_payment payment;
} wire_cst_send_payment_response;

void frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_add_event_listener(int64_t port_,
                                                                                    uintptr_t that,
                                                                                    struct wire_cst_list_prim_u_8_strict *listener);

WireSyncRust2DartDco frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_backup(uintptr_t that,
                                                                                        struct wire_cst_backup_request *req);

void frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_disconnect(int64_t port_,
                                                                            uintptr_t that);

WireSyncRust2DartDco frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_empty_wallet_cache(uintptr_t that);

void frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_get_info(int64_t port_,
                                                                          uintptr_t that);

void frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_list_payments(int64_t port_,
                                                                               uintptr_t that);

void frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_list_refundables(int64_t port_,
                                                                                  uintptr_t that);

void frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_lnurl_auth(int64_t port_,
                                                                            uintptr_t that,
                                                                            struct wire_cst_ln_url_auth_request_data *req_data);

void frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_lnurl_pay(int64_t port_,
                                                                           uintptr_t that,
                                                                           struct wire_cst_ln_url_pay_request *req);

void frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_lnurl_withdraw(int64_t port_,
                                                                                uintptr_t that,
                                                                                struct wire_cst_ln_url_withdraw_request *req);

void frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_pay_onchain(int64_t port_,
                                                                             uintptr_t that,
                                                                             struct wire_cst_pay_onchain_request *req);

void frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_prepare_pay_onchain(int64_t port_,
                                                                                     uintptr_t that,
                                                                                     struct wire_cst_prepare_pay_onchain_request *req);

void frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_prepare_receive_onchain(int64_t port_,
                                                                                         uintptr_t that,
                                                                                         struct wire_cst_prepare_receive_onchain_request *req);

void frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_prepare_receive_payment(int64_t port_,
                                                                                         uintptr_t that,
                                                                                         struct wire_cst_prepare_receive_request *req);

void frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_prepare_refund(int64_t port_,
                                                                                uintptr_t that,
                                                                                struct wire_cst_prepare_refund_request *req);

void frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_prepare_send_payment(int64_t port_,
                                                                                      uintptr_t that,
                                                                                      struct wire_cst_prepare_send_request *req);

void frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_receive_onchain(int64_t port_,
                                                                                 uintptr_t that,
                                                                                 struct wire_cst_receive_onchain_request *req);

void frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_receive_payment(int64_t port_,
                                                                                 uintptr_t that,
                                                                                 struct wire_cst_prepare_receive_response *req);

void frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_refund(int64_t port_,
                                                                        uintptr_t that,
                                                                        struct wire_cst_refund_request *req);

WireSyncRust2DartDco frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_restore(uintptr_t that,
                                                                                         struct wire_cst_restore_request *req);

void frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_send_payment(int64_t port_,
                                                                              uintptr_t that,
                                                                              struct wire_cst_prepare_send_response *req);

void frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_sync(int64_t port_,
                                                                      uintptr_t that);

void frbgen_breez_liquid_wire__crate__bindings__binding_event_listener_on_event(int64_t port_,
                                                                                struct wire_cst_binding_event_listener *that,
                                                                                struct wire_cst_liquid_sdk_event *e);

void frbgen_breez_liquid_wire__crate__bindings__breez_log_stream(int64_t port_,
                                                                 struct wire_cst_list_prim_u_8_strict *s);

void frbgen_breez_liquid_wire__crate__bindings__connect(int64_t port_,
                                                        struct wire_cst_connect_request *req);

WireSyncRust2DartDco frbgen_breez_liquid_wire__crate__bindings__default_config(int32_t network);

void frbgen_breez_liquid_wire__crate__bindings__parse(int64_t port_,
                                                      struct wire_cst_list_prim_u_8_strict *input);

WireSyncRust2DartDco frbgen_breez_liquid_wire__crate__bindings__parse_invoice(struct wire_cst_list_prim_u_8_strict *input);

void frbgen_breez_liquid_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk(const void *ptr);

void frbgen_breez_liquid_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk(const void *ptr);

struct wire_cst_aes_success_action_data_decrypted *frbgen_breez_liquid_cst_new_box_autoadd_aes_success_action_data_decrypted(void);

struct wire_cst_aes_success_action_data_result *frbgen_breez_liquid_cst_new_box_autoadd_aes_success_action_data_result(void);

struct wire_cst_backup_request *frbgen_breez_liquid_cst_new_box_autoadd_backup_request(void);

struct wire_cst_binding_event_listener *frbgen_breez_liquid_cst_new_box_autoadd_binding_event_listener(void);

struct wire_cst_bitcoin_address_data *frbgen_breez_liquid_cst_new_box_autoadd_bitcoin_address_data(void);

struct wire_cst_connect_request *frbgen_breez_liquid_cst_new_box_autoadd_connect_request(void);

struct wire_cst_liquid_sdk_event *frbgen_breez_liquid_cst_new_box_autoadd_liquid_sdk_event(void);

struct wire_cst_ln_invoice *frbgen_breez_liquid_cst_new_box_autoadd_ln_invoice(void);

struct wire_cst_ln_url_auth_request_data *frbgen_breez_liquid_cst_new_box_autoadd_ln_url_auth_request_data(void);

struct wire_cst_ln_url_error_data *frbgen_breez_liquid_cst_new_box_autoadd_ln_url_error_data(void);

struct wire_cst_ln_url_pay_error_data *frbgen_breez_liquid_cst_new_box_autoadd_ln_url_pay_error_data(void);

struct wire_cst_ln_url_pay_request *frbgen_breez_liquid_cst_new_box_autoadd_ln_url_pay_request(void);

struct wire_cst_ln_url_pay_request_data *frbgen_breez_liquid_cst_new_box_autoadd_ln_url_pay_request_data(void);

struct wire_cst_ln_url_pay_success_data *frbgen_breez_liquid_cst_new_box_autoadd_ln_url_pay_success_data(void);

struct wire_cst_ln_url_withdraw_request *frbgen_breez_liquid_cst_new_box_autoadd_ln_url_withdraw_request(void);

struct wire_cst_ln_url_withdraw_request_data *frbgen_breez_liquid_cst_new_box_autoadd_ln_url_withdraw_request_data(void);

struct wire_cst_ln_url_withdraw_success_data *frbgen_breez_liquid_cst_new_box_autoadd_ln_url_withdraw_success_data(void);

struct wire_cst_message_success_action_data *frbgen_breez_liquid_cst_new_box_autoadd_message_success_action_data(void);

struct wire_cst_pay_onchain_request *frbgen_breez_liquid_cst_new_box_autoadd_pay_onchain_request(void);

struct wire_cst_payment *frbgen_breez_liquid_cst_new_box_autoadd_payment(void);

struct wire_cst_prepare_pay_onchain_request *frbgen_breez_liquid_cst_new_box_autoadd_prepare_pay_onchain_request(void);

struct wire_cst_prepare_receive_onchain_request *frbgen_breez_liquid_cst_new_box_autoadd_prepare_receive_onchain_request(void);

struct wire_cst_prepare_receive_request *frbgen_breez_liquid_cst_new_box_autoadd_prepare_receive_request(void);

struct wire_cst_prepare_receive_response *frbgen_breez_liquid_cst_new_box_autoadd_prepare_receive_response(void);

struct wire_cst_prepare_refund_request *frbgen_breez_liquid_cst_new_box_autoadd_prepare_refund_request(void);

struct wire_cst_prepare_send_request *frbgen_breez_liquid_cst_new_box_autoadd_prepare_send_request(void);

struct wire_cst_prepare_send_response *frbgen_breez_liquid_cst_new_box_autoadd_prepare_send_response(void);

struct wire_cst_receive_onchain_request *frbgen_breez_liquid_cst_new_box_autoadd_receive_onchain_request(void);

struct wire_cst_refund_request *frbgen_breez_liquid_cst_new_box_autoadd_refund_request(void);

struct wire_cst_restore_request *frbgen_breez_liquid_cst_new_box_autoadd_restore_request(void);

struct wire_cst_success_action_processed *frbgen_breez_liquid_cst_new_box_autoadd_success_action_processed(void);

uint64_t *frbgen_breez_liquid_cst_new_box_autoadd_u_64(uint64_t value);

struct wire_cst_url_success_action_data *frbgen_breez_liquid_cst_new_box_autoadd_url_success_action_data(void);

struct wire_cst_list_payment *frbgen_breez_liquid_cst_new_list_payment(int32_t len);

struct wire_cst_list_prim_u_8_strict *frbgen_breez_liquid_cst_new_list_prim_u_8_strict(int32_t len);

struct wire_cst_list_refundable_swap *frbgen_breez_liquid_cst_new_list_refundable_swap(int32_t len);

struct wire_cst_list_route_hint *frbgen_breez_liquid_cst_new_list_route_hint(int32_t len);

struct wire_cst_list_route_hint_hop *frbgen_breez_liquid_cst_new_list_route_hint_hop(int32_t len);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_aes_success_action_data_decrypted);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_aes_success_action_data_result);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_backup_request);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_binding_event_listener);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_bitcoin_address_data);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_connect_request);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_liquid_sdk_event);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_ln_invoice);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_ln_url_auth_request_data);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_ln_url_error_data);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_ln_url_pay_error_data);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_ln_url_pay_request);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_ln_url_pay_request_data);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_ln_url_pay_success_data);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_ln_url_withdraw_request);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_ln_url_withdraw_request_data);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_ln_url_withdraw_success_data);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_message_success_action_data);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_pay_onchain_request);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_payment);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_prepare_pay_onchain_request);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_prepare_receive_onchain_request);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_prepare_receive_request);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_prepare_receive_response);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_prepare_refund_request);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_prepare_send_request);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_prepare_send_response);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_receive_onchain_request);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_refund_request);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_restore_request);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_success_action_processed);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_u_64);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_url_success_action_data);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_list_payment);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_list_prim_u_8_strict);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_list_refundable_swap);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_list_route_hint);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_list_route_hint_hop);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_add_event_listener);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_backup);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_disconnect);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_empty_wallet_cache);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_get_info);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_list_payments);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_list_refundables);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_lnurl_auth);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_lnurl_pay);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_lnurl_withdraw);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_pay_onchain);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_prepare_pay_onchain);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_prepare_receive_onchain);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_prepare_receive_payment);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_prepare_refund);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_prepare_send_payment);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_receive_onchain);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_receive_payment);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_refund);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_restore);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_send_payment);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_sync);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__binding_event_listener_on_event);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__breez_log_stream);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__connect);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__default_config);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__parse);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire__crate__bindings__parse_invoice);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
