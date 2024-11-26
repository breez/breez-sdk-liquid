// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'model.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$GetPaymentRequest {
  String get paymentHash => throw _privateConstructorUsedError;

  /// Create a copy of GetPaymentRequest
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $GetPaymentRequestCopyWith<GetPaymentRequest> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $GetPaymentRequestCopyWith<$Res> {
  factory $GetPaymentRequestCopyWith(GetPaymentRequest value, $Res Function(GetPaymentRequest) then) =
      _$GetPaymentRequestCopyWithImpl<$Res, GetPaymentRequest>;
  @useResult
  $Res call({String paymentHash});
}

/// @nodoc
class _$GetPaymentRequestCopyWithImpl<$Res, $Val extends GetPaymentRequest>
    implements $GetPaymentRequestCopyWith<$Res> {
  _$GetPaymentRequestCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of GetPaymentRequest
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? paymentHash = null,
  }) {
    return _then(_value.copyWith(
      paymentHash: null == paymentHash
          ? _value.paymentHash
          : paymentHash // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$GetPaymentRequest_LightningImplCopyWith<$Res> implements $GetPaymentRequestCopyWith<$Res> {
  factory _$$GetPaymentRequest_LightningImplCopyWith(
          _$GetPaymentRequest_LightningImpl value, $Res Function(_$GetPaymentRequest_LightningImpl) then) =
      __$$GetPaymentRequest_LightningImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String paymentHash});
}

/// @nodoc
class __$$GetPaymentRequest_LightningImplCopyWithImpl<$Res>
    extends _$GetPaymentRequestCopyWithImpl<$Res, _$GetPaymentRequest_LightningImpl>
    implements _$$GetPaymentRequest_LightningImplCopyWith<$Res> {
  __$$GetPaymentRequest_LightningImplCopyWithImpl(
      _$GetPaymentRequest_LightningImpl _value, $Res Function(_$GetPaymentRequest_LightningImpl) _then)
      : super(_value, _then);

  /// Create a copy of GetPaymentRequest
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? paymentHash = null,
  }) {
    return _then(_$GetPaymentRequest_LightningImpl(
      paymentHash: null == paymentHash
          ? _value.paymentHash
          : paymentHash // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$GetPaymentRequest_LightningImpl extends GetPaymentRequest_Lightning {
  const _$GetPaymentRequest_LightningImpl({required this.paymentHash}) : super._();

  @override
  final String paymentHash;

  @override
  String toString() {
    return 'GetPaymentRequest.lightning(paymentHash: $paymentHash)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$GetPaymentRequest_LightningImpl &&
            (identical(other.paymentHash, paymentHash) || other.paymentHash == paymentHash));
  }

  @override
  int get hashCode => Object.hash(runtimeType, paymentHash);

  /// Create a copy of GetPaymentRequest
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$GetPaymentRequest_LightningImplCopyWith<_$GetPaymentRequest_LightningImpl> get copyWith =>
      __$$GetPaymentRequest_LightningImplCopyWithImpl<_$GetPaymentRequest_LightningImpl>(this, _$identity);
}

abstract class GetPaymentRequest_Lightning extends GetPaymentRequest {
  const factory GetPaymentRequest_Lightning({required final String paymentHash}) =
      _$GetPaymentRequest_LightningImpl;
  const GetPaymentRequest_Lightning._() : super._();

  @override
  String get paymentHash;

  /// Create a copy of GetPaymentRequest
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$GetPaymentRequest_LightningImplCopyWith<_$GetPaymentRequest_LightningImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$ListPaymentDetails {}

/// @nodoc
abstract class $ListPaymentDetailsCopyWith<$Res> {
  factory $ListPaymentDetailsCopyWith(ListPaymentDetails value, $Res Function(ListPaymentDetails) then) =
      _$ListPaymentDetailsCopyWithImpl<$Res, ListPaymentDetails>;
}

/// @nodoc
class _$ListPaymentDetailsCopyWithImpl<$Res, $Val extends ListPaymentDetails>
    implements $ListPaymentDetailsCopyWith<$Res> {
  _$ListPaymentDetailsCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of ListPaymentDetails
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$ListPaymentDetails_LiquidImplCopyWith<$Res> {
  factory _$$ListPaymentDetails_LiquidImplCopyWith(
          _$ListPaymentDetails_LiquidImpl value, $Res Function(_$ListPaymentDetails_LiquidImpl) then) =
      __$$ListPaymentDetails_LiquidImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String destination});
}

/// @nodoc
class __$$ListPaymentDetails_LiquidImplCopyWithImpl<$Res>
    extends _$ListPaymentDetailsCopyWithImpl<$Res, _$ListPaymentDetails_LiquidImpl>
    implements _$$ListPaymentDetails_LiquidImplCopyWith<$Res> {
  __$$ListPaymentDetails_LiquidImplCopyWithImpl(
      _$ListPaymentDetails_LiquidImpl _value, $Res Function(_$ListPaymentDetails_LiquidImpl) _then)
      : super(_value, _then);

  /// Create a copy of ListPaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? destination = null,
  }) {
    return _then(_$ListPaymentDetails_LiquidImpl(
      destination: null == destination
          ? _value.destination
          : destination // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$ListPaymentDetails_LiquidImpl extends ListPaymentDetails_Liquid {
  const _$ListPaymentDetails_LiquidImpl({required this.destination}) : super._();

  @override
  final String destination;

  @override
  String toString() {
    return 'ListPaymentDetails.liquid(destination: $destination)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ListPaymentDetails_LiquidImpl &&
            (identical(other.destination, destination) || other.destination == destination));
  }

  @override
  int get hashCode => Object.hash(runtimeType, destination);

  /// Create a copy of ListPaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ListPaymentDetails_LiquidImplCopyWith<_$ListPaymentDetails_LiquidImpl> get copyWith =>
      __$$ListPaymentDetails_LiquidImplCopyWithImpl<_$ListPaymentDetails_LiquidImpl>(this, _$identity);
}

abstract class ListPaymentDetails_Liquid extends ListPaymentDetails {
  const factory ListPaymentDetails_Liquid({required final String destination}) =
      _$ListPaymentDetails_LiquidImpl;
  const ListPaymentDetails_Liquid._() : super._();

  String get destination;

  /// Create a copy of ListPaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ListPaymentDetails_LiquidImplCopyWith<_$ListPaymentDetails_LiquidImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ListPaymentDetails_BitcoinImplCopyWith<$Res> {
  factory _$$ListPaymentDetails_BitcoinImplCopyWith(
          _$ListPaymentDetails_BitcoinImpl value, $Res Function(_$ListPaymentDetails_BitcoinImpl) then) =
      __$$ListPaymentDetails_BitcoinImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String address});
}

