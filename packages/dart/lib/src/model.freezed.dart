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
  'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models',
);

/// @nodoc
mixin _$GetPaymentRequest {}

/// @nodoc
abstract class $GetPaymentRequestCopyWith<$Res> {
  factory $GetPaymentRequestCopyWith(GetPaymentRequest value, $Res Function(GetPaymentRequest) then) =
      _$GetPaymentRequestCopyWithImpl<$Res, GetPaymentRequest>;
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
}

/// @nodoc
abstract class _$$GetPaymentRequest_PaymentHashImplCopyWith<$Res> {
  factory _$$GetPaymentRequest_PaymentHashImplCopyWith(
    _$GetPaymentRequest_PaymentHashImpl value,
    $Res Function(_$GetPaymentRequest_PaymentHashImpl) then,
  ) = __$$GetPaymentRequest_PaymentHashImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String paymentHash});
}

/// @nodoc
class __$$GetPaymentRequest_PaymentHashImplCopyWithImpl<$Res>
    extends _$GetPaymentRequestCopyWithImpl<$Res, _$GetPaymentRequest_PaymentHashImpl>
    implements _$$GetPaymentRequest_PaymentHashImplCopyWith<$Res> {
  __$$GetPaymentRequest_PaymentHashImplCopyWithImpl(
    _$GetPaymentRequest_PaymentHashImpl _value,
    $Res Function(_$GetPaymentRequest_PaymentHashImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of GetPaymentRequest
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? paymentHash = null}) {
    return _then(
      _$GetPaymentRequest_PaymentHashImpl(
        paymentHash:
            null == paymentHash
                ? _value.paymentHash
                : paymentHash // ignore: cast_nullable_to_non_nullable
                    as String,
      ),
    );
  }
}

/// @nodoc

class _$GetPaymentRequest_PaymentHashImpl extends GetPaymentRequest_PaymentHash {
  const _$GetPaymentRequest_PaymentHashImpl({required this.paymentHash}) : super._();

  @override
  final String paymentHash;

  @override
  String toString() {
    return 'GetPaymentRequest.paymentHash(paymentHash: $paymentHash)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$GetPaymentRequest_PaymentHashImpl &&
            (identical(other.paymentHash, paymentHash) || other.paymentHash == paymentHash));
  }

  @override
  int get hashCode => Object.hash(runtimeType, paymentHash);

  /// Create a copy of GetPaymentRequest
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$GetPaymentRequest_PaymentHashImplCopyWith<_$GetPaymentRequest_PaymentHashImpl> get copyWith =>
      __$$GetPaymentRequest_PaymentHashImplCopyWithImpl<_$GetPaymentRequest_PaymentHashImpl>(
        this,
        _$identity,
      );
}

abstract class GetPaymentRequest_PaymentHash extends GetPaymentRequest {
  const factory GetPaymentRequest_PaymentHash({required final String paymentHash}) =
      _$GetPaymentRequest_PaymentHashImpl;
  const GetPaymentRequest_PaymentHash._() : super._();

  String get paymentHash;

  /// Create a copy of GetPaymentRequest
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$GetPaymentRequest_PaymentHashImplCopyWith<_$GetPaymentRequest_PaymentHashImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$GetPaymentRequest_SwapIdImplCopyWith<$Res> {
  factory _$$GetPaymentRequest_SwapIdImplCopyWith(
    _$GetPaymentRequest_SwapIdImpl value,
    $Res Function(_$GetPaymentRequest_SwapIdImpl) then,
  ) = __$$GetPaymentRequest_SwapIdImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String swapId});
}

/// @nodoc
class __$$GetPaymentRequest_SwapIdImplCopyWithImpl<$Res>
    extends _$GetPaymentRequestCopyWithImpl<$Res, _$GetPaymentRequest_SwapIdImpl>
    implements _$$GetPaymentRequest_SwapIdImplCopyWith<$Res> {
  __$$GetPaymentRequest_SwapIdImplCopyWithImpl(
    _$GetPaymentRequest_SwapIdImpl _value,
    $Res Function(_$GetPaymentRequest_SwapIdImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of GetPaymentRequest
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? swapId = null}) {
    return _then(
      _$GetPaymentRequest_SwapIdImpl(
        swapId:
            null == swapId
                ? _value.swapId
                : swapId // ignore: cast_nullable_to_non_nullable
                    as String,
      ),
    );
  }
}

/// @nodoc

class _$GetPaymentRequest_SwapIdImpl extends GetPaymentRequest_SwapId {
  const _$GetPaymentRequest_SwapIdImpl({required this.swapId}) : super._();

  @override
  final String swapId;

  @override
  String toString() {
    return 'GetPaymentRequest.swapId(swapId: $swapId)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$GetPaymentRequest_SwapIdImpl &&
            (identical(other.swapId, swapId) || other.swapId == swapId));
  }

  @override
  int get hashCode => Object.hash(runtimeType, swapId);

  /// Create a copy of GetPaymentRequest
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$GetPaymentRequest_SwapIdImplCopyWith<_$GetPaymentRequest_SwapIdImpl> get copyWith =>
      __$$GetPaymentRequest_SwapIdImplCopyWithImpl<_$GetPaymentRequest_SwapIdImpl>(this, _$identity);
}

abstract class GetPaymentRequest_SwapId extends GetPaymentRequest {
  const factory GetPaymentRequest_SwapId({required final String swapId}) = _$GetPaymentRequest_SwapIdImpl;
  const GetPaymentRequest_SwapId._() : super._();

  String get swapId;

  /// Create a copy of GetPaymentRequest
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$GetPaymentRequest_SwapIdImplCopyWith<_$GetPaymentRequest_SwapIdImpl> get copyWith =>
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
    _$ListPaymentDetails_LiquidImpl value,
    $Res Function(_$ListPaymentDetails_LiquidImpl) then,
  ) = __$$ListPaymentDetails_LiquidImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String? assetId, String? destination});
}

/// @nodoc
class __$$ListPaymentDetails_LiquidImplCopyWithImpl<$Res>
    extends _$ListPaymentDetailsCopyWithImpl<$Res, _$ListPaymentDetails_LiquidImpl>
    implements _$$ListPaymentDetails_LiquidImplCopyWith<$Res> {
  __$$ListPaymentDetails_LiquidImplCopyWithImpl(
    _$ListPaymentDetails_LiquidImpl _value,
    $Res Function(_$ListPaymentDetails_LiquidImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ListPaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? assetId = freezed, Object? destination = freezed}) {
    return _then(
      _$ListPaymentDetails_LiquidImpl(
        assetId:
            freezed == assetId
                ? _value.assetId
                : assetId // ignore: cast_nullable_to_non_nullable
                    as String?,
        destination:
            freezed == destination
                ? _value.destination
                : destination // ignore: cast_nullable_to_non_nullable
                    as String?,
      ),
    );
  }
}

/// @nodoc

class _$ListPaymentDetails_LiquidImpl extends ListPaymentDetails_Liquid {
  const _$ListPaymentDetails_LiquidImpl({this.assetId, this.destination}) : super._();

  /// Optional asset id
  @override
  final String? assetId;

  /// Optional BIP21 URI or address
  @override
  final String? destination;

  @override
  String toString() {
    return 'ListPaymentDetails.liquid(assetId: $assetId, destination: $destination)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ListPaymentDetails_LiquidImpl &&
            (identical(other.assetId, assetId) || other.assetId == assetId) &&
            (identical(other.destination, destination) || other.destination == destination));
  }

  @override
  int get hashCode => Object.hash(runtimeType, assetId, destination);

  /// Create a copy of ListPaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ListPaymentDetails_LiquidImplCopyWith<_$ListPaymentDetails_LiquidImpl> get copyWith =>
      __$$ListPaymentDetails_LiquidImplCopyWithImpl<_$ListPaymentDetails_LiquidImpl>(this, _$identity);
}

abstract class ListPaymentDetails_Liquid extends ListPaymentDetails {
  const factory ListPaymentDetails_Liquid({final String? assetId, final String? destination}) =
      _$ListPaymentDetails_LiquidImpl;
  const ListPaymentDetails_Liquid._() : super._();

  /// Optional asset id
  String? get assetId;

  /// Optional BIP21 URI or address
  String? get destination;

  /// Create a copy of ListPaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ListPaymentDetails_LiquidImplCopyWith<_$ListPaymentDetails_LiquidImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ListPaymentDetails_BitcoinImplCopyWith<$Res> {
  factory _$$ListPaymentDetails_BitcoinImplCopyWith(
    _$ListPaymentDetails_BitcoinImpl value,
    $Res Function(_$ListPaymentDetails_BitcoinImpl) then,
  ) = __$$ListPaymentDetails_BitcoinImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String? address});
}

