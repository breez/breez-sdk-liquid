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
mixin _$PayOnchainAmount {}

/// @nodoc
abstract class $PayOnchainAmountCopyWith<$Res> {
  factory $PayOnchainAmountCopyWith(PayOnchainAmount value, $Res Function(PayOnchainAmount) then) =
      _$PayOnchainAmountCopyWithImpl<$Res, PayOnchainAmount>;
}

/// @nodoc
class _$PayOnchainAmountCopyWithImpl<$Res, $Val extends PayOnchainAmount>
    implements $PayOnchainAmountCopyWith<$Res> {
  _$PayOnchainAmountCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of PayOnchainAmount
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$PayOnchainAmount_ReceiverImplCopyWith<$Res> {
  factory _$$PayOnchainAmount_ReceiverImplCopyWith(
          _$PayOnchainAmount_ReceiverImpl value, $Res Function(_$PayOnchainAmount_ReceiverImpl) then) =
      __$$PayOnchainAmount_ReceiverImplCopyWithImpl<$Res>;
  @useResult
  $Res call({BigInt amountSat});
}

/// @nodoc
class __$$PayOnchainAmount_ReceiverImplCopyWithImpl<$Res>
    extends _$PayOnchainAmountCopyWithImpl<$Res, _$PayOnchainAmount_ReceiverImpl>
    implements _$$PayOnchainAmount_ReceiverImplCopyWith<$Res> {
  __$$PayOnchainAmount_ReceiverImplCopyWithImpl(
      _$PayOnchainAmount_ReceiverImpl _value, $Res Function(_$PayOnchainAmount_ReceiverImpl) _then)
      : super(_value, _then);

  /// Create a copy of PayOnchainAmount
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? amountSat = null,
  }) {
    return _then(_$PayOnchainAmount_ReceiverImpl(
      amountSat: null == amountSat
          ? _value.amountSat
          : amountSat // ignore: cast_nullable_to_non_nullable
              as BigInt,
    ));
  }
}

/// @nodoc

class _$PayOnchainAmount_ReceiverImpl extends PayOnchainAmount_Receiver {
  const _$PayOnchainAmount_ReceiverImpl({required this.amountSat}) : super._();

  @override
  final BigInt amountSat;

  @override
  String toString() {
    return 'PayOnchainAmount.receiver(amountSat: $amountSat)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PayOnchainAmount_ReceiverImpl &&
            (identical(other.amountSat, amountSat) || other.amountSat == amountSat));
  }

  @override
  int get hashCode => Object.hash(runtimeType, amountSat);

  /// Create a copy of PayOnchainAmount
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PayOnchainAmount_ReceiverImplCopyWith<_$PayOnchainAmount_ReceiverImpl> get copyWith =>
      __$$PayOnchainAmount_ReceiverImplCopyWithImpl<_$PayOnchainAmount_ReceiverImpl>(this, _$identity);
}

abstract class PayOnchainAmount_Receiver extends PayOnchainAmount {
  const factory PayOnchainAmount_Receiver({required final BigInt amountSat}) =
      _$PayOnchainAmount_ReceiverImpl;
  const PayOnchainAmount_Receiver._() : super._();

  BigInt get amountSat;

