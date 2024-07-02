// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'duplicates.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$LnUrlAuthError {
  String get err => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) serviceConnectivity,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? serviceConnectivity,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlAuthError_Generic value) generic,
    required TResult Function(LnUrlAuthError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlAuthError_ServiceConnectivity value) serviceConnectivity,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlAuthError_Generic value)? generic,
    TResult? Function(LnUrlAuthError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlAuthError_ServiceConnectivity value)? serviceConnectivity,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlAuthError_Generic value)? generic,
    TResult Function(LnUrlAuthError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlAuthError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $LnUrlAuthErrorCopyWith<LnUrlAuthError> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LnUrlAuthErrorCopyWith<$Res> {
  factory $LnUrlAuthErrorCopyWith(LnUrlAuthError value, $Res Function(LnUrlAuthError) then) =
      _$LnUrlAuthErrorCopyWithImpl<$Res, LnUrlAuthError>;
  @useResult
  $Res call({String err});
}

/// @nodoc
class _$LnUrlAuthErrorCopyWithImpl<$Res, $Val extends LnUrlAuthError>
    implements $LnUrlAuthErrorCopyWith<$Res> {
  _$LnUrlAuthErrorCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_value.copyWith(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$LnUrlAuthError_GenericImplCopyWith<$Res> implements $LnUrlAuthErrorCopyWith<$Res> {
  factory _$$LnUrlAuthError_GenericImplCopyWith(
          _$LnUrlAuthError_GenericImpl value, $Res Function(_$LnUrlAuthError_GenericImpl) then) =
      __$$LnUrlAuthError_GenericImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$LnUrlAuthError_GenericImplCopyWithImpl<$Res>
    extends _$LnUrlAuthErrorCopyWithImpl<$Res, _$LnUrlAuthError_GenericImpl>
    implements _$$LnUrlAuthError_GenericImplCopyWith<$Res> {
  __$$LnUrlAuthError_GenericImplCopyWithImpl(
      _$LnUrlAuthError_GenericImpl _value, $Res Function(_$LnUrlAuthError_GenericImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$LnUrlAuthError_GenericImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LnUrlAuthError_GenericImpl extends LnUrlAuthError_Generic {
  const _$LnUrlAuthError_GenericImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'LnUrlAuthError.generic(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlAuthError_GenericImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlAuthError_GenericImplCopyWith<_$LnUrlAuthError_GenericImpl> get copyWith =>
      __$$LnUrlAuthError_GenericImplCopyWithImpl<_$LnUrlAuthError_GenericImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) serviceConnectivity,
  }) {
    return generic(err);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? serviceConnectivity,
  }) {
    return generic?.call(err);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (generic != null) {
      return generic(err);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlAuthError_Generic value) generic,
    required TResult Function(LnUrlAuthError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlAuthError_ServiceConnectivity value) serviceConnectivity,
  }) {
    return generic(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlAuthError_Generic value)? generic,
    TResult? Function(LnUrlAuthError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlAuthError_ServiceConnectivity value)? serviceConnectivity,
  }) {
    return generic?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlAuthError_Generic value)? generic,
    TResult Function(LnUrlAuthError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlAuthError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (generic != null) {
      return generic(this);
    }
    return orElse();
  }
}

abstract class LnUrlAuthError_Generic extends LnUrlAuthError {
  const factory LnUrlAuthError_Generic({required final String err}) = _$LnUrlAuthError_GenericImpl;
  const LnUrlAuthError_Generic._() : super._();

  @override
  String get err;
  @override
  @JsonKey(ignore: true)
  _$$LnUrlAuthError_GenericImplCopyWith<_$LnUrlAuthError_GenericImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlAuthError_InvalidUriImplCopyWith<$Res> implements $LnUrlAuthErrorCopyWith<$Res> {
  factory _$$LnUrlAuthError_InvalidUriImplCopyWith(
          _$LnUrlAuthError_InvalidUriImpl value, $Res Function(_$LnUrlAuthError_InvalidUriImpl) then) =
      __$$LnUrlAuthError_InvalidUriImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$LnUrlAuthError_InvalidUriImplCopyWithImpl<$Res>
    extends _$LnUrlAuthErrorCopyWithImpl<$Res, _$LnUrlAuthError_InvalidUriImpl>
    implements _$$LnUrlAuthError_InvalidUriImplCopyWith<$Res> {
  __$$LnUrlAuthError_InvalidUriImplCopyWithImpl(
      _$LnUrlAuthError_InvalidUriImpl _value, $Res Function(_$LnUrlAuthError_InvalidUriImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$LnUrlAuthError_InvalidUriImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LnUrlAuthError_InvalidUriImpl extends LnUrlAuthError_InvalidUri {
  const _$LnUrlAuthError_InvalidUriImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'LnUrlAuthError.invalidUri(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlAuthError_InvalidUriImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlAuthError_InvalidUriImplCopyWith<_$LnUrlAuthError_InvalidUriImpl> get copyWith =>
      __$$LnUrlAuthError_InvalidUriImplCopyWithImpl<_$LnUrlAuthError_InvalidUriImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) serviceConnectivity,
  }) {
    return invalidUri(err);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? serviceConnectivity,
  }) {
    return invalidUri?.call(err);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (invalidUri != null) {
      return invalidUri(err);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlAuthError_Generic value) generic,
    required TResult Function(LnUrlAuthError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlAuthError_ServiceConnectivity value) serviceConnectivity,
  }) {
    return invalidUri(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlAuthError_Generic value)? generic,
    TResult? Function(LnUrlAuthError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlAuthError_ServiceConnectivity value)? serviceConnectivity,
  }) {
    return invalidUri?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlAuthError_Generic value)? generic,
    TResult Function(LnUrlAuthError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlAuthError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (invalidUri != null) {
      return invalidUri(this);
    }
    return orElse();
  }
}

abstract class LnUrlAuthError_InvalidUri extends LnUrlAuthError {
  const factory LnUrlAuthError_InvalidUri({required final String err}) = _$LnUrlAuthError_InvalidUriImpl;
  const LnUrlAuthError_InvalidUri._() : super._();

  @override
  String get err;
  @override
  @JsonKey(ignore: true)
  _$$LnUrlAuthError_InvalidUriImplCopyWith<_$LnUrlAuthError_InvalidUriImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlAuthError_ServiceConnectivityImplCopyWith<$Res>
    implements $LnUrlAuthErrorCopyWith<$Res> {
  factory _$$LnUrlAuthError_ServiceConnectivityImplCopyWith(_$LnUrlAuthError_ServiceConnectivityImpl value,
          $Res Function(_$LnUrlAuthError_ServiceConnectivityImpl) then) =
      __$$LnUrlAuthError_ServiceConnectivityImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$LnUrlAuthError_ServiceConnectivityImplCopyWithImpl<$Res>
    extends _$LnUrlAuthErrorCopyWithImpl<$Res, _$LnUrlAuthError_ServiceConnectivityImpl>
    implements _$$LnUrlAuthError_ServiceConnectivityImplCopyWith<$Res> {
  __$$LnUrlAuthError_ServiceConnectivityImplCopyWithImpl(_$LnUrlAuthError_ServiceConnectivityImpl _value,
      $Res Function(_$LnUrlAuthError_ServiceConnectivityImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$LnUrlAuthError_ServiceConnectivityImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LnUrlAuthError_ServiceConnectivityImpl extends LnUrlAuthError_ServiceConnectivity {
  const _$LnUrlAuthError_ServiceConnectivityImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'LnUrlAuthError.serviceConnectivity(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlAuthError_ServiceConnectivityImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlAuthError_ServiceConnectivityImplCopyWith<_$LnUrlAuthError_ServiceConnectivityImpl> get copyWith =>
      __$$LnUrlAuthError_ServiceConnectivityImplCopyWithImpl<_$LnUrlAuthError_ServiceConnectivityImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) serviceConnectivity,
  }) {
    return serviceConnectivity(err);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? serviceConnectivity,
  }) {
    return serviceConnectivity?.call(err);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (serviceConnectivity != null) {
      return serviceConnectivity(err);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlAuthError_Generic value) generic,
    required TResult Function(LnUrlAuthError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlAuthError_ServiceConnectivity value) serviceConnectivity,
  }) {
    return serviceConnectivity(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlAuthError_Generic value)? generic,
    TResult? Function(LnUrlAuthError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlAuthError_ServiceConnectivity value)? serviceConnectivity,
  }) {
    return serviceConnectivity?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlAuthError_Generic value)? generic,
    TResult Function(LnUrlAuthError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlAuthError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (serviceConnectivity != null) {
      return serviceConnectivity(this);
    }
    return orElse();
  }
}

abstract class LnUrlAuthError_ServiceConnectivity extends LnUrlAuthError {
  const factory LnUrlAuthError_ServiceConnectivity({required final String err}) =
      _$LnUrlAuthError_ServiceConnectivityImpl;
  const LnUrlAuthError_ServiceConnectivity._() : super._();

  @override
  String get err;
  @override
  @JsonKey(ignore: true)
  _$$LnUrlAuthError_ServiceConnectivityImplCopyWith<_$LnUrlAuthError_ServiceConnectivityImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LnUrlCallbackStatus {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() ok,
    required TResult Function(LnUrlErrorData data) errorStatus,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? ok,
    TResult? Function(LnUrlErrorData data)? errorStatus,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? ok,
    TResult Function(LnUrlErrorData data)? errorStatus,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlCallbackStatus_Ok value) ok,
    required TResult Function(LnUrlCallbackStatus_ErrorStatus value) errorStatus,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlCallbackStatus_Ok value)? ok,
    TResult? Function(LnUrlCallbackStatus_ErrorStatus value)? errorStatus,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlCallbackStatus_Ok value)? ok,
    TResult Function(LnUrlCallbackStatus_ErrorStatus value)? errorStatus,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LnUrlCallbackStatusCopyWith<$Res> {
  factory $LnUrlCallbackStatusCopyWith(LnUrlCallbackStatus value, $Res Function(LnUrlCallbackStatus) then) =
      _$LnUrlCallbackStatusCopyWithImpl<$Res, LnUrlCallbackStatus>;
}

/// @nodoc
class _$LnUrlCallbackStatusCopyWithImpl<$Res, $Val extends LnUrlCallbackStatus>
    implements $LnUrlCallbackStatusCopyWith<$Res> {
  _$LnUrlCallbackStatusCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$LnUrlCallbackStatus_OkImplCopyWith<$Res> {
  factory _$$LnUrlCallbackStatus_OkImplCopyWith(
          _$LnUrlCallbackStatus_OkImpl value, $Res Function(_$LnUrlCallbackStatus_OkImpl) then) =
      __$$LnUrlCallbackStatus_OkImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$LnUrlCallbackStatus_OkImplCopyWithImpl<$Res>
    extends _$LnUrlCallbackStatusCopyWithImpl<$Res, _$LnUrlCallbackStatus_OkImpl>
    implements _$$LnUrlCallbackStatus_OkImplCopyWith<$Res> {
  __$$LnUrlCallbackStatus_OkImplCopyWithImpl(
      _$LnUrlCallbackStatus_OkImpl _value, $Res Function(_$LnUrlCallbackStatus_OkImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$LnUrlCallbackStatus_OkImpl extends LnUrlCallbackStatus_Ok {
  const _$LnUrlCallbackStatus_OkImpl() : super._();

  @override
  String toString() {
    return 'LnUrlCallbackStatus.ok()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$LnUrlCallbackStatus_OkImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() ok,
    required TResult Function(LnUrlErrorData data) errorStatus,
  }) {
    return ok();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? ok,
    TResult? Function(LnUrlErrorData data)? errorStatus,
  }) {
    return ok?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? ok,
    TResult Function(LnUrlErrorData data)? errorStatus,
    required TResult orElse(),
  }) {
    if (ok != null) {
      return ok();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlCallbackStatus_Ok value) ok,
    required TResult Function(LnUrlCallbackStatus_ErrorStatus value) errorStatus,
  }) {
    return ok(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlCallbackStatus_Ok value)? ok,
    TResult? Function(LnUrlCallbackStatus_ErrorStatus value)? errorStatus,
  }) {
    return ok?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlCallbackStatus_Ok value)? ok,
    TResult Function(LnUrlCallbackStatus_ErrorStatus value)? errorStatus,
    required TResult orElse(),
  }) {
    if (ok != null) {
      return ok(this);
    }
    return orElse();
  }
}

abstract class LnUrlCallbackStatus_Ok extends LnUrlCallbackStatus {
  const factory LnUrlCallbackStatus_Ok() = _$LnUrlCallbackStatus_OkImpl;
  const LnUrlCallbackStatus_Ok._() : super._();
}

/// @nodoc
abstract class _$$LnUrlCallbackStatus_ErrorStatusImplCopyWith<$Res> {
  factory _$$LnUrlCallbackStatus_ErrorStatusImplCopyWith(_$LnUrlCallbackStatus_ErrorStatusImpl value,
          $Res Function(_$LnUrlCallbackStatus_ErrorStatusImpl) then) =
      __$$LnUrlCallbackStatus_ErrorStatusImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlErrorData data});
}

/// @nodoc
class __$$LnUrlCallbackStatus_ErrorStatusImplCopyWithImpl<$Res>
    extends _$LnUrlCallbackStatusCopyWithImpl<$Res, _$LnUrlCallbackStatus_ErrorStatusImpl>
    implements _$$LnUrlCallbackStatus_ErrorStatusImplCopyWith<$Res> {
  __$$LnUrlCallbackStatus_ErrorStatusImplCopyWithImpl(_$LnUrlCallbackStatus_ErrorStatusImpl _value,
      $Res Function(_$LnUrlCallbackStatus_ErrorStatusImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$LnUrlCallbackStatus_ErrorStatusImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlErrorData,
    ));
  }
}

