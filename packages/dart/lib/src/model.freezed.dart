// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'model.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$BlockchainExplorer {

 String get url;
/// Create a copy of BlockchainExplorer
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$BlockchainExplorerCopyWith<BlockchainExplorer> get copyWith => _$BlockchainExplorerCopyWithImpl<BlockchainExplorer>(this as BlockchainExplorer, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is BlockchainExplorer&&(identical(other.url, url) || other.url == url));
}


@override
int get hashCode => Object.hash(runtimeType,url);

@override
String toString() {
  return 'BlockchainExplorer(url: $url)';
}


}

/// @nodoc
abstract mixin class $BlockchainExplorerCopyWith<$Res>  {
  factory $BlockchainExplorerCopyWith(BlockchainExplorer value, $Res Function(BlockchainExplorer) _then) = _$BlockchainExplorerCopyWithImpl;
@useResult
$Res call({
 String url
});




}
/// @nodoc
class _$BlockchainExplorerCopyWithImpl<$Res>
    implements $BlockchainExplorerCopyWith<$Res> {
  _$BlockchainExplorerCopyWithImpl(this._self, this._then);

  final BlockchainExplorer _self;
  final $Res Function(BlockchainExplorer) _then;

/// Create a copy of BlockchainExplorer
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? url = null,}) {
  return _then(_self.copyWith(
url: null == url ? _self.url : url // ignore: cast_nullable_to_non_nullable
as String,
  ));
}

}



/// @nodoc


class BlockchainExplorer_Electrum extends BlockchainExplorer {
  const BlockchainExplorer_Electrum({required this.url}): super._();
  

@override final  String url;

/// Create a copy of BlockchainExplorer
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$BlockchainExplorer_ElectrumCopyWith<BlockchainExplorer_Electrum> get copyWith => _$BlockchainExplorer_ElectrumCopyWithImpl<BlockchainExplorer_Electrum>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is BlockchainExplorer_Electrum&&(identical(other.url, url) || other.url == url));
}


@override
int get hashCode => Object.hash(runtimeType,url);

@override
String toString() {
  return 'BlockchainExplorer.electrum(url: $url)';
}


}

/// @nodoc
abstract mixin class $BlockchainExplorer_ElectrumCopyWith<$Res> implements $BlockchainExplorerCopyWith<$Res> {
  factory $BlockchainExplorer_ElectrumCopyWith(BlockchainExplorer_Electrum value, $Res Function(BlockchainExplorer_Electrum) _then) = _$BlockchainExplorer_ElectrumCopyWithImpl;
@override @useResult
$Res call({
 String url
});




}
/// @nodoc
class _$BlockchainExplorer_ElectrumCopyWithImpl<$Res>
    implements $BlockchainExplorer_ElectrumCopyWith<$Res> {
  _$BlockchainExplorer_ElectrumCopyWithImpl(this._self, this._then);

  final BlockchainExplorer_Electrum _self;
  final $Res Function(BlockchainExplorer_Electrum) _then;

/// Create a copy of BlockchainExplorer
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? url = null,}) {
  return _then(BlockchainExplorer_Electrum(
url: null == url ? _self.url : url // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class BlockchainExplorer_Esplora extends BlockchainExplorer {
  const BlockchainExplorer_Esplora({required this.url, required this.useWaterfalls}): super._();
  

@override final  String url;
/// Whether or not to use the "waterfalls" extension
 final  bool useWaterfalls;

/// Create a copy of BlockchainExplorer
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$BlockchainExplorer_EsploraCopyWith<BlockchainExplorer_Esplora> get copyWith => _$BlockchainExplorer_EsploraCopyWithImpl<BlockchainExplorer_Esplora>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is BlockchainExplorer_Esplora&&(identical(other.url, url) || other.url == url)&&(identical(other.useWaterfalls, useWaterfalls) || other.useWaterfalls == useWaterfalls));
}


@override
int get hashCode => Object.hash(runtimeType,url,useWaterfalls);

@override
String toString() {
  return 'BlockchainExplorer.esplora(url: $url, useWaterfalls: $useWaterfalls)';
}


}

/// @nodoc
abstract mixin class $BlockchainExplorer_EsploraCopyWith<$Res> implements $BlockchainExplorerCopyWith<$Res> {
  factory $BlockchainExplorer_EsploraCopyWith(BlockchainExplorer_Esplora value, $Res Function(BlockchainExplorer_Esplora) _then) = _$BlockchainExplorer_EsploraCopyWithImpl;
@override @useResult
$Res call({
 String url, bool useWaterfalls
});




}
/// @nodoc
class _$BlockchainExplorer_EsploraCopyWithImpl<$Res>
    implements $BlockchainExplorer_EsploraCopyWith<$Res> {
  _$BlockchainExplorer_EsploraCopyWithImpl(this._self, this._then);

  final BlockchainExplorer_Esplora _self;
  final $Res Function(BlockchainExplorer_Esplora) _then;

/// Create a copy of BlockchainExplorer
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? url = null,Object? useWaterfalls = null,}) {
  return _then(BlockchainExplorer_Esplora(
url: null == url ? _self.url : url // ignore: cast_nullable_to_non_nullable
as String,useWaterfalls: null == useWaterfalls ? _self.useWaterfalls : useWaterfalls // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}


}

/// @nodoc
mixin _$GetPaymentRequest {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GetPaymentRequest);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'GetPaymentRequest()';
}


}

/// @nodoc
class $GetPaymentRequestCopyWith<$Res>  {
$GetPaymentRequestCopyWith(GetPaymentRequest _, $Res Function(GetPaymentRequest) __);
}



/// @nodoc


class GetPaymentRequest_PaymentHash extends GetPaymentRequest {
  const GetPaymentRequest_PaymentHash({required this.paymentHash}): super._();
  

 final  String paymentHash;

/// Create a copy of GetPaymentRequest
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GetPaymentRequest_PaymentHashCopyWith<GetPaymentRequest_PaymentHash> get copyWith => _$GetPaymentRequest_PaymentHashCopyWithImpl<GetPaymentRequest_PaymentHash>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GetPaymentRequest_PaymentHash&&(identical(other.paymentHash, paymentHash) || other.paymentHash == paymentHash));
}


@override
int get hashCode => Object.hash(runtimeType,paymentHash);

@override
String toString() {
  return 'GetPaymentRequest.paymentHash(paymentHash: $paymentHash)';
}


}

/// @nodoc
abstract mixin class $GetPaymentRequest_PaymentHashCopyWith<$Res> implements $GetPaymentRequestCopyWith<$Res> {
  factory $GetPaymentRequest_PaymentHashCopyWith(GetPaymentRequest_PaymentHash value, $Res Function(GetPaymentRequest_PaymentHash) _then) = _$GetPaymentRequest_PaymentHashCopyWithImpl;
@useResult
$Res call({
 String paymentHash
});




}
/// @nodoc
class _$GetPaymentRequest_PaymentHashCopyWithImpl<$Res>
    implements $GetPaymentRequest_PaymentHashCopyWith<$Res> {
  _$GetPaymentRequest_PaymentHashCopyWithImpl(this._self, this._then);

  final GetPaymentRequest_PaymentHash _self;
  final $Res Function(GetPaymentRequest_PaymentHash) _then;

/// Create a copy of GetPaymentRequest
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? paymentHash = null,}) {
  return _then(GetPaymentRequest_PaymentHash(
paymentHash: null == paymentHash ? _self.paymentHash : paymentHash // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class GetPaymentRequest_SwapId extends GetPaymentRequest {
  const GetPaymentRequest_SwapId({required this.swapId}): super._();
  

 final  String swapId;

/// Create a copy of GetPaymentRequest
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GetPaymentRequest_SwapIdCopyWith<GetPaymentRequest_SwapId> get copyWith => _$GetPaymentRequest_SwapIdCopyWithImpl<GetPaymentRequest_SwapId>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GetPaymentRequest_SwapId&&(identical(other.swapId, swapId) || other.swapId == swapId));
}


@override
int get hashCode => Object.hash(runtimeType,swapId);

@override
String toString() {
  return 'GetPaymentRequest.swapId(swapId: $swapId)';
}


}

