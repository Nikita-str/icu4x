// generated by diplomat-tool

part of 'lib.g.dart';

/// An ICU4X TimeFormatter object capable of formatting an [`Time`] type (and others) as a string
///
/// See the [Rust documentation for `datetime`](https://docs.rs/icu/latest/icu/datetime/index.html) for more information.
final class TimeFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  TimeFormatter._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_icu4x_TimeFormatter_destroy_mv1));

  /// Creates a new [`TimeFormatter`] from locale data.
  ///
  /// Throws [DateTimeFormatterLoadError] on failure.
  factory TimeFormatter.withLength(DataProvider provider, Locale locale, DateTimeLength length) {
    final result = _icu4x_TimeFormatter_create_with_length_mv1(provider._ffi, locale._ffi, length.index);
    if (!result.isOk) {
      throw DateTimeFormatterLoadError.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return TimeFormatter._fromFfi(result.union.ok, []);
  }

  /// Formats a [`Time`] to a string.
  String formatTime(Time value) {
    final write = _Write();
    _icu4x_TimeFormatter_format_time_mv1(_ffi, value._ffi, write._ffi);
    return write.finalize();
  }

  /// Formats a [`DateTime`] to a string.
  String formatDatetime(DateTime value) {
    final write = _Write();
    _icu4x_TimeFormatter_format_datetime_mv1(_ffi, value._ffi, write._ffi);
    return write.finalize();
  }

  /// Formats a [`IsoDateTime`] to a string.
  String formatIsoDatetime(IsoDateTime value) {
    final write = _Write();
    _icu4x_TimeFormatter_format_iso_datetime_mv1(_ffi, value._ffi, write._ffi);
    return write.finalize();
  }
}

@meta.RecordUse()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'icu4x_TimeFormatter_destroy_mv1')
// ignore: non_constant_identifier_names
external void _icu4x_TimeFormatter_destroy_mv1(ffi.Pointer<ffi.Void> self);

@meta.RecordUse()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Int32)>(isLeaf: true, symbol: 'icu4x_TimeFormatter_create_with_length_mv1')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _icu4x_TimeFormatter_create_with_length_mv1(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> locale, int length);

@meta.RecordUse()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_TimeFormatter_format_time_mv1')
// ignore: non_constant_identifier_names
external void _icu4x_TimeFormatter_format_time_mv1(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> value, ffi.Pointer<ffi.Opaque> write);

@meta.RecordUse()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_TimeFormatter_format_datetime_mv1')
// ignore: non_constant_identifier_names
external void _icu4x_TimeFormatter_format_datetime_mv1(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> value, ffi.Pointer<ffi.Opaque> write);

@meta.RecordUse()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_TimeFormatter_format_iso_datetime_mv1')
// ignore: non_constant_identifier_names
external void _icu4x_TimeFormatter_format_iso_datetime_mv1(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> value, ffi.Pointer<ffi.Opaque> write);
