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
mixin _$LiquidSdkEvent {}

/// @nodoc
abstract class $LiquidSdkEventCopyWith<$Res> {
  factory $LiquidSdkEventCopyWith(LiquidSdkEvent value, $Res Function(LiquidSdkEvent) then) =
      _$LiquidSdkEventCopyWithImpl<$Res, LiquidSdkEvent>;
}

/// @nodoc
class _$LiquidSdkEventCopyWithImpl<$Res, $Val extends LiquidSdkEvent>
    implements $LiquidSdkEventCopyWith<$Res> {
  _$LiquidSdkEventCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of LiquidSdkEvent
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$LiquidSdkEvent_PaymentFailedImplCopyWith<$Res> {
  factory _$$LiquidSdkEvent_PaymentFailedImplCopyWith(
          _$LiquidSdkEvent_PaymentFailedImpl value, $Res Function(_$LiquidSdkEvent_PaymentFailedImpl) then) =
      __$$LiquidSdkEvent_PaymentFailedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$LiquidSdkEvent_PaymentFailedImplCopyWithImpl<$Res>
    extends _$LiquidSdkEventCopyWithImpl<$Res, _$LiquidSdkEvent_PaymentFailedImpl>
    implements _$$LiquidSdkEvent_PaymentFailedImplCopyWith<$Res> {
  __$$LiquidSdkEvent_PaymentFailedImplCopyWithImpl(
      _$LiquidSdkEvent_PaymentFailedImpl _value, $Res Function(_$LiquidSdkEvent_PaymentFailedImpl) _then)
      : super(_value, _then);

  /// Create a copy of LiquidSdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$LiquidSdkEvent_PaymentFailedImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as Payment,
    ));
  }
}

/// @nodoc

class _$LiquidSdkEvent_PaymentFailedImpl extends LiquidSdkEvent_PaymentFailed {
  const _$LiquidSdkEvent_PaymentFailedImpl({required this.details}) : super._();

  @override
  final Payment details;

  @override
  String toString() {
    return 'LiquidSdkEvent.paymentFailed(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiquidSdkEvent_PaymentFailedImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  /// Create a copy of LiquidSdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$LiquidSdkEvent_PaymentFailedImplCopyWith<_$LiquidSdkEvent_PaymentFailedImpl> get copyWith =>
      __$$LiquidSdkEvent_PaymentFailedImplCopyWithImpl<_$LiquidSdkEvent_PaymentFailedImpl>(this, _$identity);
}

abstract class LiquidSdkEvent_PaymentFailed extends LiquidSdkEvent {
  const factory LiquidSdkEvent_PaymentFailed({required final Payment details}) =
      _$LiquidSdkEvent_PaymentFailedImpl;
  const LiquidSdkEvent_PaymentFailed._() : super._();

  Payment get details;

  /// Create a copy of LiquidSdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$LiquidSdkEvent_PaymentFailedImplCopyWith<_$LiquidSdkEvent_PaymentFailedImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LiquidSdkEvent_PaymentPendingImplCopyWith<$Res> {
  factory _$$LiquidSdkEvent_PaymentPendingImplCopyWith(_$LiquidSdkEvent_PaymentPendingImpl value,
          $Res Function(_$LiquidSdkEvent_PaymentPendingImpl) then) =
      __$$LiquidSdkEvent_PaymentPendingImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$LiquidSdkEvent_PaymentPendingImplCopyWithImpl<$Res>
    extends _$LiquidSdkEventCopyWithImpl<$Res, _$LiquidSdkEvent_PaymentPendingImpl>
    implements _$$LiquidSdkEvent_PaymentPendingImplCopyWith<$Res> {
  __$$LiquidSdkEvent_PaymentPendingImplCopyWithImpl(
      _$LiquidSdkEvent_PaymentPendingImpl _value, $Res Function(_$LiquidSdkEvent_PaymentPendingImpl) _then)
      : super(_value, _then);

  /// Create a copy of LiquidSdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$LiquidSdkEvent_PaymentPendingImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as Payment,
    ));
  }
}

/// @nodoc

class _$LiquidSdkEvent_PaymentPendingImpl extends LiquidSdkEvent_PaymentPending {
  const _$LiquidSdkEvent_PaymentPendingImpl({required this.details}) : super._();