/// @nodoc

class _$LnUrlCallbackStatus_ErrorStatusImpl extends LnUrlCallbackStatus_ErrorStatus {
  const _$LnUrlCallbackStatus_ErrorStatusImpl({required this.data}) : super._();

  @override
  final LnUrlErrorData data;

  @override
  String toString() {
    return 'LnUrlCallbackStatus.errorStatus(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlCallbackStatus_ErrorStatusImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlCallbackStatus_ErrorStatusImplCopyWith<_$LnUrlCallbackStatus_ErrorStatusImpl> get copyWith =>
      __$$LnUrlCallbackStatus_ErrorStatusImplCopyWithImpl<_$LnUrlCallbackStatus_ErrorStatusImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() ok,
    required TResult Function(LnUrlErrorData data) errorStatus,
  }) {
    return errorStatus(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? ok,
    TResult? Function(LnUrlErrorData data)? errorStatus,
  }) {
    return errorStatus?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? ok,
    TResult Function(LnUrlErrorData data)? errorStatus,
    required TResult orElse(),
  }) {
    if (errorStatus != null) {
      return errorStatus(data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlCallbackStatus_Ok value) ok,
    required TResult Function(LnUrlCallbackStatus_ErrorStatus value) errorStatus,
  }) {
    return errorStatus(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlCallbackStatus_Ok value)? ok,
    TResult? Function(LnUrlCallbackStatus_ErrorStatus value)? errorStatus,
  }) {
    return errorStatus?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlCallbackStatus_Ok value)? ok,
    TResult Function(LnUrlCallbackStatus_ErrorStatus value)? errorStatus,
    required TResult orElse(),
  }) {
    if (errorStatus != null) {
      return errorStatus(this);
    }
    return orElse();
  }
}

abstract class LnUrlCallbackStatus_ErrorStatus extends LnUrlCallbackStatus {
  const factory LnUrlCallbackStatus_ErrorStatus({required final LnUrlErrorData data}) =
      _$LnUrlCallbackStatus_ErrorStatusImpl;
  const LnUrlCallbackStatus_ErrorStatus._() : super._();

  LnUrlErrorData get data;
  @JsonKey(ignore: true)
  _$$LnUrlCallbackStatus_ErrorStatusImplCopyWith<_$LnUrlCallbackStatus_ErrorStatusImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LnUrlPayError {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() alreadyPaid,
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidAmount,
    required TResult Function(String err) invalidInvoice,
    required TResult Function(String err) invalidNetwork,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) invoiceExpired,
    required TResult Function(String err) paymentFailed,
    required TResult Function(String err) paymentTimeout,
    required TResult Function(String err) routeNotFound,
    required TResult Function(String err) routeTooExpensive,
    required TResult Function(String err) serviceConnectivity,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? alreadyPaid,
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidAmount,
    TResult? Function(String err)? invalidInvoice,
    TResult? Function(String err)? invalidNetwork,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? invoiceExpired,
    TResult? Function(String err)? paymentFailed,
    TResult? Function(String err)? paymentTimeout,
    TResult? Function(String err)? routeNotFound,
    TResult? Function(String err)? routeTooExpensive,
    TResult? Function(String err)? serviceConnectivity,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? alreadyPaid,
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidAmount,
    TResult Function(String err)? invalidInvoice,
    TResult Function(String err)? invalidNetwork,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? invoiceExpired,
    TResult Function(String err)? paymentFailed,
    TResult Function(String err)? paymentTimeout,
    TResult Function(String err)? routeNotFound,
    TResult Function(String err)? routeTooExpensive,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlPayError_AlreadyPaid value) alreadyPaid,
    required TResult Function(LnUrlPayError_Generic value) generic,
    required TResult Function(LnUrlPayError_InvalidAmount value) invalidAmount,
    required TResult Function(LnUrlPayError_InvalidInvoice value) invalidInvoice,
    required TResult Function(LnUrlPayError_InvalidNetwork value) invalidNetwork,
    required TResult Function(LnUrlPayError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlPayError_InvoiceExpired value) invoiceExpired,
    required TResult Function(LnUrlPayError_PaymentFailed value) paymentFailed,
    required TResult Function(LnUrlPayError_PaymentTimeout value) paymentTimeout,
    required TResult Function(LnUrlPayError_RouteNotFound value) routeNotFound,
    required TResult Function(LnUrlPayError_RouteTooExpensive value) routeTooExpensive,
    required TResult Function(LnUrlPayError_ServiceConnectivity value) serviceConnectivity,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult? Function(LnUrlPayError_Generic value)? generic,
    TResult? Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult? Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult? Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult? Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult? Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult? Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult? Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult? Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult? Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult Function(LnUrlPayError_Generic value)? generic,
    TResult Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LnUrlPayErrorCopyWith<$Res> {
  factory $LnUrlPayErrorCopyWith(LnUrlPayError value, $Res Function(LnUrlPayError) then) =
      _$LnUrlPayErrorCopyWithImpl<$Res, LnUrlPayError>;
}

/// @nodoc
class _$LnUrlPayErrorCopyWithImpl<$Res, $Val extends LnUrlPayError> implements $LnUrlPayErrorCopyWith<$Res> {
  _$LnUrlPayErrorCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$LnUrlPayError_AlreadyPaidImplCopyWith<$Res> {
  factory _$$LnUrlPayError_AlreadyPaidImplCopyWith(
          _$LnUrlPayError_AlreadyPaidImpl value, $Res Function(_$LnUrlPayError_AlreadyPaidImpl) then) =
      __$$LnUrlPayError_AlreadyPaidImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$LnUrlPayError_AlreadyPaidImplCopyWithImpl<$Res>
    extends _$LnUrlPayErrorCopyWithImpl<$Res, _$LnUrlPayError_AlreadyPaidImpl>
    implements _$$LnUrlPayError_AlreadyPaidImplCopyWith<$Res> {
  __$$LnUrlPayError_AlreadyPaidImplCopyWithImpl(
      _$LnUrlPayError_AlreadyPaidImpl _value, $Res Function(_$LnUrlPayError_AlreadyPaidImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$LnUrlPayError_AlreadyPaidImpl extends LnUrlPayError_AlreadyPaid {
  const _$LnUrlPayError_AlreadyPaidImpl() : super._();

  @override
  String toString() {
    return 'LnUrlPayError.alreadyPaid()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$LnUrlPayError_AlreadyPaidImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() alreadyPaid,
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidAmount,
    required TResult Function(String err) invalidInvoice,
    required TResult Function(String err) invalidNetwork,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) invoiceExpired,
    required TResult Function(String err) paymentFailed,
    required TResult Function(String err) paymentTimeout,
    required TResult Function(String err) routeNotFound,
    required TResult Function(String err) routeTooExpensive,
    required TResult Function(String err) serviceConnectivity,
  }) {
    return alreadyPaid();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? alreadyPaid,
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidAmount,
    TResult? Function(String err)? invalidInvoice,
    TResult? Function(String err)? invalidNetwork,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? invoiceExpired,
    TResult? Function(String err)? paymentFailed,
    TResult? Function(String err)? paymentTimeout,
    TResult? Function(String err)? routeNotFound,
    TResult? Function(String err)? routeTooExpensive,
    TResult? Function(String err)? serviceConnectivity,
  }) {
    return alreadyPaid?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? alreadyPaid,
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidAmount,
    TResult Function(String err)? invalidInvoice,
    TResult Function(String err)? invalidNetwork,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? invoiceExpired,
    TResult Function(String err)? paymentFailed,
    TResult Function(String err)? paymentTimeout,
    TResult Function(String err)? routeNotFound,
    TResult Function(String err)? routeTooExpensive,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (alreadyPaid != null) {
      return alreadyPaid();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlPayError_AlreadyPaid value) alreadyPaid,
    required TResult Function(LnUrlPayError_Generic value) generic,
    required TResult Function(LnUrlPayError_InvalidAmount value) invalidAmount,
    required TResult Function(LnUrlPayError_InvalidInvoice value) invalidInvoice,
    required TResult Function(LnUrlPayError_InvalidNetwork value) invalidNetwork,
    required TResult Function(LnUrlPayError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlPayError_InvoiceExpired value) invoiceExpired,
    required TResult Function(LnUrlPayError_PaymentFailed value) paymentFailed,
    required TResult Function(LnUrlPayError_PaymentTimeout value) paymentTimeout,
    required TResult Function(LnUrlPayError_RouteNotFound value) routeNotFound,
    required TResult Function(LnUrlPayError_RouteTooExpensive value) routeTooExpensive,
    required TResult Function(LnUrlPayError_ServiceConnectivity value) serviceConnectivity,
  }) {
    return alreadyPaid(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult? Function(LnUrlPayError_Generic value)? generic,
    TResult? Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult? Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult? Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult? Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult? Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult? Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult? Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult? Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult? Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
  }) {
    return alreadyPaid?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult Function(LnUrlPayError_Generic value)? generic,
    TResult Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (alreadyPaid != null) {
      return alreadyPaid(this);
    }
    return orElse();
  }
}

abstract class LnUrlPayError_AlreadyPaid extends LnUrlPayError {
  const factory LnUrlPayError_AlreadyPaid() = _$LnUrlPayError_AlreadyPaidImpl;
  const LnUrlPayError_AlreadyPaid._() : super._();
}

/// @nodoc
abstract class _$$LnUrlPayError_GenericImplCopyWith<$Res> {
  factory _$$LnUrlPayError_GenericImplCopyWith(
          _$LnUrlPayError_GenericImpl value, $Res Function(_$LnUrlPayError_GenericImpl) then) =
      __$$LnUrlPayError_GenericImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$LnUrlPayError_GenericImplCopyWithImpl<$Res>
    extends _$LnUrlPayErrorCopyWithImpl<$Res, _$LnUrlPayError_GenericImpl>
    implements _$$LnUrlPayError_GenericImplCopyWith<$Res> {
  __$$LnUrlPayError_GenericImplCopyWithImpl(
      _$LnUrlPayError_GenericImpl _value, $Res Function(_$LnUrlPayError_GenericImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$LnUrlPayError_GenericImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LnUrlPayError_GenericImpl extends LnUrlPayError_Generic {
  const _$LnUrlPayError_GenericImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'LnUrlPayError.generic(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayError_GenericImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayError_GenericImplCopyWith<_$LnUrlPayError_GenericImpl> get copyWith =>
      __$$LnUrlPayError_GenericImplCopyWithImpl<_$LnUrlPayError_GenericImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() alreadyPaid,
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidAmount,
    required TResult Function(String err) invalidInvoice,
    required TResult Function(String err) invalidNetwork,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) invoiceExpired,
    required TResult Function(String err) paymentFailed,
    required TResult Function(String err) paymentTimeout,
    required TResult Function(String err) routeNotFound,
    required TResult Function(String err) routeTooExpensive,
    required TResult Function(String err) serviceConnectivity,
  }) {
    return generic(err);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? alreadyPaid,
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidAmount,
    TResult? Function(String err)? invalidInvoice,
    TResult? Function(String err)? invalidNetwork,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? invoiceExpired,
    TResult? Function(String err)? paymentFailed,
    TResult? Function(String err)? paymentTimeout,
    TResult? Function(String err)? routeNotFound,
    TResult? Function(String err)? routeTooExpensive,
    TResult? Function(String err)? serviceConnectivity,
  }) {
    return generic?.call(err);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? alreadyPaid,
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidAmount,
    TResult Function(String err)? invalidInvoice,
    TResult Function(String err)? invalidNetwork,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? invoiceExpired,
    TResult Function(String err)? paymentFailed,
    TResult Function(String err)? paymentTimeout,
    TResult Function(String err)? routeNotFound,
    TResult Function(String err)? routeTooExpensive,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (generic != null) {
      return generic(err);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlPayError_AlreadyPaid value) alreadyPaid,
    required TResult Function(LnUrlPayError_Generic value) generic,
    required TResult Function(LnUrlPayError_InvalidAmount value) invalidAmount,
    required TResult Function(LnUrlPayError_InvalidInvoice value) invalidInvoice,
    required TResult Function(LnUrlPayError_InvalidNetwork value) invalidNetwork,
    required TResult Function(LnUrlPayError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlPayError_InvoiceExpired value) invoiceExpired,
    required TResult Function(LnUrlPayError_PaymentFailed value) paymentFailed,
    required TResult Function(LnUrlPayError_PaymentTimeout value) paymentTimeout,
    required TResult Function(LnUrlPayError_RouteNotFound value) routeNotFound,
    required TResult Function(LnUrlPayError_RouteTooExpensive value) routeTooExpensive,
    required TResult Function(LnUrlPayError_ServiceConnectivity value) serviceConnectivity,
  }) {
    return generic(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult? Function(LnUrlPayError_Generic value)? generic,
    TResult? Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult? Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult? Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult? Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult? Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult? Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult? Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult? Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult? Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
  }) {
    return generic?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult Function(LnUrlPayError_Generic value)? generic,
    TResult Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (generic != null) {
      return generic(this);
    }
    return orElse();
  }
}

abstract class LnUrlPayError_Generic extends LnUrlPayError {
  const factory LnUrlPayError_Generic({required final String err}) = _$LnUrlPayError_GenericImpl;
  const LnUrlPayError_Generic._() : super._();

