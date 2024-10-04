// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.4.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'bindings/duplicates.dart';
import 'error.dart';
import 'frb_generated.dart';
import 'model.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'bindings.freezed.dart';

// These functions are ignored because they are not marked as `pub`: `init`
// These types are ignored because they are not used by any `pub` functions: `DartBindingLogger`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `enabled`, `flush`, `log`

Future<BindingLiquidSdk> connect({required ConnectRequest req}) =>
    RustLib.instance.api.crateBindingsConnect(req: req);

/// If used, this must be called before `connect`. It can only be called once.
Stream<LogEntry> breezLogStream() => RustLib.instance.api.crateBindingsBreezLogStream();

Config defaultConfig({required LiquidNetwork network, required String breezApiKey}) =>
    RustLib.instance.api.crateBindingsDefaultConfig(network: network, breezApiKey: breezApiKey);

Future<InputType> parse({required String input}) => RustLib.instance.api.crateBindingsParse(input: input);

LNInvoice parseInvoice({required String input}) =>
    RustLib.instance.api.crateBindingsParseInvoice(input: input);

// Rust type: RustOpaqueNom<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<BindingLiquidSdk>>
abstract class BindingLiquidSdk implements RustOpaqueInterface {
  Stream<SdkEvent> addEventListener();

  void backup({required BackupRequest req});

  Future<String> buyBitcoin({required BuyBitcoinRequest req});

  CheckMessageResponse checkMessage({required CheckMessageRequest req});

  Future<void> disconnect();

  void emptyWalletCache();

  Future<List<Rate>> fetchFiatRates();

  Future<LightningPaymentLimitsResponse> fetchLightningLimits();

  Future<OnchainPaymentLimitsResponse> fetchOnchainLimits();

  Future<GetInfoResponse> getInfo();

  Future<List<FiatCurrency>> listFiatCurrencies();

  Future<List<Payment>> listPayments({required ListPaymentsRequest req});

  Future<List<RefundableSwap>> listRefundables();

  Future<LnUrlCallbackStatus> lnurlAuth({required LnUrlAuthRequestData reqData});

  Future<LnUrlPayResult> lnurlPay({required LnUrlPayRequest req});

  Future<LnUrlWithdrawResult> lnurlWithdraw({required LnUrlWithdrawRequest req});

  Future<SendPaymentResponse> payOnchain({required PayOnchainRequest req});

  Future<PrepareBuyBitcoinResponse> prepareBuyBitcoin({required PrepareBuyBitcoinRequest req});

  Future<PreparePayOnchainResponse> preparePayOnchain({required PreparePayOnchainRequest req});

  Future<PrepareReceiveResponse> prepareReceivePayment({required PrepareReceiveRequest req});

  Future<PrepareRefundResponse> prepareRefund({required PrepareRefundRequest req});

  Future<PrepareSendResponse> prepareSendPayment({required PrepareSendRequest req});

  Future<ReceivePaymentResponse> receivePayment({required ReceivePaymentRequest req});

  Future<RecommendedFees> recommendedFees();

  Future<RefundResponse> refund({required RefundRequest req});

  Future<void> registerWebhook({required String webhookUrl});

  Future<void> rescanOnchainSwaps();

  void restore({required RestoreRequest req});

  Future<SendPaymentResponse> sendPayment({required SendPaymentRequest req});

  SignMessageResponse signMessage({required SignMessageRequest req});

  Future<void> sync();

  Future<void> unregisterWebhook();
}

class AesSuccessActionDataDecrypted {
  final String description;
  final String plaintext;

  const AesSuccessActionDataDecrypted({
    required this.description,
    required this.plaintext,
  });

  @override
  int get hashCode => description.hashCode ^ plaintext.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is AesSuccessActionDataDecrypted &&
          runtimeType == other.runtimeType &&
          description == other.description &&
          plaintext == other.plaintext;
}