/// @nodoc
abstract mixin class $GetPaymentRequest_SwapIdCopyWith<$Res> implements $GetPaymentRequestCopyWith<$Res> {
  factory $GetPaymentRequest_SwapIdCopyWith(GetPaymentRequest_SwapId value, $Res Function(GetPaymentRequest_SwapId) _then) = _$GetPaymentRequest_SwapIdCopyWithImpl;
@useResult
$Res call({
 String swapId
});




}
/// @nodoc
class _$GetPaymentRequest_SwapIdCopyWithImpl<$Res>
    implements $GetPaymentRequest_SwapIdCopyWith<$Res> {
  _$GetPaymentRequest_SwapIdCopyWithImpl(this._self, this._then);

  final GetPaymentRequest_SwapId _self;
  final $Res Function(GetPaymentRequest_SwapId) _then;

/// Create a copy of GetPaymentRequest
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? swapId = null,}) {
  return _then(GetPaymentRequest_SwapId(
swapId: null == swapId ? _self.swapId : swapId // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$ListPaymentDetails {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ListPaymentDetails);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'ListPaymentDetails()';
}


}

/// @nodoc
class $ListPaymentDetailsCopyWith<$Res>  {
$ListPaymentDetailsCopyWith(ListPaymentDetails _, $Res Function(ListPaymentDetails) __);
}



/// @nodoc


class ListPaymentDetails_Liquid extends ListPaymentDetails {
  const ListPaymentDetails_Liquid({this.assetId, this.destination}): super._();
  

/// Optional asset id
 final  String? assetId;
/// Optional BIP21 URI or address
 final  String? destination;

/// Create a copy of ListPaymentDetails
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ListPaymentDetails_LiquidCopyWith<ListPaymentDetails_Liquid> get copyWith => _$ListPaymentDetails_LiquidCopyWithImpl<ListPaymentDetails_Liquid>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ListPaymentDetails_Liquid&&(identical(other.assetId, assetId) || other.assetId == assetId)&&(identical(other.destination, destination) || other.destination == destination));
}


@override
int get hashCode => Object.hash(runtimeType,assetId,destination);

@override
String toString() {
  return 'ListPaymentDetails.liquid(assetId: $assetId, destination: $destination)';
}


}

/// @nodoc
abstract mixin class $ListPaymentDetails_LiquidCopyWith<$Res> implements $ListPaymentDetailsCopyWith<$Res> {
  factory $ListPaymentDetails_LiquidCopyWith(ListPaymentDetails_Liquid value, $Res Function(ListPaymentDetails_Liquid) _then) = _$ListPaymentDetails_LiquidCopyWithImpl;
@useResult
$Res call({
 String? assetId, String? destination
});




}
/// @nodoc
class _$ListPaymentDetails_LiquidCopyWithImpl<$Res>
    implements $ListPaymentDetails_LiquidCopyWith<$Res> {
  _$ListPaymentDetails_LiquidCopyWithImpl(this._self, this._then);

  final ListPaymentDetails_Liquid _self;
  final $Res Function(ListPaymentDetails_Liquid) _then;

/// Create a copy of ListPaymentDetails
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? assetId = freezed,Object? destination = freezed,}) {
  return _then(ListPaymentDetails_Liquid(
assetId: freezed == assetId ? _self.assetId : assetId // ignore: cast_nullable_to_non_nullable
as String?,destination: freezed == destination ? _self.destination : destination // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}


}

/// @nodoc


class ListPaymentDetails_Bitcoin extends ListPaymentDetails {
  const ListPaymentDetails_Bitcoin({this.address}): super._();
  

/// Optional address
 final  String? address;

/// Create a copy of ListPaymentDetails
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ListPaymentDetails_BitcoinCopyWith<ListPaymentDetails_Bitcoin> get copyWith => _$ListPaymentDetails_BitcoinCopyWithImpl<ListPaymentDetails_Bitcoin>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ListPaymentDetails_Bitcoin&&(identical(other.address, address) || other.address == address));
}


@override
int get hashCode => Object.hash(runtimeType,address);

@override
String toString() {
  return 'ListPaymentDetails.bitcoin(address: $address)';
}


}

/// @nodoc
abstract mixin class $ListPaymentDetails_BitcoinCopyWith<$Res> implements $ListPaymentDetailsCopyWith<$Res> {
  factory $ListPaymentDetails_BitcoinCopyWith(ListPaymentDetails_Bitcoin value, $Res Function(ListPaymentDetails_Bitcoin) _then) = _$ListPaymentDetails_BitcoinCopyWithImpl;
@useResult
$Res call({
 String? address
});




}
/// @nodoc
class _$ListPaymentDetails_BitcoinCopyWithImpl<$Res>
    implements $ListPaymentDetails_BitcoinCopyWith<$Res> {
  _$ListPaymentDetails_BitcoinCopyWithImpl(this._self, this._then);

  final ListPaymentDetails_Bitcoin _self;
  final $Res Function(ListPaymentDetails_Bitcoin) _then;

/// Create a copy of ListPaymentDetails
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? address = freezed,}) {
  return _then(ListPaymentDetails_Bitcoin(
address: freezed == address ? _self.address : address // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}


}

/// @nodoc
mixin _$LnUrlPayResult {

 Object get data;



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LnUrlPayResult&&const DeepCollectionEquality().equals(other.data, data));
}


@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(data));

@override
String toString() {
  return 'LnUrlPayResult(data: $data)';
}


}

/// @nodoc
class $LnUrlPayResultCopyWith<$Res>  {
$LnUrlPayResultCopyWith(LnUrlPayResult _, $Res Function(LnUrlPayResult) __);
}



/// @nodoc


class LnUrlPayResult_EndpointSuccess extends LnUrlPayResult {
  const LnUrlPayResult_EndpointSuccess({required this.data}): super._();
  

@override final  LnUrlPaySuccessData data;

/// Create a copy of LnUrlPayResult
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$LnUrlPayResult_EndpointSuccessCopyWith<LnUrlPayResult_EndpointSuccess> get copyWith => _$LnUrlPayResult_EndpointSuccessCopyWithImpl<LnUrlPayResult_EndpointSuccess>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LnUrlPayResult_EndpointSuccess&&(identical(other.data, data) || other.data == data));
}


@override
int get hashCode => Object.hash(runtimeType,data);

@override
String toString() {
  return 'LnUrlPayResult.endpointSuccess(data: $data)';
}


}

/// @nodoc
abstract mixin class $LnUrlPayResult_EndpointSuccessCopyWith<$Res> implements $LnUrlPayResultCopyWith<$Res> {
  factory $LnUrlPayResult_EndpointSuccessCopyWith(LnUrlPayResult_EndpointSuccess value, $Res Function(LnUrlPayResult_EndpointSuccess) _then) = _$LnUrlPayResult_EndpointSuccessCopyWithImpl;
@useResult
$Res call({
 LnUrlPaySuccessData data
});




}
/// @nodoc
class _$LnUrlPayResult_EndpointSuccessCopyWithImpl<$Res>
    implements $LnUrlPayResult_EndpointSuccessCopyWith<$Res> {
  _$LnUrlPayResult_EndpointSuccessCopyWithImpl(this._self, this._then);

  final LnUrlPayResult_EndpointSuccess _self;
  final $Res Function(LnUrlPayResult_EndpointSuccess) _then;

/// Create a copy of LnUrlPayResult
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? data = null,}) {
  return _then(LnUrlPayResult_EndpointSuccess(
data: null == data ? _self.data : data // ignore: cast_nullable_to_non_nullable
as LnUrlPaySuccessData,
  ));
}


}

/// @nodoc


class LnUrlPayResult_EndpointError extends LnUrlPayResult {
  const LnUrlPayResult_EndpointError({required this.data}): super._();
  

@override final  LnUrlErrorData data;

/// Create a copy of LnUrlPayResult
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$LnUrlPayResult_EndpointErrorCopyWith<LnUrlPayResult_EndpointError> get copyWith => _$LnUrlPayResult_EndpointErrorCopyWithImpl<LnUrlPayResult_EndpointError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LnUrlPayResult_EndpointError&&(identical(other.data, data) || other.data == data));
}


@override
int get hashCode => Object.hash(runtimeType,data);

@override
String toString() {
  return 'LnUrlPayResult.endpointError(data: $data)';
}


}

/// @nodoc
abstract mixin class $LnUrlPayResult_EndpointErrorCopyWith<$Res> implements $LnUrlPayResultCopyWith<$Res> {
  factory $LnUrlPayResult_EndpointErrorCopyWith(LnUrlPayResult_EndpointError value, $Res Function(LnUrlPayResult_EndpointError) _then) = _$LnUrlPayResult_EndpointErrorCopyWithImpl;
@useResult
$Res call({
 LnUrlErrorData data
});




}
/// @nodoc
class _$LnUrlPayResult_EndpointErrorCopyWithImpl<$Res>
    implements $LnUrlPayResult_EndpointErrorCopyWith<$Res> {
  _$LnUrlPayResult_EndpointErrorCopyWithImpl(this._self, this._then);

  final LnUrlPayResult_EndpointError _self;
  final $Res Function(LnUrlPayResult_EndpointError) _then;

/// Create a copy of LnUrlPayResult
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? data = null,}) {
  return _then(LnUrlPayResult_EndpointError(
data: null == data ? _self.data : data // ignore: cast_nullable_to_non_nullable
as LnUrlErrorData,
  ));
}


}

/// @nodoc


class LnUrlPayResult_PayError extends LnUrlPayResult {
  const LnUrlPayResult_PayError({required this.data}): super._();
  

@override final  LnUrlPayErrorData data;

/// Create a copy of LnUrlPayResult
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$LnUrlPayResult_PayErrorCopyWith<LnUrlPayResult_PayError> get copyWith => _$LnUrlPayResult_PayErrorCopyWithImpl<LnUrlPayResult_PayError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LnUrlPayResult_PayError&&(identical(other.data, data) || other.data == data));
}


@override
int get hashCode => Object.hash(runtimeType,data);