  String get err;
  @JsonKey(ignore: true)
  _$$LnUrlPayError_GenericImplCopyWith<_$LnUrlPayError_GenericImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlPayError_InvalidAmountImplCopyWith<$Res> {
  factory _$$LnUrlPayError_InvalidAmountImplCopyWith(
          _$LnUrlPayError_InvalidAmountImpl value, $Res Function(_$LnUrlPayError_InvalidAmountImpl) then) =
      __$$LnUrlPayError_InvalidAmountImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$LnUrlPayError_InvalidAmountImplCopyWithImpl<$Res>
    extends _$LnUrlPayErrorCopyWithImpl<$Res, _$LnUrlPayError_InvalidAmountImpl>
    implements _$$LnUrlPayError_InvalidAmountImplCopyWith<$Res> {
  __$$LnUrlPayError_InvalidAmountImplCopyWithImpl(
      _$LnUrlPayError_InvalidAmountImpl _value, $Res Function(_$LnUrlPayError_InvalidAmountImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$LnUrlPayError_InvalidAmountImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LnUrlPayError_InvalidAmountImpl extends LnUrlPayError_InvalidAmount {
  const _$LnUrlPayError_InvalidAmountImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'LnUrlPayError.invalidAmount(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayError_InvalidAmountImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayError_InvalidAmountImplCopyWith<_$LnUrlPayError_InvalidAmountImpl> get copyWith =>
      __$$LnUrlPayError_InvalidAmountImplCopyWithImpl<_$LnUrlPayError_InvalidAmountImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() alreadyPaid,
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidAmount,
    required TResult Function(String err) invalidInvoice,
    required TResult Function(String err) invalidNetwork,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) invoiceExpired,
    required TResult Function(String err) paymentFailed,
    required TResult Function(String err) paymentTimeout,
    required TResult Function(String err) routeNotFound,
    required TResult Function(String err) routeTooExpensive,
    required TResult Function(String err) serviceConnectivity,
  }) {
    return invalidAmount(err);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? alreadyPaid,
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidAmount,
    TResult? Function(String err)? invalidInvoice,
    TResult? Function(String err)? invalidNetwork,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? invoiceExpired,
    TResult? Function(String err)? paymentFailed,
    TResult? Function(String err)? paymentTimeout,
    TResult? Function(String err)? routeNotFound,
    TResult? Function(String err)? routeTooExpensive,
    TResult? Function(String err)? serviceConnectivity,
  }) {
    return invalidAmount?.call(err);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? alreadyPaid,
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidAmount,
    TResult Function(String err)? invalidInvoice,
    TResult Function(String err)? invalidNetwork,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? invoiceExpired,
    TResult Function(String err)? paymentFailed,
    TResult Function(String err)? paymentTimeout,
    TResult Function(String err)? routeNotFound,
    TResult Function(String err)? routeTooExpensive,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (invalidAmount != null) {
      return invalidAmount(err);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlPayError_AlreadyPaid value) alreadyPaid,
    required TResult Function(LnUrlPayError_Generic value) generic,
    required TResult Function(LnUrlPayError_InvalidAmount value) invalidAmount,
    required TResult Function(LnUrlPayError_InvalidInvoice value) invalidInvoice,
    required TResult Function(LnUrlPayError_InvalidNetwork value) invalidNetwork,
    required TResult Function(LnUrlPayError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlPayError_InvoiceExpired value) invoiceExpired,
    required TResult Function(LnUrlPayError_PaymentFailed value) paymentFailed,
    required TResult Function(LnUrlPayError_PaymentTimeout value) paymentTimeout,
    required TResult Function(LnUrlPayError_RouteNotFound value) routeNotFound,
    required TResult Function(LnUrlPayError_RouteTooExpensive value) routeTooExpensive,
    required TResult Function(LnUrlPayError_ServiceConnectivity value) serviceConnectivity,
  }) {
    return invalidAmount(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult? Function(LnUrlPayError_Generic value)? generic,
    TResult? Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult? Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult? Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult? Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult? Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult? Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult? Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult? Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult? Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
  }) {
    return invalidAmount?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult Function(LnUrlPayError_Generic value)? generic,
    TResult Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (invalidAmount != null) {
      return invalidAmount(this);
    }
    return orElse();
  }
}

abstract class LnUrlPayError_InvalidAmount extends LnUrlPayError {
  const factory LnUrlPayError_InvalidAmount({required final String err}) = _$LnUrlPayError_InvalidAmountImpl;
  const LnUrlPayError_InvalidAmount._() : super._();

  String get err;
  @JsonKey(ignore: true)
  _$$LnUrlPayError_InvalidAmountImplCopyWith<_$LnUrlPayError_InvalidAmountImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlPayError_InvalidInvoiceImplCopyWith<$Res> {
  factory _$$LnUrlPayError_InvalidInvoiceImplCopyWith(
          _$LnUrlPayError_InvalidInvoiceImpl value, $Res Function(_$LnUrlPayError_InvalidInvoiceImpl) then) =
      __$$LnUrlPayError_InvalidInvoiceImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$LnUrlPayError_InvalidInvoiceImplCopyWithImpl<$Res>
    extends _$LnUrlPayErrorCopyWithImpl<$Res, _$LnUrlPayError_InvalidInvoiceImpl>
    implements _$$LnUrlPayError_InvalidInvoiceImplCopyWith<$Res> {
  __$$LnUrlPayError_InvalidInvoiceImplCopyWithImpl(
      _$LnUrlPayError_InvalidInvoiceImpl _value, $Res Function(_$LnUrlPayError_InvalidInvoiceImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$LnUrlPayError_InvalidInvoiceImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LnUrlPayError_InvalidInvoiceImpl extends LnUrlPayError_InvalidInvoice {
  const _$LnUrlPayError_InvalidInvoiceImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'LnUrlPayError.invalidInvoice(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayError_InvalidInvoiceImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayError_InvalidInvoiceImplCopyWith<_$LnUrlPayError_InvalidInvoiceImpl> get copyWith =>
      __$$LnUrlPayError_InvalidInvoiceImplCopyWithImpl<_$LnUrlPayError_InvalidInvoiceImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() alreadyPaid,
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidAmount,
    required TResult Function(String err) invalidInvoice,
    required TResult Function(String err) invalidNetwork,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) invoiceExpired,
    required TResult Function(String err) paymentFailed,
    required TResult Function(String err) paymentTimeout,
    required TResult Function(String err) routeNotFound,
    required TResult Function(String err) routeTooExpensive,
    required TResult Function(String err) serviceConnectivity,
  }) {
    return invalidInvoice(err);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? alreadyPaid,
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidAmount,
    TResult? Function(String err)? invalidInvoice,
    TResult? Function(String err)? invalidNetwork,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? invoiceExpired,
    TResult? Function(String err)? paymentFailed,
    TResult? Function(String err)? paymentTimeout,
    TResult? Function(String err)? routeNotFound,
    TResult? Function(String err)? routeTooExpensive,
    TResult? Function(String err)? serviceConnectivity,
  }) {
    return invalidInvoice?.call(err);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? alreadyPaid,
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidAmount,
    TResult Function(String err)? invalidInvoice,
    TResult Function(String err)? invalidNetwork,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? invoiceExpired,
    TResult Function(String err)? paymentFailed,
    TResult Function(String err)? paymentTimeout,
    TResult Function(String err)? routeNotFound,
    TResult Function(String err)? routeTooExpensive,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (invalidInvoice != null) {
      return invalidInvoice(err);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlPayError_AlreadyPaid value) alreadyPaid,
    required TResult Function(LnUrlPayError_Generic value) generic,
    required TResult Function(LnUrlPayError_InvalidAmount value) invalidAmount,
    required TResult Function(LnUrlPayError_InvalidInvoice value) invalidInvoice,
    required TResult Function(LnUrlPayError_InvalidNetwork value) invalidNetwork,
    required TResult Function(LnUrlPayError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlPayError_InvoiceExpired value) invoiceExpired,
    required TResult Function(LnUrlPayError_PaymentFailed value) paymentFailed,
    required TResult Function(LnUrlPayError_PaymentTimeout value) paymentTimeout,
    required TResult Function(LnUrlPayError_RouteNotFound value) routeNotFound,
    required TResult Function(LnUrlPayError_RouteTooExpensive value) routeTooExpensive,
    required TResult Function(LnUrlPayError_ServiceConnectivity value) serviceConnectivity,
  }) {
    return invalidInvoice(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult? Function(LnUrlPayError_Generic value)? generic,
    TResult? Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult? Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult? Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult? Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult? Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult? Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult? Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult? Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult? Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
  }) {
    return invalidInvoice?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult Function(LnUrlPayError_Generic value)? generic,
    TResult Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (invalidInvoice != null) {
      return invalidInvoice(this);
    }
    return orElse();
  }
}

abstract class LnUrlPayError_InvalidInvoice extends LnUrlPayError {
  const factory LnUrlPayError_InvalidInvoice({required final String err}) =
      _$LnUrlPayError_InvalidInvoiceImpl;
  const LnUrlPayError_InvalidInvoice._() : super._();

  String get err;
  @JsonKey(ignore: true)
  _$$LnUrlPayError_InvalidInvoiceImplCopyWith<_$LnUrlPayError_InvalidInvoiceImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlPayError_InvalidNetworkImplCopyWith<$Res> {
  factory _$$LnUrlPayError_InvalidNetworkImplCopyWith(
          _$LnUrlPayError_InvalidNetworkImpl value, $Res Function(_$LnUrlPayError_InvalidNetworkImpl) then) =
      __$$LnUrlPayError_InvalidNetworkImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$LnUrlPayError_InvalidNetworkImplCopyWithImpl<$Res>
    extends _$LnUrlPayErrorCopyWithImpl<$Res, _$LnUrlPayError_InvalidNetworkImpl>
    implements _$$LnUrlPayError_InvalidNetworkImplCopyWith<$Res> {
  __$$LnUrlPayError_InvalidNetworkImplCopyWithImpl(
      _$LnUrlPayError_InvalidNetworkImpl _value, $Res Function(_$LnUrlPayError_InvalidNetworkImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$LnUrlPayError_InvalidNetworkImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LnUrlPayError_InvalidNetworkImpl extends LnUrlPayError_InvalidNetwork {
  const _$LnUrlPayError_InvalidNetworkImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'LnUrlPayError.invalidNetwork(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayError_InvalidNetworkImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayError_InvalidNetworkImplCopyWith<_$LnUrlPayError_InvalidNetworkImpl> get copyWith =>
      __$$LnUrlPayError_InvalidNetworkImplCopyWithImpl<_$LnUrlPayError_InvalidNetworkImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() alreadyPaid,
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidAmount,
    required TResult Function(String err) invalidInvoice,
    required TResult Function(String err) invalidNetwork,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) invoiceExpired,
    required TResult Function(String err) paymentFailed,
    required TResult Function(String err) paymentTimeout,
    required TResult Function(String err) routeNotFound,
    required TResult Function(String err) routeTooExpensive,
    required TResult Function(String err) serviceConnectivity,
  }) {
    return invalidNetwork(err);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? alreadyPaid,
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidAmount,
    TResult? Function(String err)? invalidInvoice,
    TResult? Function(String err)? invalidNetwork,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? invoiceExpired,
    TResult? Function(String err)? paymentFailed,
    TResult? Function(String err)? paymentTimeout,
    TResult? Function(String err)? routeNotFound,
    TResult? Function(String err)? routeTooExpensive,
    TResult? Function(String err)? serviceConnectivity,
  }) {
    return invalidNetwork?.call(err);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? alreadyPaid,
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidAmount,
    TResult Function(String err)? invalidInvoice,
    TResult Function(String err)? invalidNetwork,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? invoiceExpired,
    TResult Function(String err)? paymentFailed,
    TResult Function(String err)? paymentTimeout,
    TResult Function(String err)? routeNotFound,
    TResult Function(String err)? routeTooExpensive,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (invalidNetwork != null) {
      return invalidNetwork(err);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlPayError_AlreadyPaid value) alreadyPaid,
    required TResult Function(LnUrlPayError_Generic value) generic,
    required TResult Function(LnUrlPayError_InvalidAmount value) invalidAmount,
    required TResult Function(LnUrlPayError_InvalidInvoice value) invalidInvoice,
    required TResult Function(LnUrlPayError_InvalidNetwork value) invalidNetwork,
    required TResult Function(LnUrlPayError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlPayError_InvoiceExpired value) invoiceExpired,
    required TResult Function(LnUrlPayError_PaymentFailed value) paymentFailed,
    required TResult Function(LnUrlPayError_PaymentTimeout value) paymentTimeout,
    required TResult Function(LnUrlPayError_RouteNotFound value) routeNotFound,
    required TResult Function(LnUrlPayError_RouteTooExpensive value) routeTooExpensive,
    required TResult Function(LnUrlPayError_ServiceConnectivity value) serviceConnectivity,
  }) {
    return invalidNetwork(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult? Function(LnUrlPayError_Generic value)? generic,
    TResult? Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult? Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult? Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult? Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult? Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult? Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult? Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult? Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult? Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
  }) {
    return invalidNetwork?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult Function(LnUrlPayError_Generic value)? generic,
    TResult Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (invalidNetwork != null) {
      return invalidNetwork(this);
    }
    return orElse();
  }
}

abstract class LnUrlPayError_InvalidNetwork extends LnUrlPayError {
  const factory LnUrlPayError_InvalidNetwork({required final String err}) =
      _$LnUrlPayError_InvalidNetworkImpl;
  const LnUrlPayError_InvalidNetwork._() : super._();

