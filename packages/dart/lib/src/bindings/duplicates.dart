// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.3.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../bindings.dart';
import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'duplicates.freezed.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `clone`, `clone`, `clone`, `clone`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`

@freezed
sealed class LnUrlAuthError with _$LnUrlAuthError implements FrbException {
  const LnUrlAuthError._();

  /// This error is raised when a general error occurs not specific to other error variants
  /// in this enum.
  const factory LnUrlAuthError.generic({
    required String err,
  }) = LnUrlAuthError_Generic;

  /// This error is raised when the decoded LNURL URI is not compliant to the specification.
  const factory LnUrlAuthError.invalidUri({
    required String err,
  }) = LnUrlAuthError_InvalidUri;

  /// This error is raised when a connection to an external service fails.
  const factory LnUrlAuthError.serviceConnectivity({
    required String err,
  }) = LnUrlAuthError_ServiceConnectivity;
}

@freezed
sealed class LnUrlCallbackStatus with _$LnUrlCallbackStatus {
  const LnUrlCallbackStatus._();

  /// On-wire format is: `{"status": "OK"}`
  const factory LnUrlCallbackStatus.ok() = LnUrlCallbackStatus_Ok;

  /// On-wire format is: `{"status": "ERROR", "reason": "error details..."}`
  const factory LnUrlCallbackStatus.errorStatus({
    required LnUrlErrorData data,
  }) = LnUrlCallbackStatus_ErrorStatus;
}

@freezed
sealed class LnUrlPayError with _$LnUrlPayError implements FrbException {
  const LnUrlPayError._();

  /// This error is raised when attempting to pay an invoice that has already being paid.
  const factory LnUrlPayError.alreadyPaid() = LnUrlPayError_AlreadyPaid;

  /// This error is raised when a general error occurs not specific to other error variants
  /// in this enum.
  const factory LnUrlPayError.generic({
    required String err,
  }) = LnUrlPayError_Generic;

  /// This error is raised when the amount from the parsed invoice is not set.
  const factory LnUrlPayError.invalidAmount({
    required String err,
  }) = LnUrlPayError_InvalidAmount;

  /// This error is raised when the lightning invoice cannot be parsed.
  const factory LnUrlPayError.invalidInvoice({
    required String err,
  }) = LnUrlPayError_InvalidInvoice;

  /// This error is raised when the lightning invoice is for a different Bitcoin network.
  const factory LnUrlPayError.invalidNetwork({
    required String err,
  }) = LnUrlPayError_InvalidNetwork;

  /// This error is raised when the decoded LNURL URI is not compliant to the specification.
  const factory LnUrlPayError.invalidUri({
    required String err,
  }) = LnUrlPayError_InvalidUri;

  /// This error is raised when the lightning invoice has passed it's expiry time.
  const factory LnUrlPayError.invoiceExpired({
    required String err,
  }) = LnUrlPayError_InvoiceExpired;

  /// This error is raised when attempting to make a payment by the node fails.
  const factory LnUrlPayError.paymentFailed({
    required String err,
  }) = LnUrlPayError_PaymentFailed;

  /// This error is raised when attempting to make a payment takes too long.
  const factory LnUrlPayError.paymentTimeout({
    required String err,
  }) = LnUrlPayError_PaymentTimeout;

  /// This error is raised when no route can be found when attempting to make a
  /// payment by the node.
  const factory LnUrlPayError.routeNotFound({
    required String err,
  }) = LnUrlPayError_RouteNotFound;

  /// This error is raised when the route is considered too expensive when
  /// attempting to make a payment by the node.
  const factory LnUrlPayError.routeTooExpensive({
    required String err,
  }) = LnUrlPayError_RouteTooExpensive;

  /// This error is raised when a connection to an external service fails.
  const factory LnUrlPayError.serviceConnectivity({
    required String err,
  }) = LnUrlPayError_ServiceConnectivity;
}

@freezed
sealed class LnUrlWithdrawError with _$LnUrlWithdrawError implements FrbException {
  const LnUrlWithdrawError._();

  /// This error is raised when a general error occurs not specific to other error variants
  /// in this enum.
  const factory LnUrlWithdrawError.generic({
    required String err,
  }) = LnUrlWithdrawError_Generic;

  /// This error is raised when the amount is zero or the amount does not cover
  /// the cost to open a new channel.
  const factory LnUrlWithdrawError.invalidAmount({
    required String err,
  }) = LnUrlWithdrawError_InvalidAmount;

  /// This error is raised when the lightning invoice cannot be parsed.
  const factory LnUrlWithdrawError.invalidInvoice({
    required String err,
  }) = LnUrlWithdrawError_InvalidInvoice;

  /// This error is raised when the decoded LNURL URI is not compliant to the specification.
  const factory LnUrlWithdrawError.invalidUri({
    required String err,
  }) = LnUrlWithdrawError_InvalidUri;

  /// This error is raised when no routing hints were able to be added to the invoice
  /// while trying to receive a payment.
  const factory LnUrlWithdrawError.invoiceNoRoutingHints({
    required String err,
  }) = LnUrlWithdrawError_InvoiceNoRoutingHints;

  /// This error is raised when a connection to an external service fails.
  const factory LnUrlWithdrawError.serviceConnectivity({
    required String err,
  }) = LnUrlWithdrawError_ServiceConnectivity;
}

@freezed
sealed class LnUrlWithdrawResult with _$LnUrlWithdrawResult {
  const LnUrlWithdrawResult._();

  const factory LnUrlWithdrawResult.ok({
    required LnUrlWithdrawSuccessData data,
  }) = LnUrlWithdrawResult_Ok;
  const factory LnUrlWithdrawResult.timeout({
    required LnUrlWithdrawSuccessData data,
  }) = LnUrlWithdrawResult_Timeout;
  const factory LnUrlWithdrawResult.errorStatus({
    required LnUrlErrorData data,
  }) = LnUrlWithdrawResult_ErrorStatus;
}

class LnUrlWithdrawSuccessData {
  final LNInvoice invoice;

  const LnUrlWithdrawSuccessData({
    required this.invoice,
  });

  @override
  int get hashCode => invoice.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LnUrlWithdrawSuccessData && runtimeType == other.runtimeType && invoice == other.invoice;
}