@override
String toString() {
  return 'LnUrlPayResult.payError(data: $data)';
}


}

/// @nodoc
abstract mixin class $LnUrlPayResult_PayErrorCopyWith<$Res> implements $LnUrlPayResultCopyWith<$Res> {
  factory $LnUrlPayResult_PayErrorCopyWith(LnUrlPayResult_PayError value, $Res Function(LnUrlPayResult_PayError) _then) = _$LnUrlPayResult_PayErrorCopyWithImpl;
@useResult
$Res call({
 LnUrlPayErrorData data
});




}
/// @nodoc
class _$LnUrlPayResult_PayErrorCopyWithImpl<$Res>
    implements $LnUrlPayResult_PayErrorCopyWith<$Res> {
  _$LnUrlPayResult_PayErrorCopyWithImpl(this._self, this._then);

  final LnUrlPayResult_PayError _self;
  final $Res Function(LnUrlPayResult_PayError) _then;

/// Create a copy of LnUrlPayResult
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? data = null,}) {
  return _then(LnUrlPayResult_PayError(
data: null == data ? _self.data : data // ignore: cast_nullable_to_non_nullable
as LnUrlPayErrorData,
  ));
}


}

/// @nodoc
mixin _$PayAmount {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PayAmount);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'PayAmount()';
}


}

/// @nodoc
class $PayAmountCopyWith<$Res>  {
$PayAmountCopyWith(PayAmount _, $Res Function(PayAmount) __);
}



/// @nodoc


class PayAmount_Bitcoin extends PayAmount {
  const PayAmount_Bitcoin({required this.receiverAmountSat}): super._();
  

 final  BigInt receiverAmountSat;

/// Create a copy of PayAmount
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PayAmount_BitcoinCopyWith<PayAmount_Bitcoin> get copyWith => _$PayAmount_BitcoinCopyWithImpl<PayAmount_Bitcoin>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PayAmount_Bitcoin&&(identical(other.receiverAmountSat, receiverAmountSat) || other.receiverAmountSat == receiverAmountSat));
}


@override
int get hashCode => Object.hash(runtimeType,receiverAmountSat);

@override
String toString() {
  return 'PayAmount.bitcoin(receiverAmountSat: $receiverAmountSat)';
}


}

/// @nodoc
abstract mixin class $PayAmount_BitcoinCopyWith<$Res> implements $PayAmountCopyWith<$Res> {
  factory $PayAmount_BitcoinCopyWith(PayAmount_Bitcoin value, $Res Function(PayAmount_Bitcoin) _then) = _$PayAmount_BitcoinCopyWithImpl;
@useResult
$Res call({
 BigInt receiverAmountSat
});




}
/// @nodoc
class _$PayAmount_BitcoinCopyWithImpl<$Res>
    implements $PayAmount_BitcoinCopyWith<$Res> {
  _$PayAmount_BitcoinCopyWithImpl(this._self, this._then);

  final PayAmount_Bitcoin _self;
  final $Res Function(PayAmount_Bitcoin) _then;

/// Create a copy of PayAmount
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? receiverAmountSat = null,}) {
  return _then(PayAmount_Bitcoin(
receiverAmountSat: null == receiverAmountSat ? _self.receiverAmountSat : receiverAmountSat // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class PayAmount_Asset extends PayAmount {
  const PayAmount_Asset({required this.toAsset, required this.receiverAmount, this.estimateAssetFees, this.fromAsset}): super._();
  

/// The asset id specifying which asset will be sent
 final  String toAsset;
 final  double receiverAmount;
 final  bool? estimateAssetFees;
/// The asset id whose balance we want to send funds with.
/// Defaults to the value provided for [PayAmount::Asset::to_asset]
 final  String? fromAsset;

/// Create a copy of PayAmount
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PayAmount_AssetCopyWith<PayAmount_Asset> get copyWith => _$PayAmount_AssetCopyWithImpl<PayAmount_Asset>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PayAmount_Asset&&(identical(other.toAsset, toAsset) || other.toAsset == toAsset)&&(identical(other.receiverAmount, receiverAmount) || other.receiverAmount == receiverAmount)&&(identical(other.estimateAssetFees, estimateAssetFees) || other.estimateAssetFees == estimateAssetFees)&&(identical(other.fromAsset, fromAsset) || other.fromAsset == fromAsset));
}


@override
int get hashCode => Object.hash(runtimeType,toAsset,receiverAmount,estimateAssetFees,fromAsset);

@override
String toString() {
  return 'PayAmount.asset(toAsset: $toAsset, receiverAmount: $receiverAmount, estimateAssetFees: $estimateAssetFees, fromAsset: $fromAsset)';
}


}

/// @nodoc
abstract mixin class $PayAmount_AssetCopyWith<$Res> implements $PayAmountCopyWith<$Res> {
  factory $PayAmount_AssetCopyWith(PayAmount_Asset value, $Res Function(PayAmount_Asset) _then) = _$PayAmount_AssetCopyWithImpl;
@useResult
$Res call({
 String toAsset, double receiverAmount, bool? estimateAssetFees, String? fromAsset
});




}
/// @nodoc
class _$PayAmount_AssetCopyWithImpl<$Res>
    implements $PayAmount_AssetCopyWith<$Res> {
  _$PayAmount_AssetCopyWithImpl(this._self, this._then);

  final PayAmount_Asset _self;
  final $Res Function(PayAmount_Asset) _then;

/// Create a copy of PayAmount
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? toAsset = null,Object? receiverAmount = null,Object? estimateAssetFees = freezed,Object? fromAsset = freezed,}) {
  return _then(PayAmount_Asset(
toAsset: null == toAsset ? _self.toAsset : toAsset // ignore: cast_nullable_to_non_nullable
as String,receiverAmount: null == receiverAmount ? _self.receiverAmount : receiverAmount // ignore: cast_nullable_to_non_nullable
as double,estimateAssetFees: freezed == estimateAssetFees ? _self.estimateAssetFees : estimateAssetFees // ignore: cast_nullable_to_non_nullable
as bool?,fromAsset: freezed == fromAsset ? _self.fromAsset : fromAsset // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}


}

/// @nodoc


class PayAmount_Drain extends PayAmount {
  const PayAmount_Drain(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PayAmount_Drain);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'PayAmount.drain()';
}


}




/// @nodoc
mixin _$PaymentDetails {

/// Represents the invoice description
 String get description;
/// Create a copy of PaymentDetails
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PaymentDetailsCopyWith<PaymentDetails> get copyWith => _$PaymentDetailsCopyWithImpl<PaymentDetails>(this as PaymentDetails, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentDetails&&(identical(other.description, description) || other.description == description));
}


@override
int get hashCode => Object.hash(runtimeType,description);

@override
String toString() {
  return 'PaymentDetails(description: $description)';
}


}

/// @nodoc
abstract mixin class $PaymentDetailsCopyWith<$Res>  {
  factory $PaymentDetailsCopyWith(PaymentDetails value, $Res Function(PaymentDetails) _then) = _$PaymentDetailsCopyWithImpl;
@useResult
$Res call({
 String description
});




}
/// @nodoc
class _$PaymentDetailsCopyWithImpl<$Res>
    implements $PaymentDetailsCopyWith<$Res> {
  _$PaymentDetailsCopyWithImpl(this._self, this._then);

  final PaymentDetails _self;
  final $Res Function(PaymentDetails) _then;

/// Create a copy of PaymentDetails
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? description = null,}) {
  return _then(_self.copyWith(
description: null == description ? _self.description : description // ignore: cast_nullable_to_non_nullable
as String,
  ));
}

}



/// @nodoc


class PaymentDetails_Lightning extends PaymentDetails {
  const PaymentDetails_Lightning({required this.swapId, required this.description, required this.liquidExpirationBlockheight, this.preimage, this.invoice, this.bolt12Offer, this.paymentHash, this.destinationPubkey, this.lnurlInfo, this.bip353Address, this.payerNote, this.claimTxId, this.refundTxId, this.refundTxAmountSat}): super._();
  