/// @nodoc
class __$$ListPaymentDetails_BitcoinImplCopyWithImpl<$Res>
    extends _$ListPaymentDetailsCopyWithImpl<$Res, _$ListPaymentDetails_BitcoinImpl>
    implements _$$ListPaymentDetails_BitcoinImplCopyWith<$Res> {
  __$$ListPaymentDetails_BitcoinImplCopyWithImpl(
      _$ListPaymentDetails_BitcoinImpl _value, $Res Function(_$ListPaymentDetails_BitcoinImpl) _then)
      : super(_value, _then);

  /// Create a copy of ListPaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? address = null,
  }) {
    return _then(_$ListPaymentDetails_BitcoinImpl(
      address: null == address
          ? _value.address
          : address // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$ListPaymentDetails_BitcoinImpl extends ListPaymentDetails_Bitcoin {
  const _$ListPaymentDetails_BitcoinImpl({required this.address}) : super._();

  @override
  final String address;

  @override
  String toString() {
    return 'ListPaymentDetails.bitcoin(address: $address)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ListPaymentDetails_BitcoinImpl &&
            (identical(other.address, address) || other.address == address));
  }

  @override
  int get hashCode => Object.hash(runtimeType, address);

  /// Create a copy of ListPaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ListPaymentDetails_BitcoinImplCopyWith<_$ListPaymentDetails_BitcoinImpl> get copyWith =>
      __$$ListPaymentDetails_BitcoinImplCopyWithImpl<_$ListPaymentDetails_BitcoinImpl>(this, _$identity);
}

abstract class ListPaymentDetails_Bitcoin extends ListPaymentDetails {
  const factory ListPaymentDetails_Bitcoin({required final String address}) =
      _$ListPaymentDetails_BitcoinImpl;
  const ListPaymentDetails_Bitcoin._() : super._();

  String get address;

  /// Create a copy of ListPaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ListPaymentDetails_BitcoinImplCopyWith<_$ListPaymentDetails_BitcoinImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LnUrlPayResult {
  Object get data => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LnUrlPayResultCopyWith<$Res> {
  factory $LnUrlPayResultCopyWith(LnUrlPayResult value, $Res Function(LnUrlPayResult) then) =
      _$LnUrlPayResultCopyWithImpl<$Res, LnUrlPayResult>;
}

/// @nodoc
class _$LnUrlPayResultCopyWithImpl<$Res, $Val extends LnUrlPayResult>
    implements $LnUrlPayResultCopyWith<$Res> {
  _$LnUrlPayResultCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of LnUrlPayResult
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$LnUrlPayResult_EndpointSuccessImplCopyWith<$Res> {
  factory _$$LnUrlPayResult_EndpointSuccessImplCopyWith(_$LnUrlPayResult_EndpointSuccessImpl value,
          $Res Function(_$LnUrlPayResult_EndpointSuccessImpl) then) =
      __$$LnUrlPayResult_EndpointSuccessImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlPaySuccessData data});
}

/// @nodoc
class __$$LnUrlPayResult_EndpointSuccessImplCopyWithImpl<$Res>
    extends _$LnUrlPayResultCopyWithImpl<$Res, _$LnUrlPayResult_EndpointSuccessImpl>
    implements _$$LnUrlPayResult_EndpointSuccessImplCopyWith<$Res> {
  __$$LnUrlPayResult_EndpointSuccessImplCopyWithImpl(
      _$LnUrlPayResult_EndpointSuccessImpl _value, $Res Function(_$LnUrlPayResult_EndpointSuccessImpl) _then)
      : super(_value, _then);

  /// Create a copy of LnUrlPayResult
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$LnUrlPayResult_EndpointSuccessImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlPaySuccessData,
    ));
  }
}

/// @nodoc

class _$LnUrlPayResult_EndpointSuccessImpl extends LnUrlPayResult_EndpointSuccess {
  const _$LnUrlPayResult_EndpointSuccessImpl({required this.data}) : super._();

  @override
  final LnUrlPaySuccessData data;

  @override
  String toString() {
    return 'LnUrlPayResult.endpointSuccess(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayResult_EndpointSuccessImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  /// Create a copy of LnUrlPayResult
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayResult_EndpointSuccessImplCopyWith<_$LnUrlPayResult_EndpointSuccessImpl> get copyWith =>
      __$$LnUrlPayResult_EndpointSuccessImplCopyWithImpl<_$LnUrlPayResult_EndpointSuccessImpl>(
          this, _$identity);
}

abstract class LnUrlPayResult_EndpointSuccess extends LnUrlPayResult {
  const factory LnUrlPayResult_EndpointSuccess({required final LnUrlPaySuccessData data}) =
      _$LnUrlPayResult_EndpointSuccessImpl;
  const LnUrlPayResult_EndpointSuccess._() : super._();

  @override
  LnUrlPaySuccessData get data;

  /// Create a copy of LnUrlPayResult
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$LnUrlPayResult_EndpointSuccessImplCopyWith<_$LnUrlPayResult_EndpointSuccessImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlPayResult_EndpointErrorImplCopyWith<$Res> {
  factory _$$LnUrlPayResult_EndpointErrorImplCopyWith(
          _$LnUrlPayResult_EndpointErrorImpl value, $Res Function(_$LnUrlPayResult_EndpointErrorImpl) then) =
      __$$LnUrlPayResult_EndpointErrorImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlErrorData data});
}

/// @nodoc
class __$$LnUrlPayResult_EndpointErrorImplCopyWithImpl<$Res>
    extends _$LnUrlPayResultCopyWithImpl<$Res, _$LnUrlPayResult_EndpointErrorImpl>
    implements _$$LnUrlPayResult_EndpointErrorImplCopyWith<$Res> {
  __$$LnUrlPayResult_EndpointErrorImplCopyWithImpl(
      _$LnUrlPayResult_EndpointErrorImpl _value, $Res Function(_$LnUrlPayResult_EndpointErrorImpl) _then)
      : super(_value, _then);

  /// Create a copy of LnUrlPayResult
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$LnUrlPayResult_EndpointErrorImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlErrorData,
    ));
  }
}

/// @nodoc

class _$LnUrlPayResult_EndpointErrorImpl extends LnUrlPayResult_EndpointError {
  const _$LnUrlPayResult_EndpointErrorImpl({required this.data}) : super._();

  @override
  final LnUrlErrorData data;

  @override
  String toString() {
    return 'LnUrlPayResult.endpointError(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayResult_EndpointErrorImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  /// Create a copy of LnUrlPayResult
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayResult_EndpointErrorImplCopyWith<_$LnUrlPayResult_EndpointErrorImpl> get copyWith =>
      __$$LnUrlPayResult_EndpointErrorImplCopyWithImpl<_$LnUrlPayResult_EndpointErrorImpl>(this, _$identity);
}

abstract class LnUrlPayResult_EndpointError extends LnUrlPayResult {
  const factory LnUrlPayResult_EndpointError({required final LnUrlErrorData data}) =
      _$LnUrlPayResult_EndpointErrorImpl;
  const LnUrlPayResult_EndpointError._() : super._();

  @override
  LnUrlErrorData get data;