/// @nodoc
class __$$ListPaymentDetails_BitcoinImplCopyWithImpl<$Res>
    extends _$ListPaymentDetailsCopyWithImpl<$Res, _$ListPaymentDetails_BitcoinImpl>
    implements _$$ListPaymentDetails_BitcoinImplCopyWith<$Res> {
  __$$ListPaymentDetails_BitcoinImplCopyWithImpl(
    _$ListPaymentDetails_BitcoinImpl _value,
    $Res Function(_$ListPaymentDetails_BitcoinImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ListPaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? address = freezed}) {
    return _then(
      _$ListPaymentDetails_BitcoinImpl(
        address:
            freezed == address
                ? _value.address
                : address // ignore: cast_nullable_to_non_nullable
                    as String?,
      ),
    );
  }
}

/// @nodoc

class _$ListPaymentDetails_BitcoinImpl extends ListPaymentDetails_Bitcoin {
  const _$ListPaymentDetails_BitcoinImpl({this.address}) : super._();

  /// Optional address
  @override
  final String? address;

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
  const factory ListPaymentDetails_Bitcoin({final String? address}) = _$ListPaymentDetails_BitcoinImpl;
  const ListPaymentDetails_Bitcoin._() : super._();

  /// Optional address
  String? get address;

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
  factory _$$LnUrlPayResult_EndpointSuccessImplCopyWith(
    _$LnUrlPayResult_EndpointSuccessImpl value,
    $Res Function(_$LnUrlPayResult_EndpointSuccessImpl) then,
  ) = __$$LnUrlPayResult_EndpointSuccessImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlPaySuccessData data});
}

/// @nodoc
class __$$LnUrlPayResult_EndpointSuccessImplCopyWithImpl<$Res>
    extends _$LnUrlPayResultCopyWithImpl<$Res, _$LnUrlPayResult_EndpointSuccessImpl>
    implements _$$LnUrlPayResult_EndpointSuccessImplCopyWith<$Res> {
  __$$LnUrlPayResult_EndpointSuccessImplCopyWithImpl(
    _$LnUrlPayResult_EndpointSuccessImpl _value,
    $Res Function(_$LnUrlPayResult_EndpointSuccessImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of LnUrlPayResult
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? data = null}) {
    return _then(
      _$LnUrlPayResult_EndpointSuccessImpl(
        data:
            null == data
                ? _value.data
                : data // ignore: cast_nullable_to_non_nullable
                    as LnUrlPaySuccessData,
      ),
    );
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
        this,
        _$identity,
      );
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
    _$LnUrlPayResult_EndpointErrorImpl value,
    $Res Function(_$LnUrlPayResult_EndpointErrorImpl) then,
  ) = __$$LnUrlPayResult_EndpointErrorImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlErrorData data});
}

/// @nodoc
class __$$LnUrlPayResult_EndpointErrorImplCopyWithImpl<$Res>
    extends _$LnUrlPayResultCopyWithImpl<$Res, _$LnUrlPayResult_EndpointErrorImpl>
    implements _$$LnUrlPayResult_EndpointErrorImplCopyWith<$Res> {
  __$$LnUrlPayResult_EndpointErrorImplCopyWithImpl(
    _$LnUrlPayResult_EndpointErrorImpl _value,
    $Res Function(_$LnUrlPayResult_EndpointErrorImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of LnUrlPayResult
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? data = null}) {
    return _then(
      _$LnUrlPayResult_EndpointErrorImpl(
        data:
            null == data
                ? _value.data
                : data // ignore: cast_nullable_to_non_nullable
                    as LnUrlErrorData,
      ),
    );
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
    _$LnUrlPayResult_PayErrorImpl value,
    $Res Function(_$LnUrlPayResult_PayErrorImpl) then,
  ) = __$$LnUrlPayResult_PayErrorImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlPayErrorData data});
}

/// @nodoc
class __$$LnUrlPayResult_PayErrorImplCopyWithImpl<$Res>
    extends _$LnUrlPayResultCopyWithImpl<$Res, _$LnUrlPayResult_PayErrorImpl>
    implements _$$LnUrlPayResult_PayErrorImplCopyWith<$Res> {
  __$$LnUrlPayResult_PayErrorImplCopyWithImpl(
    _$LnUrlPayResult_PayErrorImpl _value,
    $Res Function(_$LnUrlPayResult_PayErrorImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of LnUrlPayResult
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? data = null}) {
    return _then(
      _$LnUrlPayResult_PayErrorImpl(
        data:
            null == data
                ? _value.data
                : data // ignore: cast_nullable_to_non_nullable
                    as LnUrlPayErrorData,
      ),
    );
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
abstract class _$$PayAmount_BitcoinImplCopyWith<$Res> {
  factory _$$PayAmount_BitcoinImplCopyWith(
    _$PayAmount_BitcoinImpl value,
    $Res Function(_$PayAmount_BitcoinImpl) then,
  ) = __$$PayAmount_BitcoinImplCopyWithImpl<$Res>;
  @useResult
  $Res call({BigInt receiverAmountSat});
}

/// @nodoc
class __$$PayAmount_BitcoinImplCopyWithImpl<$Res>
    extends _$PayAmountCopyWithImpl<$Res, _$PayAmount_BitcoinImpl>
    implements _$$PayAmount_BitcoinImplCopyWith<$Res> {
  __$$PayAmount_BitcoinImplCopyWithImpl(
    _$PayAmount_BitcoinImpl _value,
    $Res Function(_$PayAmount_BitcoinImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of PayAmount
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? receiverAmountSat = null}) {
    return _then(
      _$PayAmount_BitcoinImpl(
        receiverAmountSat:
            null == receiverAmountSat
                ? _value.receiverAmountSat
                : receiverAmountSat // ignore: cast_nullable_to_non_nullable
                    as BigInt,
      ),
    );
  }
}

/// @nodoc

class _$PayAmount_BitcoinImpl extends PayAmount_Bitcoin {
  const _$PayAmount_BitcoinImpl({required this.receiverAmountSat}) : super._();

  @override
  final BigInt receiverAmountSat;

  @override
  String toString() {
    return 'PayAmount.bitcoin(receiverAmountSat: $receiverAmountSat)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PayAmount_BitcoinImpl &&
            (identical(other.receiverAmountSat, receiverAmountSat) ||
                other.receiverAmountSat == receiverAmountSat));
  }

  @override
  int get hashCode => Object.hash(runtimeType, receiverAmountSat);

  /// Create a copy of PayAmount
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PayAmount_BitcoinImplCopyWith<_$PayAmount_BitcoinImpl> get copyWith =>
      __$$PayAmount_BitcoinImplCopyWithImpl<_$PayAmount_BitcoinImpl>(this, _$identity);
}

abstract class PayAmount_Bitcoin extends PayAmount {
  const factory PayAmount_Bitcoin({required final BigInt receiverAmountSat}) = _$PayAmount_BitcoinImpl;
  const PayAmount_Bitcoin._() : super._();

  BigInt get receiverAmountSat;

  /// Create a copy of PayAmount
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$PayAmount_BitcoinImplCopyWith<_$PayAmount_BitcoinImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PayAmount_AssetImplCopyWith<$Res> {
  factory _$$PayAmount_AssetImplCopyWith(
    _$PayAmount_AssetImpl value,
    $Res Function(_$PayAmount_AssetImpl) then,
  ) = __$$PayAmount_AssetImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String assetId, double receiverAmount});
}

/// @nodoc
class __$$PayAmount_AssetImplCopyWithImpl<$Res> extends _$PayAmountCopyWithImpl<$Res, _$PayAmount_AssetImpl>
    implements _$$PayAmount_AssetImplCopyWith<$Res> {
  __$$PayAmount_AssetImplCopyWithImpl(
    _$PayAmount_AssetImpl _value,
    $Res Function(_$PayAmount_AssetImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of PayAmount
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? assetId = null, Object? receiverAmount = null}) {
    return _then(
      _$PayAmount_AssetImpl(
        assetId:
            null == assetId
                ? _value.assetId
                : assetId // ignore: cast_nullable_to_non_nullable
                    as String,
        receiverAmount:
            null == receiverAmount
                ? _value.receiverAmount
                : receiverAmount // ignore: cast_nullable_to_non_nullable
                    as double,
      ),
    );
  }
}

/// @nodoc

class _$PayAmount_AssetImpl extends PayAmount_Asset {
  const _$PayAmount_AssetImpl({required this.assetId, required this.receiverAmount}) : super._();