 final  String swapId;
/// Represents the invoice description
@override final  String description;
/// The height of the block at which the swap will no longer be valid
 final  int liquidExpirationBlockheight;
/// The preimage of the paid invoice (proof of payment).
 final  String? preimage;
/// Represents the Bolt11/Bolt12 invoice associated with a payment
/// In the case of a Send payment, this is the invoice paid by the swapper
/// In the case of a Receive payment, this is the invoice paid by the user
 final  String? invoice;
 final  String? bolt12Offer;
/// The payment hash of the invoice
 final  String? paymentHash;
/// The invoice destination/payee pubkey
 final  String? destinationPubkey;
/// The payment LNURL info
 final  LnUrlInfo? lnurlInfo;
/// The BIP353 address used to resolve this payment
 final  String? bip353Address;
/// The payer note
 final  String? payerNote;
/// For a Receive payment, this is the claim tx id in case it has already been broadcast
 final  String? claimTxId;
/// For a Send swap which was refunded, this is the refund tx id
 final  String? refundTxId;
/// For a Send swap which was refunded, this is the refund amount
 final  BigInt? refundTxAmountSat;

/// Create a copy of PaymentDetails
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PaymentDetails_LightningCopyWith<PaymentDetails_Lightning> get copyWith => _$PaymentDetails_LightningCopyWithImpl<PaymentDetails_Lightning>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentDetails_Lightning&&(identical(other.swapId, swapId) || other.swapId == swapId)&&(identical(other.description, description) || other.description == description)&&(identical(other.liquidExpirationBlockheight, liquidExpirationBlockheight) || other.liquidExpirationBlockheight == liquidExpirationBlockheight)&&(identical(other.preimage, preimage) || other.preimage == preimage)&&(identical(other.invoice, invoice) || other.invoice == invoice)&&(identical(other.bolt12Offer, bolt12Offer) || other.bolt12Offer == bolt12Offer)&&(identical(other.paymentHash, paymentHash) || other.paymentHash == paymentHash)&&(identical(other.destinationPubkey, destinationPubkey) || other.destinationPubkey == destinationPubkey)&&(identical(other.lnurlInfo, lnurlInfo) || other.lnurlInfo == lnurlInfo)&&(identical(other.bip353Address, bip353Address) || other.bip353Address == bip353Address)&&(identical(other.payerNote, payerNote) || other.payerNote == payerNote)&&(identical(other.claimTxId, claimTxId) || other.claimTxId == claimTxId)&&(identical(other.refundTxId, refundTxId) || other.refundTxId == refundTxId)&&(identical(other.refundTxAmountSat, refundTxAmountSat) || other.refundTxAmountSat == refundTxAmountSat));
}


@override
int get hashCode => Object.hash(runtimeType,swapId,description,liquidExpirationBlockheight,preimage,invoice,bolt12Offer,paymentHash,destinationPubkey,lnurlInfo,bip353Address,payerNote,claimTxId,refundTxId,refundTxAmountSat);

@override
String toString() {
  return 'PaymentDetails.lightning(swapId: $swapId, description: $description, liquidExpirationBlockheight: $liquidExpirationBlockheight, preimage: $preimage, invoice: $invoice, bolt12Offer: $bolt12Offer, paymentHash: $paymentHash, destinationPubkey: $destinationPubkey, lnurlInfo: $lnurlInfo, bip353Address: $bip353Address, payerNote: $payerNote, claimTxId: $claimTxId, refundTxId: $refundTxId, refundTxAmountSat: $refundTxAmountSat)';
}


}

/// @nodoc
abstract mixin class $PaymentDetails_LightningCopyWith<$Res> implements $PaymentDetailsCopyWith<$Res> {
  factory $PaymentDetails_LightningCopyWith(PaymentDetails_Lightning value, $Res Function(PaymentDetails_Lightning) _then) = _$PaymentDetails_LightningCopyWithImpl;
@override @useResult
$Res call({
 String swapId, String description, int liquidExpirationBlockheight, String? preimage, String? invoice, String? bolt12Offer, String? paymentHash, String? destinationPubkey, LnUrlInfo? lnurlInfo, String? bip353Address, String? payerNote, String? claimTxId, String? refundTxId, BigInt? refundTxAmountSat
});




}
/// @nodoc
class _$PaymentDetails_LightningCopyWithImpl<$Res>
    implements $PaymentDetails_LightningCopyWith<$Res> {
  _$PaymentDetails_LightningCopyWithImpl(this._self, this._then);

  final PaymentDetails_Lightning _self;
  final $Res Function(PaymentDetails_Lightning) _then;

/// Create a copy of PaymentDetails
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? swapId = null,Object? description = null,Object? liquidExpirationBlockheight = null,Object? preimage = freezed,Object? invoice = freezed,Object? bolt12Offer = freezed,Object? paymentHash = freezed,Object? destinationPubkey = freezed,Object? lnurlInfo = freezed,Object? bip353Address = freezed,Object? payerNote = freezed,Object? claimTxId = freezed,Object? refundTxId = freezed,Object? refundTxAmountSat = freezed,}) {
  return _then(PaymentDetails_Lightning(
swapId: null == swapId ? _self.swapId : swapId // ignore: cast_nullable_to_non_nullable
as String,description: null == description ? _self.description : description // ignore: cast_nullable_to_non_nullable
as String,liquidExpirationBlockheight: null == liquidExpirationBlockheight ? _self.liquidExpirationBlockheight : liquidExpirationBlockheight // ignore: cast_nullable_to_non_nullable
as int,preimage: freezed == preimage ? _self.preimage : preimage // ignore: cast_nullable_to_non_nullable
as String?,invoice: freezed == invoice ? _self.invoice : invoice // ignore: cast_nullable_to_non_nullable
as String?,bolt12Offer: freezed == bolt12Offer ? _self.bolt12Offer : bolt12Offer // ignore: cast_nullable_to_non_nullable
as String?,paymentHash: freezed == paymentHash ? _self.paymentHash : paymentHash // ignore: cast_nullable_to_non_nullable
as String?,destinationPubkey: freezed == destinationPubkey ? _self.destinationPubkey : destinationPubkey // ignore: cast_nullable_to_non_nullable
as String?,lnurlInfo: freezed == lnurlInfo ? _self.lnurlInfo : lnurlInfo // ignore: cast_nullable_to_non_nullable
as LnUrlInfo?,bip353Address: freezed == bip353Address ? _self.bip353Address : bip353Address // ignore: cast_nullable_to_non_nullable
as String?,payerNote: freezed == payerNote ? _self.payerNote : payerNote // ignore: cast_nullable_to_non_nullable
as String?,claimTxId: freezed == claimTxId ? _self.claimTxId : claimTxId // ignore: cast_nullable_to_non_nullable
as String?,refundTxId: freezed == refundTxId ? _self.refundTxId : refundTxId // ignore: cast_nullable_to_non_nullable
as String?,refundTxAmountSat: freezed == refundTxAmountSat ? _self.refundTxAmountSat : refundTxAmountSat // ignore: cast_nullable_to_non_nullable
as BigInt?,
  ));
}


}

/// @nodoc


class PaymentDetails_Liquid extends PaymentDetails {
  const PaymentDetails_Liquid({required this.destination, required this.description, required this.assetId, this.assetInfo, this.lnurlInfo, this.bip353Address, this.payerNote}): super._();
  

/// Represents either a Liquid BIP21 URI or pure address
 final  String destination;
/// Represents the BIP21 `message` field
@override final  String description;
/// The asset id
 final  String assetId;
/// The asset info derived from the [AssetMetadata]
 final  AssetInfo? assetInfo;
/// The payment LNURL info
 final  LnUrlInfo? lnurlInfo;
/// The BIP353 address used to resolve this payment
 final  String? bip353Address;
/// The payer note
 final  String? payerNote;

/// Create a copy of PaymentDetails
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PaymentDetails_LiquidCopyWith<PaymentDetails_Liquid> get copyWith => _$PaymentDetails_LiquidCopyWithImpl<PaymentDetails_Liquid>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentDetails_Liquid&&(identical(other.destination, destination) || other.destination == destination)&&(identical(other.description, description) || other.description == description)&&(identical(other.assetId, assetId) || other.assetId == assetId)&&(identical(other.assetInfo, assetInfo) || other.assetInfo == assetInfo)&&(identical(other.lnurlInfo, lnurlInfo) || other.lnurlInfo == lnurlInfo)&&(identical(other.bip353Address, bip353Address) || other.bip353Address == bip353Address)&&(identical(other.payerNote, payerNote) || other.payerNote == payerNote));
}


@override
int get hashCode => Object.hash(runtimeType,destination,description,assetId,assetInfo,lnurlInfo,bip353Address,payerNote);

@override
String toString() {
  return 'PaymentDetails.liquid(destination: $destination, description: $description, assetId: $assetId, assetInfo: $assetInfo, lnurlInfo: $lnurlInfo, bip353Address: $bip353Address, payerNote: $payerNote)';
}


}

