// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'error.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$PaymentError {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentError);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'PaymentError()';
}


}

/// @nodoc
class $PaymentErrorCopyWith<$Res>  {
$PaymentErrorCopyWith(PaymentError _, $Res Function(PaymentError) __);
}


/// @nodoc


class PaymentError_AlreadyClaimed extends PaymentError {
  const PaymentError_AlreadyClaimed(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentError_AlreadyClaimed);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'PaymentError.alreadyClaimed()';
}


}




/// @nodoc


class PaymentError_AlreadyPaid extends PaymentError {
  const PaymentError_AlreadyPaid(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentError_AlreadyPaid);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'PaymentError.alreadyPaid()';
}


}




/// @nodoc


class PaymentError_PaymentInProgress extends PaymentError {
  const PaymentError_PaymentInProgress(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentError_PaymentInProgress);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'PaymentError.paymentInProgress()';
}


}




/// @nodoc


class PaymentError_AmountOutOfRange extends PaymentError {
  const PaymentError_AmountOutOfRange({required this.min, required this.max}): super._();
  

 final  BigInt min;
 final  BigInt max;

/// Create a copy of PaymentError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PaymentError_AmountOutOfRangeCopyWith<PaymentError_AmountOutOfRange> get copyWith => _$PaymentError_AmountOutOfRangeCopyWithImpl<PaymentError_AmountOutOfRange>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentError_AmountOutOfRange&&(identical(other.min, min) || other.min == min)&&(identical(other.max, max) || other.max == max));
}


@override
int get hashCode => Object.hash(runtimeType,min,max);

@override
String toString() {
  return 'PaymentError.amountOutOfRange(min: $min, max: $max)';
}


}