@freezed
sealed class AesSuccessActionDataResult with _$AesSuccessActionDataResult {
  const AesSuccessActionDataResult._();

  const factory AesSuccessActionDataResult.decrypted({
    required AesSuccessActionDataDecrypted data,
  }) = AesSuccessActionDataResult_Decrypted;
  const factory AesSuccessActionDataResult.errorStatus({
    required String reason,
  }) = AesSuccessActionDataResult_ErrorStatus;
}

class BindingEventListener {
  final RustStreamSink<SdkEvent> stream;

  const BindingEventListener({
    required this.stream,
  });

  Future<void> onEvent({required SdkEvent e}) =>
      RustLib.instance.api.crateBindingsBindingEventListenerOnEvent(that: this, e: e);

  @override
  int get hashCode => stream.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is BindingEventListener && runtimeType == other.runtimeType && stream == other.stream;
}

class BitcoinAddressData {
  final String address;
  final Network network;
  final BigInt? amountSat;
  final String? label;
  final String? message;

  const BitcoinAddressData({
    required this.address,
    required this.network,
    this.amountSat,
    this.label,
    this.message,
  });

  @override
  int get hashCode =>
      address.hashCode ^ network.hashCode ^ amountSat.hashCode ^ label.hashCode ^ message.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is BitcoinAddressData &&
          runtimeType == other.runtimeType &&
          address == other.address &&
          network == other.network &&
          amountSat == other.amountSat &&
          label == other.label &&
          message == other.message;
}

class CurrencyInfo {
  final String name;
  final int fractionSize;
  final int? spacing;
  final Symbol? symbol;
  final Symbol? uniqSymbol;
  final List<LocalizedName> localizedName;
  final List<LocaleOverrides> localeOverrides;

  const CurrencyInfo({
    required this.name,
    required this.fractionSize,
    this.spacing,
    this.symbol,
    this.uniqSymbol,
    required this.localizedName,
    required this.localeOverrides,
  });

  @override
  int get hashCode =>
      name.hashCode ^
      fractionSize.hashCode ^
      spacing.hashCode ^
      symbol.hashCode ^
      uniqSymbol.hashCode ^
      localizedName.hashCode ^
      localeOverrides.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CurrencyInfo &&
          runtimeType == other.runtimeType &&
          name == other.name &&
          fractionSize == other.fractionSize &&
          spacing == other.spacing &&
          symbol == other.symbol &&
          uniqSymbol == other.uniqSymbol &&
          localizedName == other.localizedName &&
          localeOverrides == other.localeOverrides;
}

class FiatCurrency {
  final String id;
  final CurrencyInfo info;

  const FiatCurrency({
    required this.id,
    required this.info,
  });

  @override
  int get hashCode => id.hashCode ^ info.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is FiatCurrency && runtimeType == other.runtimeType && id == other.id && info == other.info;
}

@freezed
sealed class InputType with _$InputType {
  const InputType._();

  const factory InputType.bitcoinAddress({
    required BitcoinAddressData address,
  }) = InputType_BitcoinAddress;
  const factory InputType.liquidAddress({
    required LiquidAddressData address,
  }) = InputType_LiquidAddress;
  const factory InputType.bolt11({
    required LNInvoice invoice,
  }) = InputType_Bolt11;
  const factory InputType.nodeId({
    required String nodeId,
  }) = InputType_NodeId;
  const factory InputType.url({
    required String url,
  }) = InputType_Url;
  const factory InputType.lnUrlPay({
    required LnUrlPayRequestData data,
  }) = InputType_LnUrlPay;
  const factory InputType.lnUrlWithdraw({
    required LnUrlWithdrawRequestData data,
  }) = InputType_LnUrlWithdraw;
  const factory InputType.lnUrlAuth({
    required LnUrlAuthRequestData data,
  }) = InputType_LnUrlAuth;
  const factory InputType.lnUrlError({
    required LnUrlErrorData data,
  }) = InputType_LnUrlError;
}