/// @nodoc
abstract mixin class $PaymentDetails_LiquidCopyWith<$Res> implements $PaymentDetailsCopyWith<$Res> {
  factory $PaymentDetails_LiquidCopyWith(PaymentDetails_Liquid value, $Res Function(PaymentDetails_Liquid) _then) = _$PaymentDetails_LiquidCopyWithImpl;
@override @useResult
$Res call({
 String destination, String description, String assetId, AssetInfo? assetInfo, LnUrlInfo? lnurlInfo, String? bip353Address, String? payerNote
});




}
/// @nodoc
class _$PaymentDetails_LiquidCopyWithImpl<$Res>
    implements $PaymentDetails_LiquidCopyWith<$Res> {
  _$PaymentDetails_LiquidCopyWithImpl(this._self, this._then);

  final PaymentDetails_Liquid _self;
  final $Res Function(PaymentDetails_Liquid) _then;

/// Create a copy of PaymentDetails
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? destination = null,Object? description = null,Object? assetId = null,Object? assetInfo = freezed,Object? lnurlInfo = freezed,Object? bip353Address = freezed,Object? payerNote = freezed,}) {
  return _then(PaymentDetails_Liquid(
destination: null == destination ? _self.destination : destination // ignore: cast_nullable_to_non_nullable
as String,description: null == description ? _self.description : description // ignore: cast_nullable_to_non_nullable
as String,assetId: null == assetId ? _self.assetId : assetId // ignore: cast_nullable_to_non_nullable
as String,assetInfo: freezed == assetInfo ? _self.assetInfo : assetInfo // ignore: cast_nullable_to_non_nullable
as AssetInfo?,lnurlInfo: freezed == lnurlInfo ? _self.lnurlInfo : lnurlInfo // ignore: cast_nullable_to_non_nullable
as LnUrlInfo?,bip353Address: freezed == bip353Address ? _self.bip353Address : bip353Address // ignore: cast_nullable_to_non_nullable
as String?,payerNote: freezed == payerNote ? _self.payerNote : payerNote // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}


}

/// @nodoc


class PaymentDetails_Bitcoin extends PaymentDetails {
  const PaymentDetails_Bitcoin({required this.swapId, required this.bitcoinAddress, required this.description, required this.autoAcceptedFees, required this.liquidExpirationBlockheight, required this.bitcoinExpirationBlockheight, this.lockupTxId, this.claimTxId, this.refundTxId, this.refundTxAmountSat}): super._();
  

 final  String swapId;
/// The Bitcoin address that receives funds.
 final  String bitcoinAddress;
/// Represents the invoice description
@override final  String description;
/// For an amountless receive swap, this indicates if fees were automatically accepted.
/// Fees are auto accepted when the swapper proposes fees that are within the initial
/// estimate, plus the `onchain_fee_rate_leeway_sat_per_vbyte` set in the [Config], if any.
 final  bool autoAcceptedFees;
/// The height of the Liquid block at which the swap will no longer be valid
 final  int liquidExpirationBlockheight;
/// The height of the Bitcoin block at which the swap will no longer be valid
 final  int bitcoinExpirationBlockheight;
/// The lockup tx id that initiates the swap
 final  String? lockupTxId;
/// The claim tx id that claims the server lockup tx
 final  String? claimTxId;
/// For a Send swap which was refunded, this is the refund tx id
 final  String? refundTxId;
/// For a Send swap which was refunded, this is the refund amount
 final  BigInt? refundTxAmountSat;

/// Create a copy of PaymentDetails
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PaymentDetails_BitcoinCopyWith<PaymentDetails_Bitcoin> get copyWith => _$PaymentDetails_BitcoinCopyWithImpl<PaymentDetails_Bitcoin>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentDetails_Bitcoin&&(identical(other.swapId, swapId) || other.swapId == swapId)&&(identical(other.bitcoinAddress, bitcoinAddress) || other.bitcoinAddress == bitcoinAddress)&&(identical(other.description, description) || other.description == description)&&(identical(other.autoAcceptedFees, autoAcceptedFees) || other.autoAcceptedFees == autoAcceptedFees)&&(identical(other.liquidExpirationBlockheight, liquidExpirationBlockheight) || other.liquidExpirationBlockheight == liquidExpirationBlockheight)&&(identical(other.bitcoinExpirationBlockheight, bitcoinExpirationBlockheight) || other.bitcoinExpirationBlockheight == bitcoinExpirationBlockheight)&&(identical(other.lockupTxId, lockupTxId) || other.lockupTxId == lockupTxId)&&(identical(other.claimTxId, claimTxId) || other.claimTxId == claimTxId)&&(identical(other.refundTxId, refundTxId) || other.refundTxId == refundTxId)&&(identical(other.refundTxAmountSat, refundTxAmountSat) || other.refundTxAmountSat == refundTxAmountSat));
}


@override
int get hashCode => Object.hash(runtimeType,swapId,bitcoinAddress,description,autoAcceptedFees,liquidExpirationBlockheight,bitcoinExpirationBlockheight,lockupTxId,claimTxId,refundTxId,refundTxAmountSat);

@override
String toString() {
  return 'PaymentDetails.bitcoin(swapId: $swapId, bitcoinAddress: $bitcoinAddress, description: $description, autoAcceptedFees: $autoAcceptedFees, liquidExpirationBlockheight: $liquidExpirationBlockheight, bitcoinExpirationBlockheight: $bitcoinExpirationBlockheight, lockupTxId: $lockupTxId, claimTxId: $claimTxId, refundTxId: $refundTxId, refundTxAmountSat: $refundTxAmountSat)';
}


}

/// @nodoc
abstract mixin class $PaymentDetails_BitcoinCopyWith<$Res> implements $PaymentDetailsCopyWith<$Res> {
  factory $PaymentDetails_BitcoinCopyWith(PaymentDetails_Bitcoin value, $Res Function(PaymentDetails_Bitcoin) _then) = _$PaymentDetails_BitcoinCopyWithImpl;
@override @useResult
$Res call({
 String swapId, String bitcoinAddress, String description, bool autoAcceptedFees, int liquidExpirationBlockheight, int bitcoinExpirationBlockheight, String? lockupTxId, String? claimTxId, String? refundTxId, BigInt? refundTxAmountSat
});




}
/// @nodoc
class _$PaymentDetails_BitcoinCopyWithImpl<$Res>
    implements $PaymentDetails_BitcoinCopyWith<$Res> {
  _$PaymentDetails_BitcoinCopyWithImpl(this._self, this._then);

  final PaymentDetails_Bitcoin _self;
  final $Res Function(PaymentDetails_Bitcoin) _then;

/// Create a copy of PaymentDetails
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? swapId = null,Object? bitcoinAddress = null,Object? description = null,Object? autoAcceptedFees = null,Object? liquidExpirationBlockheight = null,Object? bitcoinExpirationBlockheight = null,Object? lockupTxId = freezed,Object? claimTxId = freezed,Object? refundTxId = freezed,Object? refundTxAmountSat = freezed,}) {
  return _then(PaymentDetails_Bitcoin(
swapId: null == swapId ? _self.swapId : swapId // ignore: cast_nullable_to_non_nullable
as String,bitcoinAddress: null == bitcoinAddress ? _self.bitcoinAddress : bitcoinAddress // ignore: cast_nullable_to_non_nullable
as String,description: null == description ? _self.description : description // ignore: cast_nullable_to_non_nullable
as String,autoAcceptedFees: null == autoAcceptedFees ? _self.autoAcceptedFees : autoAcceptedFees // ignore: cast_nullable_to_non_nullable
as bool,liquidExpirationBlockheight: null == liquidExpirationBlockheight ? _self.liquidExpirationBlockheight : liquidExpirationBlockheight // ignore: cast_nullable_to_non_nullable
as int,bitcoinExpirationBlockheight: null == bitcoinExpirationBlockheight ? _self.bitcoinExpirationBlockheight : bitcoinExpirationBlockheight // ignore: cast_nullable_to_non_nullable
as int,lockupTxId: freezed == lockupTxId ? _self.lockupTxId : lockupTxId // ignore: cast_nullable_to_non_nullable
as String?,claimTxId: freezed == claimTxId ? _self.claimTxId : claimTxId // ignore: cast_nullable_to_non_nullable
as String?,refundTxId: freezed == refundTxId ? _self.refundTxId : refundTxId // ignore: cast_nullable_to_non_nullable
as String?,refundTxAmountSat: freezed == refundTxAmountSat ? _self.refundTxAmountSat : refundTxAmountSat // ignore: cast_nullable_to_non_nullable
as BigInt?,
  ));
}


}

/// @nodoc
mixin _$ReceiveAmount {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ReceiveAmount);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'ReceiveAmount()';
}


}

/// @nodoc
class $ReceiveAmountCopyWith<$Res>  {
$ReceiveAmountCopyWith(ReceiveAmount _, $Res Function(ReceiveAmount) __);
}



/// @nodoc


class ReceiveAmount_Bitcoin extends ReceiveAmount {
  const ReceiveAmount_Bitcoin({required this.payerAmountSat}): super._();
  

 final  BigInt payerAmountSat;

/// Create a copy of ReceiveAmount
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ReceiveAmount_BitcoinCopyWith<ReceiveAmount_Bitcoin> get copyWith => _$ReceiveAmount_BitcoinCopyWithImpl<ReceiveAmount_Bitcoin>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ReceiveAmount_Bitcoin&&(identical(other.payerAmountSat, payerAmountSat) || other.payerAmountSat == payerAmountSat));
}


@override
int get hashCode => Object.hash(runtimeType,payerAmountSat);

@override
String toString() {
  return 'ReceiveAmount.bitcoin(payerAmountSat: $payerAmountSat)';
}


}