  @override
  final String assetId;
  @override
  final double receiverAmount;

  @override
  String toString() {
    return 'PayAmount.asset(assetId: $assetId, receiverAmount: $receiverAmount)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PayAmount_AssetImpl &&
            (identical(other.assetId, assetId) || other.assetId == assetId) &&
            (identical(other.receiverAmount, receiverAmount) || other.receiverAmount == receiverAmount));
  }

  @override
  int get hashCode => Object.hash(runtimeType, assetId, receiverAmount);

  /// Create a copy of PayAmount
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PayAmount_AssetImplCopyWith<_$PayAmount_AssetImpl> get copyWith =>
      __$$PayAmount_AssetImplCopyWithImpl<_$PayAmount_AssetImpl>(this, _$identity);
}

abstract class PayAmount_Asset extends PayAmount {
  const factory PayAmount_Asset({required final String assetId, required final double receiverAmount}) =
      _$PayAmount_AssetImpl;
  const PayAmount_Asset._() : super._();

  String get assetId;
  double get receiverAmount;

  /// Create a copy of PayAmount
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$PayAmount_AssetImplCopyWith<_$PayAmount_AssetImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PayAmount_DrainImplCopyWith<$Res> {
  factory _$$PayAmount_DrainImplCopyWith(
    _$PayAmount_DrainImpl value,
    $Res Function(_$PayAmount_DrainImpl) then,
  ) = __$$PayAmount_DrainImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$PayAmount_DrainImplCopyWithImpl<$Res> extends _$PayAmountCopyWithImpl<$Res, _$PayAmount_DrainImpl>
    implements _$$PayAmount_DrainImplCopyWith<$Res> {
  __$$PayAmount_DrainImplCopyWithImpl(
    _$PayAmount_DrainImpl _value,
    $Res Function(_$PayAmount_DrainImpl) _then,
  ) : super(_value, _then);

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
  $Res call({Object? description = null}) {
    return _then(
      _value.copyWith(
            description:
                null == description
                    ? _value.description
                    : description // ignore: cast_nullable_to_non_nullable
                        as String,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$PaymentDetails_LightningImplCopyWith<$Res> implements $PaymentDetailsCopyWith<$Res> {
  factory _$$PaymentDetails_LightningImplCopyWith(
    _$PaymentDetails_LightningImpl value,
    $Res Function(_$PaymentDetails_LightningImpl) then,
  ) = __$$PaymentDetails_LightningImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({
    String swapId,
    String description,
    int liquidExpirationBlockheight,
    String? preimage,
    String? invoice,
    String? bolt12Offer,
    String? paymentHash,
    String? destinationPubkey,
    LnUrlInfo? lnurlInfo,
    String? bip353Address,
    String? claimTxId,
    String? refundTxId,
    BigInt? refundTxAmountSat,
  });
}

/// @nodoc
class __$$PaymentDetails_LightningImplCopyWithImpl<$Res>
    extends _$PaymentDetailsCopyWithImpl<$Res, _$PaymentDetails_LightningImpl>
    implements _$$PaymentDetails_LightningImplCopyWith<$Res> {
  __$$PaymentDetails_LightningImplCopyWithImpl(
    _$PaymentDetails_LightningImpl _value,
    $Res Function(_$PaymentDetails_LightningImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of PaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? swapId = null,
    Object? description = null,
    Object? liquidExpirationBlockheight = null,
    Object? preimage = freezed,
    Object? invoice = freezed,
    Object? bolt12Offer = freezed,
    Object? paymentHash = freezed,
    Object? destinationPubkey = freezed,
    Object? lnurlInfo = freezed,
    Object? bip353Address = freezed,
    Object? claimTxId = freezed,
    Object? refundTxId = freezed,
    Object? refundTxAmountSat = freezed,
  }) {
    return _then(
      _$PaymentDetails_LightningImpl(
        swapId:
            null == swapId
                ? _value.swapId
                : swapId // ignore: cast_nullable_to_non_nullable
                    as String,
        description:
            null == description
                ? _value.description
                : description // ignore: cast_nullable_to_non_nullable
                    as String,
        liquidExpirationBlockheight:
            null == liquidExpirationBlockheight
                ? _value.liquidExpirationBlockheight
                : liquidExpirationBlockheight // ignore: cast_nullable_to_non_nullable
                    as int,
        preimage:
            freezed == preimage
                ? _value.preimage
                : preimage // ignore: cast_nullable_to_non_nullable
                    as String?,
        invoice:
            freezed == invoice
                ? _value.invoice
                : invoice // ignore: cast_nullable_to_non_nullable
                    as String?,
        bolt12Offer:
            freezed == bolt12Offer
                ? _value.bolt12Offer
                : bolt12Offer // ignore: cast_nullable_to_non_nullable
                    as String?,
        paymentHash:
            freezed == paymentHash
                ? _value.paymentHash
                : paymentHash // ignore: cast_nullable_to_non_nullable
                    as String?,
        destinationPubkey:
            freezed == destinationPubkey
                ? _value.destinationPubkey
                : destinationPubkey // ignore: cast_nullable_to_non_nullable
                    as String?,
        lnurlInfo:
            freezed == lnurlInfo
                ? _value.lnurlInfo
                : lnurlInfo // ignore: cast_nullable_to_non_nullable
                    as LnUrlInfo?,
        bip353Address:
            freezed == bip353Address
                ? _value.bip353Address
                : bip353Address // ignore: cast_nullable_to_non_nullable
                    as String?,
        claimTxId:
            freezed == claimTxId
                ? _value.claimTxId
                : claimTxId // ignore: cast_nullable_to_non_nullable
                    as String?,
        refundTxId:
            freezed == refundTxId
                ? _value.refundTxId
                : refundTxId // ignore: cast_nullable_to_non_nullable
                    as String?,
        refundTxAmountSat:
            freezed == refundTxAmountSat
                ? _value.refundTxAmountSat
                : refundTxAmountSat // ignore: cast_nullable_to_non_nullable
                    as BigInt?,
      ),
    );
  }
}

/// @nodoc

class _$PaymentDetails_LightningImpl extends PaymentDetails_Lightning {
  const _$PaymentDetails_LightningImpl({
    required this.swapId,
    required this.description,
    required this.liquidExpirationBlockheight,
    this.preimage,
    this.invoice,
    this.bolt12Offer,
    this.paymentHash,
    this.destinationPubkey,
    this.lnurlInfo,
    this.bip353Address,
    this.claimTxId,
    this.refundTxId,
    this.refundTxAmountSat,
  }) : super._();

  @override
  final String swapId;

  /// Represents the invoice description
  @override
  final String description;

  /// The height of the block at which the swap will no longer be valid
  @override
  final int liquidExpirationBlockheight;

  /// The preimage of the paid invoice (proof of payment).
  @override
  final String? preimage;

  /// Represents the Bolt11/Bolt12 invoice associated with a payment
  /// In the case of a Send payment, this is the invoice paid by the swapper
  /// In the case of a Receive payment, this is the invoice paid by the user
  @override
  final String? invoice;
  @override
  final String? bolt12Offer;

  /// The payment hash of the invoice
  @override
  final String? paymentHash;

  /// The invoice destination/payee pubkey
  @override
  final String? destinationPubkey;

  /// The payment LNURL info
  @override
  final LnUrlInfo? lnurlInfo;

  /// The BIP353 address used to resolve this payment
  @override
  final String? bip353Address;

  /// For a Receive payment, this is the claim tx id in case it has already been broadcast
  @override
  final String? claimTxId;

  /// For a Send swap which was refunded, this is the refund tx id
  @override
  final String? refundTxId;

  /// For a Send swap which was refunded, this is the refund amount
  @override
  final BigInt? refundTxAmountSat;

  @override
  String toString() {
    return 'PaymentDetails.lightning(swapId: $swapId, description: $description, liquidExpirationBlockheight: $liquidExpirationBlockheight, preimage: $preimage, invoice: $invoice, bolt12Offer: $bolt12Offer, paymentHash: $paymentHash, destinationPubkey: $destinationPubkey, lnurlInfo: $lnurlInfo, bip353Address: $bip353Address, claimTxId: $claimTxId, refundTxId: $refundTxId, refundTxAmountSat: $refundTxAmountSat)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PaymentDetails_LightningImpl &&
            (identical(other.swapId, swapId) || other.swapId == swapId) &&
            (identical(other.description, description) || other.description == description) &&
            (identical(other.liquidExpirationBlockheight, liquidExpirationBlockheight) ||
                other.liquidExpirationBlockheight == liquidExpirationBlockheight) &&
            (identical(other.preimage, preimage) || other.preimage == preimage) &&
            (identical(other.invoice, invoice) || other.invoice == invoice) &&
            (identical(other.bolt12Offer, bolt12Offer) || other.bolt12Offer == bolt12Offer) &&
            (identical(other.paymentHash, paymentHash) || other.paymentHash == paymentHash) &&
            (identical(other.destinationPubkey, destinationPubkey) ||
                other.destinationPubkey == destinationPubkey) &&
            (identical(other.lnurlInfo, lnurlInfo) || other.lnurlInfo == lnurlInfo) &&
            (identical(other.bip353Address, bip353Address) || other.bip353Address == bip353Address) &&
            (identical(other.claimTxId, claimTxId) || other.claimTxId == claimTxId) &&
            (identical(other.refundTxId, refundTxId) || other.refundTxId == refundTxId) &&
            (identical(other.refundTxAmountSat, refundTxAmountSat) ||
                other.refundTxAmountSat == refundTxAmountSat));
  }

  @override
  int get hashCode => Object.hash(
    runtimeType,
    swapId,
    description,
    liquidExpirationBlockheight,
    preimage,
    invoice,
    bolt12Offer,
    paymentHash,
    destinationPubkey,
    lnurlInfo,
    bip353Address,
    claimTxId,
    refundTxId,
    refundTxAmountSat,
  );

  /// Create a copy of PaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PaymentDetails_LightningImplCopyWith<_$PaymentDetails_LightningImpl> get copyWith =>
      __$$PaymentDetails_LightningImplCopyWithImpl<_$PaymentDetails_LightningImpl>(this, _$identity);
}

abstract class PaymentDetails_Lightning extends PaymentDetails {
  const factory PaymentDetails_Lightning({
    required final String swapId,
    required final String description,
    required final int liquidExpirationBlockheight,
    final String? preimage,
    final String? invoice,
    final String? bolt12Offer,
    final String? paymentHash,
    final String? destinationPubkey,
    final LnUrlInfo? lnurlInfo,
    final String? bip353Address,
    final String? claimTxId,
    final String? refundTxId,
    final BigInt? refundTxAmountSat,
  }) = _$PaymentDetails_LightningImpl;
  const PaymentDetails_Lightning._() : super._();

  String get swapId;

  /// Represents the invoice description
  @override
  String get description;

  /// The height of the block at which the swap will no longer be valid
  int get liquidExpirationBlockheight;

  /// The preimage of the paid invoice (proof of payment).
  String? get preimage;

  /// Represents the Bolt11/Bolt12 invoice associated with a payment
  /// In the case of a Send payment, this is the invoice paid by the swapper
  /// In the case of a Receive payment, this is the invoice paid by the user
  String? get invoice;
  String? get bolt12Offer;

  /// The payment hash of the invoice
  String? get paymentHash;

  /// The invoice destination/payee pubkey
  String? get destinationPubkey;

  /// The payment LNURL info
  LnUrlInfo? get lnurlInfo;

  /// The BIP353 address used to resolve this payment
  String? get bip353Address;

  /// For a Receive payment, this is the claim tx id in case it has already been broadcast
  String? get claimTxId;

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
    _$PaymentDetails_LiquidImpl value,
    $Res Function(_$PaymentDetails_LiquidImpl) then,
  ) = __$$PaymentDetails_LiquidImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({
    String destination,
    String description,
    String assetId,
    AssetInfo? assetInfo,
    LnUrlInfo? lnurlInfo,
    String? bip353Address,
  });
}

/// @nodoc
class __$$PaymentDetails_LiquidImplCopyWithImpl<$Res>
    extends _$PaymentDetailsCopyWithImpl<$Res, _$PaymentDetails_LiquidImpl>
    implements _$$PaymentDetails_LiquidImplCopyWith<$Res> {
  __$$PaymentDetails_LiquidImplCopyWithImpl(
    _$PaymentDetails_LiquidImpl _value,
    $Res Function(_$PaymentDetails_LiquidImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of PaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? destination = null,
    Object? description = null,
    Object? assetId = null,
    Object? assetInfo = freezed,
    Object? lnurlInfo = freezed,
    Object? bip353Address = freezed,
  }) {
    return _then(
      _$PaymentDetails_LiquidImpl(
        destination:
            null == destination
                ? _value.destination
                : destination // ignore: cast_nullable_to_non_nullable
                    as String,
        description:
            null == description
                ? _value.description
                : description // ignore: cast_nullable_to_non_nullable
                    as String,
        assetId:
            null == assetId
                ? _value.assetId
                : assetId // ignore: cast_nullable_to_non_nullable
                    as String,
        assetInfo:
            freezed == assetInfo
                ? _value.assetInfo
                : assetInfo // ignore: cast_nullable_to_non_nullable
                    as AssetInfo?,
        lnurlInfo:
            freezed == lnurlInfo
                ? _value.lnurlInfo
                : lnurlInfo // ignore: cast_nullable_to_non_nullable
                    as LnUrlInfo?,
        bip353Address:
            freezed == bip353Address
                ? _value.bip353Address
                : bip353Address // ignore: cast_nullable_to_non_nullable
                    as String?,
      ),
    );
  }
}

/// @nodoc

class _$PaymentDetails_LiquidImpl extends PaymentDetails_Liquid {
  const _$PaymentDetails_LiquidImpl({
    required this.destination,
    required this.description,
    required this.assetId,
    this.assetInfo,
    this.lnurlInfo,
    this.bip353Address,
  }) : super._();

  /// Represents either a Liquid BIP21 URI or pure address
  @override
  final String destination;

  /// Represents the BIP21 `message` field
  @override
  final String description;

  /// The asset id
  @override
  final String assetId;

  /// The asset info derived from the [AssetMetadata]
  @override
  final AssetInfo? assetInfo;

  /// The payment LNURL info
  @override
  final LnUrlInfo? lnurlInfo;

  /// The BIP353 address used to resolve this payment
  @override
  final String? bip353Address;

  @override
  String toString() {
    return 'PaymentDetails.liquid(destination: $destination, description: $description, assetId: $assetId, assetInfo: $assetInfo, lnurlInfo: $lnurlInfo, bip353Address: $bip353Address)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PaymentDetails_LiquidImpl &&
            (identical(other.destination, destination) || other.destination == destination) &&
            (identical(other.description, description) || other.description == description) &&
            (identical(other.assetId, assetId) || other.assetId == assetId) &&
            (identical(other.assetInfo, assetInfo) || other.assetInfo == assetInfo) &&
            (identical(other.lnurlInfo, lnurlInfo) || other.lnurlInfo == lnurlInfo) &&
            (identical(other.bip353Address, bip353Address) || other.bip353Address == bip353Address));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, destination, description, assetId, assetInfo, lnurlInfo, bip353Address);

  /// Create a copy of PaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PaymentDetails_LiquidImplCopyWith<_$PaymentDetails_LiquidImpl> get copyWith =>
      __$$PaymentDetails_LiquidImplCopyWithImpl<_$PaymentDetails_LiquidImpl>(this, _$identity);
}

abstract class PaymentDetails_Liquid extends PaymentDetails {
  const factory PaymentDetails_Liquid({
    required final String destination,
    required final String description,
    required final String assetId,
    final AssetInfo? assetInfo,
    final LnUrlInfo? lnurlInfo,
    final String? bip353Address,
  }) = _$PaymentDetails_LiquidImpl;
  const PaymentDetails_Liquid._() : super._();

  /// Represents either a Liquid BIP21 URI or pure address
  String get destination;

  /// Represents the BIP21 `message` field
  @override
  String get description;

  /// The asset id
  String get assetId;

  /// The asset info derived from the [AssetMetadata]
  AssetInfo? get assetInfo;

  /// The payment LNURL info
  LnUrlInfo? get lnurlInfo;

  /// The BIP353 address used to resolve this payment
  String? get bip353Address;

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
    _$PaymentDetails_BitcoinImpl value,
    $Res Function(_$PaymentDetails_BitcoinImpl) then,
  ) = __$$PaymentDetails_BitcoinImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({
    String swapId,
    String description,
    bool autoAcceptedFees,
    int? liquidExpirationBlockheight,
    int? bitcoinExpirationBlockheight,
    String? claimTxId,
    String? refundTxId,
    BigInt? refundTxAmountSat,
  });
}

/// @nodoc
class __$$PaymentDetails_BitcoinImplCopyWithImpl<$Res>
    extends _$PaymentDetailsCopyWithImpl<$Res, _$PaymentDetails_BitcoinImpl>
    implements _$$PaymentDetails_BitcoinImplCopyWith<$Res> {
  __$$PaymentDetails_BitcoinImplCopyWithImpl(
    _$PaymentDetails_BitcoinImpl _value,
    $Res Function(_$PaymentDetails_BitcoinImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of PaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? swapId = null,
    Object? description = null,
    Object? autoAcceptedFees = null,
    Object? liquidExpirationBlockheight = freezed,
    Object? bitcoinExpirationBlockheight = freezed,
    Object? claimTxId = freezed,
    Object? refundTxId = freezed,
    Object? refundTxAmountSat = freezed,
  }) {
    return _then(
      _$PaymentDetails_BitcoinImpl(
        swapId:
            null == swapId
                ? _value.swapId
                : swapId // ignore: cast_nullable_to_non_nullable
                    as String,
        description:
            null == description
                ? _value.description
                : description // ignore: cast_nullable_to_non_nullable
                    as String,
        autoAcceptedFees:
            null == autoAcceptedFees
                ? _value.autoAcceptedFees
                : autoAcceptedFees // ignore: cast_nullable_to_non_nullable
                    as bool,
        liquidExpirationBlockheight:
            freezed == liquidExpirationBlockheight
                ? _value.liquidExpirationBlockheight
                : liquidExpirationBlockheight // ignore: cast_nullable_to_non_nullable
                    as int?,
        bitcoinExpirationBlockheight:
            freezed == bitcoinExpirationBlockheight
                ? _value.bitcoinExpirationBlockheight
                : bitcoinExpirationBlockheight // ignore: cast_nullable_to_non_nullable
                    as int?,
        claimTxId:
            freezed == claimTxId
                ? _value.claimTxId
                : claimTxId // ignore: cast_nullable_to_non_nullable
                    as String?,
        refundTxId:
            freezed == refundTxId
                ? _value.refundTxId
                : refundTxId // ignore: cast_nullable_to_non_nullable
                    as String?,
        refundTxAmountSat:
            freezed == refundTxAmountSat
                ? _value.refundTxAmountSat
                : refundTxAmountSat // ignore: cast_nullable_to_non_nullable
                    as BigInt?,
      ),
    );
  }
}

/// @nodoc

class _$PaymentDetails_BitcoinImpl extends PaymentDetails_Bitcoin {
  const _$PaymentDetails_BitcoinImpl({
    required this.swapId,
    required this.description,
    required this.autoAcceptedFees,
    this.liquidExpirationBlockheight,
    this.bitcoinExpirationBlockheight,
    this.claimTxId,
    this.refundTxId,
    this.refundTxAmountSat,
  }) : super._();

  @override
  final String swapId;

  /// Represents the invoice description
  @override
  final String description;

  /// For an amountless receive swap, this indicates if fees were automatically accepted.
  /// Fees are auto accepted when the swapper proposes fees that are within the initial
  /// estimate, plus the `onchain_fee_rate_leeway_sat_per_vbyte` set in the [Config], if any.
  @override
  final bool autoAcceptedFees;

  /// The height of the Liquid block at which the swap will no longer be valid
  /// It should always be populated in case of an outgoing chain swap
  @override
  final int? liquidExpirationBlockheight;

  /// The height of the Bitcoin block at which the swap will no longer be valid
  /// It should always be populated in case of an incoming chain swap
  @override
  final int? bitcoinExpirationBlockheight;

  /// The claim tx id in case it has already been broadcast
  @override
  final String? claimTxId;

  /// For a Send swap which was refunded, this is the refund tx id
  @override
  final String? refundTxId;

  /// For a Send swap which was refunded, this is the refund amount
  @override
  final BigInt? refundTxAmountSat;

  @override
  String toString() {
    return 'PaymentDetails.bitcoin(swapId: $swapId, description: $description, autoAcceptedFees: $autoAcceptedFees, liquidExpirationBlockheight: $liquidExpirationBlockheight, bitcoinExpirationBlockheight: $bitcoinExpirationBlockheight, claimTxId: $claimTxId, refundTxId: $refundTxId, refundTxAmountSat: $refundTxAmountSat)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PaymentDetails_BitcoinImpl &&
            (identical(other.swapId, swapId) || other.swapId == swapId) &&
            (identical(other.description, description) || other.description == description) &&
            (identical(other.autoAcceptedFees, autoAcceptedFees) ||
                other.autoAcceptedFees == autoAcceptedFees) &&
            (identical(other.liquidExpirationBlockheight, liquidExpirationBlockheight) ||
                other.liquidExpirationBlockheight == liquidExpirationBlockheight) &&
            (identical(other.bitcoinExpirationBlockheight, bitcoinExpirationBlockheight) ||
                other.bitcoinExpirationBlockheight == bitcoinExpirationBlockheight) &&
            (identical(other.claimTxId, claimTxId) || other.claimTxId == claimTxId) &&
            (identical(other.refundTxId, refundTxId) || other.refundTxId == refundTxId) &&
            (identical(other.refundTxAmountSat, refundTxAmountSat) ||
                other.refundTxAmountSat == refundTxAmountSat));
  }

  @override
  int get hashCode => Object.hash(
    runtimeType,
    swapId,
    description,
    autoAcceptedFees,
    liquidExpirationBlockheight,
    bitcoinExpirationBlockheight,
    claimTxId,
    refundTxId,
    refundTxAmountSat,
  );

  /// Create a copy of PaymentDetails
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PaymentDetails_BitcoinImplCopyWith<_$PaymentDetails_BitcoinImpl> get copyWith =>
      __$$PaymentDetails_BitcoinImplCopyWithImpl<_$PaymentDetails_BitcoinImpl>(this, _$identity);
}

abstract class PaymentDetails_Bitcoin extends PaymentDetails {
  const factory PaymentDetails_Bitcoin({
    required final String swapId,
    required final String description,
    required final bool autoAcceptedFees,
    final int? liquidExpirationBlockheight,
    final int? bitcoinExpirationBlockheight,
    final String? claimTxId,
    final String? refundTxId,
    final BigInt? refundTxAmountSat,
  }) = _$PaymentDetails_BitcoinImpl;
  const PaymentDetails_Bitcoin._() : super._();