  @override
  final Payment details;

  @override
  String toString() {
    return 'LiquidSdkEvent.paymentPending(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiquidSdkEvent_PaymentPendingImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  /// Create a copy of LiquidSdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$LiquidSdkEvent_PaymentPendingImplCopyWith<_$LiquidSdkEvent_PaymentPendingImpl> get copyWith =>
      __$$LiquidSdkEvent_PaymentPendingImplCopyWithImpl<_$LiquidSdkEvent_PaymentPendingImpl>(
          this, _$identity);
}

abstract class LiquidSdkEvent_PaymentPending extends LiquidSdkEvent {
  const factory LiquidSdkEvent_PaymentPending({required final Payment details}) =
      _$LiquidSdkEvent_PaymentPendingImpl;
  const LiquidSdkEvent_PaymentPending._() : super._();

  Payment get details;

  /// Create a copy of LiquidSdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$LiquidSdkEvent_PaymentPendingImplCopyWith<_$LiquidSdkEvent_PaymentPendingImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LiquidSdkEvent_PaymentRefundedImplCopyWith<$Res> {
  factory _$$LiquidSdkEvent_PaymentRefundedImplCopyWith(_$LiquidSdkEvent_PaymentRefundedImpl value,
          $Res Function(_$LiquidSdkEvent_PaymentRefundedImpl) then) =
      __$$LiquidSdkEvent_PaymentRefundedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$LiquidSdkEvent_PaymentRefundedImplCopyWithImpl<$Res>
    extends _$LiquidSdkEventCopyWithImpl<$Res, _$LiquidSdkEvent_PaymentRefundedImpl>
    implements _$$LiquidSdkEvent_PaymentRefundedImplCopyWith<$Res> {
  __$$LiquidSdkEvent_PaymentRefundedImplCopyWithImpl(
      _$LiquidSdkEvent_PaymentRefundedImpl _value, $Res Function(_$LiquidSdkEvent_PaymentRefundedImpl) _then)
      : super(_value, _then);

  /// Create a copy of LiquidSdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$LiquidSdkEvent_PaymentRefundedImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as Payment,
    ));
  }
}

/// @nodoc

class _$LiquidSdkEvent_PaymentRefundedImpl extends LiquidSdkEvent_PaymentRefunded {
  const _$LiquidSdkEvent_PaymentRefundedImpl({required this.details}) : super._();

  @override
  final Payment details;

  @override
  String toString() {
    return 'LiquidSdkEvent.paymentRefunded(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiquidSdkEvent_PaymentRefundedImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  /// Create a copy of LiquidSdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$LiquidSdkEvent_PaymentRefundedImplCopyWith<_$LiquidSdkEvent_PaymentRefundedImpl> get copyWith =>
      __$$LiquidSdkEvent_PaymentRefundedImplCopyWithImpl<_$LiquidSdkEvent_PaymentRefundedImpl>(
          this, _$identity);
}

abstract class LiquidSdkEvent_PaymentRefunded extends LiquidSdkEvent {
  const factory LiquidSdkEvent_PaymentRefunded({required final Payment details}) =
      _$LiquidSdkEvent_PaymentRefundedImpl;
  const LiquidSdkEvent_PaymentRefunded._() : super._();

  Payment get details;

  /// Create a copy of LiquidSdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$LiquidSdkEvent_PaymentRefundedImplCopyWith<_$LiquidSdkEvent_PaymentRefundedImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LiquidSdkEvent_PaymentRefundPendingImplCopyWith<$Res> {
  factory _$$LiquidSdkEvent_PaymentRefundPendingImplCopyWith(_$LiquidSdkEvent_PaymentRefundPendingImpl value,
          $Res Function(_$LiquidSdkEvent_PaymentRefundPendingImpl) then) =
      __$$LiquidSdkEvent_PaymentRefundPendingImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$LiquidSdkEvent_PaymentRefundPendingImplCopyWithImpl<$Res>
    extends _$LiquidSdkEventCopyWithImpl<$Res, _$LiquidSdkEvent_PaymentRefundPendingImpl>
    implements _$$LiquidSdkEvent_PaymentRefundPendingImplCopyWith<$Res> {
  __$$LiquidSdkEvent_PaymentRefundPendingImplCopyWithImpl(_$LiquidSdkEvent_PaymentRefundPendingImpl _value,
      $Res Function(_$LiquidSdkEvent_PaymentRefundPendingImpl) _then)
      : super(_value, _then);

  /// Create a copy of LiquidSdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$LiquidSdkEvent_PaymentRefundPendingImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as Payment,
    ));
  }
}