  /// Create a copy of LnUrlPayResult
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$LnUrlPayResult_EndpointErrorImplCopyWith<_$LnUrlPayResult_EndpointErrorImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlPayResult_PayErrorImplCopyWith<$Res> {
  factory _$$LnUrlPayResult_PayErrorImplCopyWith(
          _$LnUrlPayResult_PayErrorImpl value, $Res Function(_$LnUrlPayResult_PayErrorImpl) then) =
      __$$LnUrlPayResult_PayErrorImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlPayErrorData data});
}

/// @nodoc
class __$$LnUrlPayResult_PayErrorImplCopyWithImpl<$Res>
    extends _$LnUrlPayResultCopyWithImpl<$Res, _$LnUrlPayResult_PayErrorImpl>
    implements _$$LnUrlPayResult_PayErrorImplCopyWith<$Res> {
  __$$LnUrlPayResult_PayErrorImplCopyWithImpl(
      _$LnUrlPayResult_PayErrorImpl _value, $Res Function(_$LnUrlPayResult_PayErrorImpl) _then)
      : super(_value, _then);

  /// Create a copy of LnUrlPayResult
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$LnUrlPayResult_PayErrorImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlPayErrorData,
    ));
  }
}

/// @nodoc

class _$LnUrlPayResult_PayErrorImpl extends LnUrlPayResult_PayError {
  const _$LnUrlPayResult_PayErrorImpl({required this.data}) : super._();

  @override
  final LnUrlPayErrorData data;

  @override
  String toString() {
    return 'LnUrlPayResult.payError(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayResult_PayErrorImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  /// Create a copy of LnUrlPayResult
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayResult_PayErrorImplCopyWith<_$LnUrlPayResult_PayErrorImpl> get copyWith =>
      __$$LnUrlPayResult_PayErrorImplCopyWithImpl<_$LnUrlPayResult_PayErrorImpl>(this, _$identity);
}

abstract class LnUrlPayResult_PayError extends LnUrlPayResult {
  const factory LnUrlPayResult_PayError({required final LnUrlPayErrorData data}) =
      _$LnUrlPayResult_PayErrorImpl;
  const LnUrlPayResult_PayError._() : super._();

  @override
  LnUrlPayErrorData get data;

  /// Create a copy of LnUrlPayResult
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$LnUrlPayResult_PayErrorImplCopyWith<_$LnUrlPayResult_PayErrorImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$PayAmount {}

/// @nodoc
abstract class $PayAmountCopyWith<$Res> {
  factory $PayAmountCopyWith(PayAmount value, $Res Function(PayAmount) then) =
      _$PayAmountCopyWithImpl<$Res, PayAmount>;
}

/// @nodoc
class _$PayAmountCopyWithImpl<$Res, $Val extends PayAmount> implements $PayAmountCopyWith<$Res> {
  _$PayAmountCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of PayAmount
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$PayAmount_ReceiverImplCopyWith<$Res> {
  factory _$$PayAmount_ReceiverImplCopyWith(
          _$PayAmount_ReceiverImpl value, $Res Function(_$PayAmount_ReceiverImpl) then) =
      __$$PayAmount_ReceiverImplCopyWithImpl<$Res>;
  @useResult
  $Res call({BigInt amountSat});
}

/// @nodoc
class __$$PayAmount_ReceiverImplCopyWithImpl<$Res>
    extends _$PayAmountCopyWithImpl<$Res, _$PayAmount_ReceiverImpl>
    implements _$$PayAmount_ReceiverImplCopyWith<$Res> {
  __$$PayAmount_ReceiverImplCopyWithImpl(
      _$PayAmount_ReceiverImpl _value, $Res Function(_$PayAmount_ReceiverImpl) _then)
      : super(_value, _then);

  /// Create a copy of PayAmount
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? amountSat = null,
  }) {
    return _then(_$PayAmount_ReceiverImpl(
      amountSat: null == amountSat
          ? _value.amountSat
          : amountSat // ignore: cast_nullable_to_non_nullable
              as BigInt,
    ));
  }
}

/// @nodoc

class _$PayAmount_ReceiverImpl extends PayAmount_Receiver {
  const _$PayAmount_ReceiverImpl({required this.amountSat}) : super._();

  @override
  final BigInt amountSat;

  @override
  String toString() {
    return 'PayAmount.receiver(amountSat: $amountSat)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PayAmount_ReceiverImpl &&
            (identical(other.amountSat, amountSat) || other.amountSat == amountSat));
  }

  @override
  int get hashCode => Object.hash(runtimeType, amountSat);

  /// Create a copy of PayAmount
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PayAmount_ReceiverImplCopyWith<_$PayAmount_ReceiverImpl> get copyWith =>
      __$$PayAmount_ReceiverImplCopyWithImpl<_$PayAmount_ReceiverImpl>(this, _$identity);
}

abstract class PayAmount_Receiver extends PayAmount {
  const factory PayAmount_Receiver({required final BigInt amountSat}) = _$PayAmount_ReceiverImpl;
  const PayAmount_Receiver._() : super._();

  BigInt get amountSat;