/// @nodoc
abstract mixin class $ReceiveAmount_BitcoinCopyWith<$Res> implements $ReceiveAmountCopyWith<$Res> {
  factory $ReceiveAmount_BitcoinCopyWith(ReceiveAmount_Bitcoin value, $Res Function(ReceiveAmount_Bitcoin) _then) = _$ReceiveAmount_BitcoinCopyWithImpl;
@useResult
$Res call({
 BigInt payerAmountSat
});




}
/// @nodoc
class _$ReceiveAmount_BitcoinCopyWithImpl<$Res>
    implements $ReceiveAmount_BitcoinCopyWith<$Res> {
  _$ReceiveAmount_BitcoinCopyWithImpl(this._self, this._then);

  final ReceiveAmount_Bitcoin _self;
  final $Res Function(ReceiveAmount_Bitcoin) _then;

/// Create a copy of ReceiveAmount
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? payerAmountSat = null,}) {
  return _then(ReceiveAmount_Bitcoin(
payerAmountSat: null == payerAmountSat ? _self.payerAmountSat : payerAmountSat // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class ReceiveAmount_Asset extends ReceiveAmount {
  const ReceiveAmount_Asset({required this.assetId, this.payerAmount}): super._();
  

 final  String assetId;
 final  double? payerAmount;

/// Create a copy of ReceiveAmount
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ReceiveAmount_AssetCopyWith<ReceiveAmount_Asset> get copyWith => _$ReceiveAmount_AssetCopyWithImpl<ReceiveAmount_Asset>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ReceiveAmount_Asset&&(identical(other.assetId, assetId) || other.assetId == assetId)&&(identical(other.payerAmount, payerAmount) || other.payerAmount == payerAmount));
}


@override
int get hashCode => Object.hash(runtimeType,assetId,payerAmount);

@override
String toString() {
  return 'ReceiveAmount.asset(assetId: $assetId, payerAmount: $payerAmount)';
}


}

/// @nodoc
abstract mixin class $ReceiveAmount_AssetCopyWith<$Res> implements $ReceiveAmountCopyWith<$Res> {
  factory $ReceiveAmount_AssetCopyWith(ReceiveAmount_Asset value, $Res Function(ReceiveAmount_Asset) _then) = _$ReceiveAmount_AssetCopyWithImpl;
@useResult
$Res call({
 String assetId, double? payerAmount
});




}
/// @nodoc
class _$ReceiveAmount_AssetCopyWithImpl<$Res>
    implements $ReceiveAmount_AssetCopyWith<$Res> {
  _$ReceiveAmount_AssetCopyWithImpl(this._self, this._then);

  final ReceiveAmount_Asset _self;
  final $Res Function(ReceiveAmount_Asset) _then;

/// Create a copy of ReceiveAmount
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? assetId = null,Object? payerAmount = freezed,}) {
  return _then(ReceiveAmount_Asset(
assetId: null == assetId ? _self.assetId : assetId // ignore: cast_nullable_to_non_nullable
as String,payerAmount: freezed == payerAmount ? _self.payerAmount : payerAmount // ignore: cast_nullable_to_non_nullable
as double?,
  ));
}


}

/// @nodoc
mixin _$SdkEvent {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SdkEvent);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'SdkEvent()';
}


}

/// @nodoc
class $SdkEventCopyWith<$Res>  {
$SdkEventCopyWith(SdkEvent _, $Res Function(SdkEvent) __);
}



/// @nodoc


class SdkEvent_PaymentFailed extends SdkEvent {
  const SdkEvent_PaymentFailed({required this.details}): super._();
  

 final  Payment details;

/// Create a copy of SdkEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SdkEvent_PaymentFailedCopyWith<SdkEvent_PaymentFailed> get copyWith => _$SdkEvent_PaymentFailedCopyWithImpl<SdkEvent_PaymentFailed>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SdkEvent_PaymentFailed&&(identical(other.details, details) || other.details == details));
}


@override
int get hashCode => Object.hash(runtimeType,details);

@override
String toString() {
  return 'SdkEvent.paymentFailed(details: $details)';
}


}

/// @nodoc
abstract mixin class $SdkEvent_PaymentFailedCopyWith<$Res> implements $SdkEventCopyWith<$Res> {
  factory $SdkEvent_PaymentFailedCopyWith(SdkEvent_PaymentFailed value, $Res Function(SdkEvent_PaymentFailed) _then) = _$SdkEvent_PaymentFailedCopyWithImpl;
@useResult
$Res call({
 Payment details
});




}
/// @nodoc
class _$SdkEvent_PaymentFailedCopyWithImpl<$Res>
    implements $SdkEvent_PaymentFailedCopyWith<$Res> {
  _$SdkEvent_PaymentFailedCopyWithImpl(this._self, this._then);

  final SdkEvent_PaymentFailed _self;
  final $Res Function(SdkEvent_PaymentFailed) _then;

/// Create a copy of SdkEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? details = null,}) {
  return _then(SdkEvent_PaymentFailed(
details: null == details ? _self.details : details // ignore: cast_nullable_to_non_nullable
as Payment,
  ));
}


}

/// @nodoc


class SdkEvent_PaymentPending extends SdkEvent {
  const SdkEvent_PaymentPending({required this.details}): super._();
  

 final  Payment details;

/// Create a copy of SdkEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SdkEvent_PaymentPendingCopyWith<SdkEvent_PaymentPending> get copyWith => _$SdkEvent_PaymentPendingCopyWithImpl<SdkEvent_PaymentPending>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SdkEvent_PaymentPending&&(identical(other.details, details) || other.details == details));
}


@override
int get hashCode => Object.hash(runtimeType,details);

@override
String toString() {
  return 'SdkEvent.paymentPending(details: $details)';
}


}

/// @nodoc
abstract mixin class $SdkEvent_PaymentPendingCopyWith<$Res> implements $SdkEventCopyWith<$Res> {
  factory $SdkEvent_PaymentPendingCopyWith(SdkEvent_PaymentPending value, $Res Function(SdkEvent_PaymentPending) _then) = _$SdkEvent_PaymentPendingCopyWithImpl;
@useResult
$Res call({
 Payment details
});




}
/// @nodoc
class _$SdkEvent_PaymentPendingCopyWithImpl<$Res>
    implements $SdkEvent_PaymentPendingCopyWith<$Res> {
  _$SdkEvent_PaymentPendingCopyWithImpl(this._self, this._then);

  final SdkEvent_PaymentPending _self;
  final $Res Function(SdkEvent_PaymentPending) _then;

/// Create a copy of SdkEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? details = null,}) {
  return _then(SdkEvent_PaymentPending(
details: null == details ? _self.details : details // ignore: cast_nullable_to_non_nullable
as Payment,
  ));
}


}

/// @nodoc


class SdkEvent_PaymentRefundable extends SdkEvent {
  const SdkEvent_PaymentRefundable({required this.details}): super._();
  

 final  Payment details;

/// Create a copy of SdkEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SdkEvent_PaymentRefundableCopyWith<SdkEvent_PaymentRefundable> get copyWith => _$SdkEvent_PaymentRefundableCopyWithImpl<SdkEvent_PaymentRefundable>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SdkEvent_PaymentRefundable&&(identical(other.details, details) || other.details == details));
}


@override
int get hashCode => Object.hash(runtimeType,details);

@override
String toString() {
  return 'SdkEvent.paymentRefundable(details: $details)';
}


}

/// @nodoc
abstract mixin class $SdkEvent_PaymentRefundableCopyWith<$Res> implements $SdkEventCopyWith<$Res> {
  factory $SdkEvent_PaymentRefundableCopyWith(SdkEvent_PaymentRefundable value, $Res Function(SdkEvent_PaymentRefundable) _then) = _$SdkEvent_PaymentRefundableCopyWithImpl;
@useResult
$Res call({
 Payment details
});




}
/// @nodoc
class _$SdkEvent_PaymentRefundableCopyWithImpl<$Res>
    implements $SdkEvent_PaymentRefundableCopyWith<$Res> {
  _$SdkEvent_PaymentRefundableCopyWithImpl(this._self, this._then);

  final SdkEvent_PaymentRefundable _self;
  final $Res Function(SdkEvent_PaymentRefundable) _then;

/// Create a copy of SdkEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? details = null,}) {
  return _then(SdkEvent_PaymentRefundable(
details: null == details ? _self.details : details // ignore: cast_nullable_to_non_nullable
as Payment,
  ));
}


}

/// @nodoc


class SdkEvent_PaymentRefunded extends SdkEvent {
  const SdkEvent_PaymentRefunded({required this.details}): super._();
  

 final  Payment details;

/// Create a copy of SdkEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SdkEvent_PaymentRefundedCopyWith<SdkEvent_PaymentRefunded> get copyWith => _$SdkEvent_PaymentRefundedCopyWithImpl<SdkEvent_PaymentRefunded>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SdkEvent_PaymentRefunded&&(identical(other.details, details) || other.details == details));
}


@override
int get hashCode => Object.hash(runtimeType,details);

@override
String toString() {
  return 'SdkEvent.paymentRefunded(details: $details)';
}


}

/// @nodoc
abstract mixin class $SdkEvent_PaymentRefundedCopyWith<$Res> implements $SdkEventCopyWith<$Res> {
  factory $SdkEvent_PaymentRefundedCopyWith(SdkEvent_PaymentRefunded value, $Res Function(SdkEvent_PaymentRefunded) _then) = _$SdkEvent_PaymentRefundedCopyWithImpl;
@useResult
$Res call({
 Payment details
});




}
/// @nodoc
class _$SdkEvent_PaymentRefundedCopyWithImpl<$Res>
    implements $SdkEvent_PaymentRefundedCopyWith<$Res> {
  _$SdkEvent_PaymentRefundedCopyWithImpl(this._self, this._then);

  final SdkEvent_PaymentRefunded _self;
  final $Res Function(SdkEvent_PaymentRefunded) _then;

/// Create a copy of SdkEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? details = null,}) {
  return _then(SdkEvent_PaymentRefunded(
details: null == details ? _self.details : details // ignore: cast_nullable_to_non_nullable
as Payment,
  ));
}


}