/// @nodoc

class _$LiquidSdkEvent_PaymentRefundPendingImpl extends LiquidSdkEvent_PaymentRefundPending {
  const _$LiquidSdkEvent_PaymentRefundPendingImpl({required this.details}) : super._();

  @override
  final Payment details;

  @override
  String toString() {
    return 'LiquidSdkEvent.paymentRefundPending(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiquidSdkEvent_PaymentRefundPendingImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  /// Create a copy of LiquidSdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$LiquidSdkEvent_PaymentRefundPendingImplCopyWith<_$LiquidSdkEvent_PaymentRefundPendingImpl>
      get copyWith =>
          __$$LiquidSdkEvent_PaymentRefundPendingImplCopyWithImpl<_$LiquidSdkEvent_PaymentRefundPendingImpl>(
              this, _$identity);
}

abstract class LiquidSdkEvent_PaymentRefundPending extends LiquidSdkEvent {
  const factory LiquidSdkEvent_PaymentRefundPending({required final Payment details}) =
      _$LiquidSdkEvent_PaymentRefundPendingImpl;
  const LiquidSdkEvent_PaymentRefundPending._() : super._();

  Payment get details;

  /// Create a copy of LiquidSdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$LiquidSdkEvent_PaymentRefundPendingImplCopyWith<_$LiquidSdkEvent_PaymentRefundPendingImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LiquidSdkEvent_PaymentSucceededImplCopyWith<$Res> {
  factory _$$LiquidSdkEvent_PaymentSucceededImplCopyWith(_$LiquidSdkEvent_PaymentSucceededImpl value,
          $Res Function(_$LiquidSdkEvent_PaymentSucceededImpl) then) =
      __$$LiquidSdkEvent_PaymentSucceededImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$LiquidSdkEvent_PaymentSucceededImplCopyWithImpl<$Res>
    extends _$LiquidSdkEventCopyWithImpl<$Res, _$LiquidSdkEvent_PaymentSucceededImpl>
    implements _$$LiquidSdkEvent_PaymentSucceededImplCopyWith<$Res> {
  __$$LiquidSdkEvent_PaymentSucceededImplCopyWithImpl(_$LiquidSdkEvent_PaymentSucceededImpl _value,
      $Res Function(_$LiquidSdkEvent_PaymentSucceededImpl) _then)
      : super(_value, _then);

  /// Create a copy of LiquidSdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$LiquidSdkEvent_PaymentSucceededImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as Payment,
    ));
  }
}

/// @nodoc

class _$LiquidSdkEvent_PaymentSucceededImpl extends LiquidSdkEvent_PaymentSucceeded {
  const _$LiquidSdkEvent_PaymentSucceededImpl({required this.details}) : super._();

  @override
  final Payment details;

  @override
  String toString() {
    return 'LiquidSdkEvent.paymentSucceeded(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiquidSdkEvent_PaymentSucceededImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  /// Create a copy of LiquidSdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$LiquidSdkEvent_PaymentSucceededImplCopyWith<_$LiquidSdkEvent_PaymentSucceededImpl> get copyWith =>
      __$$LiquidSdkEvent_PaymentSucceededImplCopyWithImpl<_$LiquidSdkEvent_PaymentSucceededImpl>(
          this, _$identity);
}

abstract class LiquidSdkEvent_PaymentSucceeded extends LiquidSdkEvent {
  const factory LiquidSdkEvent_PaymentSucceeded({required final Payment details}) =
      _$LiquidSdkEvent_PaymentSucceededImpl;
  const LiquidSdkEvent_PaymentSucceeded._() : super._();

  Payment get details;