  /// Create a copy of PayAmount
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$PayAmount_ReceiverImplCopyWith<_$PayAmount_ReceiverImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PayAmount_DrainImplCopyWith<$Res> {
  factory _$$PayAmount_DrainImplCopyWith(
          _$PayAmount_DrainImpl value, $Res Function(_$PayAmount_DrainImpl) then) =
      __$$PayAmount_DrainImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$PayAmount_DrainImplCopyWithImpl<$Res> extends _$PayAmountCopyWithImpl<$Res, _$PayAmount_DrainImpl>
    implements _$$PayAmount_DrainImplCopyWith<$Res> {
  __$$PayAmount_DrainImplCopyWithImpl(
      _$PayAmount_DrainImpl _value, $Res Function(_$PayAmount_DrainImpl) _then)
      : super(_value, _then);

  /// Create a copy of PayAmount
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$PayAmount_DrainImpl extends PayAmount_Drain {
  const _$PayAmount_DrainImpl() : super._();

  @override
  String toString() {
    return 'PayAmount.drain()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$PayAmount_DrainImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class PayAmount_Drain extends PayAmount {
  const factory PayAmount_Drain() = _$PayAmount_DrainImpl;
  const PayAmount_Drain._() : super._();
}

/// @nodoc
mixin _$PaymentDetails {
  /// Represents the invoice description
  String get description => throw _privateConstructorUsedError;

  /// Create a copy of PaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $PaymentDetailsCopyWith<PaymentDetails> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $PaymentDetailsCopyWith<$Res> {
  factory $PaymentDetailsCopyWith(PaymentDetails value, $Res Function(PaymentDetails) then) =
      _$PaymentDetailsCopyWithImpl<$Res, PaymentDetails>;
  @useResult
  $Res call({String description});
}

/// @nodoc
class _$PaymentDetailsCopyWithImpl<$Res, $Val extends PaymentDetails>
    implements $PaymentDetailsCopyWith<$Res> {
  _$PaymentDetailsCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of PaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? description = null,
  }) {
    return _then(_value.copyWith(
      description: null == description
          ? _value.description
          : description // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$PaymentDetails_LightningImplCopyWith<$Res> implements $PaymentDetailsCopyWith<$Res> {
  factory _$$PaymentDetails_LightningImplCopyWith(
          _$PaymentDetails_LightningImpl value, $Res Function(_$PaymentDetails_LightningImpl) then) =
      __$$PaymentDetails_LightningImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String swapId,
      String description,
      String? preimage,
      String? bolt11,
      String? bolt12Offer,
      String? paymentHash,
      String? refundTxId,
      BigInt? refundTxAmountSat});
}

/// @nodoc
class __$$PaymentDetails_LightningImplCopyWithImpl<$Res>
    extends _$PaymentDetailsCopyWithImpl<$Res, _$PaymentDetails_LightningImpl>
    implements _$$PaymentDetails_LightningImplCopyWith<$Res> {
  __$$PaymentDetails_LightningImplCopyWithImpl(
      _$PaymentDetails_LightningImpl _value, $Res Function(_$PaymentDetails_LightningImpl) _then)
      : super(_value, _then);

  /// Create a copy of PaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? swapId = null,
    Object? description = null,
    Object? preimage = freezed,
    Object? bolt11 = freezed,
    Object? bolt12Offer = freezed,
    Object? paymentHash = freezed,
    Object? refundTxId = freezed,
    Object? refundTxAmountSat = freezed,
  }) {
    return _then(_$PaymentDetails_LightningImpl(
      swapId: null == swapId
          ? _value.swapId
          : swapId // ignore: cast_nullable_to_non_nullable
              as String,
      description: null == description
          ? _value.description
          : description // ignore: cast_nullable_to_non_nullable
              as String,
      preimage: freezed == preimage
          ? _value.preimage
          : preimage // ignore: cast_nullable_to_non_nullable
              as String?,
      bolt11: freezed == bolt11
          ? _value.bolt11
          : bolt11 // ignore: cast_nullable_to_non_nullable
              as String?,
      bolt12Offer: freezed == bolt12Offer
          ? _value.bolt12Offer
          : bolt12Offer // ignore: cast_nullable_to_non_nullable
              as String?,
      paymentHash: freezed == paymentHash
          ? _value.paymentHash
          : paymentHash // ignore: cast_nullable_to_non_nullable
              as String?,
      refundTxId: freezed == refundTxId
          ? _value.refundTxId
          : refundTxId // ignore: cast_nullable_to_non_nullable
              as String?,
      refundTxAmountSat: freezed == refundTxAmountSat
          ? _value.refundTxAmountSat
          : refundTxAmountSat // ignore: cast_nullable_to_non_nullable
              as BigInt?,
    ));
  }
}

/// @nodoc

class _$PaymentDetails_LightningImpl extends PaymentDetails_Lightning {
  const _$PaymentDetails_LightningImpl(
      {required this.swapId,
      required this.description,
      this.preimage,
      this.bolt11,
      this.bolt12Offer,
      this.paymentHash,
      this.refundTxId,
      this.refundTxAmountSat})
      : super._();

  @override
  final String swapId;

  /// Represents the invoice description
  @override
  final String description;

  /// In case of a Send swap, this is the preimage of the paid invoice (proof of payment).
  @override
  final String? preimage;

  /// Represents the Bolt11 invoice associated with a payment
  /// In the case of a Send payment, this is the invoice paid by the swapper
  /// In the case of a Receive payment, this is the invoice paid by the user
  @override
  final String? bolt11;
  @override
  final String? bolt12Offer;

  /// The payment hash of the invoice
  @override
  final String? paymentHash;

  /// For a Send swap which was refunded, this is the refund tx id
  @override
  final String? refundTxId;

  /// For a Send swap which was refunded, this is the refund amount
  @override
  final BigInt? refundTxAmountSat;

  @override
  String toString() {
    return 'PaymentDetails.lightning(swapId: $swapId, description: $description, preimage: $preimage, bolt11: $bolt11, bolt12Offer: $bolt12Offer, paymentHash: $paymentHash, refundTxId: $refundTxId, refundTxAmountSat: $refundTxAmountSat)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PaymentDetails_LightningImpl &&
            (identical(other.swapId, swapId) || other.swapId == swapId) &&
            (identical(other.description, description) || other.description == description) &&
            (identical(other.preimage, preimage) || other.preimage == preimage) &&
            (identical(other.bolt11, bolt11) || other.bolt11 == bolt11) &&
            (identical(other.bolt12Offer, bolt12Offer) || other.bolt12Offer == bolt12Offer) &&
            (identical(other.paymentHash, paymentHash) || other.paymentHash == paymentHash) &&
            (identical(other.refundTxId, refundTxId) || other.refundTxId == refundTxId) &&
            (identical(other.refundTxAmountSat, refundTxAmountSat) ||
                other.refundTxAmountSat == refundTxAmountSat));
  }

  @override
  int get hashCode => Object.hash(runtimeType, swapId, description, preimage, bolt11, bolt12Offer,
      paymentHash, refundTxId, refundTxAmountSat);

  /// Create a copy of PaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PaymentDetails_LightningImplCopyWith<_$PaymentDetails_LightningImpl> get copyWith =>
      __$$PaymentDetails_LightningImplCopyWithImpl<_$PaymentDetails_LightningImpl>(this, _$identity);
}

abstract class PaymentDetails_Lightning extends PaymentDetails {
  const factory PaymentDetails_Lightning(
      {required final String swapId,
      required final String description,
      final String? preimage,
      final String? bolt11,
      final String? bolt12Offer,
      final String? paymentHash,
      final String? refundTxId,
      final BigInt? refundTxAmountSat}) = _$PaymentDetails_LightningImpl;
  const PaymentDetails_Lightning._() : super._();

  String get swapId;

  /// Represents the invoice description
  @override
  String get description;

  /// In case of a Send swap, this is the preimage of the paid invoice (proof of payment).
  String? get preimage;

  /// Represents the Bolt11 invoice associated with a payment
  /// In the case of a Send payment, this is the invoice paid by the swapper
  /// In the case of a Receive payment, this is the invoice paid by the user
  String? get bolt11;
  String? get bolt12Offer;

  /// The payment hash of the invoice
  String? get paymentHash;

  /// For a Send swap which was refunded, this is the refund tx id
  String? get refundTxId;

  /// For a Send swap which was refunded, this is the refund amount
  BigInt? get refundTxAmountSat;

  /// Create a copy of PaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$PaymentDetails_LightningImplCopyWith<_$PaymentDetails_LightningImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PaymentDetails_LiquidImplCopyWith<$Res> implements $PaymentDetailsCopyWith<$Res> {
  factory _$$PaymentDetails_LiquidImplCopyWith(
          _$PaymentDetails_LiquidImpl value, $Res Function(_$PaymentDetails_LiquidImpl) then) =
      __$$PaymentDetails_LiquidImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String destination, String description});
}