  String get err;
  @JsonKey(ignore: true)
  _$$LnUrlPayError_InvalidNetworkImplCopyWith<_$LnUrlPayError_InvalidNetworkImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlPayError_InvalidUriImplCopyWith<$Res> {
  factory _$$LnUrlPayError_InvalidUriImplCopyWith(
          _$LnUrlPayError_InvalidUriImpl value, $Res Function(_$LnUrlPayError_InvalidUriImpl) then) =
      __$$LnUrlPayError_InvalidUriImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$LnUrlPayError_InvalidUriImplCopyWithImpl<$Res>
    extends _$LnUrlPayErrorCopyWithImpl<$Res, _$LnUrlPayError_InvalidUriImpl>
    implements _$$LnUrlPayError_InvalidUriImplCopyWith<$Res> {
  __$$LnUrlPayError_InvalidUriImplCopyWithImpl(
      _$LnUrlPayError_InvalidUriImpl _value, $Res Function(_$LnUrlPayError_InvalidUriImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$LnUrlPayError_InvalidUriImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LnUrlPayError_InvalidUriImpl extends LnUrlPayError_InvalidUri {
  const _$LnUrlPayError_InvalidUriImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'LnUrlPayError.invalidUri(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayError_InvalidUriImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayError_InvalidUriImplCopyWith<_$LnUrlPayError_InvalidUriImpl> get copyWith =>
      __$$LnUrlPayError_InvalidUriImplCopyWithImpl<_$LnUrlPayError_InvalidUriImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() alreadyPaid,
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidAmount,
    required TResult Function(String err) invalidInvoice,
    required TResult Function(String err) invalidNetwork,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) invoiceExpired,
    required TResult Function(String err) paymentFailed,
    required TResult Function(String err) paymentTimeout,
    required TResult Function(String err) routeNotFound,
    required TResult Function(String err) routeTooExpensive,
    required TResult Function(String err) serviceConnectivity,
  }) {
    return invalidUri(err);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? alreadyPaid,
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidAmount,
    TResult? Function(String err)? invalidInvoice,
    TResult? Function(String err)? invalidNetwork,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? invoiceExpired,
    TResult? Function(String err)? paymentFailed,
    TResult? Function(String err)? paymentTimeout,
    TResult? Function(String err)? routeNotFound,
    TResult? Function(String err)? routeTooExpensive,
    TResult? Function(String err)? serviceConnectivity,
  }) {
    return invalidUri?.call(err);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? alreadyPaid,
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidAmount,
    TResult Function(String err)? invalidInvoice,
    TResult Function(String err)? invalidNetwork,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? invoiceExpired,
    TResult Function(String err)? paymentFailed,
    TResult Function(String err)? paymentTimeout,
    TResult Function(String err)? routeNotFound,
    TResult Function(String err)? routeTooExpensive,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (invalidUri != null) {
      return invalidUri(err);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlPayError_AlreadyPaid value) alreadyPaid,
    required TResult Function(LnUrlPayError_Generic value) generic,
    required TResult Function(LnUrlPayError_InvalidAmount value) invalidAmount,
    required TResult Function(LnUrlPayError_InvalidInvoice value) invalidInvoice,
    required TResult Function(LnUrlPayError_InvalidNetwork value) invalidNetwork,
    required TResult Function(LnUrlPayError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlPayError_InvoiceExpired value) invoiceExpired,
    required TResult Function(LnUrlPayError_PaymentFailed value) paymentFailed,
    required TResult Function(LnUrlPayError_PaymentTimeout value) paymentTimeout,
    required TResult Function(LnUrlPayError_RouteNotFound value) routeNotFound,
    required TResult Function(LnUrlPayError_RouteTooExpensive value) routeTooExpensive,
    required TResult Function(LnUrlPayError_ServiceConnectivity value) serviceConnectivity,
  }) {
    return invalidUri(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult? Function(LnUrlPayError_Generic value)? generic,
    TResult? Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult? Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult? Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult? Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult? Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult? Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult? Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult? Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult? Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
  }) {
    return invalidUri?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult Function(LnUrlPayError_Generic value)? generic,
    TResult Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (invalidUri != null) {
      return invalidUri(this);
    }
    return orElse();
  }
}

abstract class LnUrlPayError_InvalidUri extends LnUrlPayError {
  const factory LnUrlPayError_InvalidUri({required final String err}) = _$LnUrlPayError_InvalidUriImpl;
  const LnUrlPayError_InvalidUri._() : super._();

  String get err;
  @JsonKey(ignore: true)
  _$$LnUrlPayError_InvalidUriImplCopyWith<_$LnUrlPayError_InvalidUriImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlPayError_InvoiceExpiredImplCopyWith<$Res> {
  factory _$$LnUrlPayError_InvoiceExpiredImplCopyWith(
          _$LnUrlPayError_InvoiceExpiredImpl value, $Res Function(_$LnUrlPayError_InvoiceExpiredImpl) then) =
      __$$LnUrlPayError_InvoiceExpiredImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$LnUrlPayError_InvoiceExpiredImplCopyWithImpl<$Res>
    extends _$LnUrlPayErrorCopyWithImpl<$Res, _$LnUrlPayError_InvoiceExpiredImpl>
    implements _$$LnUrlPayError_InvoiceExpiredImplCopyWith<$Res> {
  __$$LnUrlPayError_InvoiceExpiredImplCopyWithImpl(
      _$LnUrlPayError_InvoiceExpiredImpl _value, $Res Function(_$LnUrlPayError_InvoiceExpiredImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$LnUrlPayError_InvoiceExpiredImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LnUrlPayError_InvoiceExpiredImpl extends LnUrlPayError_InvoiceExpired {
  const _$LnUrlPayError_InvoiceExpiredImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'LnUrlPayError.invoiceExpired(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayError_InvoiceExpiredImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayError_InvoiceExpiredImplCopyWith<_$LnUrlPayError_InvoiceExpiredImpl> get copyWith =>
      __$$LnUrlPayError_InvoiceExpiredImplCopyWithImpl<_$LnUrlPayError_InvoiceExpiredImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() alreadyPaid,
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidAmount,
    required TResult Function(String err) invalidInvoice,
    required TResult Function(String err) invalidNetwork,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) invoiceExpired,
    required TResult Function(String err) paymentFailed,
    required TResult Function(String err) paymentTimeout,
    required TResult Function(String err) routeNotFound,
    required TResult Function(String err) routeTooExpensive,
    required TResult Function(String err) serviceConnectivity,
  }) {
    return invoiceExpired(err);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? alreadyPaid,
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidAmount,
    TResult? Function(String err)? invalidInvoice,
    TResult? Function(String err)? invalidNetwork,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? invoiceExpired,
    TResult? Function(String err)? paymentFailed,
    TResult? Function(String err)? paymentTimeout,
    TResult? Function(String err)? routeNotFound,
    TResult? Function(String err)? routeTooExpensive,
    TResult? Function(String err)? serviceConnectivity,
  }) {
    return invoiceExpired?.call(err);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? alreadyPaid,
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidAmount,
    TResult Function(String err)? invalidInvoice,
    TResult Function(String err)? invalidNetwork,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? invoiceExpired,
    TResult Function(String err)? paymentFailed,
    TResult Function(String err)? paymentTimeout,
    TResult Function(String err)? routeNotFound,
    TResult Function(String err)? routeTooExpensive,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (invoiceExpired != null) {
      return invoiceExpired(err);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlPayError_AlreadyPaid value) alreadyPaid,
    required TResult Function(LnUrlPayError_Generic value) generic,
    required TResult Function(LnUrlPayError_InvalidAmount value) invalidAmount,
    required TResult Function(LnUrlPayError_InvalidInvoice value) invalidInvoice,
    required TResult Function(LnUrlPayError_InvalidNetwork value) invalidNetwork,
    required TResult Function(LnUrlPayError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlPayError_InvoiceExpired value) invoiceExpired,
    required TResult Function(LnUrlPayError_PaymentFailed value) paymentFailed,
    required TResult Function(LnUrlPayError_PaymentTimeout value) paymentTimeout,
    required TResult Function(LnUrlPayError_RouteNotFound value) routeNotFound,
    required TResult Function(LnUrlPayError_RouteTooExpensive value) routeTooExpensive,
    required TResult Function(LnUrlPayError_ServiceConnectivity value) serviceConnectivity,
  }) {
    return invoiceExpired(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult? Function(LnUrlPayError_Generic value)? generic,
    TResult? Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult? Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult? Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult? Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult? Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult? Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult? Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult? Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult? Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
  }) {
    return invoiceExpired?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult Function(LnUrlPayError_Generic value)? generic,
    TResult Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (invoiceExpired != null) {
      return invoiceExpired(this);
    }
    return orElse();
  }
}

abstract class LnUrlPayError_InvoiceExpired extends LnUrlPayError {
  const factory LnUrlPayError_InvoiceExpired({required final String err}) =
      _$LnUrlPayError_InvoiceExpiredImpl;
  const LnUrlPayError_InvoiceExpired._() : super._();

  String get err;
  @JsonKey(ignore: true)
  _$$LnUrlPayError_InvoiceExpiredImplCopyWith<_$LnUrlPayError_InvoiceExpiredImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlPayError_PaymentFailedImplCopyWith<$Res> {
  factory _$$LnUrlPayError_PaymentFailedImplCopyWith(
          _$LnUrlPayError_PaymentFailedImpl value, $Res Function(_$LnUrlPayError_PaymentFailedImpl) then) =
      __$$LnUrlPayError_PaymentFailedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$LnUrlPayError_PaymentFailedImplCopyWithImpl<$Res>
    extends _$LnUrlPayErrorCopyWithImpl<$Res, _$LnUrlPayError_PaymentFailedImpl>
    implements _$$LnUrlPayError_PaymentFailedImplCopyWith<$Res> {
  __$$LnUrlPayError_PaymentFailedImplCopyWithImpl(
      _$LnUrlPayError_PaymentFailedImpl _value, $Res Function(_$LnUrlPayError_PaymentFailedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$LnUrlPayError_PaymentFailedImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LnUrlPayError_PaymentFailedImpl extends LnUrlPayError_PaymentFailed {
  const _$LnUrlPayError_PaymentFailedImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'LnUrlPayError.paymentFailed(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayError_PaymentFailedImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayError_PaymentFailedImplCopyWith<_$LnUrlPayError_PaymentFailedImpl> get copyWith =>
      __$$LnUrlPayError_PaymentFailedImplCopyWithImpl<_$LnUrlPayError_PaymentFailedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() alreadyPaid,
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidAmount,
    required TResult Function(String err) invalidInvoice,
    required TResult Function(String err) invalidNetwork,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) invoiceExpired,
    required TResult Function(String err) paymentFailed,
    required TResult Function(String err) paymentTimeout,
    required TResult Function(String err) routeNotFound,
    required TResult Function(String err) routeTooExpensive,
    required TResult Function(String err) serviceConnectivity,
  }) {
    return paymentFailed(err);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? alreadyPaid,
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidAmount,
    TResult? Function(String err)? invalidInvoice,
    TResult? Function(String err)? invalidNetwork,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? invoiceExpired,
    TResult? Function(String err)? paymentFailed,
    TResult? Function(String err)? paymentTimeout,
    TResult? Function(String err)? routeNotFound,
    TResult? Function(String err)? routeTooExpensive,
    TResult? Function(String err)? serviceConnectivity,
  }) {
    return paymentFailed?.call(err);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? alreadyPaid,
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidAmount,
    TResult Function(String err)? invalidInvoice,
    TResult Function(String err)? invalidNetwork,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? invoiceExpired,
    TResult Function(String err)? paymentFailed,
    TResult Function(String err)? paymentTimeout,
    TResult Function(String err)? routeNotFound,
    TResult Function(String err)? routeTooExpensive,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (paymentFailed != null) {
      return paymentFailed(err);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlPayError_AlreadyPaid value) alreadyPaid,
    required TResult Function(LnUrlPayError_Generic value) generic,
    required TResult Function(LnUrlPayError_InvalidAmount value) invalidAmount,
    required TResult Function(LnUrlPayError_InvalidInvoice value) invalidInvoice,
    required TResult Function(LnUrlPayError_InvalidNetwork value) invalidNetwork,
    required TResult Function(LnUrlPayError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlPayError_InvoiceExpired value) invoiceExpired,
    required TResult Function(LnUrlPayError_PaymentFailed value) paymentFailed,
    required TResult Function(LnUrlPayError_PaymentTimeout value) paymentTimeout,
    required TResult Function(LnUrlPayError_RouteNotFound value) routeNotFound,
    required TResult Function(LnUrlPayError_RouteTooExpensive value) routeTooExpensive,
    required TResult Function(LnUrlPayError_ServiceConnectivity value) serviceConnectivity,
  }) {
    return paymentFailed(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult? Function(LnUrlPayError_Generic value)? generic,
    TResult? Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult? Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult? Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult? Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult? Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult? Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult? Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult? Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult? Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
  }) {
    return paymentFailed?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult Function(LnUrlPayError_Generic value)? generic,
    TResult Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (paymentFailed != null) {
      return paymentFailed(this);
    }
    return orElse();
  }
}

abstract class LnUrlPayError_PaymentFailed extends LnUrlPayError {
  const factory LnUrlPayError_PaymentFailed({required final String err}) = _$LnUrlPayError_PaymentFailedImpl;
  const LnUrlPayError_PaymentFailed._() : super._();