  /// Create a copy of LiquidSdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$LiquidSdkEvent_PaymentSucceededImplCopyWith<_$LiquidSdkEvent_PaymentSucceededImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LiquidSdkEvent_PaymentWaitingConfirmationImplCopyWith<$Res> {
  factory _$$LiquidSdkEvent_PaymentWaitingConfirmationImplCopyWith(
          _$LiquidSdkEvent_PaymentWaitingConfirmationImpl value,
          $Res Function(_$LiquidSdkEvent_PaymentWaitingConfirmationImpl) then) =
      __$$LiquidSdkEvent_PaymentWaitingConfirmationImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$LiquidSdkEvent_PaymentWaitingConfirmationImplCopyWithImpl<$Res>
    extends _$LiquidSdkEventCopyWithImpl<$Res, _$LiquidSdkEvent_PaymentWaitingConfirmationImpl>
    implements _$$LiquidSdkEvent_PaymentWaitingConfirmationImplCopyWith<$Res> {
  __$$LiquidSdkEvent_PaymentWaitingConfirmationImplCopyWithImpl(
      _$LiquidSdkEvent_PaymentWaitingConfirmationImpl _value,
      $Res Function(_$LiquidSdkEvent_PaymentWaitingConfirmationImpl) _then)
      : super(_value, _then);

  /// Create a copy of LiquidSdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$LiquidSdkEvent_PaymentWaitingConfirmationImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as Payment,
    ));
  }
}

/// @nodoc

class _$LiquidSdkEvent_PaymentWaitingConfirmationImpl extends LiquidSdkEvent_PaymentWaitingConfirmation {
  const _$LiquidSdkEvent_PaymentWaitingConfirmationImpl({required this.details}) : super._();

  @override
  final Payment details;

  @override
  String toString() {
    return 'LiquidSdkEvent.paymentWaitingConfirmation(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiquidSdkEvent_PaymentWaitingConfirmationImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  /// Create a copy of LiquidSdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$LiquidSdkEvent_PaymentWaitingConfirmationImplCopyWith<_$LiquidSdkEvent_PaymentWaitingConfirmationImpl>
      get copyWith => __$$LiquidSdkEvent_PaymentWaitingConfirmationImplCopyWithImpl<
          _$LiquidSdkEvent_PaymentWaitingConfirmationImpl>(this, _$identity);
}

abstract class LiquidSdkEvent_PaymentWaitingConfirmation extends LiquidSdkEvent {
  const factory LiquidSdkEvent_PaymentWaitingConfirmation({required final Payment details}) =
      _$LiquidSdkEvent_PaymentWaitingConfirmationImpl;
  const LiquidSdkEvent_PaymentWaitingConfirmation._() : super._();

  Payment get details;

  /// Create a copy of LiquidSdkEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$LiquidSdkEvent_PaymentWaitingConfirmationImplCopyWith<_$LiquidSdkEvent_PaymentWaitingConfirmationImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LiquidSdkEvent_SyncedImplCopyWith<$Res> {
  factory _$$LiquidSdkEvent_SyncedImplCopyWith(
          _$LiquidSdkEvent_SyncedImpl value, $Res Function(_$LiquidSdkEvent_SyncedImpl) then) =
      __$$LiquidSdkEvent_SyncedImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$LiquidSdkEvent_SyncedImplCopyWithImpl<$Res>
    extends _$LiquidSdkEventCopyWithImpl<$Res, _$LiquidSdkEvent_SyncedImpl>
    implements _$$LiquidSdkEvent_SyncedImplCopyWith<$Res> {
  __$$LiquidSdkEvent_SyncedImplCopyWithImpl(
      _$LiquidSdkEvent_SyncedImpl _value, $Res Function(_$LiquidSdkEvent_SyncedImpl) _then)
      : super(_value, _then);

  /// Create a copy of LiquidSdkEvent
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$LiquidSdkEvent_SyncedImpl extends LiquidSdkEvent_Synced {
  const _$LiquidSdkEvent_SyncedImpl() : super._();

  @override
  String toString() {
    return 'LiquidSdkEvent.synced()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$LiquidSdkEvent_SyncedImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class LiquidSdkEvent_Synced extends LiquidSdkEvent {
  const factory LiquidSdkEvent_Synced() = _$LiquidSdkEvent_SyncedImpl;
  const LiquidSdkEvent_Synced._() : super._();
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