/// @nodoc
class __$$PaymentDetails_LiquidImplCopyWithImpl<$Res>
    extends _$PaymentDetailsCopyWithImpl<$Res, _$PaymentDetails_LiquidImpl>
    implements _$$PaymentDetails_LiquidImplCopyWith<$Res> {
  __$$PaymentDetails_LiquidImplCopyWithImpl(
      _$PaymentDetails_LiquidImpl _value, $Res Function(_$PaymentDetails_LiquidImpl) _then)
      : super(_value, _then);

  /// Create a copy of PaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? destination = null,
    Object? description = null,
  }) {
    return _then(_$PaymentDetails_LiquidImpl(
      destination: null == destination
          ? _value.destination
          : destination // ignore: cast_nullable_to_non_nullable
              as String,
      description: null == description
          ? _value.description
          : description // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$PaymentDetails_LiquidImpl extends PaymentDetails_Liquid {
  const _$PaymentDetails_LiquidImpl({required this.destination, required this.description}) : super._();

  /// Represents either a Liquid BIP21 URI or pure address
  @override
  final String destination;

  /// Represents the BIP21 `message` field
  @override
  final String description;

  @override
  String toString() {
    return 'PaymentDetails.liquid(destination: $destination, description: $description)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PaymentDetails_LiquidImpl &&
            (identical(other.destination, destination) || other.destination == destination) &&
            (identical(other.description, description) || other.description == description));
  }

  @override
  int get hashCode => Object.hash(runtimeType, destination, description);

  /// Create a copy of PaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PaymentDetails_LiquidImplCopyWith<_$PaymentDetails_LiquidImpl> get copyWith =>
      __$$PaymentDetails_LiquidImplCopyWithImpl<_$PaymentDetails_LiquidImpl>(this, _$identity);
}

abstract class PaymentDetails_Liquid extends PaymentDetails {
  const factory PaymentDetails_Liquid(
      {required final String destination, required final String description}) = _$PaymentDetails_LiquidImpl;
  const PaymentDetails_Liquid._() : super._();

  /// Represents either a Liquid BIP21 URI or pure address
  String get destination;

  /// Represents the BIP21 `message` field
  @override
  String get description;

  /// Create a copy of PaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$PaymentDetails_LiquidImplCopyWith<_$PaymentDetails_LiquidImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PaymentDetails_BitcoinImplCopyWith<$Res> implements $PaymentDetailsCopyWith<$Res> {
  factory _$$PaymentDetails_BitcoinImplCopyWith(
          _$PaymentDetails_BitcoinImpl value, $Res Function(_$PaymentDetails_BitcoinImpl) then) =
      __$$PaymentDetails_BitcoinImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String swapId, String description, String? refundTxId, BigInt? refundTxAmountSat});
}

/// @nodoc
class __$$PaymentDetails_BitcoinImplCopyWithImpl<$Res>
    extends _$PaymentDetailsCopyWithImpl<$Res, _$PaymentDetails_BitcoinImpl>
    implements _$$PaymentDetails_BitcoinImplCopyWith<$Res> {
  __$$PaymentDetails_BitcoinImplCopyWithImpl(
      _$PaymentDetails_BitcoinImpl _value, $Res Function(_$PaymentDetails_BitcoinImpl) _then)
      : super(_value, _then);

  /// Create a copy of PaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? swapId = null,
    Object? description = null,
    Object? refundTxId = freezed,
    Object? refundTxAmountSat = freezed,
  }) {
    return _then(_$PaymentDetails_BitcoinImpl(
      swapId: null == swapId
          ? _value.swapId
          : swapId // ignore: cast_nullable_to_non_nullable
              as String,
      description: null == description
          ? _value.description
          : description // ignore: cast_nullable_to_non_nullable
              as String,
      refundTxId: freezed == refundTxId
          ? _value.refundTxId
          : refundTxId // ignore: cast_nullable_to_non_nullable
              as String?,
      refundTxAmountSat: freezed == refundTxAmountSat
          ? _value.refundTxAmountSat
          : refundTxAmountSat // ignore: cast_nullable_to_non_nullable
              as BigInt?,
    ));
  }
}

/// @nodoc

class _$PaymentDetails_BitcoinImpl extends PaymentDetails_Bitcoin {
  const _$PaymentDetails_BitcoinImpl(
      {required this.swapId, required this.description, this.refundTxId, this.refundTxAmountSat})
      : super._();

  @override
  final String swapId;

  /// Represents the invoice description
  @override
  final String description;

  /// For a Send swap which was refunded, this is the refund tx id
  @override
  final String? refundTxId;

  /// For a Send swap which was refunded, this is the refund amount
  @override
  final BigInt? refundTxAmountSat;

  @override
  String toString() {
    return 'PaymentDetails.bitcoin(swapId: $swapId, description: $description, refundTxId: $refundTxId, refundTxAmountSat: $refundTxAmountSat)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PaymentDetails_BitcoinImpl &&
            (identical(other.swapId, swapId) || other.swapId == swapId) &&
            (identical(other.description, description) || other.description == description) &&
            (identical(other.refundTxId, refundTxId) || other.refundTxId == refundTxId) &&
            (identical(other.refundTxAmountSat, refundTxAmountSat) ||
                other.refundTxAmountSat == refundTxAmountSat));
  }

  @override
  int get hashCode => Object.hash(runtimeType, swapId, description, refundTxId, refundTxAmountSat);

  /// Create a copy of PaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PaymentDetails_BitcoinImplCopyWith<_$PaymentDetails_BitcoinImpl> get copyWith =>
      __$$PaymentDetails_BitcoinImplCopyWithImpl<_$PaymentDetails_BitcoinImpl>(this, _$identity);
}

abstract class PaymentDetails_Bitcoin extends PaymentDetails {
  const factory PaymentDetails_Bitcoin(
      {required final String swapId,
      required final String description,
      final String? refundTxId,
      final BigInt? refundTxAmountSat}) = _$PaymentDetails_BitcoinImpl;
  const PaymentDetails_Bitcoin._() : super._();

  String get swapId;

  /// Represents the invoice description
  @override
  String get description;

  /// For a Send swap which was refunded, this is the refund tx id
  String? get refundTxId;

  /// For a Send swap which was refunded, this is the refund amount
  BigInt? get refundTxAmountSat;