  String get swapId;

  /// Represents the invoice description
  @override
  String get description;

  /// For an amountless receive swap, this indicates if fees were automatically accepted.
  /// Fees are auto accepted when the swapper proposes fees that are within the initial
  /// estimate, plus the `onchain_fee_rate_leeway_sat_per_vbyte` set in the [Config], if any.
  bool get autoAcceptedFees;

  /// The height of the Liquid block at which the swap will no longer be valid
  /// It should always be populated in case of an outgoing chain swap
  int? get liquidExpirationBlockheight;

  /// The height of the Bitcoin block at which the swap will no longer be valid
  /// It should always be populated in case of an incoming chain swap
  int? get bitcoinExpirationBlockheight;

  /// The claim tx id in case it has already been broadcast
  String? get claimTxId;

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
mixin _$ReceiveAmount {}

/// @nodoc
abstract class $ReceiveAmountCopyWith<$Res> {
  factory $ReceiveAmountCopyWith(ReceiveAmount value, $Res Function(ReceiveAmount) then) =
      _$ReceiveAmountCopyWithImpl<$Res, ReceiveAmount>;
}

/// @nodoc
class _$ReceiveAmountCopyWithImpl<$Res, $Val extends ReceiveAmount> implements $ReceiveAmountCopyWith<$Res> {
  _$ReceiveAmountCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of ReceiveAmount
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$ReceiveAmount_BitcoinImplCopyWith<$Res> {
  factory _$$ReceiveAmount_BitcoinImplCopyWith(
    _$ReceiveAmount_BitcoinImpl value,
    $Res Function(_$ReceiveAmount_BitcoinImpl) then,
  ) = __$$ReceiveAmount_BitcoinImplCopyWithImpl<$Res>;
  @useResult
  $Res call({BigInt payerAmountSat});
}

/// @nodoc
class __$$ReceiveAmount_BitcoinImplCopyWithImpl<$Res>
    extends _$ReceiveAmountCopyWithImpl<$Res, _$ReceiveAmount_BitcoinImpl>
    implements _$$ReceiveAmount_BitcoinImplCopyWith<$Res> {
  __$$ReceiveAmount_BitcoinImplCopyWithImpl(
    _$ReceiveAmount_BitcoinImpl _value,
    $Res Function(_$ReceiveAmount_BitcoinImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ReceiveAmount
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? payerAmountSat = null}) {
    return _then(
      _$ReceiveAmount_BitcoinImpl(
        payerAmountSat:
            null == payerAmountSat
                ? _value.payerAmountSat
                : payerAmountSat // ignore: cast_nullable_to_non_nullable
                    as BigInt,
      ),
    );
  }
}

/// @nodoc

class _$ReceiveAmount_BitcoinImpl extends ReceiveAmount_Bitcoin {
  const _$ReceiveAmount_BitcoinImpl({required this.payerAmountSat}) : super._();