  String get err;
  @JsonKey(ignore: true)
  _$$LnUrlPayError_PaymentFailedImplCopyWith<_$LnUrlPayError_PaymentFailedImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlPayError_PaymentTimeoutImplCopyWith<$Res> {
  factory _$$LnUrlPayError_PaymentTimeoutImplCopyWith(
          _$LnUrlPayError_PaymentTimeoutImpl value, $Res Function(_$LnUrlPayError_PaymentTimeoutImpl) then) =
      __$$LnUrlPayError_PaymentTimeoutImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$LnUrlPayError_PaymentTimeoutImplCopyWithImpl<$Res>
    extends _$LnUrlPayErrorCopyWithImpl<$Res, _$LnUrlPayError_PaymentTimeoutImpl>
    implements _$$LnUrlPayError_PaymentTimeoutImplCopyWith<$Res> {
  __$$LnUrlPayError_PaymentTimeoutImplCopyWithImpl(
      _$LnUrlPayError_PaymentTimeoutImpl _value, $Res Function(_$LnUrlPayError_PaymentTimeoutImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$LnUrlPayError_PaymentTimeoutImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LnUrlPayError_PaymentTimeoutImpl extends LnUrlPayError_PaymentTimeout {
  const _$LnUrlPayError_PaymentTimeoutImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'LnUrlPayError.paymentTimeout(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayError_PaymentTimeoutImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayError_PaymentTimeoutImplCopyWith<_$LnUrlPayError_PaymentTimeoutImpl> get copyWith =>
      __$$LnUrlPayError_PaymentTimeoutImplCopyWithImpl<_$LnUrlPayError_PaymentTimeoutImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() alreadyPaid,
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidAmount,
    required TResult Function(String err) invalidInvoice,
    required TResult Function(String err) invalidNetwork,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) invoiceExpired,
    required TResult Function(String err) paymentFailed,
    required TResult Function(String err) paymentTimeout,
    required TResult Function(String err) routeNotFound,
    required TResult Function(String err) routeTooExpensive,
    required TResult Function(String err) serviceConnectivity,
  }) {
    return paymentTimeout(err);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? alreadyPaid,
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidAmount,
    TResult? Function(String err)? invalidInvoice,
    TResult? Function(String err)? invalidNetwork,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? invoiceExpired,
    TResult? Function(String err)? paymentFailed,
    TResult? Function(String err)? paymentTimeout,
    TResult? Function(String err)? routeNotFound,
    TResult? Function(String err)? routeTooExpensive,
    TResult? Function(String err)? serviceConnectivity,
  }) {
    return paymentTimeout?.call(err);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? alreadyPaid,
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidAmount,
    TResult Function(String err)? invalidInvoice,
    TResult Function(String err)? invalidNetwork,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? invoiceExpired,
    TResult Function(String err)? paymentFailed,
    TResult Function(String err)? paymentTimeout,
    TResult Function(String err)? routeNotFound,
    TResult Function(String err)? routeTooExpensive,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (paymentTimeout != null) {
      return paymentTimeout(err);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlPayError_AlreadyPaid value) alreadyPaid,
    required TResult Function(LnUrlPayError_Generic value) generic,
    required TResult Function(LnUrlPayError_InvalidAmount value) invalidAmount,
    required TResult Function(LnUrlPayError_InvalidInvoice value) invalidInvoice,
    required TResult Function(LnUrlPayError_InvalidNetwork value) invalidNetwork,
    required TResult Function(LnUrlPayError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlPayError_InvoiceExpired value) invoiceExpired,
    required TResult Function(LnUrlPayError_PaymentFailed value) paymentFailed,
    required TResult Function(LnUrlPayError_PaymentTimeout value) paymentTimeout,
    required TResult Function(LnUrlPayError_RouteNotFound value) routeNotFound,
    required TResult Function(LnUrlPayError_RouteTooExpensive value) routeTooExpensive,
    required TResult Function(LnUrlPayError_ServiceConnectivity value) serviceConnectivity,
  }) {
    return paymentTimeout(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult? Function(LnUrlPayError_Generic value)? generic,
    TResult? Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult? Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult? Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult? Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult? Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult? Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult? Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult? Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult? Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
  }) {
    return paymentTimeout?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult Function(LnUrlPayError_Generic value)? generic,
    TResult Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (paymentTimeout != null) {
      return paymentTimeout(this);
    }
    return orElse();
  }
}

abstract class LnUrlPayError_PaymentTimeout extends LnUrlPayError {
  const factory LnUrlPayError_PaymentTimeout({required final String err}) =
      _$LnUrlPayError_PaymentTimeoutImpl;
  const LnUrlPayError_PaymentTimeout._() : super._();

  String get err;
  @JsonKey(ignore: true)
  _$$LnUrlPayError_PaymentTimeoutImplCopyWith<_$LnUrlPayError_PaymentTimeoutImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlPayError_RouteNotFoundImplCopyWith<$Res> {
  factory _$$LnUrlPayError_RouteNotFoundImplCopyWith(
          _$LnUrlPayError_RouteNotFoundImpl value, $Res Function(_$LnUrlPayError_RouteNotFoundImpl) then) =
      __$$LnUrlPayError_RouteNotFoundImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$LnUrlPayError_RouteNotFoundImplCopyWithImpl<$Res>
    extends _$LnUrlPayErrorCopyWithImpl<$Res, _$LnUrlPayError_RouteNotFoundImpl>
    implements _$$LnUrlPayError_RouteNotFoundImplCopyWith<$Res> {
  __$$LnUrlPayError_RouteNotFoundImplCopyWithImpl(
      _$LnUrlPayError_RouteNotFoundImpl _value, $Res Function(_$LnUrlPayError_RouteNotFoundImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$LnUrlPayError_RouteNotFoundImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LnUrlPayError_RouteNotFoundImpl extends LnUrlPayError_RouteNotFound {
  const _$LnUrlPayError_RouteNotFoundImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'LnUrlPayError.routeNotFound(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayError_RouteNotFoundImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayError_RouteNotFoundImplCopyWith<_$LnUrlPayError_RouteNotFoundImpl> get copyWith =>
      __$$LnUrlPayError_RouteNotFoundImplCopyWithImpl<_$LnUrlPayError_RouteNotFoundImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() alreadyPaid,
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidAmount,
    required TResult Function(String err) invalidInvoice,
    required TResult Function(String err) invalidNetwork,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) invoiceExpired,
    required TResult Function(String err) paymentFailed,
    required TResult Function(String err) paymentTimeout,
    required TResult Function(String err) routeNotFound,
    required TResult Function(String err) routeTooExpensive,
    required TResult Function(String err) serviceConnectivity,
  }) {
    return routeNotFound(err);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? alreadyPaid,
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidAmount,
    TResult? Function(String err)? invalidInvoice,
    TResult? Function(String err)? invalidNetwork,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? invoiceExpired,
    TResult? Function(String err)? paymentFailed,
    TResult? Function(String err)? paymentTimeout,
    TResult? Function(String err)? routeNotFound,
    TResult? Function(String err)? routeTooExpensive,
    TResult? Function(String err)? serviceConnectivity,
  }) {
    return routeNotFound?.call(err);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? alreadyPaid,
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidAmount,
    TResult Function(String err)? invalidInvoice,
    TResult Function(String err)? invalidNetwork,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? invoiceExpired,
    TResult Function(String err)? paymentFailed,
    TResult Function(String err)? paymentTimeout,
    TResult Function(String err)? routeNotFound,
    TResult Function(String err)? routeTooExpensive,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (routeNotFound != null) {
      return routeNotFound(err);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlPayError_AlreadyPaid value) alreadyPaid,
    required TResult Function(LnUrlPayError_Generic value) generic,
    required TResult Function(LnUrlPayError_InvalidAmount value) invalidAmount,
    required TResult Function(LnUrlPayError_InvalidInvoice value) invalidInvoice,
    required TResult Function(LnUrlPayError_InvalidNetwork value) invalidNetwork,
    required TResult Function(LnUrlPayError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlPayError_InvoiceExpired value) invoiceExpired,
    required TResult Function(LnUrlPayError_PaymentFailed value) paymentFailed,
    required TResult Function(LnUrlPayError_PaymentTimeout value) paymentTimeout,
    required TResult Function(LnUrlPayError_RouteNotFound value) routeNotFound,
    required TResult Function(LnUrlPayError_RouteTooExpensive value) routeTooExpensive,
    required TResult Function(LnUrlPayError_ServiceConnectivity value) serviceConnectivity,
  }) {
    return routeNotFound(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult? Function(LnUrlPayError_Generic value)? generic,
    TResult? Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult? Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult? Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult? Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult? Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult? Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult? Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult? Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult? Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
  }) {
    return routeNotFound?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult Function(LnUrlPayError_Generic value)? generic,
    TResult Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (routeNotFound != null) {
      return routeNotFound(this);
    }
    return orElse();
  }
}

abstract class LnUrlPayError_RouteNotFound extends LnUrlPayError {
  const factory LnUrlPayError_RouteNotFound({required final String err}) = _$LnUrlPayError_RouteNotFoundImpl;
  const LnUrlPayError_RouteNotFound._() : super._();

  String get err;
  @JsonKey(ignore: true)
  _$$LnUrlPayError_RouteNotFoundImplCopyWith<_$LnUrlPayError_RouteNotFoundImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlPayError_RouteTooExpensiveImplCopyWith<$Res> {
  factory _$$LnUrlPayError_RouteTooExpensiveImplCopyWith(_$LnUrlPayError_RouteTooExpensiveImpl value,
          $Res Function(_$LnUrlPayError_RouteTooExpensiveImpl) then) =
      __$$LnUrlPayError_RouteTooExpensiveImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$LnUrlPayError_RouteTooExpensiveImplCopyWithImpl<$Res>
    extends _$LnUrlPayErrorCopyWithImpl<$Res, _$LnUrlPayError_RouteTooExpensiveImpl>
    implements _$$LnUrlPayError_RouteTooExpensiveImplCopyWith<$Res> {
  __$$LnUrlPayError_RouteTooExpensiveImplCopyWithImpl(_$LnUrlPayError_RouteTooExpensiveImpl _value,
      $Res Function(_$LnUrlPayError_RouteTooExpensiveImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$LnUrlPayError_RouteTooExpensiveImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LnUrlPayError_RouteTooExpensiveImpl extends LnUrlPayError_RouteTooExpensive {
  const _$LnUrlPayError_RouteTooExpensiveImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'LnUrlPayError.routeTooExpensive(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayError_RouteTooExpensiveImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayError_RouteTooExpensiveImplCopyWith<_$LnUrlPayError_RouteTooExpensiveImpl> get copyWith =>
      __$$LnUrlPayError_RouteTooExpensiveImplCopyWithImpl<_$LnUrlPayError_RouteTooExpensiveImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() alreadyPaid,
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidAmount,
    required TResult Function(String err) invalidInvoice,
    required TResult Function(String err) invalidNetwork,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) invoiceExpired,
    required TResult Function(String err) paymentFailed,
    required TResult Function(String err) paymentTimeout,
    required TResult Function(String err) routeNotFound,
    required TResult Function(String err) routeTooExpensive,
    required TResult Function(String err) serviceConnectivity,
  }) {
    return routeTooExpensive(err);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? alreadyPaid,
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidAmount,
    TResult? Function(String err)? invalidInvoice,
    TResult? Function(String err)? invalidNetwork,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? invoiceExpired,
    TResult? Function(String err)? paymentFailed,
    TResult? Function(String err)? paymentTimeout,
    TResult? Function(String err)? routeNotFound,
    TResult? Function(String err)? routeTooExpensive,
    TResult? Function(String err)? serviceConnectivity,
  }) {
    return routeTooExpensive?.call(err);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? alreadyPaid,
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidAmount,
    TResult Function(String err)? invalidInvoice,
    TResult Function(String err)? invalidNetwork,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? invoiceExpired,
    TResult Function(String err)? paymentFailed,
    TResult Function(String err)? paymentTimeout,
    TResult Function(String err)? routeNotFound,
    TResult Function(String err)? routeTooExpensive,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (routeTooExpensive != null) {
      return routeTooExpensive(err);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlPayError_AlreadyPaid value) alreadyPaid,
    required TResult Function(LnUrlPayError_Generic value) generic,
    required TResult Function(LnUrlPayError_InvalidAmount value) invalidAmount,
    required TResult Function(LnUrlPayError_InvalidInvoice value) invalidInvoice,
    required TResult Function(LnUrlPayError_InvalidNetwork value) invalidNetwork,
    required TResult Function(LnUrlPayError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlPayError_InvoiceExpired value) invoiceExpired,
    required TResult Function(LnUrlPayError_PaymentFailed value) paymentFailed,
    required TResult Function(LnUrlPayError_PaymentTimeout value) paymentTimeout,
    required TResult Function(LnUrlPayError_RouteNotFound value) routeNotFound,
    required TResult Function(LnUrlPayError_RouteTooExpensive value) routeTooExpensive,
    required TResult Function(LnUrlPayError_ServiceConnectivity value) serviceConnectivity,
  }) {
    return routeTooExpensive(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult? Function(LnUrlPayError_Generic value)? generic,
    TResult? Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult? Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult? Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult? Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult? Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult? Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult? Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult? Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult? Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
  }) {
    return routeTooExpensive?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult Function(LnUrlPayError_Generic value)? generic,
    TResult Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (routeTooExpensive != null) {
      return routeTooExpensive(this);
    }
    return orElse();
  }
}

abstract class LnUrlPayError_RouteTooExpensive extends LnUrlPayError {
  const factory LnUrlPayError_RouteTooExpensive({required final String err}) =
      _$LnUrlPayError_RouteTooExpensiveImpl;
  const LnUrlPayError_RouteTooExpensive._() : super._();