  /// Create a copy of PaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$PaymentDetails_BitcoinImplCopyWith<_$PaymentDetails_BitcoinImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$SdkEvent {}

/// @nodoc
abstract class $SdkEventCopyWith<$Res> {
  factory $SdkEventCopyWith(SdkEvent value, $Res Function(SdkEvent) then) =
      _$SdkEventCopyWithImpl<$Res, SdkEvent>;
}

/// @nodoc
class _$SdkEventCopyWithImpl<$Res, $Val extends SdkEvent> implements $SdkEventCopyWith<$Res> {
  _$SdkEventCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$SdkEvent_PaymentFailedImplCopyWith<$Res> {
  factory _$$SdkEvent_PaymentFailedImplCopyWith(
          _$SdkEvent_PaymentFailedImpl value, $Res Function(_$SdkEvent_PaymentFailedImpl) then) =
      __$$SdkEvent_PaymentFailedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$SdkEvent_PaymentFailedImplCopyWithImpl<$Res>
    extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_PaymentFailedImpl>
    implements _$$SdkEvent_PaymentFailedImplCopyWith<$Res> {
  __$$SdkEvent_PaymentFailedImplCopyWithImpl(
      _$SdkEvent_PaymentFailedImpl _value, $Res Function(_$SdkEvent_PaymentFailedImpl) _then)
      : super(_value, _then);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$SdkEvent_PaymentFailedImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as Payment,
    ));
  }
}

/// @nodoc

class _$SdkEvent_PaymentFailedImpl extends SdkEvent_PaymentFailed {
  const _$SdkEvent_PaymentFailedImpl({required this.details}) : super._();

  @override
  final Payment details;

  @override
  String toString() {
    return 'SdkEvent.paymentFailed(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SdkEvent_PaymentFailedImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$SdkEvent_PaymentFailedImplCopyWith<_$SdkEvent_PaymentFailedImpl> get copyWith =>
      __$$SdkEvent_PaymentFailedImplCopyWithImpl<_$SdkEvent_PaymentFailedImpl>(this, _$identity);
}

abstract class SdkEvent_PaymentFailed extends SdkEvent {
  const factory SdkEvent_PaymentFailed({required final Payment details}) = _$SdkEvent_PaymentFailedImpl;
  const SdkEvent_PaymentFailed._() : super._();

  Payment get details;

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$SdkEvent_PaymentFailedImplCopyWith<_$SdkEvent_PaymentFailedImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SdkEvent_PaymentPendingImplCopyWith<$Res> {
  factory _$$SdkEvent_PaymentPendingImplCopyWith(
          _$SdkEvent_PaymentPendingImpl value, $Res Function(_$SdkEvent_PaymentPendingImpl) then) =
      __$$SdkEvent_PaymentPendingImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$SdkEvent_PaymentPendingImplCopyWithImpl<$Res>
    extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_PaymentPendingImpl>
    implements _$$SdkEvent_PaymentPendingImplCopyWith<$Res> {
  __$$SdkEvent_PaymentPendingImplCopyWithImpl(
      _$SdkEvent_PaymentPendingImpl _value, $Res Function(_$SdkEvent_PaymentPendingImpl) _then)
      : super(_value, _then);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$SdkEvent_PaymentPendingImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as Payment,
    ));
  }
}

/// @nodoc

class _$SdkEvent_PaymentPendingImpl extends SdkEvent_PaymentPending {
  const _$SdkEvent_PaymentPendingImpl({required this.details}) : super._();

  @override
  final Payment details;

  @override
  String toString() {
    return 'SdkEvent.paymentPending(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SdkEvent_PaymentPendingImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$SdkEvent_PaymentPendingImplCopyWith<_$SdkEvent_PaymentPendingImpl> get copyWith =>
      __$$SdkEvent_PaymentPendingImplCopyWithImpl<_$SdkEvent_PaymentPendingImpl>(this, _$identity);
}

abstract class SdkEvent_PaymentPending extends SdkEvent {
  const factory SdkEvent_PaymentPending({required final Payment details}) = _$SdkEvent_PaymentPendingImpl;
  const SdkEvent_PaymentPending._() : super._();

  Payment get details;

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$SdkEvent_PaymentPendingImplCopyWith<_$SdkEvent_PaymentPendingImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SdkEvent_PaymentRefundedImplCopyWith<$Res> {
  factory _$$SdkEvent_PaymentRefundedImplCopyWith(
          _$SdkEvent_PaymentRefundedImpl value, $Res Function(_$SdkEvent_PaymentRefundedImpl) then) =
      __$$SdkEvent_PaymentRefundedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$SdkEvent_PaymentRefundedImplCopyWithImpl<$Res>
    extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_PaymentRefundedImpl>
    implements _$$SdkEvent_PaymentRefundedImplCopyWith<$Res> {
  __$$SdkEvent_PaymentRefundedImplCopyWithImpl(
      _$SdkEvent_PaymentRefundedImpl _value, $Res Function(_$SdkEvent_PaymentRefundedImpl) _then)
      : super(_value, _then);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$SdkEvent_PaymentRefundedImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as Payment,
    ));
  }
}

/// @nodoc

class _$SdkEvent_PaymentRefundedImpl extends SdkEvent_PaymentRefunded {
  const _$SdkEvent_PaymentRefundedImpl({required this.details}) : super._();

  @override
  final Payment details;

  @override
  String toString() {
    return 'SdkEvent.paymentRefunded(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SdkEvent_PaymentRefundedImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$SdkEvent_PaymentRefundedImplCopyWith<_$SdkEvent_PaymentRefundedImpl> get copyWith =>
      __$$SdkEvent_PaymentRefundedImplCopyWithImpl<_$SdkEvent_PaymentRefundedImpl>(this, _$identity);
}

abstract class SdkEvent_PaymentRefunded extends SdkEvent {
  const factory SdkEvent_PaymentRefunded({required final Payment details}) = _$SdkEvent_PaymentRefundedImpl;
  const SdkEvent_PaymentRefunded._() : super._();

  Payment get details;

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$SdkEvent_PaymentRefundedImplCopyWith<_$SdkEvent_PaymentRefundedImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SdkEvent_PaymentRefundPendingImplCopyWith<$Res> {
  factory _$$SdkEvent_PaymentRefundPendingImplCopyWith(_$SdkEvent_PaymentRefundPendingImpl value,
          $Res Function(_$SdkEvent_PaymentRefundPendingImpl) then) =
      __$$SdkEvent_PaymentRefundPendingImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$SdkEvent_PaymentRefundPendingImplCopyWithImpl<$Res>
    extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_PaymentRefundPendingImpl>
    implements _$$SdkEvent_PaymentRefundPendingImplCopyWith<$Res> {
  __$$SdkEvent_PaymentRefundPendingImplCopyWithImpl(
      _$SdkEvent_PaymentRefundPendingImpl _value, $Res Function(_$SdkEvent_PaymentRefundPendingImpl) _then)
      : super(_value, _then);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$SdkEvent_PaymentRefundPendingImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as Payment,
    ));
  }
}

/// @nodoc

class _$SdkEvent_PaymentRefundPendingImpl extends SdkEvent_PaymentRefundPending {
  const _$SdkEvent_PaymentRefundPendingImpl({required this.details}) : super._();

  @override
  final Payment details;

  @override
  String toString() {
    return 'SdkEvent.paymentRefundPending(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SdkEvent_PaymentRefundPendingImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$SdkEvent_PaymentRefundPendingImplCopyWith<_$SdkEvent_PaymentRefundPendingImpl> get copyWith =>
      __$$SdkEvent_PaymentRefundPendingImplCopyWithImpl<_$SdkEvent_PaymentRefundPendingImpl>(
          this, _$identity);
}

abstract class SdkEvent_PaymentRefundPending extends SdkEvent {
  const factory SdkEvent_PaymentRefundPending({required final Payment details}) =
      _$SdkEvent_PaymentRefundPendingImpl;
  const SdkEvent_PaymentRefundPending._() : super._();