  @override
  final BigInt payerAmountSat;

  @override
  String toString() {
    return 'ReceiveAmount.bitcoin(payerAmountSat: $payerAmountSat)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ReceiveAmount_BitcoinImpl &&
            (identical(other.payerAmountSat, payerAmountSat) || other.payerAmountSat == payerAmountSat));
  }

  @override
  int get hashCode => Object.hash(runtimeType, payerAmountSat);

  /// Create a copy of ReceiveAmount
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ReceiveAmount_BitcoinImplCopyWith<_$ReceiveAmount_BitcoinImpl> get copyWith =>
      __$$ReceiveAmount_BitcoinImplCopyWithImpl<_$ReceiveAmount_BitcoinImpl>(this, _$identity);
}

abstract class ReceiveAmount_Bitcoin extends ReceiveAmount {
  const factory ReceiveAmount_Bitcoin({required final BigInt payerAmountSat}) = _$ReceiveAmount_BitcoinImpl;
  const ReceiveAmount_Bitcoin._() : super._();

  BigInt get payerAmountSat;

  /// Create a copy of ReceiveAmount
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ReceiveAmount_BitcoinImplCopyWith<_$ReceiveAmount_BitcoinImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ReceiveAmount_AssetImplCopyWith<$Res> {
  factory _$$ReceiveAmount_AssetImplCopyWith(
    _$ReceiveAmount_AssetImpl value,
    $Res Function(_$ReceiveAmount_AssetImpl) then,
  ) = __$$ReceiveAmount_AssetImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String assetId, double? payerAmount});
}

/// @nodoc
class __$$ReceiveAmount_AssetImplCopyWithImpl<$Res>
    extends _$ReceiveAmountCopyWithImpl<$Res, _$ReceiveAmount_AssetImpl>
    implements _$$ReceiveAmount_AssetImplCopyWith<$Res> {
  __$$ReceiveAmount_AssetImplCopyWithImpl(
    _$ReceiveAmount_AssetImpl _value,
    $Res Function(_$ReceiveAmount_AssetImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ReceiveAmount
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? assetId = null, Object? payerAmount = freezed}) {
    return _then(
      _$ReceiveAmount_AssetImpl(
        assetId:
            null == assetId
                ? _value.assetId
                : assetId // ignore: cast_nullable_to_non_nullable
                    as String,
        payerAmount:
            freezed == payerAmount
                ? _value.payerAmount
                : payerAmount // ignore: cast_nullable_to_non_nullable
                    as double?,
      ),
    );
  }
}

/// @nodoc

class _$ReceiveAmount_AssetImpl extends ReceiveAmount_Asset {
  const _$ReceiveAmount_AssetImpl({required this.assetId, this.payerAmount}) : super._();

