// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.35.

// ignore_for_file: unused_import, unused_element, unnecessary_import, duplicate_ignore, invalid_use_of_internal_member, annotate_overrides, non_constant_identifier_names, curly_braces_in_flow_control_structures, prefer_const_literals_to_create_immutables, unused_field

import 'bindings.dart';
import 'dart:async';
import 'dart:convert';
import 'dart:ffi' as ffi;
import 'error.dart';
import 'frb_generated.dart';
import 'model.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_io.dart';

abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
  RustLibApiImplPlatform({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  CrossPlatformFinalizerArg get rust_arc_decrement_strong_count_BindingLiquidSdkPtr => wire
      ._rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdkPtr;

  @protected
  BindingLiquidSdk
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk(
          dynamic raw);

  @protected
  BindingLiquidSdk
      dco_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk(
          dynamic raw);

  @protected
  BindingLiquidSdk dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk(
      dynamic raw);

  @protected
  String dco_decode_String(dynamic raw);

  @protected
  bool dco_decode_bool(dynamic raw);

  @protected
  ConnectRequest dco_decode_box_autoadd_connect_request(dynamic raw);

  @protected
  GetInfoRequest dco_decode_box_autoadd_get_info_request(dynamic raw);

  @protected
  PrepareReceiveRequest dco_decode_box_autoadd_prepare_receive_request(dynamic raw);

  @protected
  PrepareReceiveResponse dco_decode_box_autoadd_prepare_receive_response(dynamic raw);

  @protected
  PrepareSendRequest dco_decode_box_autoadd_prepare_send_request(dynamic raw);

  @protected
  PrepareSendResponse dco_decode_box_autoadd_prepare_send_response(dynamic raw);

  @protected
  RestoreRequest dco_decode_box_autoadd_restore_request(dynamic raw);

  @protected
  ConnectRequest dco_decode_connect_request(dynamic raw);

  @protected
  GetInfoRequest dco_decode_get_info_request(dynamic raw);

  @protected
  GetInfoResponse dco_decode_get_info_response(dynamic raw);

  @protected
  int dco_decode_i_32(dynamic raw);

  @protected
  LiquidSdkError dco_decode_liquid_sdk_error(dynamic raw);

  @protected
  Uint8List dco_decode_list_prim_u_8_strict(dynamic raw);

  @protected
  Network dco_decode_network(dynamic raw);

  @protected
  String? dco_decode_opt_String(dynamic raw);

  @protected
  PaymentError dco_decode_payment_error(dynamic raw);

  @protected
  PrepareReceiveRequest dco_decode_prepare_receive_request(dynamic raw);

  @protected
  PrepareReceiveResponse dco_decode_prepare_receive_response(dynamic raw);

  @protected
  PrepareSendRequest dco_decode_prepare_send_request(dynamic raw);

  @protected
  PrepareSendResponse dco_decode_prepare_send_response(dynamic raw);

  @protected
  ReceivePaymentResponse dco_decode_receive_payment_response(dynamic raw);

  @protected
  RestoreRequest dco_decode_restore_request(dynamic raw);

  @protected
  SendPaymentResponse dco_decode_send_payment_response(dynamic raw);

  @protected
  int dco_decode_u_64(dynamic raw);

  @protected
  int dco_decode_u_8(dynamic raw);

  @protected
  void dco_decode_unit(dynamic raw);

  @protected
  int dco_decode_usize(dynamic raw);

  @protected
  BindingLiquidSdk
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk(
          SseDeserializer deserializer);

  @protected
  BindingLiquidSdk
      sse_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk(
          SseDeserializer deserializer);

  @protected
  BindingLiquidSdk sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk(
      SseDeserializer deserializer);

  @protected
  String sse_decode_String(SseDeserializer deserializer);

  @protected
  bool sse_decode_bool(SseDeserializer deserializer);

  @protected
  ConnectRequest sse_decode_box_autoadd_connect_request(SseDeserializer deserializer);

  @protected
  GetInfoRequest sse_decode_box_autoadd_get_info_request(SseDeserializer deserializer);

  @protected
  PrepareReceiveRequest sse_decode_box_autoadd_prepare_receive_request(SseDeserializer deserializer);

  @protected
  PrepareReceiveResponse sse_decode_box_autoadd_prepare_receive_response(SseDeserializer deserializer);

  @protected
  PrepareSendRequest sse_decode_box_autoadd_prepare_send_request(SseDeserializer deserializer);

  @protected
  PrepareSendResponse sse_decode_box_autoadd_prepare_send_response(SseDeserializer deserializer);

  @protected
  RestoreRequest sse_decode_box_autoadd_restore_request(SseDeserializer deserializer);

  @protected
  ConnectRequest sse_decode_connect_request(SseDeserializer deserializer);

  @protected
  GetInfoRequest sse_decode_get_info_request(SseDeserializer deserializer);

  @protected
  GetInfoResponse sse_decode_get_info_response(SseDeserializer deserializer);

  @protected
  int sse_decode_i_32(SseDeserializer deserializer);

  @protected
  LiquidSdkError sse_decode_liquid_sdk_error(SseDeserializer deserializer);

  @protected
  Uint8List sse_decode_list_prim_u_8_strict(SseDeserializer deserializer);

  @protected
  Network sse_decode_network(SseDeserializer deserializer);

  @protected
  String? sse_decode_opt_String(SseDeserializer deserializer);

  @protected
  PaymentError sse_decode_payment_error(SseDeserializer deserializer);

  @protected
  PrepareReceiveRequest sse_decode_prepare_receive_request(SseDeserializer deserializer);

  @protected
  PrepareReceiveResponse sse_decode_prepare_receive_response(SseDeserializer deserializer);

  @protected
  PrepareSendRequest sse_decode_prepare_send_request(SseDeserializer deserializer);

  @protected
  PrepareSendResponse sse_decode_prepare_send_response(SseDeserializer deserializer);

  @protected
  ReceivePaymentResponse sse_decode_receive_payment_response(SseDeserializer deserializer);

  @protected
  RestoreRequest sse_decode_restore_request(SseDeserializer deserializer);

  @protected
  SendPaymentResponse sse_decode_send_payment_response(SseDeserializer deserializer);

  @protected
  int sse_decode_u_64(SseDeserializer deserializer);

  @protected
  int sse_decode_u_8(SseDeserializer deserializer);

  @protected
  void sse_decode_unit(SseDeserializer deserializer);

  @protected
  int sse_decode_usize(SseDeserializer deserializer);

  @protected
  ffi.Pointer<wire_cst_list_prim_u_8_strict> cst_encode_String(String raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return cst_encode_list_prim_u_8_strict(utf8.encoder.convert(raw));
  }

  @protected
  ffi.Pointer<wire_cst_connect_request> cst_encode_box_autoadd_connect_request(ConnectRequest raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    final ptr = wire.cst_new_box_autoadd_connect_request();
    cst_api_fill_to_wire_connect_request(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_cst_get_info_request> cst_encode_box_autoadd_get_info_request(GetInfoRequest raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    final ptr = wire.cst_new_box_autoadd_get_info_request();
    cst_api_fill_to_wire_get_info_request(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_cst_prepare_receive_request> cst_encode_box_autoadd_prepare_receive_request(
      PrepareReceiveRequest raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    final ptr = wire.cst_new_box_autoadd_prepare_receive_request();
    cst_api_fill_to_wire_prepare_receive_request(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_cst_prepare_receive_response> cst_encode_box_autoadd_prepare_receive_response(
      PrepareReceiveResponse raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    final ptr = wire.cst_new_box_autoadd_prepare_receive_response();
    cst_api_fill_to_wire_prepare_receive_response(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_cst_prepare_send_request> cst_encode_box_autoadd_prepare_send_request(
      PrepareSendRequest raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    final ptr = wire.cst_new_box_autoadd_prepare_send_request();
    cst_api_fill_to_wire_prepare_send_request(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_cst_prepare_send_response> cst_encode_box_autoadd_prepare_send_response(
      PrepareSendResponse raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    final ptr = wire.cst_new_box_autoadd_prepare_send_response();
    cst_api_fill_to_wire_prepare_send_response(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_cst_restore_request> cst_encode_box_autoadd_restore_request(RestoreRequest raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    final ptr = wire.cst_new_box_autoadd_restore_request();
    cst_api_fill_to_wire_restore_request(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_cst_list_prim_u_8_strict> cst_encode_list_prim_u_8_strict(Uint8List raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    final ans = wire.cst_new_list_prim_u_8_strict(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

  @protected
  ffi.Pointer<wire_cst_list_prim_u_8_strict> cst_encode_opt_String(String? raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return raw == null ? ffi.nullptr : cst_encode_String(raw);
  }

  @protected
  int cst_encode_u_64(int raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return raw.toInt();
  }

  @protected
  void cst_api_fill_to_wire_box_autoadd_connect_request(
      ConnectRequest apiObj, ffi.Pointer<wire_cst_connect_request> wireObj) {
    cst_api_fill_to_wire_connect_request(apiObj, wireObj.ref);
  }

  @protected
  void cst_api_fill_to_wire_box_autoadd_get_info_request(
      GetInfoRequest apiObj, ffi.Pointer<wire_cst_get_info_request> wireObj) {
    cst_api_fill_to_wire_get_info_request(apiObj, wireObj.ref);
  }

  @protected
  void cst_api_fill_to_wire_box_autoadd_prepare_receive_request(
      PrepareReceiveRequest apiObj, ffi.Pointer<wire_cst_prepare_receive_request> wireObj) {
    cst_api_fill_to_wire_prepare_receive_request(apiObj, wireObj.ref);
  }

  @protected
  void cst_api_fill_to_wire_box_autoadd_prepare_receive_response(
      PrepareReceiveResponse apiObj, ffi.Pointer<wire_cst_prepare_receive_response> wireObj) {
    cst_api_fill_to_wire_prepare_receive_response(apiObj, wireObj.ref);
  }

  @protected
  void cst_api_fill_to_wire_box_autoadd_prepare_send_request(
      PrepareSendRequest apiObj, ffi.Pointer<wire_cst_prepare_send_request> wireObj) {
    cst_api_fill_to_wire_prepare_send_request(apiObj, wireObj.ref);
  }

  @protected
  void cst_api_fill_to_wire_box_autoadd_prepare_send_response(
      PrepareSendResponse apiObj, ffi.Pointer<wire_cst_prepare_send_response> wireObj) {
    cst_api_fill_to_wire_prepare_send_response(apiObj, wireObj.ref);
  }

  @protected
  void cst_api_fill_to_wire_box_autoadd_restore_request(
      RestoreRequest apiObj, ffi.Pointer<wire_cst_restore_request> wireObj) {
    cst_api_fill_to_wire_restore_request(apiObj, wireObj.ref);
  }

  @protected
  void cst_api_fill_to_wire_connect_request(ConnectRequest apiObj, wire_cst_connect_request wireObj) {
    wireObj.mnemonic = cst_encode_String(apiObj.mnemonic);
    wireObj.data_dir = cst_encode_opt_String(apiObj.dataDir);
    wireObj.network = cst_encode_network(apiObj.network);
  }

  @protected
  void cst_api_fill_to_wire_get_info_request(GetInfoRequest apiObj, wire_cst_get_info_request wireObj) {
    wireObj.with_scan = cst_encode_bool(apiObj.withScan);
  }

  @protected
  void cst_api_fill_to_wire_get_info_response(GetInfoResponse apiObj, wire_cst_get_info_response wireObj) {
    wireObj.balance_sat = cst_encode_u_64(apiObj.balanceSat);
    wireObj.pubkey = cst_encode_String(apiObj.pubkey);
  }

  @protected
  void cst_api_fill_to_wire_liquid_sdk_error(LiquidSdkError apiObj, wire_cst_liquid_sdk_error wireObj) {
    if (apiObj is LiquidSdkError_Generic) {
      var pre_err = cst_encode_String(apiObj.err);
      wireObj.tag = 0;
      wireObj.kind.Generic.err = pre_err;
      return;
    }
  }

  @protected
  void cst_api_fill_to_wire_payment_error(PaymentError apiObj, wire_cst_payment_error wireObj) {
    if (apiObj is PaymentError_AlreadyClaimed) {
      wireObj.tag = 0;
      return;
    }
    if (apiObj is PaymentError_AmountOutOfRange) {
      wireObj.tag = 1;
      return;
    }
    if (apiObj is PaymentError_Generic) {
      var pre_err = cst_encode_String(apiObj.err);
      wireObj.tag = 2;
      wireObj.kind.Generic.err = pre_err;
      return;
    }
    if (apiObj is PaymentError_InvalidOrExpiredFees) {
      wireObj.tag = 3;
      return;
    }
    if (apiObj is PaymentError_InsufficientFunds) {
      wireObj.tag = 4;
      return;
    }
    if (apiObj is PaymentError_InvalidInvoice) {
      wireObj.tag = 5;
      return;
    }
    if (apiObj is PaymentError_InvalidPreimage) {
      wireObj.tag = 6;
      return;
    }
    if (apiObj is PaymentError_LwkError) {
      var pre_err = cst_encode_String(apiObj.err);
      wireObj.tag = 7;
      wireObj.kind.LwkError.err = pre_err;
      return;
    }
    if (apiObj is PaymentError_PairsNotFound) {
      wireObj.tag = 8;
      return;
    }
    if (apiObj is PaymentError_PersistError) {
      wireObj.tag = 9;
      return;
    }
    if (apiObj is PaymentError_Refunded) {
      var pre_err = cst_encode_String(apiObj.err);
      var pre_txid = cst_encode_String(apiObj.txid);
      wireObj.tag = 10;
      wireObj.kind.Refunded.err = pre_err;
      wireObj.kind.Refunded.txid = pre_txid;
      return;
    }
    if (apiObj is PaymentError_SendError) {
      var pre_err = cst_encode_String(apiObj.err);
      wireObj.tag = 11;
      wireObj.kind.SendError.err = pre_err;
      return;
    }
    if (apiObj is PaymentError_SignerError) {
      var pre_err = cst_encode_String(apiObj.err);
      wireObj.tag = 12;
      wireObj.kind.SignerError.err = pre_err;
      return;
    }
  }

  @protected
  void cst_api_fill_to_wire_prepare_receive_request(
      PrepareReceiveRequest apiObj, wire_cst_prepare_receive_request wireObj) {
    wireObj.payer_amount_sat = cst_encode_u_64(apiObj.payerAmountSat);
  }

  @protected
  void cst_api_fill_to_wire_prepare_receive_response(
      PrepareReceiveResponse apiObj, wire_cst_prepare_receive_response wireObj) {
    wireObj.payer_amount_sat = cst_encode_u_64(apiObj.payerAmountSat);
    wireObj.fees_sat = cst_encode_u_64(apiObj.feesSat);
  }

  @protected
  void cst_api_fill_to_wire_prepare_send_request(
      PrepareSendRequest apiObj, wire_cst_prepare_send_request wireObj) {
    wireObj.invoice = cst_encode_String(apiObj.invoice);
  }

  @protected
  void cst_api_fill_to_wire_prepare_send_response(
      PrepareSendResponse apiObj, wire_cst_prepare_send_response wireObj) {
    wireObj.invoice = cst_encode_String(apiObj.invoice);
    wireObj.fees_sat = cst_encode_u_64(apiObj.feesSat);
  }

  @protected
  void cst_api_fill_to_wire_receive_payment_response(
      ReceivePaymentResponse apiObj, wire_cst_receive_payment_response wireObj) {
    wireObj.id = cst_encode_String(apiObj.id);
    wireObj.invoice = cst_encode_String(apiObj.invoice);
  }

  @protected
  void cst_api_fill_to_wire_restore_request(RestoreRequest apiObj, wire_cst_restore_request wireObj) {
    wireObj.backup_path = cst_encode_opt_String(apiObj.backupPath);
  }

  @protected
  void cst_api_fill_to_wire_send_payment_response(
      SendPaymentResponse apiObj, wire_cst_send_payment_response wireObj) {
    wireObj.txid = cst_encode_String(apiObj.txid);
  }

  @protected
  int cst_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk(
      BindingLiquidSdk raw);

  @protected
  int cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk(
      BindingLiquidSdk raw);

  @protected
  int cst_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk(
      BindingLiquidSdk raw);

  @protected
  bool cst_encode_bool(bool raw);

  @protected
  int cst_encode_i_32(int raw);

  @protected
  int cst_encode_network(Network raw);

  @protected
  int cst_encode_u_8(int raw);

  @protected
  void cst_encode_unit(void raw);

  @protected
  int cst_encode_usize(int raw);

  @protected
  void sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk(
      BindingLiquidSdk self, SseSerializer serializer);

  @protected
  void sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk(
      BindingLiquidSdk self, SseSerializer serializer);

  @protected
  void sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk(
      BindingLiquidSdk self, SseSerializer serializer);

  @protected
  void sse_encode_String(String self, SseSerializer serializer);

  @protected
  void sse_encode_bool(bool self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_connect_request(ConnectRequest self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_get_info_request(GetInfoRequest self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_prepare_receive_request(PrepareReceiveRequest self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_prepare_receive_response(PrepareReceiveResponse self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_prepare_send_request(PrepareSendRequest self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_prepare_send_response(PrepareSendResponse self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_restore_request(RestoreRequest self, SseSerializer serializer);

  @protected
  void sse_encode_connect_request(ConnectRequest self, SseSerializer serializer);

  @protected
  void sse_encode_get_info_request(GetInfoRequest self, SseSerializer serializer);

  @protected
  void sse_encode_get_info_response(GetInfoResponse self, SseSerializer serializer);

  @protected
  void sse_encode_i_32(int self, SseSerializer serializer);

  @protected
  void sse_encode_liquid_sdk_error(LiquidSdkError self, SseSerializer serializer);

  @protected
  void sse_encode_list_prim_u_8_strict(Uint8List self, SseSerializer serializer);

  @protected
  void sse_encode_network(Network self, SseSerializer serializer);

  @protected
  void sse_encode_opt_String(String? self, SseSerializer serializer);

  @protected
  void sse_encode_payment_error(PaymentError self, SseSerializer serializer);

  @protected
  void sse_encode_prepare_receive_request(PrepareReceiveRequest self, SseSerializer serializer);

  @protected
  void sse_encode_prepare_receive_response(PrepareReceiveResponse self, SseSerializer serializer);

  @protected
  void sse_encode_prepare_send_request(PrepareSendRequest self, SseSerializer serializer);

  @protected
  void sse_encode_prepare_send_response(PrepareSendResponse self, SseSerializer serializer);

  @protected
  void sse_encode_receive_payment_response(ReceivePaymentResponse self, SseSerializer serializer);

  @protected
  void sse_encode_restore_request(RestoreRequest self, SseSerializer serializer);

  @protected
  void sse_encode_send_payment_response(SendPaymentResponse self, SseSerializer serializer);

  @protected
  void sse_encode_u_64(int self, SseSerializer serializer);

  @protected
  void sse_encode_u_8(int self, SseSerializer serializer);

  @protected
  void sse_encode_unit(void self, SseSerializer serializer);

  @protected
  void sse_encode_usize(int self, SseSerializer serializer);
}

// Section: wire_class

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names
// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.
// ignore_for_file: type=lint

/// generated by flutter_rust_bridge
class RustLibWire implements BaseWire {
  factory RustLibWire.fromExternalLibrary(ExternalLibrary lib) => RustLibWire(lib.ffiDynamicLibrary);

  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName) _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  RustLibWire(ffi.DynamicLibrary dynamicLibrary) : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  RustLibWire.fromLookup(ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName) lookup)
      : _lookup = lookup;

  void store_dart_post_cobject(
    DartPostCObjectFnType ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(DartPostCObjectFnType)>>('store_dart_post_cobject');
  late final _store_dart_post_cobject =
      _store_dart_post_cobjectPtr.asFunction<void Function(DartPostCObjectFnType)>();

  void wire__crate__bindings__BindingLiquidSdk_backup(
    int port_,
    int that,
  ) {
    return _wire__crate__bindings__BindingLiquidSdk_backup(
      port_,
      that,
    );
  }

  late final _wire__crate__bindings__BindingLiquidSdk_backupPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.UintPtr)>>(
          'frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_backup');
  late final _wire__crate__bindings__BindingLiquidSdk_backup =
      _wire__crate__bindings__BindingLiquidSdk_backupPtr.asFunction<void Function(int, int)>();

  void wire__crate__bindings__BindingLiquidSdk_get_info(
    int port_,
    int that,
    ffi.Pointer<wire_cst_get_info_request> req,
  ) {
    return _wire__crate__bindings__BindingLiquidSdk_get_info(
      port_,
      that,
      req,
    );
  }

  late final _wire__crate__bindings__BindingLiquidSdk_get_infoPtr = _lookup<
          ffi
          .NativeFunction<ffi.Void Function(ffi.Int64, ffi.UintPtr, ffi.Pointer<wire_cst_get_info_request>)>>(
      'frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_get_info');
  late final _wire__crate__bindings__BindingLiquidSdk_get_info =
      _wire__crate__bindings__BindingLiquidSdk_get_infoPtr
          .asFunction<void Function(int, int, ffi.Pointer<wire_cst_get_info_request>)>();

  void wire__crate__bindings__BindingLiquidSdk_prepare_receive_payment(
    int port_,
    int that,
    ffi.Pointer<wire_cst_prepare_receive_request> req,
  ) {
    return _wire__crate__bindings__BindingLiquidSdk_prepare_receive_payment(
      port_,
      that,
      req,
    );
  }

  late final _wire__crate__bindings__BindingLiquidSdk_prepare_receive_paymentPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.UintPtr, ffi.Pointer<wire_cst_prepare_receive_request>)>>(
      'frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_prepare_receive_payment');
  late final _wire__crate__bindings__BindingLiquidSdk_prepare_receive_payment =
      _wire__crate__bindings__BindingLiquidSdk_prepare_receive_paymentPtr
          .asFunction<void Function(int, int, ffi.Pointer<wire_cst_prepare_receive_request>)>();

  void wire__crate__bindings__BindingLiquidSdk_prepare_send_payment(
    int port_,
    int that,
    ffi.Pointer<wire_cst_prepare_send_request> req,
  ) {
    return _wire__crate__bindings__BindingLiquidSdk_prepare_send_payment(
      port_,
      that,
      req,
    );
  }

  late final _wire__crate__bindings__BindingLiquidSdk_prepare_send_paymentPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.UintPtr, ffi.Pointer<wire_cst_prepare_send_request>)>>(
      'frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_prepare_send_payment');
  late final _wire__crate__bindings__BindingLiquidSdk_prepare_send_payment =
      _wire__crate__bindings__BindingLiquidSdk_prepare_send_paymentPtr
          .asFunction<void Function(int, int, ffi.Pointer<wire_cst_prepare_send_request>)>();

  void wire__crate__bindings__BindingLiquidSdk_receive_payment(
    int port_,
    int that,
    ffi.Pointer<wire_cst_prepare_receive_response> req,
  ) {
    return _wire__crate__bindings__BindingLiquidSdk_receive_payment(
      port_,
      that,
      req,
    );
  }

  late final _wire__crate__bindings__BindingLiquidSdk_receive_paymentPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.UintPtr, ffi.Pointer<wire_cst_prepare_receive_response>)>>(
      'frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_receive_payment');
  late final _wire__crate__bindings__BindingLiquidSdk_receive_payment =
      _wire__crate__bindings__BindingLiquidSdk_receive_paymentPtr
          .asFunction<void Function(int, int, ffi.Pointer<wire_cst_prepare_receive_response>)>();

  void wire__crate__bindings__BindingLiquidSdk_restore(
    int port_,
    int that,
    ffi.Pointer<wire_cst_restore_request> req,
  ) {
    return _wire__crate__bindings__BindingLiquidSdk_restore(
      port_,
      that,
      req,
    );
  }

  late final _wire__crate__bindings__BindingLiquidSdk_restorePtr = _lookup<
          ffi
          .NativeFunction<ffi.Void Function(ffi.Int64, ffi.UintPtr, ffi.Pointer<wire_cst_restore_request>)>>(
      'frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_restore');
  late final _wire__crate__bindings__BindingLiquidSdk_restore =
      _wire__crate__bindings__BindingLiquidSdk_restorePtr
          .asFunction<void Function(int, int, ffi.Pointer<wire_cst_restore_request>)>();

  void wire__crate__bindings__BindingLiquidSdk_send_payment(
    int port_,
    int that,
    ffi.Pointer<wire_cst_prepare_send_response> req,
  ) {
    return _wire__crate__bindings__BindingLiquidSdk_send_payment(
      port_,
      that,
      req,
    );
  }

  late final _wire__crate__bindings__BindingLiquidSdk_send_paymentPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.UintPtr, ffi.Pointer<wire_cst_prepare_send_response>)>>(
      'frbgen_breez_liquid_wire__crate__bindings__BindingLiquidSdk_send_payment');
  late final _wire__crate__bindings__BindingLiquidSdk_send_payment =
      _wire__crate__bindings__BindingLiquidSdk_send_paymentPtr
          .asFunction<void Function(int, int, ffi.Pointer<wire_cst_prepare_send_response>)>();

  void wire__crate__bindings__connect(
    int port_,
    ffi.Pointer<wire_cst_connect_request> req,
  ) {
    return _wire__crate__bindings__connect(
      port_,
      req,
    );
  }

  late final _wire__crate__bindings__connectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Pointer<wire_cst_connect_request>)>>(
          'frbgen_breez_liquid_wire__crate__bindings__connect');
  late final _wire__crate__bindings__connect = _wire__crate__bindings__connectPtr
      .asFunction<void Function(int, ffi.Pointer<wire_cst_connect_request>)>();

  void
      rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk(
      ptr,
    );
  }

  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdkPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_breez_liquid_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk');
  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk =
      _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdkPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk(
      ptr,
    );
  }

  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdkPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_breez_liquid_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk');
  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdk =
      _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBindingLiquidSdkPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  ffi.Pointer<wire_cst_connect_request> cst_new_box_autoadd_connect_request() {
    return _cst_new_box_autoadd_connect_request();
  }

  late final _cst_new_box_autoadd_connect_requestPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_cst_connect_request> Function()>>(
          'frbgen_breez_liquid_cst_new_box_autoadd_connect_request');
  late final _cst_new_box_autoadd_connect_request =
      _cst_new_box_autoadd_connect_requestPtr.asFunction<ffi.Pointer<wire_cst_connect_request> Function()>();

  ffi.Pointer<wire_cst_get_info_request> cst_new_box_autoadd_get_info_request() {
    return _cst_new_box_autoadd_get_info_request();
  }

  late final _cst_new_box_autoadd_get_info_requestPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_cst_get_info_request> Function()>>(
          'frbgen_breez_liquid_cst_new_box_autoadd_get_info_request');
  late final _cst_new_box_autoadd_get_info_request = _cst_new_box_autoadd_get_info_requestPtr
      .asFunction<ffi.Pointer<wire_cst_get_info_request> Function()>();

  ffi.Pointer<wire_cst_prepare_receive_request> cst_new_box_autoadd_prepare_receive_request() {
    return _cst_new_box_autoadd_prepare_receive_request();
  }

  late final _cst_new_box_autoadd_prepare_receive_requestPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_cst_prepare_receive_request> Function()>>(
          'frbgen_breez_liquid_cst_new_box_autoadd_prepare_receive_request');
  late final _cst_new_box_autoadd_prepare_receive_request = _cst_new_box_autoadd_prepare_receive_requestPtr
      .asFunction<ffi.Pointer<wire_cst_prepare_receive_request> Function()>();

  ffi.Pointer<wire_cst_prepare_receive_response> cst_new_box_autoadd_prepare_receive_response() {
    return _cst_new_box_autoadd_prepare_receive_response();
  }

  late final _cst_new_box_autoadd_prepare_receive_responsePtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_cst_prepare_receive_response> Function()>>(
          'frbgen_breez_liquid_cst_new_box_autoadd_prepare_receive_response');
  late final _cst_new_box_autoadd_prepare_receive_response = _cst_new_box_autoadd_prepare_receive_responsePtr
      .asFunction<ffi.Pointer<wire_cst_prepare_receive_response> Function()>();

  ffi.Pointer<wire_cst_prepare_send_request> cst_new_box_autoadd_prepare_send_request() {
    return _cst_new_box_autoadd_prepare_send_request();
  }

  late final _cst_new_box_autoadd_prepare_send_requestPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_cst_prepare_send_request> Function()>>(
          'frbgen_breez_liquid_cst_new_box_autoadd_prepare_send_request');
  late final _cst_new_box_autoadd_prepare_send_request = _cst_new_box_autoadd_prepare_send_requestPtr
      .asFunction<ffi.Pointer<wire_cst_prepare_send_request> Function()>();

  ffi.Pointer<wire_cst_prepare_send_response> cst_new_box_autoadd_prepare_send_response() {
    return _cst_new_box_autoadd_prepare_send_response();
  }

  late final _cst_new_box_autoadd_prepare_send_responsePtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_cst_prepare_send_response> Function()>>(
          'frbgen_breez_liquid_cst_new_box_autoadd_prepare_send_response');
  late final _cst_new_box_autoadd_prepare_send_response = _cst_new_box_autoadd_prepare_send_responsePtr
      .asFunction<ffi.Pointer<wire_cst_prepare_send_response> Function()>();

  ffi.Pointer<wire_cst_restore_request> cst_new_box_autoadd_restore_request() {
    return _cst_new_box_autoadd_restore_request();
  }

  late final _cst_new_box_autoadd_restore_requestPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_cst_restore_request> Function()>>(
          'frbgen_breez_liquid_cst_new_box_autoadd_restore_request');
  late final _cst_new_box_autoadd_restore_request =
      _cst_new_box_autoadd_restore_requestPtr.asFunction<ffi.Pointer<wire_cst_restore_request> Function()>();

  ffi.Pointer<wire_cst_list_prim_u_8_strict> cst_new_list_prim_u_8_strict(
    int len,
  ) {
    return _cst_new_list_prim_u_8_strict(
      len,
    );
  }

  late final _cst_new_list_prim_u_8_strictPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_cst_list_prim_u_8_strict> Function(ffi.Int32)>>(
          'frbgen_breez_liquid_cst_new_list_prim_u_8_strict');
  late final _cst_new_list_prim_u_8_strict =
      _cst_new_list_prim_u_8_strictPtr.asFunction<ffi.Pointer<wire_cst_list_prim_u_8_strict> Function(int)>();

  int dummy_method_to_enforce_bundling() {
    return _dummy_method_to_enforce_bundling();
  }

  late final _dummy_method_to_enforce_bundlingPtr =
      _lookup<ffi.NativeFunction<ffi.Int64 Function()>>('dummy_method_to_enforce_bundling');
  late final _dummy_method_to_enforce_bundling =
      _dummy_method_to_enforce_bundlingPtr.asFunction<int Function()>();
}

typedef DartPostCObjectFnType = ffi.Pointer<ffi.NativeFunction<DartPostCObjectFnTypeFunction>>;
typedef DartPostCObjectFnTypeFunction = ffi.Bool Function(DartPort port_id, ffi.Pointer<ffi.Void> message);
typedef DartDartPostCObjectFnTypeFunction = bool Function(
    DartDartPort port_id, ffi.Pointer<ffi.Void> message);
typedef DartPort = ffi.Int64;
typedef DartDartPort = int;

final class wire_cst_get_info_request extends ffi.Struct {
  @ffi.Bool()
  external bool with_scan;
}

final class wire_cst_prepare_receive_request extends ffi.Struct {
  @ffi.Uint64()
  external int payer_amount_sat;
}

final class wire_cst_list_prim_u_8_strict extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_cst_prepare_send_request extends ffi.Struct {
  external ffi.Pointer<wire_cst_list_prim_u_8_strict> invoice;
}

final class wire_cst_prepare_receive_response extends ffi.Struct {
  @ffi.Uint64()
  external int payer_amount_sat;

  @ffi.Uint64()
  external int fees_sat;
}

final class wire_cst_restore_request extends ffi.Struct {
  external ffi.Pointer<wire_cst_list_prim_u_8_strict> backup_path;
}

final class wire_cst_prepare_send_response extends ffi.Struct {
  external ffi.Pointer<wire_cst_list_prim_u_8_strict> invoice;

  @ffi.Uint64()
  external int fees_sat;
}

final class wire_cst_connect_request extends ffi.Struct {
  external ffi.Pointer<wire_cst_list_prim_u_8_strict> mnemonic;

  external ffi.Pointer<wire_cst_list_prim_u_8_strict> data_dir;

  @ffi.Int32()
  external int network;
}

final class wire_cst_get_info_response extends ffi.Struct {
  @ffi.Uint64()
  external int balance_sat;

  external ffi.Pointer<wire_cst_list_prim_u_8_strict> pubkey;
}

final class wire_cst_LiquidSdkError_Generic extends ffi.Struct {
  external ffi.Pointer<wire_cst_list_prim_u_8_strict> err;
}

final class LiquidSdkErrorKind extends ffi.Union {
  external wire_cst_LiquidSdkError_Generic Generic;
}

final class wire_cst_liquid_sdk_error extends ffi.Struct {
  @ffi.Int32()
  external int tag;

  external LiquidSdkErrorKind kind;
}

final class wire_cst_PaymentError_Generic extends ffi.Struct {
  external ffi.Pointer<wire_cst_list_prim_u_8_strict> err;
}

final class wire_cst_PaymentError_LwkError extends ffi.Struct {
  external ffi.Pointer<wire_cst_list_prim_u_8_strict> err;
}

final class wire_cst_PaymentError_Refunded extends ffi.Struct {
  external ffi.Pointer<wire_cst_list_prim_u_8_strict> err;

  external ffi.Pointer<wire_cst_list_prim_u_8_strict> txid;
}

final class wire_cst_PaymentError_SendError extends ffi.Struct {
  external ffi.Pointer<wire_cst_list_prim_u_8_strict> err;
}

final class wire_cst_PaymentError_SignerError extends ffi.Struct {
  external ffi.Pointer<wire_cst_list_prim_u_8_strict> err;
}

final class PaymentErrorKind extends ffi.Union {
  external wire_cst_PaymentError_Generic Generic;

  external wire_cst_PaymentError_LwkError LwkError;

  external wire_cst_PaymentError_Refunded Refunded;

  external wire_cst_PaymentError_SendError SendError;

  external wire_cst_PaymentError_SignerError SignerError;
}

final class wire_cst_payment_error extends ffi.Struct {
  @ffi.Int32()
  external int tag;

  external PaymentErrorKind kind;
}

final class wire_cst_receive_payment_response extends ffi.Struct {
  external ffi.Pointer<wire_cst_list_prim_u_8_strict> id;

  external ffi.Pointer<wire_cst_list_prim_u_8_strict> invoice;
}

final class wire_cst_send_payment_response extends ffi.Struct {
  external ffi.Pointer<wire_cst_list_prim_u_8_strict> txid;
}

const double LIQUID_CLAIM_TX_FEERATE_MSAT = 100.0;