/// @nodoc


class SdkEvent_PaymentRefundPending extends SdkEvent {
  const SdkEvent_PaymentRefundPending({required this.details}): super._();
  

 final  Payment details;

/// Create a copy of SdkEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SdkEvent_PaymentRefundPendingCopyWith<SdkEvent_PaymentRefundPending> get copyWith => _$SdkEvent_PaymentRefundPendingCopyWithImpl<SdkEvent_PaymentRefundPending>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SdkEvent_PaymentRefundPending&&(identical(other.details, details) || other.details == details));
}


@override
int get hashCode => Object.hash(runtimeType,details);

@override
String toString() {
  return 'SdkEvent.paymentRefundPending(details: $details)';
}


}

/// @nodoc
abstract mixin class $SdkEvent_PaymentRefundPendingCopyWith<$Res> implements $SdkEventCopyWith<$Res> {
  factory $SdkEvent_PaymentRefundPendingCopyWith(SdkEvent_PaymentRefundPending value, $Res Function(SdkEvent_PaymentRefundPending) _then) = _$SdkEvent_PaymentRefundPendingCopyWithImpl;
@useResult
$Res call({
 Payment details
});




}
/// @nodoc
class _$SdkEvent_PaymentRefundPendingCopyWithImpl<$Res>
    implements $SdkEvent_PaymentRefundPendingCopyWith<$Res> {
  _$SdkEvent_PaymentRefundPendingCopyWithImpl(this._self, this._then);

  final SdkEvent_PaymentRefundPending _self;
  final $Res Function(SdkEvent_PaymentRefundPending) _then;

/// Create a copy of SdkEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? details = null,}) {
  return _then(SdkEvent_PaymentRefundPending(
details: null == details ? _self.details : details // ignore: cast_nullable_to_non_nullable
as Payment,
  ));
}


}

/// @nodoc


class SdkEvent_PaymentSucceeded extends SdkEvent {
  const SdkEvent_PaymentSucceeded({required this.details}): super._();
  

 final  Payment details;

/// Create a copy of SdkEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SdkEvent_PaymentSucceededCopyWith<SdkEvent_PaymentSucceeded> get copyWith => _$SdkEvent_PaymentSucceededCopyWithImpl<SdkEvent_PaymentSucceeded>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SdkEvent_PaymentSucceeded&&(identical(other.details, details) || other.details == details));
}


@override
int get hashCode => Object.hash(runtimeType,details);

@override
String toString() {
  return 'SdkEvent.paymentSucceeded(details: $details)';
}


}

/// @nodoc
abstract mixin class $SdkEvent_PaymentSucceededCopyWith<$Res> implements $SdkEventCopyWith<$Res> {
  factory $SdkEvent_PaymentSucceededCopyWith(SdkEvent_PaymentSucceeded value, $Res Function(SdkEvent_PaymentSucceeded) _then) = _$SdkEvent_PaymentSucceededCopyWithImpl;
@useResult
$Res call({
 Payment details
});




}
/// @nodoc
class _$SdkEvent_PaymentSucceededCopyWithImpl<$Res>
    implements $SdkEvent_PaymentSucceededCopyWith<$Res> {
  _$SdkEvent_PaymentSucceededCopyWithImpl(this._self, this._then);

  final SdkEvent_PaymentSucceeded _self;
  final $Res Function(SdkEvent_PaymentSucceeded) _then;

/// Create a copy of SdkEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? details = null,}) {
  return _then(SdkEvent_PaymentSucceeded(
details: null == details ? _self.details : details // ignore: cast_nullable_to_non_nullable
as Payment,
  ));
}


}

/// @nodoc


class SdkEvent_PaymentWaitingConfirmation extends SdkEvent {
  const SdkEvent_PaymentWaitingConfirmation({required this.details}): super._();
  

 final  Payment details;

/// Create a copy of SdkEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SdkEvent_PaymentWaitingConfirmationCopyWith<SdkEvent_PaymentWaitingConfirmation> get copyWith => _$SdkEvent_PaymentWaitingConfirmationCopyWithImpl<SdkEvent_PaymentWaitingConfirmation>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SdkEvent_PaymentWaitingConfirmation&&(identical(other.details, details) || other.details == details));
}


@override
int get hashCode => Object.hash(runtimeType,details);

@override
String toString() {
  return 'SdkEvent.paymentWaitingConfirmation(details: $details)';
}


}

/// @nodoc
abstract mixin class $SdkEvent_PaymentWaitingConfirmationCopyWith<$Res> implements $SdkEventCopyWith<$Res> {
  factory $SdkEvent_PaymentWaitingConfirmationCopyWith(SdkEvent_PaymentWaitingConfirmation value, $Res Function(SdkEvent_PaymentWaitingConfirmation) _then) = _$SdkEvent_PaymentWaitingConfirmationCopyWithImpl;
@useResult
$Res call({
 Payment details
});




}
/// @nodoc
class _$SdkEvent_PaymentWaitingConfirmationCopyWithImpl<$Res>
    implements $SdkEvent_PaymentWaitingConfirmationCopyWith<$Res> {
  _$SdkEvent_PaymentWaitingConfirmationCopyWithImpl(this._self, this._then);

  final SdkEvent_PaymentWaitingConfirmation _self;
  final $Res Function(SdkEvent_PaymentWaitingConfirmation) _then;

/// Create a copy of SdkEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? details = null,}) {
  return _then(SdkEvent_PaymentWaitingConfirmation(
details: null == details ? _self.details : details // ignore: cast_nullable_to_non_nullable
as Payment,
  ));
}


}

/// @nodoc


class SdkEvent_PaymentWaitingFeeAcceptance extends SdkEvent {
  const SdkEvent_PaymentWaitingFeeAcceptance({required this.details}): super._();
  

 final  Payment details;

/// Create a copy of SdkEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SdkEvent_PaymentWaitingFeeAcceptanceCopyWith<SdkEvent_PaymentWaitingFeeAcceptance> get copyWith => _$SdkEvent_PaymentWaitingFeeAcceptanceCopyWithImpl<SdkEvent_PaymentWaitingFeeAcceptance>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SdkEvent_PaymentWaitingFeeAcceptance&&(identical(other.details, details) || other.details == details));
}


@override
int get hashCode => Object.hash(runtimeType,details);

@override
String toString() {
  return 'SdkEvent.paymentWaitingFeeAcceptance(details: $details)';
}


}

/// @nodoc
abstract mixin class $SdkEvent_PaymentWaitingFeeAcceptanceCopyWith<$Res> implements $SdkEventCopyWith<$Res> {
  factory $SdkEvent_PaymentWaitingFeeAcceptanceCopyWith(SdkEvent_PaymentWaitingFeeAcceptance value, $Res Function(SdkEvent_PaymentWaitingFeeAcceptance) _then) = _$SdkEvent_PaymentWaitingFeeAcceptanceCopyWithImpl;
@useResult
$Res call({
 Payment details
});




}
/// @nodoc
class _$SdkEvent_PaymentWaitingFeeAcceptanceCopyWithImpl<$Res>
    implements $SdkEvent_PaymentWaitingFeeAcceptanceCopyWith<$Res> {
  _$SdkEvent_PaymentWaitingFeeAcceptanceCopyWithImpl(this._self, this._then);

  final SdkEvent_PaymentWaitingFeeAcceptance _self;
  final $Res Function(SdkEvent_PaymentWaitingFeeAcceptance) _then;

/// Create a copy of SdkEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? details = null,}) {
  return _then(SdkEvent_PaymentWaitingFeeAcceptance(
details: null == details ? _self.details : details // ignore: cast_nullable_to_non_nullable
as Payment,
  ));
}


}

/// @nodoc


class SdkEvent_Synced extends SdkEvent {
  const SdkEvent_Synced(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SdkEvent_Synced);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'SdkEvent.synced()';
}


}




/// @nodoc


class SdkEvent_DataSynced extends SdkEvent {
  const SdkEvent_DataSynced({required this.didPullNewRecords}): super._();
  

/// Indicates new data was pulled from other instances.
 final  bool didPullNewRecords;

/// Create a copy of SdkEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SdkEvent_DataSyncedCopyWith<SdkEvent_DataSynced> get copyWith => _$SdkEvent_DataSyncedCopyWithImpl<SdkEvent_DataSynced>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SdkEvent_DataSynced&&(identical(other.didPullNewRecords, didPullNewRecords) || other.didPullNewRecords == didPullNewRecords));
}


@override
int get hashCode => Object.hash(runtimeType,didPullNewRecords);