  Payment get details;

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$SdkEvent_PaymentRefundPendingImplCopyWith<_$SdkEvent_PaymentRefundPendingImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SdkEvent_PaymentSucceededImplCopyWith<$Res> {
  factory _$$SdkEvent_PaymentSucceededImplCopyWith(
          _$SdkEvent_PaymentSucceededImpl value, $Res Function(_$SdkEvent_PaymentSucceededImpl) then) =
      __$$SdkEvent_PaymentSucceededImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$SdkEvent_PaymentSucceededImplCopyWithImpl<$Res>
    extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_PaymentSucceededImpl>
    implements _$$SdkEvent_PaymentSucceededImplCopyWith<$Res> {
  __$$SdkEvent_PaymentSucceededImplCopyWithImpl(
      _$SdkEvent_PaymentSucceededImpl _value, $Res Function(_$SdkEvent_PaymentSucceededImpl) _then)
      : super(_value, _then);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$SdkEvent_PaymentSucceededImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as Payment,
    ));
  }
}

/// @nodoc

class _$SdkEvent_PaymentSucceededImpl extends SdkEvent_PaymentSucceeded {
  const _$SdkEvent_PaymentSucceededImpl({required this.details}) : super._();

  @override
  final Payment details;

  @override
  String toString() {
    return 'SdkEvent.paymentSucceeded(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SdkEvent_PaymentSucceededImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$SdkEvent_PaymentSucceededImplCopyWith<_$SdkEvent_PaymentSucceededImpl> get copyWith =>
      __$$SdkEvent_PaymentSucceededImplCopyWithImpl<_$SdkEvent_PaymentSucceededImpl>(this, _$identity);
}

abstract class SdkEvent_PaymentSucceeded extends SdkEvent {
  const factory SdkEvent_PaymentSucceeded({required final Payment details}) = _$SdkEvent_PaymentSucceededImpl;
  const SdkEvent_PaymentSucceeded._() : super._();

  Payment get details;

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$SdkEvent_PaymentSucceededImplCopyWith<_$SdkEvent_PaymentSucceededImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SdkEvent_PaymentWaitingConfirmationImplCopyWith<$Res> {
  factory _$$SdkEvent_PaymentWaitingConfirmationImplCopyWith(_$SdkEvent_PaymentWaitingConfirmationImpl value,
          $Res Function(_$SdkEvent_PaymentWaitingConfirmationImpl) then) =
      __$$SdkEvent_PaymentWaitingConfirmationImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$SdkEvent_PaymentWaitingConfirmationImplCopyWithImpl<$Res>
    extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_PaymentWaitingConfirmationImpl>
    implements _$$SdkEvent_PaymentWaitingConfirmationImplCopyWith<$Res> {
  __$$SdkEvent_PaymentWaitingConfirmationImplCopyWithImpl(_$SdkEvent_PaymentWaitingConfirmationImpl _value,
      $Res Function(_$SdkEvent_PaymentWaitingConfirmationImpl) _then)
      : super(_value, _then);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$SdkEvent_PaymentWaitingConfirmationImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as Payment,
    ));
  }
}

/// @nodoc

class _$SdkEvent_PaymentWaitingConfirmationImpl extends SdkEvent_PaymentWaitingConfirmation {
  const _$SdkEvent_PaymentWaitingConfirmationImpl({required this.details}) : super._();

  @override
  final Payment details;

  @override
  String toString() {
    return 'SdkEvent.paymentWaitingConfirmation(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SdkEvent_PaymentWaitingConfirmationImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$SdkEvent_PaymentWaitingConfirmationImplCopyWith<_$SdkEvent_PaymentWaitingConfirmationImpl>
      get copyWith =>
          __$$SdkEvent_PaymentWaitingConfirmationImplCopyWithImpl<_$SdkEvent_PaymentWaitingConfirmationImpl>(
              this, _$identity);
}

abstract class SdkEvent_PaymentWaitingConfirmation extends SdkEvent {
  const factory SdkEvent_PaymentWaitingConfirmation({required final Payment details}) =
      _$SdkEvent_PaymentWaitingConfirmationImpl;
  const SdkEvent_PaymentWaitingConfirmation._() : super._();

  Payment get details;

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$SdkEvent_PaymentWaitingConfirmationImplCopyWith<_$SdkEvent_PaymentWaitingConfirmationImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SdkEvent_SyncedImplCopyWith<$Res> {
  factory _$$SdkEvent_SyncedImplCopyWith(
          _$SdkEvent_SyncedImpl value, $Res Function(_$SdkEvent_SyncedImpl) then) =
      __$$SdkEvent_SyncedImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$SdkEvent_SyncedImplCopyWithImpl<$Res> extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_SyncedImpl>
    implements _$$SdkEvent_SyncedImplCopyWith<$Res> {
  __$$SdkEvent_SyncedImplCopyWithImpl(
      _$SdkEvent_SyncedImpl _value, $Res Function(_$SdkEvent_SyncedImpl) _then)
      : super(_value, _then);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$SdkEvent_SyncedImpl extends SdkEvent_Synced {
  const _$SdkEvent_SyncedImpl() : super._();

  @override
  String toString() {
    return 'SdkEvent.synced()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$SdkEvent_SyncedImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class SdkEvent_Synced extends SdkEvent {
  const factory SdkEvent_Synced() = _$SdkEvent_SyncedImpl;
  const SdkEvent_Synced._() : super._();
}

/// @nodoc
mixin _$SendDestination {}

/// @nodoc
abstract class $SendDestinationCopyWith<$Res> {
  factory $SendDestinationCopyWith(SendDestination value, $Res Function(SendDestination) then) =
      _$SendDestinationCopyWithImpl<$Res, SendDestination>;
}

/// @nodoc
class _$SendDestinationCopyWithImpl<$Res, $Val extends SendDestination>
    implements $SendDestinationCopyWith<$Res> {
  _$SendDestinationCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of SendDestination
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$SendDestination_LiquidAddressImplCopyWith<$Res> {
  factory _$$SendDestination_LiquidAddressImplCopyWith(_$SendDestination_LiquidAddressImpl value,
          $Res Function(_$SendDestination_LiquidAddressImpl) then) =
      __$$SendDestination_LiquidAddressImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LiquidAddressData addressData});
}

/// @nodoc
class __$$SendDestination_LiquidAddressImplCopyWithImpl<$Res>
    extends _$SendDestinationCopyWithImpl<$Res, _$SendDestination_LiquidAddressImpl>
    implements _$$SendDestination_LiquidAddressImplCopyWith<$Res> {
  __$$SendDestination_LiquidAddressImplCopyWithImpl(
      _$SendDestination_LiquidAddressImpl _value, $Res Function(_$SendDestination_LiquidAddressImpl) _then)
      : super(_value, _then);

  /// Create a copy of SendDestination
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? addressData = null,
  }) {
    return _then(_$SendDestination_LiquidAddressImpl(
      addressData: null == addressData
          ? _value.addressData
          : addressData // ignore: cast_nullable_to_non_nullable
              as LiquidAddressData,
    ));
  }
}

/// @nodoc

class _$SendDestination_LiquidAddressImpl extends SendDestination_LiquidAddress {
  const _$SendDestination_LiquidAddressImpl({required this.addressData}) : super._();