  /// Create a copy of PayOnchainAmount
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$PayOnchainAmount_ReceiverImplCopyWith<_$PayOnchainAmount_ReceiverImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PayOnchainAmount_DrainImplCopyWith<$Res> {
  factory _$$PayOnchainAmount_DrainImplCopyWith(
          _$PayOnchainAmount_DrainImpl value, $Res Function(_$PayOnchainAmount_DrainImpl) then) =
      __$$PayOnchainAmount_DrainImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$PayOnchainAmount_DrainImplCopyWithImpl<$Res>
    extends _$PayOnchainAmountCopyWithImpl<$Res, _$PayOnchainAmount_DrainImpl>
    implements _$$PayOnchainAmount_DrainImplCopyWith<$Res> {
  __$$PayOnchainAmount_DrainImplCopyWithImpl(
      _$PayOnchainAmount_DrainImpl _value, $Res Function(_$PayOnchainAmount_DrainImpl) _then)
      : super(_value, _then);

  /// Create a copy of PayOnchainAmount
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$PayOnchainAmount_DrainImpl extends PayOnchainAmount_Drain {
  const _$PayOnchainAmount_DrainImpl() : super._();

  @override
  String toString() {
    return 'PayOnchainAmount.drain()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$PayOnchainAmount_DrainImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class PayOnchainAmount_Drain extends PayOnchainAmount {
  const factory PayOnchainAmount_Drain() = _$PayOnchainAmount_DrainImpl;
  const PayOnchainAmount_Drain._() : super._();
}

/// @nodoc
mixin _$PaymentDestination {}

/// @nodoc
abstract class $PaymentDestinationCopyWith<$Res> {
  factory $PaymentDestinationCopyWith(PaymentDestination value, $Res Function(PaymentDestination) then) =
      _$PaymentDestinationCopyWithImpl<$Res, PaymentDestination>;
}

/// @nodoc
class _$PaymentDestinationCopyWithImpl<$Res, $Val extends PaymentDestination>
    implements $PaymentDestinationCopyWith<$Res> {
  _$PaymentDestinationCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of PaymentDestination
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$PaymentDestination_LightningImplCopyWith<$Res> {
  factory _$$PaymentDestination_LightningImplCopyWith(
          _$PaymentDestination_LightningImpl value, $Res Function(_$PaymentDestination_LightningImpl) then) =
      __$$PaymentDestination_LightningImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String bolt11});
}

/// @nodoc
class __$$PaymentDestination_LightningImplCopyWithImpl<$Res>
    extends _$PaymentDestinationCopyWithImpl<$Res, _$PaymentDestination_LightningImpl>
    implements _$$PaymentDestination_LightningImplCopyWith<$Res> {
  __$$PaymentDestination_LightningImplCopyWithImpl(
      _$PaymentDestination_LightningImpl _value, $Res Function(_$PaymentDestination_LightningImpl) _then)
      : super(_value, _then);

  /// Create a copy of PaymentDestination
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? bolt11 = null,
  }) {
    return _then(_$PaymentDestination_LightningImpl(
      bolt11: null == bolt11
          ? _value.bolt11
          : bolt11 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$PaymentDestination_LightningImpl extends PaymentDestination_Lightning {
  const _$PaymentDestination_LightningImpl({required this.bolt11}) : super._();

  @override
  final String bolt11;

  @override
  String toString() {
    return 'PaymentDestination.lightning(bolt11: $bolt11)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PaymentDestination_LightningImpl &&
            (identical(other.bolt11, bolt11) || other.bolt11 == bolt11));
  }

  @override
  int get hashCode => Object.hash(runtimeType, bolt11);

  /// Create a copy of PaymentDestination
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PaymentDestination_LightningImplCopyWith<_$PaymentDestination_LightningImpl> get copyWith =>
      __$$PaymentDestination_LightningImplCopyWithImpl<_$PaymentDestination_LightningImpl>(this, _$identity);
}

abstract class PaymentDestination_Lightning extends PaymentDestination {
  const factory PaymentDestination_Lightning({required final String bolt11}) =
      _$PaymentDestination_LightningImpl;
  const PaymentDestination_Lightning._() : super._();

  String get bolt11;

  /// Create a copy of PaymentDestination
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$PaymentDestination_LightningImplCopyWith<_$PaymentDestination_LightningImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PaymentDestination_LiquidImplCopyWith<$Res> {
  factory _$$PaymentDestination_LiquidImplCopyWith(
          _$PaymentDestination_LiquidImpl value, $Res Function(_$PaymentDestination_LiquidImpl) then) =
      __$$PaymentDestination_LiquidImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String destination});
}

/// @nodoc
class __$$PaymentDestination_LiquidImplCopyWithImpl<$Res>
    extends _$PaymentDestinationCopyWithImpl<$Res, _$PaymentDestination_LiquidImpl>
    implements _$$PaymentDestination_LiquidImplCopyWith<$Res> {
  __$$PaymentDestination_LiquidImplCopyWithImpl(
      _$PaymentDestination_LiquidImpl _value, $Res Function(_$PaymentDestination_LiquidImpl) _then)
      : super(_value, _then);

  /// Create a copy of PaymentDestination
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? destination = null,
  }) {
    return _then(_$PaymentDestination_LiquidImpl(
      destination: null == destination
          ? _value.destination
          : destination // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$PaymentDestination_LiquidImpl extends PaymentDestination_Liquid {
  const _$PaymentDestination_LiquidImpl({required this.destination}) : super._();

  @override
  final String destination;

  @override
  String toString() {
    return 'PaymentDestination.liquid(destination: $destination)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PaymentDestination_LiquidImpl &&
            (identical(other.destination, destination) || other.destination == destination));
  }

  @override
  int get hashCode => Object.hash(runtimeType, destination);

  /// Create a copy of PaymentDestination
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PaymentDestination_LiquidImplCopyWith<_$PaymentDestination_LiquidImpl> get copyWith =>
      __$$PaymentDestination_LiquidImplCopyWithImpl<_$PaymentDestination_LiquidImpl>(this, _$identity);
}

abstract class PaymentDestination_Liquid extends PaymentDestination {
  const factory PaymentDestination_Liquid({required final String destination}) =
      _$PaymentDestination_LiquidImpl;
  const PaymentDestination_Liquid._() : super._();

  String get destination;

  /// Create a copy of PaymentDestination
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$PaymentDestination_LiquidImplCopyWith<_$PaymentDestination_LiquidImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PaymentDestination_BitcoinImplCopyWith<$Res> {
  factory _$$PaymentDestination_BitcoinImplCopyWith(
          _$PaymentDestination_BitcoinImpl value, $Res Function(_$PaymentDestination_BitcoinImpl) then) =
      __$$PaymentDestination_BitcoinImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String address});
}

/// @nodoc
class __$$PaymentDestination_BitcoinImplCopyWithImpl<$Res>
    extends _$PaymentDestinationCopyWithImpl<$Res, _$PaymentDestination_BitcoinImpl>
    implements _$$PaymentDestination_BitcoinImplCopyWith<$Res> {
  __$$PaymentDestination_BitcoinImplCopyWithImpl(
      _$PaymentDestination_BitcoinImpl _value, $Res Function(_$PaymentDestination_BitcoinImpl) _then)
      : super(_value, _then);

  /// Create a copy of PaymentDestination
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? address = null,
  }) {
    return _then(_$PaymentDestination_BitcoinImpl(
      address: null == address
          ? _value.address
          : address // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$PaymentDestination_BitcoinImpl extends PaymentDestination_Bitcoin {
  const _$PaymentDestination_BitcoinImpl({required this.address}) : super._();

  @override
  final String address;

  @override
  String toString() {
    return 'PaymentDestination.bitcoin(address: $address)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PaymentDestination_BitcoinImpl &&
            (identical(other.address, address) || other.address == address));
  }

  @override
  int get hashCode => Object.hash(runtimeType, address);

  /// Create a copy of PaymentDestination
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PaymentDestination_BitcoinImplCopyWith<_$PaymentDestination_BitcoinImpl> get copyWith =>
      __$$PaymentDestination_BitcoinImplCopyWithImpl<_$PaymentDestination_BitcoinImpl>(this, _$identity);
}

abstract class PaymentDestination_Bitcoin extends PaymentDestination {
  const factory PaymentDestination_Bitcoin({required final String address}) =
      _$PaymentDestination_BitcoinImpl;
  const PaymentDestination_Bitcoin._() : super._();

  String get address;

  /// Create a copy of PaymentDestination
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$PaymentDestination_BitcoinImplCopyWith<_$PaymentDestination_BitcoinImpl> get copyWith =>
      throw _privateConstructorUsedError;
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

  /// Represents the invoice associated with a payment
  /// In the case of a Send payment, this is the invoice paid by the swapper
  /// In the case of a Receive payment, this is the invoice paid by the user
  @override
  final String? bolt11;

  /// For a Send swap which was refunded, this is the refund tx id
  @override
  final String? refundTxId;

  /// For a Send swap which was refunded, this is the refund amount
  @override
  final BigInt? refundTxAmountSat;

  @override
  String toString() {
    return 'PaymentDetails.lightning(swapId: $swapId, description: $description, preimage: $preimage, bolt11: $bolt11, refundTxId: $refundTxId, refundTxAmountSat: $refundTxAmountSat)';
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
            (identical(other.refundTxId, refundTxId) || other.refundTxId == refundTxId) &&
            (identical(other.refundTxAmountSat, refundTxAmountSat) ||
                other.refundTxAmountSat == refundTxAmountSat));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, swapId, description, preimage, bolt11, refundTxId, refundTxAmountSat);

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
      final String? refundTxId,
      final BigInt? refundTxAmountSat}) = _$PaymentDetails_LightningImpl;
  const PaymentDetails_Lightning._() : super._();

  String get swapId;

  /// Represents the invoice description
  @override
  String get description;

  /// In case of a Send swap, this is the preimage of the paid invoice (proof of payment).
  String? get preimage;

  /// Represents the invoice associated with a payment
  /// In the case of a Send payment, this is the invoice paid by the swapper
  /// In the case of a Receive payment, this is the invoice paid by the user
  String? get bolt11;

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