class LiquidAddressData {
  final String address;
  final Network network;
  final String? assetId;
  final BigInt? amountSat;
  final String? label;
  final String? message;

  const LiquidAddressData({
    required this.address,
    required this.network,
    this.assetId,
    this.amountSat,
    this.label,
    this.message,
  });

  @override
  int get hashCode =>
      address.hashCode ^
      network.hashCode ^
      assetId.hashCode ^
      amountSat.hashCode ^
      label.hashCode ^
      message.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LiquidAddressData &&
          runtimeType == other.runtimeType &&
          address == other.address &&
          network == other.network &&
          assetId == other.assetId &&
          amountSat == other.amountSat &&
          label == other.label &&
          message == other.message;
}

class LNInvoice {
  final String bolt11;
  final Network network;
  final String payeePubkey;
  final String paymentHash;
  final String? description;
  final String? descriptionHash;
  final BigInt? amountMsat;
  final BigInt timestamp;
  final BigInt expiry;
  final List<RouteHint> routingHints;
  final Uint8List paymentSecret;
  final BigInt minFinalCltvExpiryDelta;

  const LNInvoice({
    required this.bolt11,
    required this.network,
    required this.payeePubkey,
    required this.paymentHash,
    this.description,
    this.descriptionHash,
    this.amountMsat,
    required this.timestamp,
    required this.expiry,
    required this.routingHints,
    required this.paymentSecret,
    required this.minFinalCltvExpiryDelta,
  });

  @override
  int get hashCode =>
      bolt11.hashCode ^
      network.hashCode ^
      payeePubkey.hashCode ^
      paymentHash.hashCode ^
      description.hashCode ^
      descriptionHash.hashCode ^
      amountMsat.hashCode ^
      timestamp.hashCode ^
      expiry.hashCode ^
      routingHints.hashCode ^
      paymentSecret.hashCode ^
      minFinalCltvExpiryDelta.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LNInvoice &&
          runtimeType == other.runtimeType &&
          bolt11 == other.bolt11 &&
          network == other.network &&
          payeePubkey == other.payeePubkey &&
          paymentHash == other.paymentHash &&
          description == other.description &&
          descriptionHash == other.descriptionHash &&
          amountMsat == other.amountMsat &&
          timestamp == other.timestamp &&
          expiry == other.expiry &&
          routingHints == other.routingHints &&
          paymentSecret == other.paymentSecret &&
          minFinalCltvExpiryDelta == other.minFinalCltvExpiryDelta;
}

class LnUrlAuthRequestData {
  final String k1;
  final String? action;
  final String domain;
  final String url;

  const LnUrlAuthRequestData({
    required this.k1,
    this.action,
    required this.domain,
    required this.url,
  });

  @override
  int get hashCode => k1.hashCode ^ action.hashCode ^ domain.hashCode ^ url.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LnUrlAuthRequestData &&
          runtimeType == other.runtimeType &&
          k1 == other.k1 &&
          action == other.action &&
          domain == other.domain &&
          url == other.url;
}

class LnUrlErrorData {
  final String reason;

  const LnUrlErrorData({
    required this.reason,
  });

  @override
  int get hashCode => reason.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LnUrlErrorData && runtimeType == other.runtimeType && reason == other.reason;
}

class LnUrlPayErrorData {
  final String paymentHash;
  final String reason;

  const LnUrlPayErrorData({
    required this.paymentHash,
    required this.reason,
  });

  @override
  int get hashCode => paymentHash.hashCode ^ reason.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LnUrlPayErrorData &&
          runtimeType == other.runtimeType &&
          paymentHash == other.paymentHash &&
          reason == other.reason;
}

class LnUrlPayRequest {
  final LnUrlPayRequestData data;
  final BigInt amountMsat;
  final String? comment;
  final String? paymentLabel;
  final bool? validateSuccessActionUrl;

  const LnUrlPayRequest({
    required this.data,
    required this.amountMsat,
    this.comment,
    this.paymentLabel,
    this.validateSuccessActionUrl,
  });