  String get err;
  @JsonKey(ignore: true)
  _$$LnUrlPayError_RouteTooExpensiveImplCopyWith<_$LnUrlPayError_RouteTooExpensiveImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlPayError_ServiceConnectivityImplCopyWith<$Res> {
  factory _$$LnUrlPayError_ServiceConnectivityImplCopyWith(_$LnUrlPayError_ServiceConnectivityImpl value,
          $Res Function(_$LnUrlPayError_ServiceConnectivityImpl) then) =
      __$$LnUrlPayError_ServiceConnectivityImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$LnUrlPayError_ServiceConnectivityImplCopyWithImpl<$Res>
    extends _$LnUrlPayErrorCopyWithImpl<$Res, _$LnUrlPayError_ServiceConnectivityImpl>
    implements _$$LnUrlPayError_ServiceConnectivityImplCopyWith<$Res> {
  __$$LnUrlPayError_ServiceConnectivityImplCopyWithImpl(_$LnUrlPayError_ServiceConnectivityImpl _value,
      $Res Function(_$LnUrlPayError_ServiceConnectivityImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$LnUrlPayError_ServiceConnectivityImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LnUrlPayError_ServiceConnectivityImpl extends LnUrlPayError_ServiceConnectivity {
  const _$LnUrlPayError_ServiceConnectivityImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'LnUrlPayError.serviceConnectivity(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayError_ServiceConnectivityImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayError_ServiceConnectivityImplCopyWith<_$LnUrlPayError_ServiceConnectivityImpl> get copyWith =>
      __$$LnUrlPayError_ServiceConnectivityImplCopyWithImpl<_$LnUrlPayError_ServiceConnectivityImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() alreadyPaid,
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidAmount,
    required TResult Function(String err) invalidInvoice,
    required TResult Function(String err) invalidNetwork,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) invoiceExpired,
    required TResult Function(String err) paymentFailed,
    required TResult Function(String err) paymentTimeout,
    required TResult Function(String err) routeNotFound,
    required TResult Function(String err) routeTooExpensive,
    required TResult Function(String err) serviceConnectivity,
  }) {
    return serviceConnectivity(err);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? alreadyPaid,
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidAmount,
    TResult? Function(String err)? invalidInvoice,
    TResult? Function(String err)? invalidNetwork,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? invoiceExpired,
    TResult? Function(String err)? paymentFailed,
    TResult? Function(String err)? paymentTimeout,
    TResult? Function(String err)? routeNotFound,
    TResult? Function(String err)? routeTooExpensive,
    TResult? Function(String err)? serviceConnectivity,
  }) {
    return serviceConnectivity?.call(err);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? alreadyPaid,
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidAmount,
    TResult Function(String err)? invalidInvoice,
    TResult Function(String err)? invalidNetwork,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? invoiceExpired,
    TResult Function(String err)? paymentFailed,
    TResult Function(String err)? paymentTimeout,
    TResult Function(String err)? routeNotFound,
    TResult Function(String err)? routeTooExpensive,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (serviceConnectivity != null) {
      return serviceConnectivity(err);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlPayError_AlreadyPaid value) alreadyPaid,
    required TResult Function(LnUrlPayError_Generic value) generic,
    required TResult Function(LnUrlPayError_InvalidAmount value) invalidAmount,
    required TResult Function(LnUrlPayError_InvalidInvoice value) invalidInvoice,
    required TResult Function(LnUrlPayError_InvalidNetwork value) invalidNetwork,
    required TResult Function(LnUrlPayError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlPayError_InvoiceExpired value) invoiceExpired,
    required TResult Function(LnUrlPayError_PaymentFailed value) paymentFailed,
    required TResult Function(LnUrlPayError_PaymentTimeout value) paymentTimeout,
    required TResult Function(LnUrlPayError_RouteNotFound value) routeNotFound,
    required TResult Function(LnUrlPayError_RouteTooExpensive value) routeTooExpensive,
    required TResult Function(LnUrlPayError_ServiceConnectivity value) serviceConnectivity,
  }) {
    return serviceConnectivity(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult? Function(LnUrlPayError_Generic value)? generic,
    TResult? Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult? Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult? Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult? Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult? Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult? Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult? Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult? Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult? Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
  }) {
    return serviceConnectivity?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlPayError_AlreadyPaid value)? alreadyPaid,
    TResult Function(LnUrlPayError_Generic value)? generic,
    TResult Function(LnUrlPayError_InvalidAmount value)? invalidAmount,
    TResult Function(LnUrlPayError_InvalidInvoice value)? invalidInvoice,
    TResult Function(LnUrlPayError_InvalidNetwork value)? invalidNetwork,
    TResult Function(LnUrlPayError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlPayError_InvoiceExpired value)? invoiceExpired,
    TResult Function(LnUrlPayError_PaymentFailed value)? paymentFailed,
    TResult Function(LnUrlPayError_PaymentTimeout value)? paymentTimeout,
    TResult Function(LnUrlPayError_RouteNotFound value)? routeNotFound,
    TResult Function(LnUrlPayError_RouteTooExpensive value)? routeTooExpensive,
    TResult Function(LnUrlPayError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (serviceConnectivity != null) {
      return serviceConnectivity(this);
    }
    return orElse();
  }
}

abstract class LnUrlPayError_ServiceConnectivity extends LnUrlPayError {
  const factory LnUrlPayError_ServiceConnectivity({required final String err}) =
      _$LnUrlPayError_ServiceConnectivityImpl;
  const LnUrlPayError_ServiceConnectivity._() : super._();

  String get err;
  @JsonKey(ignore: true)
  _$$LnUrlPayError_ServiceConnectivityImplCopyWith<_$LnUrlPayError_ServiceConnectivityImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LnUrlWithdrawError {
  String get err => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidAmount,
    required TResult Function(String err) invalidInvoice,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) invoiceNoRoutingHints,
    required TResult Function(String err) serviceConnectivity,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidAmount,
    TResult? Function(String err)? invalidInvoice,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? invoiceNoRoutingHints,
    TResult? Function(String err)? serviceConnectivity,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidAmount,
    TResult Function(String err)? invalidInvoice,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? invoiceNoRoutingHints,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlWithdrawError_Generic value) generic,
    required TResult Function(LnUrlWithdrawError_InvalidAmount value) invalidAmount,
    required TResult Function(LnUrlWithdrawError_InvalidInvoice value) invalidInvoice,
    required TResult Function(LnUrlWithdrawError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlWithdrawError_InvoiceNoRoutingHints value) invoiceNoRoutingHints,
    required TResult Function(LnUrlWithdrawError_ServiceConnectivity value) serviceConnectivity,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlWithdrawError_Generic value)? generic,
    TResult? Function(LnUrlWithdrawError_InvalidAmount value)? invalidAmount,
    TResult? Function(LnUrlWithdrawError_InvalidInvoice value)? invalidInvoice,
    TResult? Function(LnUrlWithdrawError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlWithdrawError_InvoiceNoRoutingHints value)? invoiceNoRoutingHints,
    TResult? Function(LnUrlWithdrawError_ServiceConnectivity value)? serviceConnectivity,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlWithdrawError_Generic value)? generic,
    TResult Function(LnUrlWithdrawError_InvalidAmount value)? invalidAmount,
    TResult Function(LnUrlWithdrawError_InvalidInvoice value)? invalidInvoice,
    TResult Function(LnUrlWithdrawError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlWithdrawError_InvoiceNoRoutingHints value)? invoiceNoRoutingHints,
    TResult Function(LnUrlWithdrawError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $LnUrlWithdrawErrorCopyWith<LnUrlWithdrawError> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LnUrlWithdrawErrorCopyWith<$Res> {
  factory $LnUrlWithdrawErrorCopyWith(LnUrlWithdrawError value, $Res Function(LnUrlWithdrawError) then) =
      _$LnUrlWithdrawErrorCopyWithImpl<$Res, LnUrlWithdrawError>;
  @useResult
  $Res call({String err});
}

/// @nodoc
class _$LnUrlWithdrawErrorCopyWithImpl<$Res, $Val extends LnUrlWithdrawError>
    implements $LnUrlWithdrawErrorCopyWith<$Res> {
  _$LnUrlWithdrawErrorCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_value.copyWith(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$LnUrlWithdrawError_GenericImplCopyWith<$Res> implements $LnUrlWithdrawErrorCopyWith<$Res> {
  factory _$$LnUrlWithdrawError_GenericImplCopyWith(
          _$LnUrlWithdrawError_GenericImpl value, $Res Function(_$LnUrlWithdrawError_GenericImpl) then) =
      __$$LnUrlWithdrawError_GenericImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$LnUrlWithdrawError_GenericImplCopyWithImpl<$Res>
    extends _$LnUrlWithdrawErrorCopyWithImpl<$Res, _$LnUrlWithdrawError_GenericImpl>
    implements _$$LnUrlWithdrawError_GenericImplCopyWith<$Res> {
  __$$LnUrlWithdrawError_GenericImplCopyWithImpl(
      _$LnUrlWithdrawError_GenericImpl _value, $Res Function(_$LnUrlWithdrawError_GenericImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$LnUrlWithdrawError_GenericImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LnUrlWithdrawError_GenericImpl extends LnUrlWithdrawError_Generic {
  const _$LnUrlWithdrawError_GenericImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'LnUrlWithdrawError.generic(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlWithdrawError_GenericImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlWithdrawError_GenericImplCopyWith<_$LnUrlWithdrawError_GenericImpl> get copyWith =>
      __$$LnUrlWithdrawError_GenericImplCopyWithImpl<_$LnUrlWithdrawError_GenericImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidAmount,
    required TResult Function(String err) invalidInvoice,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) invoiceNoRoutingHints,
    required TResult Function(String err) serviceConnectivity,
  }) {
    return generic(err);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidAmount,
    TResult? Function(String err)? invalidInvoice,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? invoiceNoRoutingHints,
    TResult? Function(String err)? serviceConnectivity,
  }) {
    return generic?.call(err);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidAmount,
    TResult Function(String err)? invalidInvoice,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? invoiceNoRoutingHints,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (generic != null) {
      return generic(err);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlWithdrawError_Generic value) generic,
    required TResult Function(LnUrlWithdrawError_InvalidAmount value) invalidAmount,
    required TResult Function(LnUrlWithdrawError_InvalidInvoice value) invalidInvoice,
    required TResult Function(LnUrlWithdrawError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlWithdrawError_InvoiceNoRoutingHints value) invoiceNoRoutingHints,
    required TResult Function(LnUrlWithdrawError_ServiceConnectivity value) serviceConnectivity,
  }) {
    return generic(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlWithdrawError_Generic value)? generic,
    TResult? Function(LnUrlWithdrawError_InvalidAmount value)? invalidAmount,
    TResult? Function(LnUrlWithdrawError_InvalidInvoice value)? invalidInvoice,
    TResult? Function(LnUrlWithdrawError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlWithdrawError_InvoiceNoRoutingHints value)? invoiceNoRoutingHints,
    TResult? Function(LnUrlWithdrawError_ServiceConnectivity value)? serviceConnectivity,
  }) {
    return generic?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlWithdrawError_Generic value)? generic,
    TResult Function(LnUrlWithdrawError_InvalidAmount value)? invalidAmount,
    TResult Function(LnUrlWithdrawError_InvalidInvoice value)? invalidInvoice,
    TResult Function(LnUrlWithdrawError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlWithdrawError_InvoiceNoRoutingHints value)? invoiceNoRoutingHints,
    TResult Function(LnUrlWithdrawError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (generic != null) {
      return generic(this);
    }
    return orElse();
  }
}

abstract class LnUrlWithdrawError_Generic extends LnUrlWithdrawError {
  const factory LnUrlWithdrawError_Generic({required final String err}) = _$LnUrlWithdrawError_GenericImpl;
  const LnUrlWithdrawError_Generic._() : super._();

  @override
  String get err;
  @override
  @JsonKey(ignore: true)
  _$$LnUrlWithdrawError_GenericImplCopyWith<_$LnUrlWithdrawError_GenericImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlWithdrawError_InvalidAmountImplCopyWith<$Res>
    implements $LnUrlWithdrawErrorCopyWith<$Res> {
  factory _$$LnUrlWithdrawError_InvalidAmountImplCopyWith(_$LnUrlWithdrawError_InvalidAmountImpl value,
          $Res Function(_$LnUrlWithdrawError_InvalidAmountImpl) then) =
      __$$LnUrlWithdrawError_InvalidAmountImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$LnUrlWithdrawError_InvalidAmountImplCopyWithImpl<$Res>
    extends _$LnUrlWithdrawErrorCopyWithImpl<$Res, _$LnUrlWithdrawError_InvalidAmountImpl>
    implements _$$LnUrlWithdrawError_InvalidAmountImplCopyWith<$Res> {
  __$$LnUrlWithdrawError_InvalidAmountImplCopyWithImpl(_$LnUrlWithdrawError_InvalidAmountImpl _value,
      $Res Function(_$LnUrlWithdrawError_InvalidAmountImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$LnUrlWithdrawError_InvalidAmountImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LnUrlWithdrawError_InvalidAmountImpl extends LnUrlWithdrawError_InvalidAmount {
  const _$LnUrlWithdrawError_InvalidAmountImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'LnUrlWithdrawError.invalidAmount(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlWithdrawError_InvalidAmountImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlWithdrawError_InvalidAmountImplCopyWith<_$LnUrlWithdrawError_InvalidAmountImpl> get copyWith =>
      __$$LnUrlWithdrawError_InvalidAmountImplCopyWithImpl<_$LnUrlWithdrawError_InvalidAmountImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidAmount,
    required TResult Function(String err) invalidInvoice,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) invoiceNoRoutingHints,
    required TResult Function(String err) serviceConnectivity,
  }) {
    return invalidAmount(err);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidAmount,
    TResult? Function(String err)? invalidInvoice,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? invoiceNoRoutingHints,
    TResult? Function(String err)? serviceConnectivity,
  }) {
    return invalidAmount?.call(err);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidAmount,
    TResult Function(String err)? invalidInvoice,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? invoiceNoRoutingHints,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (invalidAmount != null) {
      return invalidAmount(err);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlWithdrawError_Generic value) generic,
    required TResult Function(LnUrlWithdrawError_InvalidAmount value) invalidAmount,
    required TResult Function(LnUrlWithdrawError_InvalidInvoice value) invalidInvoice,
    required TResult Function(LnUrlWithdrawError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlWithdrawError_InvoiceNoRoutingHints value) invoiceNoRoutingHints,
    required TResult Function(LnUrlWithdrawError_ServiceConnectivity value) serviceConnectivity,
  }) {
    return invalidAmount(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlWithdrawError_Generic value)? generic,
    TResult? Function(LnUrlWithdrawError_InvalidAmount value)? invalidAmount,
    TResult? Function(LnUrlWithdrawError_InvalidInvoice value)? invalidInvoice,
    TResult? Function(LnUrlWithdrawError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlWithdrawError_InvoiceNoRoutingHints value)? invoiceNoRoutingHints,
    TResult? Function(LnUrlWithdrawError_ServiceConnectivity value)? serviceConnectivity,
  }) {
    return invalidAmount?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlWithdrawError_Generic value)? generic,
    TResult Function(LnUrlWithdrawError_InvalidAmount value)? invalidAmount,
    TResult Function(LnUrlWithdrawError_InvalidInvoice value)? invalidInvoice,
    TResult Function(LnUrlWithdrawError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlWithdrawError_InvoiceNoRoutingHints value)? invoiceNoRoutingHints,
    TResult Function(LnUrlWithdrawError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (invalidAmount != null) {
      return invalidAmount(this);
    }
    return orElse();
  }
}