  @override
  final LiquidAddressData addressData;

  @override
  String toString() {
    return 'SendDestination.liquidAddress(addressData: $addressData)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SendDestination_LiquidAddressImpl &&
            (identical(other.addressData, addressData) || other.addressData == addressData));
  }

  @override
  int get hashCode => Object.hash(runtimeType, addressData);

  /// Create a copy of SendDestination
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$SendDestination_LiquidAddressImplCopyWith<_$SendDestination_LiquidAddressImpl> get copyWith =>
      __$$SendDestination_LiquidAddressImplCopyWithImpl<_$SendDestination_LiquidAddressImpl>(
          this, _$identity);
}

abstract class SendDestination_LiquidAddress extends SendDestination {
  const factory SendDestination_LiquidAddress({required final LiquidAddressData addressData}) =
      _$SendDestination_LiquidAddressImpl;
  const SendDestination_LiquidAddress._() : super._();

  LiquidAddressData get addressData;

  /// Create a copy of SendDestination
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$SendDestination_LiquidAddressImplCopyWith<_$SendDestination_LiquidAddressImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SendDestination_Bolt11ImplCopyWith<$Res> {
  factory _$$SendDestination_Bolt11ImplCopyWith(
          _$SendDestination_Bolt11Impl value, $Res Function(_$SendDestination_Bolt11Impl) then) =
      __$$SendDestination_Bolt11ImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LNInvoice invoice});
}

/// @nodoc
class __$$SendDestination_Bolt11ImplCopyWithImpl<$Res>
    extends _$SendDestinationCopyWithImpl<$Res, _$SendDestination_Bolt11Impl>
    implements _$$SendDestination_Bolt11ImplCopyWith<$Res> {
  __$$SendDestination_Bolt11ImplCopyWithImpl(
      _$SendDestination_Bolt11Impl _value, $Res Function(_$SendDestination_Bolt11Impl) _then)
      : super(_value, _then);

  /// Create a copy of SendDestination
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? invoice = null,
  }) {
    return _then(_$SendDestination_Bolt11Impl(
      invoice: null == invoice
          ? _value.invoice
          : invoice // ignore: cast_nullable_to_non_nullable
              as LNInvoice,
    ));
  }
}

/// @nodoc

class _$SendDestination_Bolt11Impl extends SendDestination_Bolt11 {
  const _$SendDestination_Bolt11Impl({required this.invoice}) : super._();

  @override
  final LNInvoice invoice;

  @override
  String toString() {
    return 'SendDestination.bolt11(invoice: $invoice)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SendDestination_Bolt11Impl &&
            (identical(other.invoice, invoice) || other.invoice == invoice));
  }

  @override
  int get hashCode => Object.hash(runtimeType, invoice);

  /// Create a copy of SendDestination
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$SendDestination_Bolt11ImplCopyWith<_$SendDestination_Bolt11Impl> get copyWith =>
      __$$SendDestination_Bolt11ImplCopyWithImpl<_$SendDestination_Bolt11Impl>(this, _$identity);
}

abstract class SendDestination_Bolt11 extends SendDestination {
  const factory SendDestination_Bolt11({required final LNInvoice invoice}) = _$SendDestination_Bolt11Impl;
  const SendDestination_Bolt11._() : super._();

  LNInvoice get invoice;

  /// Create a copy of SendDestination
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$SendDestination_Bolt11ImplCopyWith<_$SendDestination_Bolt11Impl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SendDestination_Bolt12ImplCopyWith<$Res> {
  factory _$$SendDestination_Bolt12ImplCopyWith(
          _$SendDestination_Bolt12Impl value, $Res Function(_$SendDestination_Bolt12Impl) then) =
      __$$SendDestination_Bolt12ImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LNOffer offer, BigInt receiverAmountSat});
}

/// @nodoc
class __$$SendDestination_Bolt12ImplCopyWithImpl<$Res>
    extends _$SendDestinationCopyWithImpl<$Res, _$SendDestination_Bolt12Impl>
    implements _$$SendDestination_Bolt12ImplCopyWith<$Res> {
  __$$SendDestination_Bolt12ImplCopyWithImpl(
      _$SendDestination_Bolt12Impl _value, $Res Function(_$SendDestination_Bolt12Impl) _then)
      : super(_value, _then);

  /// Create a copy of SendDestination
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? offer = null,
    Object? receiverAmountSat = null,
  }) {
    return _then(_$SendDestination_Bolt12Impl(
      offer: null == offer
          ? _value.offer
          : offer // ignore: cast_nullable_to_non_nullable
              as LNOffer,
      receiverAmountSat: null == receiverAmountSat
          ? _value.receiverAmountSat
          : receiverAmountSat // ignore: cast_nullable_to_non_nullable
              as BigInt,
    ));
  }
}

/// @nodoc

class _$SendDestination_Bolt12Impl extends SendDestination_Bolt12 {
  const _$SendDestination_Bolt12Impl({required this.offer, required this.receiverAmountSat}) : super._();

  @override
  final LNOffer offer;
  @override
  final BigInt receiverAmountSat;

  @override
  String toString() {
    return 'SendDestination.bolt12(offer: $offer, receiverAmountSat: $receiverAmountSat)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SendDestination_Bolt12Impl &&
            (identical(other.offer, offer) || other.offer == offer) &&
            (identical(other.receiverAmountSat, receiverAmountSat) ||
                other.receiverAmountSat == receiverAmountSat));
  }

  @override
  int get hashCode => Object.hash(runtimeType, offer, receiverAmountSat);

  /// Create a copy of SendDestination
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$SendDestination_Bolt12ImplCopyWith<_$SendDestination_Bolt12Impl> get copyWith =>
      __$$SendDestination_Bolt12ImplCopyWithImpl<_$SendDestination_Bolt12Impl>(this, _$identity);
}

abstract class SendDestination_Bolt12 extends SendDestination {
  const factory SendDestination_Bolt12(
      {required final LNOffer offer, required final BigInt receiverAmountSat}) = _$SendDestination_Bolt12Impl;
  const SendDestination_Bolt12._() : super._();

  LNOffer get offer;
  BigInt get receiverAmountSat;

  /// Create a copy of SendDestination
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$SendDestination_Bolt12ImplCopyWith<_$SendDestination_Bolt12Impl> get copyWith =>
      throw _privateConstructorUsedError;
}
