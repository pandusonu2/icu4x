// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// An ICU4X data provider, capable of loading ICU4X data keys from some source.
///
/// See the [Rust documentation for `icu_provider`](https://docs.rs/icu_provider/latest/icu_provider/index.html) for more information.
final class DataProvider implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  final core.List<Object> _edge_self;

  // Internal constructor from FFI.
  // isOwned is whether this is owned (has finalizer) or not
  // This also takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  DataProvider._(this._underlying, bool isOwned, this._edge_self) {
    if (isOwned) {
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XDataProvider_destroy));

  /// Constructs an [`DataProvider`] that uses compiled data.
  ///
  /// Requires the `compiled_data` feature.
  ///
  /// This provider cannot be modified or combined with other providers, so `enable_fallback`,
  /// `enabled_fallback_with`, `fork_by_locale`, and `fork_by_key` will return `Err`s.
  factory DataProvider.compiled() {
    final result = _ICU4XDataProvider_create_compiled();
    return DataProvider._(result, true, []);
  }

  /// Constructs a `BlobDataProvider` and returns it as an [`DataProvider`].
  ///
  /// See the [Rust documentation for `BlobDataProvider`](https://docs.rs/icu_provider_blob/latest/icu_provider_blob/struct.BlobDataProvider.html) for more information.
  ///
  /// Throws [Error] on failure.
  factory DataProvider.fromByteSlice(ByteBuffer blob) {
    final temp = ffi2.Arena();
    final blobView = blob;
    final result = _ICU4XDataProvider_create_from_byte_slice(blobView.pointer(temp), blobView.length);
    temp.releaseAll();
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return DataProvider._(result.union.ok, true, []);
  }

  /// Constructs an empty [`DataProvider`].
  ///
  /// See the [Rust documentation for `EmptyDataProvider`](https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/empty/struct.EmptyDataProvider.html) for more information.
  factory DataProvider.empty() {
    final result = _ICU4XDataProvider_create_empty();
    return DataProvider._(result, true, []);
  }

  /// Creates a provider that tries the current provider and then, if the current provider
  /// doesn't support the data key, another provider `other`.
  ///
  /// This takes ownership of the `other` provider, leaving an empty provider in its place.
  ///
  /// The providers must be the same type (Any or Buffer). This condition is satisfied if
  /// both providers originate from the same constructor, such as `create_from_byte_slice`
  /// or `create_fs`. If the condition is not upheld, a runtime error occurs.
  ///
  /// See the [Rust documentation for `ForkByKeyProvider`](https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fork/type.ForkByKeyProvider.html) for more information.
  ///
  /// Throws [Error] on failure.
  void forkByKey(DataProvider other) {
    final result = _ICU4XDataProvider_fork_by_key(_underlying, other._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
  }

  /// Same as `fork_by_key` but forks by locale instead of key.
  ///
  /// See the [Rust documentation for `MissingLocalePredicate`](https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fork/predicates/struct.MissingLocalePredicate.html) for more information.
  ///
  /// Throws [Error] on failure.
  void forkByLocale(DataProvider other) {
    final result = _ICU4XDataProvider_fork_by_locale(_underlying, other._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
  }

  /// Enables locale fallbacking for data requests made to this provider.
  ///
  /// Note that the test provider (from `create_test`) already has fallbacking enabled.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fallback/struct.LocaleFallbackProvider.html#method.try_new) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fallback/struct.LocaleFallbackProvider.html)
  ///
  /// Throws [Error] on failure.
  void enableLocaleFallback() {
    final result = _ICU4XDataProvider_enable_locale_fallback(_underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
  }

  /// See the [Rust documentation for `new_with_fallbacker`](https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fallback/struct.LocaleFallbackProvider.html#method.new_with_fallbacker) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fallback/struct.LocaleFallbackProvider.html)
  ///
  /// Throws [Error] on failure.
  void enableLocaleFallbackWith(LocaleFallbacker fallbacker) {
    final result = _ICU4XDataProvider_enable_locale_fallback_with(_underlying, fallbacker._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XDataProvider_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XDataProvider_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true, symbol: 'ICU4XDataProvider_create_compiled')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _ICU4XDataProvider_create_compiled();

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Uint8>, ffi.Size)>(isLeaf: true, symbol: 'ICU4XDataProvider_create_from_byte_slice')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XDataProvider_create_from_byte_slice(ffi.Pointer<ffi.Uint8> blobData, int blobLength);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true, symbol: 'ICU4XDataProvider_create_empty')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _ICU4XDataProvider_create_empty();

@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XDataProvider_fork_by_key')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _ICU4XDataProvider_fork_by_key(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> other);

@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XDataProvider_fork_by_locale')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _ICU4XDataProvider_fork_by_locale(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> other);

@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XDataProvider_enable_locale_fallback')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _ICU4XDataProvider_enable_locale_fallback(ffi.Pointer<ffi.Opaque> self);

@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XDataProvider_enable_locale_fallback_with')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _ICU4XDataProvider_enable_locale_fallback_with(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> fallbacker);