abstract class LnUrlWithdrawError_InvalidAmount extends LnUrlWithdrawError {
  const factory LnUrlWithdrawError_InvalidAmount({required final String err}) =
      _$LnUrlWithdrawError_InvalidAmountImpl;
  const LnUrlWithdrawError_InvalidAmount._() : super._();

  @override
  String get err;
  @override
  @JsonKey(ignore: true)
  _$$LnUrlWithdrawError_InvalidAmountImplCopyWith<_$LnUrlWithdrawError_InvalidAmountImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlWithdrawError_InvalidInvoiceImplCopyWith<$Res>
    implements $LnUrlWithdrawErrorCopyWith<$Res> {
  factory _$$LnUrlWithdrawError_InvalidInvoiceImplCopyWith(_$LnUrlWithdrawError_InvalidInvoiceImpl value,
          $Res Function(_$LnUrlWithdrawError_InvalidInvoiceImpl) then) =
      __$$LnUrlWithdrawError_InvalidInvoiceImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$LnUrlWithdrawError_InvalidInvoiceImplCopyWithImpl<$Res>
    extends _$LnUrlWithdrawErrorCopyWithImpl<$Res, _$LnUrlWithdrawError_InvalidInvoiceImpl>
    implements _$$LnUrlWithdrawError_InvalidInvoiceImplCopyWith<$Res> {
  __$$LnUrlWithdrawError_InvalidInvoiceImplCopyWithImpl(_$LnUrlWithdrawError_InvalidInvoiceImpl _value,
      $Res Function(_$LnUrlWithdrawError_InvalidInvoiceImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$LnUrlWithdrawError_InvalidInvoiceImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LnUrlWithdrawError_InvalidInvoiceImpl extends LnUrlWithdrawError_InvalidInvoice {
  const _$LnUrlWithdrawError_InvalidInvoiceImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'LnUrlWithdrawError.invalidInvoice(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlWithdrawError_InvalidInvoiceImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlWithdrawError_InvalidInvoiceImplCopyWith<_$LnUrlWithdrawError_InvalidInvoiceImpl> get copyWith =>
      __$$LnUrlWithdrawError_InvalidInvoiceImplCopyWithImpl<_$LnUrlWithdrawError_InvalidInvoiceImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidAmount,
    required TResult Function(String err) invalidInvoice,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) invoiceNoRoutingHints,
    required TResult Function(String err) serviceConnectivity,
  }) {
    return invalidInvoice(err);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidAmount,
    TResult? Function(String err)? invalidInvoice,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? invoiceNoRoutingHints,
    TResult? Function(String err)? serviceConnectivity,
  }) {
    return invalidInvoice?.call(err);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidAmount,
    TResult Function(String err)? invalidInvoice,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? invoiceNoRoutingHints,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (invalidInvoice != null) {
      return invalidInvoice(err);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlWithdrawError_Generic value) generic,
    required TResult Function(LnUrlWithdrawError_InvalidAmount value) invalidAmount,
    required TResult Function(LnUrlWithdrawError_InvalidInvoice value) invalidInvoice,
    required TResult Function(LnUrlWithdrawError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlWithdrawError_InvoiceNoRoutingHints value) invoiceNoRoutingHints,
    required TResult Function(LnUrlWithdrawError_ServiceConnectivity value) serviceConnectivity,
  }) {
    return invalidInvoice(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlWithdrawError_Generic value)? generic,
    TResult? Function(LnUrlWithdrawError_InvalidAmount value)? invalidAmount,
    TResult? Function(LnUrlWithdrawError_InvalidInvoice value)? invalidInvoice,
    TResult? Function(LnUrlWithdrawError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlWithdrawError_InvoiceNoRoutingHints value)? invoiceNoRoutingHints,
    TResult? Function(LnUrlWithdrawError_ServiceConnectivity value)? serviceConnectivity,
  }) {
    return invalidInvoice?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlWithdrawError_Generic value)? generic,
    TResult Function(LnUrlWithdrawError_InvalidAmount value)? invalidAmount,
    TResult Function(LnUrlWithdrawError_InvalidInvoice value)? invalidInvoice,
    TResult Function(LnUrlWithdrawError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlWithdrawError_InvoiceNoRoutingHints value)? invoiceNoRoutingHints,
    TResult Function(LnUrlWithdrawError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (invalidInvoice != null) {
      return invalidInvoice(this);
    }
    return orElse();
  }
}

abstract class LnUrlWithdrawError_InvalidInvoice extends LnUrlWithdrawError {
  const factory LnUrlWithdrawError_InvalidInvoice({required final String err}) =
      _$LnUrlWithdrawError_InvalidInvoiceImpl;
  const LnUrlWithdrawError_InvalidInvoice._() : super._();

  @override
  String get err;
  @override
  @JsonKey(ignore: true)
  _$$LnUrlWithdrawError_InvalidInvoiceImplCopyWith<_$LnUrlWithdrawError_InvalidInvoiceImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlWithdrawError_InvalidUriImplCopyWith<$Res>
    implements $LnUrlWithdrawErrorCopyWith<$Res> {
  factory _$$LnUrlWithdrawError_InvalidUriImplCopyWith(_$LnUrlWithdrawError_InvalidUriImpl value,
          $Res Function(_$LnUrlWithdrawError_InvalidUriImpl) then) =
      __$$LnUrlWithdrawError_InvalidUriImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$LnUrlWithdrawError_InvalidUriImplCopyWithImpl<$Res>
    extends _$LnUrlWithdrawErrorCopyWithImpl<$Res, _$LnUrlWithdrawError_InvalidUriImpl>
    implements _$$LnUrlWithdrawError_InvalidUriImplCopyWith<$Res> {
  __$$LnUrlWithdrawError_InvalidUriImplCopyWithImpl(
      _$LnUrlWithdrawError_InvalidUriImpl _value, $Res Function(_$LnUrlWithdrawError_InvalidUriImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$LnUrlWithdrawError_InvalidUriImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LnUrlWithdrawError_InvalidUriImpl extends LnUrlWithdrawError_InvalidUri {
  const _$LnUrlWithdrawError_InvalidUriImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'LnUrlWithdrawError.invalidUri(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlWithdrawError_InvalidUriImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlWithdrawError_InvalidUriImplCopyWith<_$LnUrlWithdrawError_InvalidUriImpl> get copyWith =>
      __$$LnUrlWithdrawError_InvalidUriImplCopyWithImpl<_$LnUrlWithdrawError_InvalidUriImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidAmount,
    required TResult Function(String err) invalidInvoice,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) invoiceNoRoutingHints,
    required TResult Function(String err) serviceConnectivity,
  }) {
    return invalidUri(err);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidAmount,
    TResult? Function(String err)? invalidInvoice,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? invoiceNoRoutingHints,
    TResult? Function(String err)? serviceConnectivity,
  }) {
    return invalidUri?.call(err);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidAmount,
    TResult Function(String err)? invalidInvoice,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? invoiceNoRoutingHints,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (invalidUri != null) {
      return invalidUri(err);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlWithdrawError_Generic value) generic,
    required TResult Function(LnUrlWithdrawError_InvalidAmount value) invalidAmount,
    required TResult Function(LnUrlWithdrawError_InvalidInvoice value) invalidInvoice,
    required TResult Function(LnUrlWithdrawError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlWithdrawError_InvoiceNoRoutingHints value) invoiceNoRoutingHints,
    required TResult Function(LnUrlWithdrawError_ServiceConnectivity value) serviceConnectivity,
  }) {
    return invalidUri(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlWithdrawError_Generic value)? generic,
    TResult? Function(LnUrlWithdrawError_InvalidAmount value)? invalidAmount,
    TResult? Function(LnUrlWithdrawError_InvalidInvoice value)? invalidInvoice,
    TResult? Function(LnUrlWithdrawError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlWithdrawError_InvoiceNoRoutingHints value)? invoiceNoRoutingHints,
    TResult? Function(LnUrlWithdrawError_ServiceConnectivity value)? serviceConnectivity,
  }) {
    return invalidUri?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlWithdrawError_Generic value)? generic,
    TResult Function(LnUrlWithdrawError_InvalidAmount value)? invalidAmount,
    TResult Function(LnUrlWithdrawError_InvalidInvoice value)? invalidInvoice,
    TResult Function(LnUrlWithdrawError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlWithdrawError_InvoiceNoRoutingHints value)? invoiceNoRoutingHints,
    TResult Function(LnUrlWithdrawError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (invalidUri != null) {
      return invalidUri(this);
    }
    return orElse();
  }
}

abstract class LnUrlWithdrawError_InvalidUri extends LnUrlWithdrawError {
  const factory LnUrlWithdrawError_InvalidUri({required final String err}) =
      _$LnUrlWithdrawError_InvalidUriImpl;
  const LnUrlWithdrawError_InvalidUri._() : super._();

  @override
  String get err;
  @override
  @JsonKey(ignore: true)
  _$$LnUrlWithdrawError_InvalidUriImplCopyWith<_$LnUrlWithdrawError_InvalidUriImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlWithdrawError_InvoiceNoRoutingHintsImplCopyWith<$Res>
    implements $LnUrlWithdrawErrorCopyWith<$Res> {
  factory _$$LnUrlWithdrawError_InvoiceNoRoutingHintsImplCopyWith(
          _$LnUrlWithdrawError_InvoiceNoRoutingHintsImpl value,
          $Res Function(_$LnUrlWithdrawError_InvoiceNoRoutingHintsImpl) then) =
      __$$LnUrlWithdrawError_InvoiceNoRoutingHintsImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$LnUrlWithdrawError_InvoiceNoRoutingHintsImplCopyWithImpl<$Res>
    extends _$LnUrlWithdrawErrorCopyWithImpl<$Res, _$LnUrlWithdrawError_InvoiceNoRoutingHintsImpl>
    implements _$$LnUrlWithdrawError_InvoiceNoRoutingHintsImplCopyWith<$Res> {
  __$$LnUrlWithdrawError_InvoiceNoRoutingHintsImplCopyWithImpl(
      _$LnUrlWithdrawError_InvoiceNoRoutingHintsImpl _value,
      $Res Function(_$LnUrlWithdrawError_InvoiceNoRoutingHintsImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$LnUrlWithdrawError_InvoiceNoRoutingHintsImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LnUrlWithdrawError_InvoiceNoRoutingHintsImpl extends LnUrlWithdrawError_InvoiceNoRoutingHints {
  const _$LnUrlWithdrawError_InvoiceNoRoutingHintsImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'LnUrlWithdrawError.invoiceNoRoutingHints(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlWithdrawError_InvoiceNoRoutingHintsImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlWithdrawError_InvoiceNoRoutingHintsImplCopyWith<_$LnUrlWithdrawError_InvoiceNoRoutingHintsImpl>
      get copyWith => __$$LnUrlWithdrawError_InvoiceNoRoutingHintsImplCopyWithImpl<
          _$LnUrlWithdrawError_InvoiceNoRoutingHintsImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidAmount,
    required TResult Function(String err) invalidInvoice,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) invoiceNoRoutingHints,
    required TResult Function(String err) serviceConnectivity,
  }) {
    return invoiceNoRoutingHints(err);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidAmount,
    TResult? Function(String err)? invalidInvoice,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? invoiceNoRoutingHints,
    TResult? Function(String err)? serviceConnectivity,
  }) {
    return invoiceNoRoutingHints?.call(err);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidAmount,
    TResult Function(String err)? invalidInvoice,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? invoiceNoRoutingHints,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (invoiceNoRoutingHints != null) {
      return invoiceNoRoutingHints(err);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlWithdrawError_Generic value) generic,
    required TResult Function(LnUrlWithdrawError_InvalidAmount value) invalidAmount,
    required TResult Function(LnUrlWithdrawError_InvalidInvoice value) invalidInvoice,
    required TResult Function(LnUrlWithdrawError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlWithdrawError_InvoiceNoRoutingHints value) invoiceNoRoutingHints,
    required TResult Function(LnUrlWithdrawError_ServiceConnectivity value) serviceConnectivity,
  }) {
    return invoiceNoRoutingHints(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlWithdrawError_Generic value)? generic,
    TResult? Function(LnUrlWithdrawError_InvalidAmount value)? invalidAmount,
    TResult? Function(LnUrlWithdrawError_InvalidInvoice value)? invalidInvoice,
    TResult? Function(LnUrlWithdrawError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlWithdrawError_InvoiceNoRoutingHints value)? invoiceNoRoutingHints,
    TResult? Function(LnUrlWithdrawError_ServiceConnectivity value)? serviceConnectivity,
  }) {
    return invoiceNoRoutingHints?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlWithdrawError_Generic value)? generic,
    TResult Function(LnUrlWithdrawError_InvalidAmount value)? invalidAmount,
    TResult Function(LnUrlWithdrawError_InvalidInvoice value)? invalidInvoice,
    TResult Function(LnUrlWithdrawError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlWithdrawError_InvoiceNoRoutingHints value)? invoiceNoRoutingHints,
    TResult Function(LnUrlWithdrawError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (invoiceNoRoutingHints != null) {
      return invoiceNoRoutingHints(this);
    }
    return orElse();
  }
}

