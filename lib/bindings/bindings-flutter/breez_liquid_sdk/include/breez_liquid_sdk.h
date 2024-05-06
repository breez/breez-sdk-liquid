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

/**
 * Claim tx feerate for Receive, in sats per vbyte.
 * Since the  Liquid blocks are consistently empty for now, we hardcode the minimum feerate.
 */
#define LIQUID_CLAIM_TX_FEERATE 0.1

typedef struct wire_cst_list_prim_u_8_strict {
  uint8_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_8_strict;

typedef struct wire_cst_connect_request {
  struct wire_cst_list_prim_u_8_strict *mnemonic;
  struct wire_cst_list_prim_u_8_strict *data_dir;
  int32_t network;
} wire_cst_connect_request;

typedef struct wire_cst_get_info_request {
  bool with_scan;
} wire_cst_get_info_request;

typedef struct wire_cst_prepare_receive_request {
  uint64_t payer_amount_sat;
} wire_cst_prepare_receive_request;

typedef struct wire_cst_prepare_send_request {
  struct wire_cst_list_prim_u_8_strict *invoice;
} wire_cst_prepare_send_request;

typedef struct wire_cst_prepare_receive_response {
  struct wire_cst_list_prim_u_8_strict *pair_hash;
  uint64_t payer_amount_sat;
  uint64_t fees_sat;
} wire_cst_prepare_receive_response;

typedef struct wire_cst_restore_request {
  struct wire_cst_list_prim_u_8_strict *backup_path;
} wire_cst_restore_request;

typedef struct wire_cst_prepare_send_response {
  struct wire_cst_list_prim_u_8_strict *id;
  uint64_t payer_amount_sat;
  uint64_t receiver_amount_sat;
  uint64_t total_fees;
  struct wire_cst_list_prim_u_8_strict *funding_address;
  struct wire_cst_list_prim_u_8_strict *invoice;
} wire_cst_prepare_send_response;

typedef struct wire_cst_payment {
  struct wire_cst_list_prim_u_8_strict *id;
  uint32_t *timestamp;
  uint64_t amount_sat;
  uint64_t *fees_sat;
  int32_t payment_type;
  struct wire_cst_list_prim_u_8_strict *invoice;
} wire_cst_payment;

typedef struct wire_cst_list_payment {
  struct wire_cst_payment *ptr;
  int32_t len;
} wire_cst_list_payment;

typedef struct wire_cst_get_info_response {
  uint64_t balance_sat;
  struct wire_cst_list_prim_u_8_strict *pubkey;
} wire_cst_get_info_response;

typedef struct wire_cst_PaymentError_Generic {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_PaymentError_Generic;

typedef struct wire_cst_PaymentError_LwkError {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_PaymentError_LwkError;

typedef struct wire_cst_PaymentError_SendError {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_PaymentError_SendError;

typedef struct wire_cst_PaymentError_SignerError {
  struct wire_cst_list_prim_u_8_strict *err;
} wire_cst_PaymentError_SignerError;

typedef union PaymentErrorKind {
  struct wire_cst_PaymentError_Generic Generic;
  struct wire_cst_PaymentError_LwkError LwkError;
  struct wire_cst_PaymentError_SendError SendError;
  struct wire_cst_PaymentError_SignerError SignerError;
} PaymentErrorKind;

typedef struct wire_cst_payment_error {
  int32_t tag;
  union PaymentErrorKind kind;
} wire_cst_payment_error;

typedef struct wire_cst_receive_payment_response {
  struct wire_cst_list_prim_u_8_strict *id;
  struct wire_cst_list_prim_u_8_strict *invoice;
} wire_cst_receive_payment_response;

typedef struct wire_cst_send_payment_response {
  struct wire_cst_list_prim_u_8_strict *txid;
} wire_cst_send_payment_response;

void frbgen_breez_liquid_wire_backup(int64_t port_);

void frbgen_breez_liquid_wire_connect(int64_t port_, struct wire_cst_connect_request *req);

void frbgen_breez_liquid_wire_empty_wallet_cache(int64_t port_);

void frbgen_breez_liquid_wire_get_info(int64_t port_, struct wire_cst_get_info_request *req);

void frbgen_breez_liquid_wire_list_payments(int64_t port_, bool with_scan, bool include_pending);

void frbgen_breez_liquid_wire_prepare_receive_payment(int64_t port_,
                                                      struct wire_cst_prepare_receive_request *req);

void frbgen_breez_liquid_wire_prepare_send_payment(int64_t port_,
                                                   struct wire_cst_prepare_send_request *req);

void frbgen_breez_liquid_wire_receive_payment(int64_t port_,
                                              struct wire_cst_prepare_receive_response *req);

void frbgen_breez_liquid_wire_recover_funds(int64_t port_, uintptr_t recovery);

void frbgen_breez_liquid_wire_restore(int64_t port_, struct wire_cst_restore_request *req);

void frbgen_breez_liquid_wire_send_payment(int64_t port_,
                                           struct wire_cst_prepare_send_response *req);

void frbgen_breez_liquid_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerLBtcReverseRecovery(const void *ptr);

void frbgen_breez_liquid_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerLBtcReverseRecovery(const void *ptr);

struct wire_cst_connect_request *frbgen_breez_liquid_cst_new_box_autoadd_connect_request(void);

struct wire_cst_get_info_request *frbgen_breez_liquid_cst_new_box_autoadd_get_info_request(void);

struct wire_cst_prepare_receive_request *frbgen_breez_liquid_cst_new_box_autoadd_prepare_receive_request(void);

struct wire_cst_prepare_receive_response *frbgen_breez_liquid_cst_new_box_autoadd_prepare_receive_response(void);

struct wire_cst_prepare_send_request *frbgen_breez_liquid_cst_new_box_autoadd_prepare_send_request(void);

struct wire_cst_prepare_send_response *frbgen_breez_liquid_cst_new_box_autoadd_prepare_send_response(void);

struct wire_cst_restore_request *frbgen_breez_liquid_cst_new_box_autoadd_restore_request(void);

uint32_t *frbgen_breez_liquid_cst_new_box_autoadd_u_32(uint32_t value);

uint64_t *frbgen_breez_liquid_cst_new_box_autoadd_u_64(uint64_t value);

struct wire_cst_list_payment *frbgen_breez_liquid_cst_new_list_payment(int32_t len);

struct wire_cst_list_prim_u_8_strict *frbgen_breez_liquid_cst_new_list_prim_u_8_strict(int32_t len);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_connect_request);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_get_info_request);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_prepare_receive_request);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_prepare_receive_response);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_prepare_send_request);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_prepare_send_response);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_restore_request);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_u_32);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_box_autoadd_u_64);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_list_payment);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_cst_new_list_prim_u_8_strict);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerLBtcReverseRecovery);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerLBtcReverseRecovery);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire_backup);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire_connect);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire_empty_wallet_cache);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire_get_info);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire_list_payments);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire_prepare_receive_payment);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire_prepare_send_payment);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire_receive_payment);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire_recover_funds);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire_restore);
    dummy_var ^= ((int64_t) (void*) frbgen_breez_liquid_wire_send_payment);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