@override
String toString() {
  return 'SdkEvent.dataSynced(didPullNewRecords: $didPullNewRecords)';
}


}

/// @nodoc
abstract mixin class $SdkEvent_DataSyncedCopyWith<$Res> implements $SdkEventCopyWith<$Res> {
  factory $SdkEvent_DataSyncedCopyWith(SdkEvent_DataSynced value, $Res Function(SdkEvent_DataSynced) _then) = _$SdkEvent_DataSyncedCopyWithImpl;
@useResult
$Res call({
 bool didPullNewRecords
});




}
/// @nodoc
class _$SdkEvent_DataSyncedCopyWithImpl<$Res>
    implements $SdkEvent_DataSyncedCopyWith<$Res> {
  _$SdkEvent_DataSyncedCopyWithImpl(this._self, this._then);

  final SdkEvent_DataSynced _self;
  final $Res Function(SdkEvent_DataSynced) _then;

/// Create a copy of SdkEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? didPullNewRecords = null,}) {
  return _then(SdkEvent_DataSynced(
didPullNewRecords: null == didPullNewRecords ? _self.didPullNewRecords : didPullNewRecords // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}


}

/// @nodoc
mixin _$SendDestination {

/// A BIP353 address, in case one was used to resolve this Liquid address
 String? get bip353Address;
/// Create a copy of SendDestination
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SendDestinationCopyWith<SendDestination> get copyWith => _$SendDestinationCopyWithImpl<SendDestination>(this as SendDestination, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SendDestination&&(identical(other.bip353Address, bip353Address) || other.bip353Address == bip353Address));
}


@override
int get hashCode => Object.hash(runtimeType,bip353Address);

@override
String toString() {
  return 'SendDestination(bip353Address: $bip353Address)';
}


}

/// @nodoc
abstract mixin class $SendDestinationCopyWith<$Res>  {
  factory $SendDestinationCopyWith(SendDestination value, $Res Function(SendDestination) _then) = _$SendDestinationCopyWithImpl;
@useResult
$Res call({
 String? bip353Address
});




}
/// @nodoc
class _$SendDestinationCopyWithImpl<$Res>
    implements $SendDestinationCopyWith<$Res> {
  _$SendDestinationCopyWithImpl(this._self, this._then);

  final SendDestination _self;
  final $Res Function(SendDestination) _then;

/// Create a copy of SendDestination
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? bip353Address = freezed,}) {
  return _then(_self.copyWith(
bip353Address: freezed == bip353Address ? _self.bip353Address : bip353Address // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}

}



/// @nodoc


class SendDestination_LiquidAddress extends SendDestination {
  const SendDestination_LiquidAddress({required this.addressData, this.bip353Address}): super._();
  

 final  LiquidAddressData addressData;
/// A BIP353 address, in case one was used to resolve this Liquid address
@override final  String? bip353Address;

/// Create a copy of SendDestination
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SendDestination_LiquidAddressCopyWith<SendDestination_LiquidAddress> get copyWith => _$SendDestination_LiquidAddressCopyWithImpl<SendDestination_LiquidAddress>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SendDestination_LiquidAddress&&(identical(other.addressData, addressData) || other.addressData == addressData)&&(identical(other.bip353Address, bip353Address) || other.bip353Address == bip353Address));
}


@override
int get hashCode => Object.hash(runtimeType,addressData,bip353Address);

@override
String toString() {
  return 'SendDestination.liquidAddress(addressData: $addressData, bip353Address: $bip353Address)';
}


}

/// @nodoc
abstract mixin class $SendDestination_LiquidAddressCopyWith<$Res> implements $SendDestinationCopyWith<$Res> {
  factory $SendDestination_LiquidAddressCopyWith(SendDestination_LiquidAddress value, $Res Function(SendDestination_LiquidAddress) _then) = _$SendDestination_LiquidAddressCopyWithImpl;
@override @useResult
$Res call({
 LiquidAddressData addressData, String? bip353Address
});




}
/// @nodoc
class _$SendDestination_LiquidAddressCopyWithImpl<$Res>
    implements $SendDestination_LiquidAddressCopyWith<$Res> {
  _$SendDestination_LiquidAddressCopyWithImpl(this._self, this._then);

  final SendDestination_LiquidAddress _self;
  final $Res Function(SendDestination_LiquidAddress) _then;

/// Create a copy of SendDestination
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? addressData = null,Object? bip353Address = freezed,}) {
  return _then(SendDestination_LiquidAddress(
addressData: null == addressData ? _self.addressData : addressData // ignore: cast_nullable_to_non_nullable
as LiquidAddressData,bip353Address: freezed == bip353Address ? _self.bip353Address : bip353Address // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}


}

/// @nodoc


class SendDestination_Bolt11 extends SendDestination {
  const SendDestination_Bolt11({required this.invoice, this.bip353Address}): super._();
  

 final  LNInvoice invoice;
/// A BIP353 address, in case one was used to resolve this BOLT11
@override final  String? bip353Address;

/// Create a copy of SendDestination
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SendDestination_Bolt11CopyWith<SendDestination_Bolt11> get copyWith => _$SendDestination_Bolt11CopyWithImpl<SendDestination_Bolt11>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SendDestination_Bolt11&&(identical(other.invoice, invoice) || other.invoice == invoice)&&(identical(other.bip353Address, bip353Address) || other.bip353Address == bip353Address));
}


@override
int get hashCode => Object.hash(runtimeType,invoice,bip353Address);

@override
String toString() {
  return 'SendDestination.bolt11(invoice: $invoice, bip353Address: $bip353Address)';
}


}

/// @nodoc
abstract mixin class $SendDestination_Bolt11CopyWith<$Res> implements $SendDestinationCopyWith<$Res> {
  factory $SendDestination_Bolt11CopyWith(SendDestination_Bolt11 value, $Res Function(SendDestination_Bolt11) _then) = _$SendDestination_Bolt11CopyWithImpl;
@override @useResult
$Res call({
 LNInvoice invoice, String? bip353Address
});




}
/// @nodoc
class _$SendDestination_Bolt11CopyWithImpl<$Res>
    implements $SendDestination_Bolt11CopyWith<$Res> {
  _$SendDestination_Bolt11CopyWithImpl(this._self, this._then);

  final SendDestination_Bolt11 _self;
  final $Res Function(SendDestination_Bolt11) _then;

/// Create a copy of SendDestination
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? invoice = null,Object? bip353Address = freezed,}) {
  return _then(SendDestination_Bolt11(
invoice: null == invoice ? _self.invoice : invoice // ignore: cast_nullable_to_non_nullable
as LNInvoice,bip353Address: freezed == bip353Address ? _self.bip353Address : bip353Address // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}


}

/// @nodoc


class SendDestination_Bolt12 extends SendDestination {
  const SendDestination_Bolt12({required this.offer, required this.receiverAmountSat, this.bip353Address}): super._();
  

 final  LNOffer offer;
 final  BigInt receiverAmountSat;
/// A BIP353 address, in case one was used to resolve this BOLT12
@override final  String? bip353Address;

/// Create a copy of SendDestination
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SendDestination_Bolt12CopyWith<SendDestination_Bolt12> get copyWith => _$SendDestination_Bolt12CopyWithImpl<SendDestination_Bolt12>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SendDestination_Bolt12&&(identical(other.offer, offer) || other.offer == offer)&&(identical(other.receiverAmountSat, receiverAmountSat) || other.receiverAmountSat == receiverAmountSat)&&(identical(other.bip353Address, bip353Address) || other.bip353Address == bip353Address));
}


@override
int get hashCode => Object.hash(runtimeType,offer,receiverAmountSat,bip353Address);

@override
String toString() {
  return 'SendDestination.bolt12(offer: $offer, receiverAmountSat: $receiverAmountSat, bip353Address: $bip353Address)';
}


}

/// @nodoc
abstract mixin class $SendDestination_Bolt12CopyWith<$Res> implements $SendDestinationCopyWith<$Res> {
  factory $SendDestination_Bolt12CopyWith(SendDestination_Bolt12 value, $Res Function(SendDestination_Bolt12) _then) = _$SendDestination_Bolt12CopyWithImpl;
@override @useResult
$Res call({
 LNOffer offer, BigInt receiverAmountSat, String? bip353Address
});




}
/// @nodoc
class _$SendDestination_Bolt12CopyWithImpl<$Res>
    implements $SendDestination_Bolt12CopyWith<$Res> {
  _$SendDestination_Bolt12CopyWithImpl(this._self, this._then);

  final SendDestination_Bolt12 _self;
  final $Res Function(SendDestination_Bolt12) _then;

/// Create a copy of SendDestination
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? offer = null,Object? receiverAmountSat = null,Object? bip353Address = freezed,}) {
  return _then(SendDestination_Bolt12(
offer: null == offer ? _self.offer : offer // ignore: cast_nullable_to_non_nullable
as LNOffer,receiverAmountSat: null == receiverAmountSat ? _self.receiverAmountSat : receiverAmountSat // ignore: cast_nullable_to_non_nullable
as BigInt,bip353Address: freezed == bip353Address ? _self.bip353Address : bip353Address // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}


}

// dart format on