abstract class LnUrlWithdrawError_InvoiceNoRoutingHints extends LnUrlWithdrawError {
  const factory LnUrlWithdrawError_InvoiceNoRoutingHints({required final String err}) =
      _$LnUrlWithdrawError_InvoiceNoRoutingHintsImpl;
  const LnUrlWithdrawError_InvoiceNoRoutingHints._() : super._();

  @override
  String get err;
  @override
  @JsonKey(ignore: true)
  _$$LnUrlWithdrawError_InvoiceNoRoutingHintsImplCopyWith<_$LnUrlWithdrawError_InvoiceNoRoutingHintsImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlWithdrawError_ServiceConnectivityImplCopyWith<$Res>
    implements $LnUrlWithdrawErrorCopyWith<$Res> {
  factory _$$LnUrlWithdrawError_ServiceConnectivityImplCopyWith(
          _$LnUrlWithdrawError_ServiceConnectivityImpl value,
          $Res Function(_$LnUrlWithdrawError_ServiceConnectivityImpl) then) =
      __$$LnUrlWithdrawError_ServiceConnectivityImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$LnUrlWithdrawError_ServiceConnectivityImplCopyWithImpl<$Res>
    extends _$LnUrlWithdrawErrorCopyWithImpl<$Res, _$LnUrlWithdrawError_ServiceConnectivityImpl>
    implements _$$LnUrlWithdrawError_ServiceConnectivityImplCopyWith<$Res> {
  __$$LnUrlWithdrawError_ServiceConnectivityImplCopyWithImpl(
      _$LnUrlWithdrawError_ServiceConnectivityImpl _value,
      $Res Function(_$LnUrlWithdrawError_ServiceConnectivityImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$LnUrlWithdrawError_ServiceConnectivityImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LnUrlWithdrawError_ServiceConnectivityImpl extends LnUrlWithdrawError_ServiceConnectivity {
  const _$LnUrlWithdrawError_ServiceConnectivityImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'LnUrlWithdrawError.serviceConnectivity(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlWithdrawError_ServiceConnectivityImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlWithdrawError_ServiceConnectivityImplCopyWith<_$LnUrlWithdrawError_ServiceConnectivityImpl>
      get copyWith => __$$LnUrlWithdrawError_ServiceConnectivityImplCopyWithImpl<
          _$LnUrlWithdrawError_ServiceConnectivityImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String err) generic,
    required TResult Function(String err) invalidAmount,
    required TResult Function(String err) invalidInvoice,
    required TResult Function(String err) invalidUri,
    required TResult Function(String err) invoiceNoRoutingHints,
    required TResult Function(String err) serviceConnectivity,
  }) {
    return serviceConnectivity(err);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String err)? generic,
    TResult? Function(String err)? invalidAmount,
    TResult? Function(String err)? invalidInvoice,
    TResult? Function(String err)? invalidUri,
    TResult? Function(String err)? invoiceNoRoutingHints,
    TResult? Function(String err)? serviceConnectivity,
  }) {
    return serviceConnectivity?.call(err);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String err)? generic,
    TResult Function(String err)? invalidAmount,
    TResult Function(String err)? invalidInvoice,
    TResult Function(String err)? invalidUri,
    TResult Function(String err)? invoiceNoRoutingHints,
    TResult Function(String err)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (serviceConnectivity != null) {
      return serviceConnectivity(err);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlWithdrawError_Generic value) generic,
    required TResult Function(LnUrlWithdrawError_InvalidAmount value) invalidAmount,
    required TResult Function(LnUrlWithdrawError_InvalidInvoice value) invalidInvoice,
    required TResult Function(LnUrlWithdrawError_InvalidUri value) invalidUri,
    required TResult Function(LnUrlWithdrawError_InvoiceNoRoutingHints value) invoiceNoRoutingHints,
    required TResult Function(LnUrlWithdrawError_ServiceConnectivity value) serviceConnectivity,
  }) {
    return serviceConnectivity(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlWithdrawError_Generic value)? generic,
    TResult? Function(LnUrlWithdrawError_InvalidAmount value)? invalidAmount,
    TResult? Function(LnUrlWithdrawError_InvalidInvoice value)? invalidInvoice,
    TResult? Function(LnUrlWithdrawError_InvalidUri value)? invalidUri,
    TResult? Function(LnUrlWithdrawError_InvoiceNoRoutingHints value)? invoiceNoRoutingHints,
    TResult? Function(LnUrlWithdrawError_ServiceConnectivity value)? serviceConnectivity,
  }) {
    return serviceConnectivity?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlWithdrawError_Generic value)? generic,
    TResult Function(LnUrlWithdrawError_InvalidAmount value)? invalidAmount,
    TResult Function(LnUrlWithdrawError_InvalidInvoice value)? invalidInvoice,
    TResult Function(LnUrlWithdrawError_InvalidUri value)? invalidUri,
    TResult Function(LnUrlWithdrawError_InvoiceNoRoutingHints value)? invoiceNoRoutingHints,
    TResult Function(LnUrlWithdrawError_ServiceConnectivity value)? serviceConnectivity,
    required TResult orElse(),
  }) {
    if (serviceConnectivity != null) {
      return serviceConnectivity(this);
    }
    return orElse();
  }
}

abstract class LnUrlWithdrawError_ServiceConnectivity extends LnUrlWithdrawError {
  const factory LnUrlWithdrawError_ServiceConnectivity({required final String err}) =
      _$LnUrlWithdrawError_ServiceConnectivityImpl;
  const LnUrlWithdrawError_ServiceConnectivity._() : super._();

  @override
  String get err;
  @override
  @JsonKey(ignore: true)
  _$$LnUrlWithdrawError_ServiceConnectivityImplCopyWith<_$LnUrlWithdrawError_ServiceConnectivityImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LnUrlWithdrawResult {
  Object get data => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(LnUrlWithdrawSuccessData data) ok,
    required TResult Function(LnUrlErrorData data) errorStatus,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(LnUrlWithdrawSuccessData data)? ok,
    TResult? Function(LnUrlErrorData data)? errorStatus,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(LnUrlWithdrawSuccessData data)? ok,
    TResult Function(LnUrlErrorData data)? errorStatus,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlWithdrawResult_Ok value) ok,
    required TResult Function(LnUrlWithdrawResult_ErrorStatus value) errorStatus,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlWithdrawResult_Ok value)? ok,
    TResult? Function(LnUrlWithdrawResult_ErrorStatus value)? errorStatus,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlWithdrawResult_Ok value)? ok,
    TResult Function(LnUrlWithdrawResult_ErrorStatus value)? errorStatus,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LnUrlWithdrawResultCopyWith<$Res> {
  factory $LnUrlWithdrawResultCopyWith(LnUrlWithdrawResult value, $Res Function(LnUrlWithdrawResult) then) =
      _$LnUrlWithdrawResultCopyWithImpl<$Res, LnUrlWithdrawResult>;
}

/// @nodoc
class _$LnUrlWithdrawResultCopyWithImpl<$Res, $Val extends LnUrlWithdrawResult>
    implements $LnUrlWithdrawResultCopyWith<$Res> {
  _$LnUrlWithdrawResultCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$LnUrlWithdrawResult_OkImplCopyWith<$Res> {
  factory _$$LnUrlWithdrawResult_OkImplCopyWith(
          _$LnUrlWithdrawResult_OkImpl value, $Res Function(_$LnUrlWithdrawResult_OkImpl) then) =
      __$$LnUrlWithdrawResult_OkImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlWithdrawSuccessData data});
}

/// @nodoc
class __$$LnUrlWithdrawResult_OkImplCopyWithImpl<$Res>
    extends _$LnUrlWithdrawResultCopyWithImpl<$Res, _$LnUrlWithdrawResult_OkImpl>
    implements _$$LnUrlWithdrawResult_OkImplCopyWith<$Res> {
  __$$LnUrlWithdrawResult_OkImplCopyWithImpl(
      _$LnUrlWithdrawResult_OkImpl _value, $Res Function(_$LnUrlWithdrawResult_OkImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$LnUrlWithdrawResult_OkImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlWithdrawSuccessData,
    ));
  }
}

/// @nodoc

class _$LnUrlWithdrawResult_OkImpl extends LnUrlWithdrawResult_Ok {
  const _$LnUrlWithdrawResult_OkImpl({required this.data}) : super._();

  @override
  final LnUrlWithdrawSuccessData data;

  @override
  String toString() {
    return 'LnUrlWithdrawResult.ok(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlWithdrawResult_OkImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlWithdrawResult_OkImplCopyWith<_$LnUrlWithdrawResult_OkImpl> get copyWith =>
      __$$LnUrlWithdrawResult_OkImplCopyWithImpl<_$LnUrlWithdrawResult_OkImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(LnUrlWithdrawSuccessData data) ok,
    required TResult Function(LnUrlErrorData data) errorStatus,
  }) {
    return ok(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(LnUrlWithdrawSuccessData data)? ok,
    TResult? Function(LnUrlErrorData data)? errorStatus,
  }) {
    return ok?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(LnUrlWithdrawSuccessData data)? ok,
    TResult Function(LnUrlErrorData data)? errorStatus,
    required TResult orElse(),
  }) {
    if (ok != null) {
      return ok(data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlWithdrawResult_Ok value) ok,
    required TResult Function(LnUrlWithdrawResult_ErrorStatus value) errorStatus,
  }) {
    return ok(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlWithdrawResult_Ok value)? ok,
    TResult? Function(LnUrlWithdrawResult_ErrorStatus value)? errorStatus,
  }) {
    return ok?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlWithdrawResult_Ok value)? ok,
    TResult Function(LnUrlWithdrawResult_ErrorStatus value)? errorStatus,
    required TResult orElse(),
  }) {
    if (ok != null) {
      return ok(this);
    }
    return orElse();
  }
}

abstract class LnUrlWithdrawResult_Ok extends LnUrlWithdrawResult {
  const factory LnUrlWithdrawResult_Ok({required final LnUrlWithdrawSuccessData data}) =
      _$LnUrlWithdrawResult_OkImpl;
  const LnUrlWithdrawResult_Ok._() : super._();

  @override
  LnUrlWithdrawSuccessData get data;
  @JsonKey(ignore: true)
  _$$LnUrlWithdrawResult_OkImplCopyWith<_$LnUrlWithdrawResult_OkImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlWithdrawResult_ErrorStatusImplCopyWith<$Res> {
  factory _$$LnUrlWithdrawResult_ErrorStatusImplCopyWith(_$LnUrlWithdrawResult_ErrorStatusImpl value,
          $Res Function(_$LnUrlWithdrawResult_ErrorStatusImpl) then) =
      __$$LnUrlWithdrawResult_ErrorStatusImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlErrorData data});
}

/// @nodoc
class __$$LnUrlWithdrawResult_ErrorStatusImplCopyWithImpl<$Res>
    extends _$LnUrlWithdrawResultCopyWithImpl<$Res, _$LnUrlWithdrawResult_ErrorStatusImpl>
    implements _$$LnUrlWithdrawResult_ErrorStatusImplCopyWith<$Res> {
  __$$LnUrlWithdrawResult_ErrorStatusImplCopyWithImpl(_$LnUrlWithdrawResult_ErrorStatusImpl _value,
      $Res Function(_$LnUrlWithdrawResult_ErrorStatusImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$LnUrlWithdrawResult_ErrorStatusImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlErrorData,
    ));
  }
}

/// @nodoc

class _$LnUrlWithdrawResult_ErrorStatusImpl extends LnUrlWithdrawResult_ErrorStatus {
  const _$LnUrlWithdrawResult_ErrorStatusImpl({required this.data}) : super._();

  @override
  final LnUrlErrorData data;

  @override
  String toString() {
    return 'LnUrlWithdrawResult.errorStatus(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlWithdrawResult_ErrorStatusImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlWithdrawResult_ErrorStatusImplCopyWith<_$LnUrlWithdrawResult_ErrorStatusImpl> get copyWith =>
      __$$LnUrlWithdrawResult_ErrorStatusImplCopyWithImpl<_$LnUrlWithdrawResult_ErrorStatusImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(LnUrlWithdrawSuccessData data) ok,
    required TResult Function(LnUrlErrorData data) errorStatus,
  }) {
    return errorStatus(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(LnUrlWithdrawSuccessData data)? ok,
    TResult? Function(LnUrlErrorData data)? errorStatus,
  }) {
    return errorStatus?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(LnUrlWithdrawSuccessData data)? ok,
    TResult Function(LnUrlErrorData data)? errorStatus,
    required TResult orElse(),
  }) {
    if (errorStatus != null) {
      return errorStatus(data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlWithdrawResult_Ok value) ok,
    required TResult Function(LnUrlWithdrawResult_ErrorStatus value) errorStatus,
  }) {
    return errorStatus(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlWithdrawResult_Ok value)? ok,
    TResult? Function(LnUrlWithdrawResult_ErrorStatus value)? errorStatus,
  }) {
    return errorStatus?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlWithdrawResult_Ok value)? ok,
    TResult Function(LnUrlWithdrawResult_ErrorStatus value)? errorStatus,
    required TResult orElse(),
  }) {
    if (errorStatus != null) {
      return errorStatus(this);
    }
    return orElse();
  }
}

abstract class LnUrlWithdrawResult_ErrorStatus extends LnUrlWithdrawResult {
  const factory LnUrlWithdrawResult_ErrorStatus({required final LnUrlErrorData data}) =
      _$LnUrlWithdrawResult_ErrorStatusImpl;
  const LnUrlWithdrawResult_ErrorStatus._() : super._();

  @override
  LnUrlErrorData get data;
  @JsonKey(ignore: true)
  _$$LnUrlWithdrawResult_ErrorStatusImplCopyWith<_$LnUrlWithdrawResult_ErrorStatusImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