  @override
  final String assetId;
  @override
  final double? payerAmount;

  @override
  String toString() {
    return 'ReceiveAmount.asset(assetId: $assetId, payerAmount: $payerAmount)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ReceiveAmount_AssetImpl &&
            (identical(other.assetId, assetId) || other.assetId == assetId) &&
            (identical(other.payerAmount, payerAmount) || other.payerAmount == payerAmount));
  }

  @override
  int get hashCode => Object.hash(runtimeType, assetId, payerAmount);

  /// Create a copy of ReceiveAmount
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ReceiveAmount_AssetImplCopyWith<_$ReceiveAmount_AssetImpl> get copyWith =>
      __$$ReceiveAmount_AssetImplCopyWithImpl<_$ReceiveAmount_AssetImpl>(this, _$identity);
}

abstract class ReceiveAmount_Asset extends ReceiveAmount {
  const factory ReceiveAmount_Asset({required final String assetId, final double? payerAmount}) =
      _$ReceiveAmount_AssetImpl;
  const ReceiveAmount_Asset._() : super._();

  String get assetId;
  double? get payerAmount;

  /// Create a copy of ReceiveAmount
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ReceiveAmount_AssetImplCopyWith<_$ReceiveAmount_AssetImpl> get copyWith =>
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
    _$SdkEvent_PaymentFailedImpl value,
    $Res Function(_$SdkEvent_PaymentFailedImpl) then,
  ) = __$$SdkEvent_PaymentFailedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$SdkEvent_PaymentFailedImplCopyWithImpl<$Res>
    extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_PaymentFailedImpl>
    implements _$$SdkEvent_PaymentFailedImplCopyWith<$Res> {
  __$$SdkEvent_PaymentFailedImplCopyWithImpl(
    _$SdkEvent_PaymentFailedImpl _value,
    $Res Function(_$SdkEvent_PaymentFailedImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? details = null}) {
    return _then(
      _$SdkEvent_PaymentFailedImpl(
        details:
            null == details
                ? _value.details
                : details // ignore: cast_nullable_to_non_nullable
                    as Payment,
      ),
    );
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
    _$SdkEvent_PaymentPendingImpl value,
    $Res Function(_$SdkEvent_PaymentPendingImpl) then,
  ) = __$$SdkEvent_PaymentPendingImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$SdkEvent_PaymentPendingImplCopyWithImpl<$Res>
    extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_PaymentPendingImpl>
    implements _$$SdkEvent_PaymentPendingImplCopyWith<$Res> {
  __$$SdkEvent_PaymentPendingImplCopyWithImpl(
    _$SdkEvent_PaymentPendingImpl _value,
    $Res Function(_$SdkEvent_PaymentPendingImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? details = null}) {
    return _then(
      _$SdkEvent_PaymentPendingImpl(
        details:
            null == details
                ? _value.details
                : details // ignore: cast_nullable_to_non_nullable
                    as Payment,
      ),
    );
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
abstract class _$$SdkEvent_PaymentRefundableImplCopyWith<$Res> {
  factory _$$SdkEvent_PaymentRefundableImplCopyWith(
    _$SdkEvent_PaymentRefundableImpl value,
    $Res Function(_$SdkEvent_PaymentRefundableImpl) then,
  ) = __$$SdkEvent_PaymentRefundableImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$SdkEvent_PaymentRefundableImplCopyWithImpl<$Res>
    extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_PaymentRefundableImpl>
    implements _$$SdkEvent_PaymentRefundableImplCopyWith<$Res> {
  __$$SdkEvent_PaymentRefundableImplCopyWithImpl(
    _$SdkEvent_PaymentRefundableImpl _value,
    $Res Function(_$SdkEvent_PaymentRefundableImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? details = null}) {
    return _then(
      _$SdkEvent_PaymentRefundableImpl(
        details:
            null == details
                ? _value.details
                : details // ignore: cast_nullable_to_non_nullable
                    as Payment,
      ),
    );
  }
}

/// @nodoc

class _$SdkEvent_PaymentRefundableImpl extends SdkEvent_PaymentRefundable {
  const _$SdkEvent_PaymentRefundableImpl({required this.details}) : super._();

  @override
  final Payment details;

  @override
  String toString() {
    return 'SdkEvent.paymentRefundable(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SdkEvent_PaymentRefundableImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$SdkEvent_PaymentRefundableImplCopyWith<_$SdkEvent_PaymentRefundableImpl> get copyWith =>
      __$$SdkEvent_PaymentRefundableImplCopyWithImpl<_$SdkEvent_PaymentRefundableImpl>(this, _$identity);
}

abstract class SdkEvent_PaymentRefundable extends SdkEvent {
  const factory SdkEvent_PaymentRefundable({required final Payment details}) =
      _$SdkEvent_PaymentRefundableImpl;
  const SdkEvent_PaymentRefundable._() : super._();

  Payment get details;

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$SdkEvent_PaymentRefundableImplCopyWith<_$SdkEvent_PaymentRefundableImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SdkEvent_PaymentRefundedImplCopyWith<$Res> {
  factory _$$SdkEvent_PaymentRefundedImplCopyWith(
    _$SdkEvent_PaymentRefundedImpl value,
    $Res Function(_$SdkEvent_PaymentRefundedImpl) then,
  ) = __$$SdkEvent_PaymentRefundedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$SdkEvent_PaymentRefundedImplCopyWithImpl<$Res>
    extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_PaymentRefundedImpl>
    implements _$$SdkEvent_PaymentRefundedImplCopyWith<$Res> {
  __$$SdkEvent_PaymentRefundedImplCopyWithImpl(
    _$SdkEvent_PaymentRefundedImpl _value,
    $Res Function(_$SdkEvent_PaymentRefundedImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? details = null}) {
    return _then(
      _$SdkEvent_PaymentRefundedImpl(
        details:
            null == details
                ? _value.details
                : details // ignore: cast_nullable_to_non_nullable
                    as Payment,
      ),
    );
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
  factory _$$SdkEvent_PaymentRefundPendingImplCopyWith(
    _$SdkEvent_PaymentRefundPendingImpl value,
    $Res Function(_$SdkEvent_PaymentRefundPendingImpl) then,
  ) = __$$SdkEvent_PaymentRefundPendingImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$SdkEvent_PaymentRefundPendingImplCopyWithImpl<$Res>
    extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_PaymentRefundPendingImpl>
    implements _$$SdkEvent_PaymentRefundPendingImplCopyWith<$Res> {
  __$$SdkEvent_PaymentRefundPendingImplCopyWithImpl(
    _$SdkEvent_PaymentRefundPendingImpl _value,
    $Res Function(_$SdkEvent_PaymentRefundPendingImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? details = null}) {
    return _then(
      _$SdkEvent_PaymentRefundPendingImpl(
        details:
            null == details
                ? _value.details
                : details // ignore: cast_nullable_to_non_nullable
                    as Payment,
      ),
    );
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
        this,
        _$identity,
      );
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
    _$SdkEvent_PaymentSucceededImpl value,
    $Res Function(_$SdkEvent_PaymentSucceededImpl) then,
  ) = __$$SdkEvent_PaymentSucceededImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$SdkEvent_PaymentSucceededImplCopyWithImpl<$Res>
    extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_PaymentSucceededImpl>
    implements _$$SdkEvent_PaymentSucceededImplCopyWith<$Res> {
  __$$SdkEvent_PaymentSucceededImplCopyWithImpl(
    _$SdkEvent_PaymentSucceededImpl _value,
    $Res Function(_$SdkEvent_PaymentSucceededImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? details = null}) {
    return _then(
      _$SdkEvent_PaymentSucceededImpl(
        details:
            null == details
                ? _value.details
                : details // ignore: cast_nullable_to_non_nullable
                    as Payment,
      ),
    );
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
  factory _$$SdkEvent_PaymentWaitingConfirmationImplCopyWith(
    _$SdkEvent_PaymentWaitingConfirmationImpl value,
    $Res Function(_$SdkEvent_PaymentWaitingConfirmationImpl) then,
  ) = __$$SdkEvent_PaymentWaitingConfirmationImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$SdkEvent_PaymentWaitingConfirmationImplCopyWithImpl<$Res>
    extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_PaymentWaitingConfirmationImpl>
    implements _$$SdkEvent_PaymentWaitingConfirmationImplCopyWith<$Res> {
  __$$SdkEvent_PaymentWaitingConfirmationImplCopyWithImpl(
    _$SdkEvent_PaymentWaitingConfirmationImpl _value,
    $Res Function(_$SdkEvent_PaymentWaitingConfirmationImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? details = null}) {
    return _then(
      _$SdkEvent_PaymentWaitingConfirmationImpl(
        details:
            null == details
                ? _value.details
                : details // ignore: cast_nullable_to_non_nullable
                    as Payment,
      ),
    );
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
        this,
        _$identity,
      );
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
abstract class _$$SdkEvent_PaymentWaitingFeeAcceptanceImplCopyWith<$Res> {
  factory _$$SdkEvent_PaymentWaitingFeeAcceptanceImplCopyWith(
    _$SdkEvent_PaymentWaitingFeeAcceptanceImpl value,
    $Res Function(_$SdkEvent_PaymentWaitingFeeAcceptanceImpl) then,
  ) = __$$SdkEvent_PaymentWaitingFeeAcceptanceImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$SdkEvent_PaymentWaitingFeeAcceptanceImplCopyWithImpl<$Res>
    extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_PaymentWaitingFeeAcceptanceImpl>
    implements _$$SdkEvent_PaymentWaitingFeeAcceptanceImplCopyWith<$Res> {
  __$$SdkEvent_PaymentWaitingFeeAcceptanceImplCopyWithImpl(
    _$SdkEvent_PaymentWaitingFeeAcceptanceImpl _value,
    $Res Function(_$SdkEvent_PaymentWaitingFeeAcceptanceImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? details = null}) {
    return _then(
      _$SdkEvent_PaymentWaitingFeeAcceptanceImpl(
        details:
            null == details
                ? _value.details
                : details // ignore: cast_nullable_to_non_nullable
                    as Payment,
      ),
    );
  }
}

/// @nodoc

class _$SdkEvent_PaymentWaitingFeeAcceptanceImpl extends SdkEvent_PaymentWaitingFeeAcceptance {
  const _$SdkEvent_PaymentWaitingFeeAcceptanceImpl({required this.details}) : super._();

  @override
  final Payment details;

  @override
  String toString() {
    return 'SdkEvent.paymentWaitingFeeAcceptance(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SdkEvent_PaymentWaitingFeeAcceptanceImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$SdkEvent_PaymentWaitingFeeAcceptanceImplCopyWith<_$SdkEvent_PaymentWaitingFeeAcceptanceImpl>
  get copyWith =>
      __$$SdkEvent_PaymentWaitingFeeAcceptanceImplCopyWithImpl<_$SdkEvent_PaymentWaitingFeeAcceptanceImpl>(
        this,
        _$identity,
      );
}

abstract class SdkEvent_PaymentWaitingFeeAcceptance extends SdkEvent {
  const factory SdkEvent_PaymentWaitingFeeAcceptance({required final Payment details}) =
      _$SdkEvent_PaymentWaitingFeeAcceptanceImpl;
  const SdkEvent_PaymentWaitingFeeAcceptance._() : super._();

  Payment get details;

  /// Create a copy of SdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$SdkEvent_PaymentWaitingFeeAcceptanceImplCopyWith<_$SdkEvent_PaymentWaitingFeeAcceptanceImpl>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SdkEvent_SyncedImplCopyWith<$Res> {
  factory _$$SdkEvent_SyncedImplCopyWith(
    _$SdkEvent_SyncedImpl value,
    $Res Function(_$SdkEvent_SyncedImpl) then,
  ) = __$$SdkEvent_SyncedImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$SdkEvent_SyncedImplCopyWithImpl<$Res> extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_SyncedImpl>
    implements _$$SdkEvent_SyncedImplCopyWith<$Res> {
  __$$SdkEvent_SyncedImplCopyWithImpl(
    _$SdkEvent_SyncedImpl _value,
    $Res Function(_$SdkEvent_SyncedImpl) _then,
  ) : super(_value, _then);

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
mixin _$SendDestination {
  /// A BIP353 address, in case one was used to resolve this Liquid address
  String? get bip353Address => throw _privateConstructorUsedError;

  /// Create a copy of SendDestination
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $SendDestinationCopyWith<SendDestination> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $SendDestinationCopyWith<$Res> {
  factory $SendDestinationCopyWith(SendDestination value, $Res Function(SendDestination) then) =
      _$SendDestinationCopyWithImpl<$Res, SendDestination>;
  @useResult
  $Res call({String? bip353Address});
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
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? bip353Address = freezed}) {
    return _then(
      _value.copyWith(
            bip353Address:
                freezed == bip353Address
                    ? _value.bip353Address
                    : bip353Address // ignore: cast_nullable_to_non_nullable
                        as String?,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$SendDestination_LiquidAddressImplCopyWith<$Res> implements $SendDestinationCopyWith<$Res> {
  factory _$$SendDestination_LiquidAddressImplCopyWith(
    _$SendDestination_LiquidAddressImpl value,
    $Res Function(_$SendDestination_LiquidAddressImpl) then,
  ) = __$$SendDestination_LiquidAddressImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({LiquidAddressData addressData, String? bip353Address});
}

/// @nodoc
class __$$SendDestination_LiquidAddressImplCopyWithImpl<$Res>
    extends _$SendDestinationCopyWithImpl<$Res, _$SendDestination_LiquidAddressImpl>
    implements _$$SendDestination_LiquidAddressImplCopyWith<$Res> {
  __$$SendDestination_LiquidAddressImplCopyWithImpl(
    _$SendDestination_LiquidAddressImpl _value,
    $Res Function(_$SendDestination_LiquidAddressImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of SendDestination
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? addressData = null, Object? bip353Address = freezed}) {
    return _then(
      _$SendDestination_LiquidAddressImpl(
        addressData:
            null == addressData
                ? _value.addressData
                : addressData // ignore: cast_nullable_to_non_nullable
                    as LiquidAddressData,
        bip353Address:
            freezed == bip353Address
                ? _value.bip353Address
                : bip353Address // ignore: cast_nullable_to_non_nullable
                    as String?,
      ),
    );
  }
}

/// @nodoc

class _$SendDestination_LiquidAddressImpl extends SendDestination_LiquidAddress {
  const _$SendDestination_LiquidAddressImpl({required this.addressData, this.bip353Address}) : super._();

  @override
  final LiquidAddressData addressData;

  /// A BIP353 address, in case one was used to resolve this Liquid address
  @override
  final String? bip353Address;

  @override
  String toString() {
    return 'SendDestination.liquidAddress(addressData: $addressData, bip353Address: $bip353Address)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SendDestination_LiquidAddressImpl &&
            (identical(other.addressData, addressData) || other.addressData == addressData) &&
            (identical(other.bip353Address, bip353Address) || other.bip353Address == bip353Address));
  }

  @override
  int get hashCode => Object.hash(runtimeType, addressData, bip353Address);

  /// Create a copy of SendDestination
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$SendDestination_LiquidAddressImplCopyWith<_$SendDestination_LiquidAddressImpl> get copyWith =>
      __$$SendDestination_LiquidAddressImplCopyWithImpl<_$SendDestination_LiquidAddressImpl>(
        this,
        _$identity,
      );
}

abstract class SendDestination_LiquidAddress extends SendDestination {
  const factory SendDestination_LiquidAddress({
    required final LiquidAddressData addressData,
    final String? bip353Address,
  }) = _$SendDestination_LiquidAddressImpl;
  const SendDestination_LiquidAddress._() : super._();

  LiquidAddressData get addressData;

  /// A BIP353 address, in case one was used to resolve this Liquid address
  @override
  String? get bip353Address;

  /// Create a copy of SendDestination
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$SendDestination_LiquidAddressImplCopyWith<_$SendDestination_LiquidAddressImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SendDestination_Bolt11ImplCopyWith<$Res> implements $SendDestinationCopyWith<$Res> {
  factory _$$SendDestination_Bolt11ImplCopyWith(
    _$SendDestination_Bolt11Impl value,
    $Res Function(_$SendDestination_Bolt11Impl) then,
  ) = __$$SendDestination_Bolt11ImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({LNInvoice invoice, String? bip353Address});
}

/// @nodoc
class __$$SendDestination_Bolt11ImplCopyWithImpl<$Res>
    extends _$SendDestinationCopyWithImpl<$Res, _$SendDestination_Bolt11Impl>
    implements _$$SendDestination_Bolt11ImplCopyWith<$Res> {
  __$$SendDestination_Bolt11ImplCopyWithImpl(
    _$SendDestination_Bolt11Impl _value,
    $Res Function(_$SendDestination_Bolt11Impl) _then,
  ) : super(_value, _then);

  /// Create a copy of SendDestination
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? invoice = null, Object? bip353Address = freezed}) {
    return _then(
      _$SendDestination_Bolt11Impl(
        invoice:
            null == invoice
                ? _value.invoice
                : invoice // ignore: cast_nullable_to_non_nullable
                    as LNInvoice,
        bip353Address:
            freezed == bip353Address
                ? _value.bip353Address
                : bip353Address // ignore: cast_nullable_to_non_nullable
                    as String?,
      ),
    );
  }
}

/// @nodoc

class _$SendDestination_Bolt11Impl extends SendDestination_Bolt11 {
  const _$SendDestination_Bolt11Impl({required this.invoice, this.bip353Address}) : super._();

  @override
  final LNInvoice invoice;

  /// A BIP353 address, in case one was used to resolve this BOLT11
  @override
  final String? bip353Address;

  @override
  String toString() {
    return 'SendDestination.bolt11(invoice: $invoice, bip353Address: $bip353Address)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SendDestination_Bolt11Impl &&
            (identical(other.invoice, invoice) || other.invoice == invoice) &&
            (identical(other.bip353Address, bip353Address) || other.bip353Address == bip353Address));
  }

  @override
  int get hashCode => Object.hash(runtimeType, invoice, bip353Address);

  /// Create a copy of SendDestination
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$SendDestination_Bolt11ImplCopyWith<_$SendDestination_Bolt11Impl> get copyWith =>
      __$$SendDestination_Bolt11ImplCopyWithImpl<_$SendDestination_Bolt11Impl>(this, _$identity);
}

abstract class SendDestination_Bolt11 extends SendDestination {
  const factory SendDestination_Bolt11({required final LNInvoice invoice, final String? bip353Address}) =
      _$SendDestination_Bolt11Impl;
  const SendDestination_Bolt11._() : super._();

  LNInvoice get invoice;

  /// A BIP353 address, in case one was used to resolve this BOLT11
  @override
  String? get bip353Address;

  /// Create a copy of SendDestination
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$SendDestination_Bolt11ImplCopyWith<_$SendDestination_Bolt11Impl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SendDestination_Bolt12ImplCopyWith<$Res> implements $SendDestinationCopyWith<$Res> {
  factory _$$SendDestination_Bolt12ImplCopyWith(
    _$SendDestination_Bolt12Impl value,
    $Res Function(_$SendDestination_Bolt12Impl) then,
  ) = __$$SendDestination_Bolt12ImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({LNOffer offer, BigInt receiverAmountSat, String? bip353Address});
}

/// @nodoc
class __$$SendDestination_Bolt12ImplCopyWithImpl<$Res>
    extends _$SendDestinationCopyWithImpl<$Res, _$SendDestination_Bolt12Impl>
    implements _$$SendDestination_Bolt12ImplCopyWith<$Res> {
  __$$SendDestination_Bolt12ImplCopyWithImpl(
    _$SendDestination_Bolt12Impl _value,
    $Res Function(_$SendDestination_Bolt12Impl) _then,
  ) : super(_value, _then);

  /// Create a copy of SendDestination
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? offer = null, Object? receiverAmountSat = null, Object? bip353Address = freezed}) {
    return _then(
      _$SendDestination_Bolt12Impl(
        offer:
            null == offer
                ? _value.offer
                : offer // ignore: cast_nullable_to_non_nullable
                    as LNOffer,
        receiverAmountSat:
            null == receiverAmountSat
                ? _value.receiverAmountSat
                : receiverAmountSat // ignore: cast_nullable_to_non_nullable
                    as BigInt,
        bip353Address:
            freezed == bip353Address
                ? _value.bip353Address
                : bip353Address // ignore: cast_nullable_to_non_nullable
                    as String?,
      ),
    );
  }
}

/// @nodoc

class _$SendDestination_Bolt12Impl extends SendDestination_Bolt12 {
  const _$SendDestination_Bolt12Impl({
    required this.offer,
    required this.receiverAmountSat,
    this.bip353Address,
  }) : super._();

  @override
  final LNOffer offer;
  @override
  final BigInt receiverAmountSat;

  /// A BIP353 address, in case one was used to resolve this BOLT12
  @override
  final String? bip353Address;

  @override
  String toString() {
    return 'SendDestination.bolt12(offer: $offer, receiverAmountSat: $receiverAmountSat, bip353Address: $bip353Address)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SendDestination_Bolt12Impl &&
            (identical(other.offer, offer) || other.offer == offer) &&
            (identical(other.receiverAmountSat, receiverAmountSat) ||
                other.receiverAmountSat == receiverAmountSat) &&
            (identical(other.bip353Address, bip353Address) || other.bip353Address == bip353Address));
  }

  @override
  int get hashCode => Object.hash(runtimeType, offer, receiverAmountSat, bip353Address);

  /// Create a copy of SendDestination
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$SendDestination_Bolt12ImplCopyWith<_$SendDestination_Bolt12Impl> get copyWith =>
      __$$SendDestination_Bolt12ImplCopyWithImpl<_$SendDestination_Bolt12Impl>(this, _$identity);
}

abstract class SendDestination_Bolt12 extends SendDestination {
  const factory SendDestination_Bolt12({
    required final LNOffer offer,
    required final BigInt receiverAmountSat,
    final String? bip353Address,
  }) = _$SendDestination_Bolt12Impl;
  const SendDestination_Bolt12._() : super._();

  LNOffer get offer;
  BigInt get receiverAmountSat;

  /// A BIP353 address, in case one was used to resolve this BOLT12
  @override
  String? get bip353Address;

  /// Create a copy of SendDestination
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$SendDestination_Bolt12ImplCopyWith<_$SendDestination_Bolt12Impl> get copyWith =>
      throw _privateConstructorUsedError;
}