  @override
  int get hashCode =>
      data.hashCode ^
      amountMsat.hashCode ^
      comment.hashCode ^
      paymentLabel.hashCode ^
      validateSuccessActionUrl.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LnUrlPayRequest &&
          runtimeType == other.runtimeType &&
          data == other.data &&
          amountMsat == other.amountMsat &&
          comment == other.comment &&
          paymentLabel == other.paymentLabel &&
          validateSuccessActionUrl == other.validateSuccessActionUrl;
}

class LnUrlPayRequestData {
  final String callback;
  final BigInt minSendable;
  final BigInt maxSendable;
  final String metadataStr;
  final int commentAllowed;
  final String domain;
  final bool allowsNostr;
  final String? nostrPubkey;
  final String? lnAddress;

  const LnUrlPayRequestData({
    required this.callback,
    required this.minSendable,
    required this.maxSendable,
    required this.metadataStr,
    required this.commentAllowed,
    required this.domain,
    required this.allowsNostr,
    this.nostrPubkey,
    this.lnAddress,
  });

  @override
  int get hashCode =>
      callback.hashCode ^
      minSendable.hashCode ^
      maxSendable.hashCode ^
      metadataStr.hashCode ^
      commentAllowed.hashCode ^
      domain.hashCode ^
      allowsNostr.hashCode ^
      nostrPubkey.hashCode ^
      lnAddress.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LnUrlPayRequestData &&
          runtimeType == other.runtimeType &&
          callback == other.callback &&
          minSendable == other.minSendable &&
          maxSendable == other.maxSendable &&
          metadataStr == other.metadataStr &&
          commentAllowed == other.commentAllowed &&
          domain == other.domain &&
          allowsNostr == other.allowsNostr &&
          nostrPubkey == other.nostrPubkey &&
          lnAddress == other.lnAddress;
}

class LnUrlWithdrawRequest {
  final LnUrlWithdrawRequestData data;
  final BigInt amountMsat;
  final String? description;

  const LnUrlWithdrawRequest({
    required this.data,
    required this.amountMsat,
    this.description,
  });

  @override
  int get hashCode => data.hashCode ^ amountMsat.hashCode ^ description.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LnUrlWithdrawRequest &&
          runtimeType == other.runtimeType &&
          data == other.data &&
          amountMsat == other.amountMsat &&
          description == other.description;
}

class LnUrlWithdrawRequestData {
  final String callback;
  final String k1;
  final String defaultDescription;
  final BigInt minWithdrawable;
  final BigInt maxWithdrawable;

  const LnUrlWithdrawRequestData({
    required this.callback,
    required this.k1,
    required this.defaultDescription,
    required this.minWithdrawable,
    required this.maxWithdrawable,
  });

  @override
  int get hashCode =>
      callback.hashCode ^
      k1.hashCode ^
      defaultDescription.hashCode ^
      minWithdrawable.hashCode ^
      maxWithdrawable.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LnUrlWithdrawRequestData &&
          runtimeType == other.runtimeType &&
          callback == other.callback &&
          k1 == other.k1 &&
          defaultDescription == other.defaultDescription &&
          minWithdrawable == other.minWithdrawable &&
          maxWithdrawable == other.maxWithdrawable;
}

class LocaleOverrides {
  final String locale;
  final int? spacing;
  final Symbol symbol;

  const LocaleOverrides({
    required this.locale,
    this.spacing,
    required this.symbol,
  });

  @override
  int get hashCode => locale.hashCode ^ spacing.hashCode ^ symbol.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LocaleOverrides &&
          runtimeType == other.runtimeType &&
          locale == other.locale &&
          spacing == other.spacing &&
          symbol == other.symbol;
}

class LocalizedName {
  final String locale;
  final String name;

  const LocalizedName({
    required this.locale,
    required this.name,
  });