/// @nodoc
abstract mixin class $PaymentError_AmountOutOfRangeCopyWith<$Res> implements $PaymentErrorCopyWith<$Res> {
  factory $PaymentError_AmountOutOfRangeCopyWith(PaymentError_AmountOutOfRange value, $Res Function(PaymentError_AmountOutOfRange) _then) = _$PaymentError_AmountOutOfRangeCopyWithImpl;
@useResult
$Res call({
 BigInt min, BigInt max
});




}
/// @nodoc
class _$PaymentError_AmountOutOfRangeCopyWithImpl<$Res>
    implements $PaymentError_AmountOutOfRangeCopyWith<$Res> {
  _$PaymentError_AmountOutOfRangeCopyWithImpl(this._self, this._then);

  final PaymentError_AmountOutOfRange _self;
  final $Res Function(PaymentError_AmountOutOfRange) _then;

/// Create a copy of PaymentError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? min = null,Object? max = null,}) {
  return _then(PaymentError_AmountOutOfRange(
min: null == min ? _self.min : min // ignore: cast_nullable_to_non_nullable
as BigInt,max: null == max ? _self.max : max // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class PaymentError_AmountMissing extends PaymentError {
  const PaymentError_AmountMissing({required this.err}): super._();
  

 final  String err;

/// Create a copy of PaymentError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PaymentError_AmountMissingCopyWith<PaymentError_AmountMissing> get copyWith => _$PaymentError_AmountMissingCopyWithImpl<PaymentError_AmountMissing>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentError_AmountMissing&&(identical(other.err, err) || other.err == err));
}


@override
int get hashCode => Object.hash(runtimeType,err);

@override
String toString() {
  return 'PaymentError.amountMissing(err: $err)';
}


}

/// @nodoc
abstract mixin class $PaymentError_AmountMissingCopyWith<$Res> implements $PaymentErrorCopyWith<$Res> {
  factory $PaymentError_AmountMissingCopyWith(PaymentError_AmountMissing value, $Res Function(PaymentError_AmountMissing) _then) = _$PaymentError_AmountMissingCopyWithImpl;
@useResult
$Res call({
 String err
});




}
/// @nodoc
class _$PaymentError_AmountMissingCopyWithImpl<$Res>
    implements $PaymentError_AmountMissingCopyWith<$Res> {
  _$PaymentError_AmountMissingCopyWithImpl(this._self, this._then);

  final PaymentError_AmountMissing _self;
  final $Res Function(PaymentError_AmountMissing) _then;

/// Create a copy of PaymentError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? err = null,}) {
  return _then(PaymentError_AmountMissing(
err: null == err ? _self.err : err // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class PaymentError_AssetError extends PaymentError {
  const PaymentError_AssetError({required this.err}): super._();
  

 final  String err;

/// Create a copy of PaymentError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PaymentError_AssetErrorCopyWith<PaymentError_AssetError> get copyWith => _$PaymentError_AssetErrorCopyWithImpl<PaymentError_AssetError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentError_AssetError&&(identical(other.err, err) || other.err == err));
}


@override
int get hashCode => Object.hash(runtimeType,err);

@override
String toString() {
  return 'PaymentError.assetError(err: $err)';
}


}

/// @nodoc
abstract mixin class $PaymentError_AssetErrorCopyWith<$Res> implements $PaymentErrorCopyWith<$Res> {
  factory $PaymentError_AssetErrorCopyWith(PaymentError_AssetError value, $Res Function(PaymentError_AssetError) _then) = _$PaymentError_AssetErrorCopyWithImpl;
@useResult
$Res call({
 String err
});




}
/// @nodoc
class _$PaymentError_AssetErrorCopyWithImpl<$Res>
    implements $PaymentError_AssetErrorCopyWith<$Res> {
  _$PaymentError_AssetErrorCopyWithImpl(this._self, this._then);

  final PaymentError_AssetError _self;
  final $Res Function(PaymentError_AssetError) _then;

/// Create a copy of PaymentError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? err = null,}) {
  return _then(PaymentError_AssetError(
err: null == err ? _self.err : err // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class PaymentError_InvalidNetwork extends PaymentError {
  const PaymentError_InvalidNetwork({required this.err}): super._();
  

 final  String err;

/// Create a copy of PaymentError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PaymentError_InvalidNetworkCopyWith<PaymentError_InvalidNetwork> get copyWith => _$PaymentError_InvalidNetworkCopyWithImpl<PaymentError_InvalidNetwork>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentError_InvalidNetwork&&(identical(other.err, err) || other.err == err));
}


@override
int get hashCode => Object.hash(runtimeType,err);

@override
String toString() {
  return 'PaymentError.invalidNetwork(err: $err)';
}


}

/// @nodoc
abstract mixin class $PaymentError_InvalidNetworkCopyWith<$Res> implements $PaymentErrorCopyWith<$Res> {
  factory $PaymentError_InvalidNetworkCopyWith(PaymentError_InvalidNetwork value, $Res Function(PaymentError_InvalidNetwork) _then) = _$PaymentError_InvalidNetworkCopyWithImpl;
@useResult
$Res call({
 String err
});




}
/// @nodoc
class _$PaymentError_InvalidNetworkCopyWithImpl<$Res>
    implements $PaymentError_InvalidNetworkCopyWith<$Res> {
  _$PaymentError_InvalidNetworkCopyWithImpl(this._self, this._then);

  final PaymentError_InvalidNetwork _self;
  final $Res Function(PaymentError_InvalidNetwork) _then;

/// Create a copy of PaymentError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? err = null,}) {
  return _then(PaymentError_InvalidNetwork(
err: null == err ? _self.err : err // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class PaymentError_Generic extends PaymentError {
  const PaymentError_Generic({required this.err}): super._();
  

 final  String err;

/// Create a copy of PaymentError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PaymentError_GenericCopyWith<PaymentError_Generic> get copyWith => _$PaymentError_GenericCopyWithImpl<PaymentError_Generic>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentError_Generic&&(identical(other.err, err) || other.err == err));
}


@override
int get hashCode => Object.hash(runtimeType,err);

@override
String toString() {
  return 'PaymentError.generic(err: $err)';
}


}

/// @nodoc
abstract mixin class $PaymentError_GenericCopyWith<$Res> implements $PaymentErrorCopyWith<$Res> {
  factory $PaymentError_GenericCopyWith(PaymentError_Generic value, $Res Function(PaymentError_Generic) _then) = _$PaymentError_GenericCopyWithImpl;
@useResult
$Res call({
 String err
});




}
/// @nodoc
class _$PaymentError_GenericCopyWithImpl<$Res>
    implements $PaymentError_GenericCopyWith<$Res> {
  _$PaymentError_GenericCopyWithImpl(this._self, this._then);

  final PaymentError_Generic _self;
  final $Res Function(PaymentError_Generic) _then;

/// Create a copy of PaymentError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? err = null,}) {
  return _then(PaymentError_Generic(
err: null == err ? _self.err : err // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class PaymentError_InvalidOrExpiredFees extends PaymentError {
  const PaymentError_InvalidOrExpiredFees(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentError_InvalidOrExpiredFees);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'PaymentError.invalidOrExpiredFees()';
}


}




/// @nodoc


class PaymentError_InsufficientFunds extends PaymentError {
  const PaymentError_InsufficientFunds(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentError_InsufficientFunds);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'PaymentError.insufficientFunds()';
}


}




/// @nodoc


class PaymentError_InvalidDescription extends PaymentError {
  const PaymentError_InvalidDescription({required this.err}): super._();
  

 final  String err;

/// Create a copy of PaymentError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PaymentError_InvalidDescriptionCopyWith<PaymentError_InvalidDescription> get copyWith => _$PaymentError_InvalidDescriptionCopyWithImpl<PaymentError_InvalidDescription>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentError_InvalidDescription&&(identical(other.err, err) || other.err == err));
}


@override
int get hashCode => Object.hash(runtimeType,err);

@override
String toString() {
  return 'PaymentError.invalidDescription(err: $err)';
}


}

/// @nodoc
abstract mixin class $PaymentError_InvalidDescriptionCopyWith<$Res> implements $PaymentErrorCopyWith<$Res> {
  factory $PaymentError_InvalidDescriptionCopyWith(PaymentError_InvalidDescription value, $Res Function(PaymentError_InvalidDescription) _then) = _$PaymentError_InvalidDescriptionCopyWithImpl;
@useResult
$Res call({
 String err
});




}
/// @nodoc
class _$PaymentError_InvalidDescriptionCopyWithImpl<$Res>
    implements $PaymentError_InvalidDescriptionCopyWith<$Res> {
  _$PaymentError_InvalidDescriptionCopyWithImpl(this._self, this._then);

  final PaymentError_InvalidDescription _self;
  final $Res Function(PaymentError_InvalidDescription) _then;

/// Create a copy of PaymentError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? err = null,}) {
  return _then(PaymentError_InvalidDescription(
err: null == err ? _self.err : err // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class PaymentError_InvalidInvoice extends PaymentError {
  const PaymentError_InvalidInvoice({required this.err}): super._();
  

 final  String err;

/// Create a copy of PaymentError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PaymentError_InvalidInvoiceCopyWith<PaymentError_InvalidInvoice> get copyWith => _$PaymentError_InvalidInvoiceCopyWithImpl<PaymentError_InvalidInvoice>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentError_InvalidInvoice&&(identical(other.err, err) || other.err == err));
}


@override
int get hashCode => Object.hash(runtimeType,err);

@override
String toString() {
  return 'PaymentError.invalidInvoice(err: $err)';
}


}

/// @nodoc
abstract mixin class $PaymentError_InvalidInvoiceCopyWith<$Res> implements $PaymentErrorCopyWith<$Res> {
  factory $PaymentError_InvalidInvoiceCopyWith(PaymentError_InvalidInvoice value, $Res Function(PaymentError_InvalidInvoice) _then) = _$PaymentError_InvalidInvoiceCopyWithImpl;
@useResult
$Res call({
 String err
});




}
/// @nodoc
class _$PaymentError_InvalidInvoiceCopyWithImpl<$Res>
    implements $PaymentError_InvalidInvoiceCopyWith<$Res> {
  _$PaymentError_InvalidInvoiceCopyWithImpl(this._self, this._then);

  final PaymentError_InvalidInvoice _self;
  final $Res Function(PaymentError_InvalidInvoice) _then;

/// Create a copy of PaymentError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? err = null,}) {
  return _then(PaymentError_InvalidInvoice(
err: null == err ? _self.err : err // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class PaymentError_InvalidPreimage extends PaymentError {
  const PaymentError_InvalidPreimage(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentError_InvalidPreimage);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'PaymentError.invalidPreimage()';
}


}




/// @nodoc


class PaymentError_PairsNotFound extends PaymentError {
  const PaymentError_PairsNotFound(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentError_PairsNotFound);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'PaymentError.pairsNotFound()';
}


}




/// @nodoc


class PaymentError_PaymentTimeout extends PaymentError {
  const PaymentError_PaymentTimeout(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentError_PaymentTimeout);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'PaymentError.paymentTimeout()';
}


}




/// @nodoc


class PaymentError_PersistError extends PaymentError {
  const PaymentError_PersistError(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentError_PersistError);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'PaymentError.persistError()';
}


}




/// @nodoc


class PaymentError_ReceiveError extends PaymentError {
  const PaymentError_ReceiveError({required this.err}): super._();
  

 final  String err;

/// Create a copy of PaymentError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PaymentError_ReceiveErrorCopyWith<PaymentError_ReceiveError> get copyWith => _$PaymentError_ReceiveErrorCopyWithImpl<PaymentError_ReceiveError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentError_ReceiveError&&(identical(other.err, err) || other.err == err));
}


@override
int get hashCode => Object.hash(runtimeType,err);

@override
String toString() {
  return 'PaymentError.receiveError(err: $err)';
}


}

/// @nodoc
abstract mixin class $PaymentError_ReceiveErrorCopyWith<$Res> implements $PaymentErrorCopyWith<$Res> {
  factory $PaymentError_ReceiveErrorCopyWith(PaymentError_ReceiveError value, $Res Function(PaymentError_ReceiveError) _then) = _$PaymentError_ReceiveErrorCopyWithImpl;
@useResult
$Res call({
 String err
});




}
/// @nodoc
class _$PaymentError_ReceiveErrorCopyWithImpl<$Res>
    implements $PaymentError_ReceiveErrorCopyWith<$Res> {
  _$PaymentError_ReceiveErrorCopyWithImpl(this._self, this._then);

  final PaymentError_ReceiveError _self;
  final $Res Function(PaymentError_ReceiveError) _then;

/// Create a copy of PaymentError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? err = null,}) {
  return _then(PaymentError_ReceiveError(
err: null == err ? _self.err : err // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class PaymentError_Refunded extends PaymentError {
  const PaymentError_Refunded({required this.err, required this.refundTxId}): super._();
  

 final  String err;
 final  String refundTxId;

/// Create a copy of PaymentError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PaymentError_RefundedCopyWith<PaymentError_Refunded> get copyWith => _$PaymentError_RefundedCopyWithImpl<PaymentError_Refunded>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentError_Refunded&&(identical(other.err, err) || other.err == err)&&(identical(other.refundTxId, refundTxId) || other.refundTxId == refundTxId));
}


@override
int get hashCode => Object.hash(runtimeType,err,refundTxId);

@override
String toString() {
  return 'PaymentError.refunded(err: $err, refundTxId: $refundTxId)';
}


}

/// @nodoc
abstract mixin class $PaymentError_RefundedCopyWith<$Res> implements $PaymentErrorCopyWith<$Res> {
  factory $PaymentError_RefundedCopyWith(PaymentError_Refunded value, $Res Function(PaymentError_Refunded) _then) = _$PaymentError_RefundedCopyWithImpl;
@useResult
$Res call({
 String err, String refundTxId
});




}
/// @nodoc
class _$PaymentError_RefundedCopyWithImpl<$Res>
    implements $PaymentError_RefundedCopyWith<$Res> {
  _$PaymentError_RefundedCopyWithImpl(this._self, this._then);

  final PaymentError_Refunded _self;
  final $Res Function(PaymentError_Refunded) _then;

/// Create a copy of PaymentError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? err = null,Object? refundTxId = null,}) {
  return _then(PaymentError_Refunded(
err: null == err ? _self.err : err // ignore: cast_nullable_to_non_nullable
as String,refundTxId: null == refundTxId ? _self.refundTxId : refundTxId // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class PaymentError_SelfTransferNotSupported extends PaymentError {
  const PaymentError_SelfTransferNotSupported(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentError_SelfTransferNotSupported);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'PaymentError.selfTransferNotSupported()';
}


}




/// @nodoc


class PaymentError_SendError extends PaymentError {
  const PaymentError_SendError({required this.err}): super._();
  

 final  String err;

/// Create a copy of PaymentError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PaymentError_SendErrorCopyWith<PaymentError_SendError> get copyWith => _$PaymentError_SendErrorCopyWithImpl<PaymentError_SendError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentError_SendError&&(identical(other.err, err) || other.err == err));
}


@override
int get hashCode => Object.hash(runtimeType,err);

@override
String toString() {
  return 'PaymentError.sendError(err: $err)';
}


}

/// @nodoc
abstract mixin class $PaymentError_SendErrorCopyWith<$Res> implements $PaymentErrorCopyWith<$Res> {
  factory $PaymentError_SendErrorCopyWith(PaymentError_SendError value, $Res Function(PaymentError_SendError) _then) = _$PaymentError_SendErrorCopyWithImpl;
@useResult
$Res call({
 String err
});




}
/// @nodoc
class _$PaymentError_SendErrorCopyWithImpl<$Res>
    implements $PaymentError_SendErrorCopyWith<$Res> {
  _$PaymentError_SendErrorCopyWithImpl(this._self, this._then);

  final PaymentError_SendError _self;
  final $Res Function(PaymentError_SendError) _then;

/// Create a copy of PaymentError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? err = null,}) {
  return _then(PaymentError_SendError(
err: null == err ? _self.err : err // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class PaymentError_SignerError extends PaymentError {
  const PaymentError_SignerError({required this.err}): super._();
  

 final  String err;

/// Create a copy of PaymentError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PaymentError_SignerErrorCopyWith<PaymentError_SignerError> get copyWith => _$PaymentError_SignerErrorCopyWithImpl<PaymentError_SignerError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PaymentError_SignerError&&(identical(other.err, err) || other.err == err));
}


@override
int get hashCode => Object.hash(runtimeType,err);

@override
String toString() {
  return 'PaymentError.signerError(err: $err)';
}


}

/// @nodoc
abstract mixin class $PaymentError_SignerErrorCopyWith<$Res> implements $PaymentErrorCopyWith<$Res> {
  factory $PaymentError_SignerErrorCopyWith(PaymentError_SignerError value, $Res Function(PaymentError_SignerError) _then) = _$PaymentError_SignerErrorCopyWithImpl;
@useResult
$Res call({
 String err
});




}
/// @nodoc
class _$PaymentError_SignerErrorCopyWithImpl<$Res>
    implements $PaymentError_SignerErrorCopyWith<$Res> {
  _$PaymentError_SignerErrorCopyWithImpl(this._self, this._then);

  final PaymentError_SignerError _self;
  final $Res Function(PaymentError_SignerError) _then;

/// Create a copy of PaymentError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? err = null,}) {
  return _then(PaymentError_SignerError(
err: null == err ? _self.err : err // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$SdkError {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SdkError);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'SdkError()';
}


}

/// @nodoc
class $SdkErrorCopyWith<$Res>  {
$SdkErrorCopyWith(SdkError _, $Res Function(SdkError) __);
}


/// @nodoc


class SdkError_AlreadyStarted extends SdkError {
  const SdkError_AlreadyStarted(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SdkError_AlreadyStarted);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'SdkError.alreadyStarted()';
}


}




/// @nodoc


class SdkError_Generic extends SdkError {
  const SdkError_Generic({required this.err}): super._();
  

 final  String err;

/// Create a copy of SdkError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SdkError_GenericCopyWith<SdkError_Generic> get copyWith => _$SdkError_GenericCopyWithImpl<SdkError_Generic>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SdkError_Generic&&(identical(other.err, err) || other.err == err));
}


@override
int get hashCode => Object.hash(runtimeType,err);

@override
String toString() {
  return 'SdkError.generic(err: $err)';
}


}

/// @nodoc
abstract mixin class $SdkError_GenericCopyWith<$Res> implements $SdkErrorCopyWith<$Res> {
  factory $SdkError_GenericCopyWith(SdkError_Generic value, $Res Function(SdkError_Generic) _then) = _$SdkError_GenericCopyWithImpl;
@useResult
$Res call({
 String err
});




}
/// @nodoc
class _$SdkError_GenericCopyWithImpl<$Res>
    implements $SdkError_GenericCopyWith<$Res> {
  _$SdkError_GenericCopyWithImpl(this._self, this._then);

  final SdkError_Generic _self;
  final $Res Function(SdkError_Generic) _then;

/// Create a copy of SdkError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? err = null,}) {
  return _then(SdkError_Generic(
err: null == err ? _self.err : err // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class SdkError_NotStarted extends SdkError {
  const SdkError_NotStarted(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SdkError_NotStarted);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'SdkError.notStarted()';
}


}




/// @nodoc


class SdkError_ServiceConnectivity extends SdkError {
  const SdkError_ServiceConnectivity({required this.err}): super._();
  

 final  String err;

/// Create a copy of SdkError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SdkError_ServiceConnectivityCopyWith<SdkError_ServiceConnectivity> get copyWith => _$SdkError_ServiceConnectivityCopyWithImpl<SdkError_ServiceConnectivity>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SdkError_ServiceConnectivity&&(identical(other.err, err) || other.err == err));
}


@override
int get hashCode => Object.hash(runtimeType,err);

@override
String toString() {
  return 'SdkError.serviceConnectivity(err: $err)';
}


}

/// @nodoc
abstract mixin class $SdkError_ServiceConnectivityCopyWith<$Res> implements $SdkErrorCopyWith<$Res> {
  factory $SdkError_ServiceConnectivityCopyWith(SdkError_ServiceConnectivity value, $Res Function(SdkError_ServiceConnectivity) _then) = _$SdkError_ServiceConnectivityCopyWithImpl;
@useResult
$Res call({
 String err
});




}
/// @nodoc
class _$SdkError_ServiceConnectivityCopyWithImpl<$Res>
    implements $SdkError_ServiceConnectivityCopyWith<$Res> {
  _$SdkError_ServiceConnectivityCopyWithImpl(this._self, this._then);

  final SdkError_ServiceConnectivity _self;
  final $Res Function(SdkError_ServiceConnectivity) _then;

/// Create a copy of SdkError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? err = null,}) {
  return _then(SdkError_ServiceConnectivity(
err: null == err ? _self.err : err // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

// dart format on