  @override
  int get hashCode => locale.hashCode ^ name.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LocalizedName &&
          runtimeType == other.runtimeType &&
          locale == other.locale &&
          name == other.name;
}

class MessageSuccessActionData {
  final String message;

  const MessageSuccessActionData({
    required this.message,
  });

  @override
  int get hashCode => message.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MessageSuccessActionData && runtimeType == other.runtimeType && message == other.message;
}

enum Network {
  bitcoin,
  testnet,
  signet,
  regtest,
  ;
}

class Rate {
  final String coin;
  final double value;

  const Rate({
    required this.coin,
    required this.value,
  });

  @override
  int get hashCode => coin.hashCode ^ value.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Rate && runtimeType == other.runtimeType && coin == other.coin && value == other.value;
}

class RouteHint {
  final List<RouteHintHop> hops;

  const RouteHint({
    required this.hops,
  });

  @override
  int get hashCode => hops.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) || other is RouteHint && runtimeType == other.runtimeType && hops == other.hops;
}

class RouteHintHop {
  final String srcNodeId;
  final String shortChannelId;
  final int feesBaseMsat;
  final int feesProportionalMillionths;
  final BigInt cltvExpiryDelta;
  final BigInt? htlcMinimumMsat;
  final BigInt? htlcMaximumMsat;

  const RouteHintHop({
    required this.srcNodeId,
    required this.shortChannelId,
    required this.feesBaseMsat,
    required this.feesProportionalMillionths,
    required this.cltvExpiryDelta,
    this.htlcMinimumMsat,
    this.htlcMaximumMsat,
  });

  @override
  int get hashCode =>
      srcNodeId.hashCode ^
      shortChannelId.hashCode ^
      feesBaseMsat.hashCode ^
      feesProportionalMillionths.hashCode ^
      cltvExpiryDelta.hashCode ^
      htlcMinimumMsat.hashCode ^
      htlcMaximumMsat.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is RouteHintHop &&
          runtimeType == other.runtimeType &&
          srcNodeId == other.srcNodeId &&
          shortChannelId == other.shortChannelId &&
          feesBaseMsat == other.feesBaseMsat &&
          feesProportionalMillionths == other.feesProportionalMillionths &&
          cltvExpiryDelta == other.cltvExpiryDelta &&
          htlcMinimumMsat == other.htlcMinimumMsat &&
          htlcMaximumMsat == other.htlcMaximumMsat;
}

@freezed
sealed class SuccessActionProcessed with _$SuccessActionProcessed {
  const SuccessActionProcessed._();

  const factory SuccessActionProcessed.aes({
    required AesSuccessActionDataResult result,
  }) = SuccessActionProcessed_Aes;
  const factory SuccessActionProcessed.message({
    required MessageSuccessActionData data,
  }) = SuccessActionProcessed_Message;
  const factory SuccessActionProcessed.url({
    required UrlSuccessActionData data,
  }) = SuccessActionProcessed_Url;
}

class Symbol {
  final String? grapheme;
  final String? template;
  final bool? rtl;
  final int? position;

  const Symbol({
    this.grapheme,
    this.template,
    this.rtl,
    this.position,
  });

  @override
  int get hashCode => grapheme.hashCode ^ template.hashCode ^ rtl.hashCode ^ position.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Symbol &&
          runtimeType == other.runtimeType &&
          grapheme == other.grapheme &&
          template == other.template &&
          rtl == other.rtl &&
          position == other.position;
}

class UrlSuccessActionData {
  final String description;
  final String url;
  final bool matchesCallbackDomain;

  const UrlSuccessActionData({
    required this.description,
    required this.url,
    required this.matchesCallbackDomain,
  });

  @override
  int get hashCode => description.hashCode ^ url.hashCode ^ matchesCallbackDomain.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is UrlSuccessActionData &&
          runtimeType == other.runtimeType &&
          description == other.description &&
          url == other.url &&
          matchesCallbackDomain == other.matchesCallbackDomain;
}
