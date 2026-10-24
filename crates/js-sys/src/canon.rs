//! Canonical JS handle types for Rust-canonical element types.

use alloc::string::String;
use wasm_bindgen::convert::{Upcast, UpcastFrom};
use wasm_bindgen::JsValue;

use crate::*;

/// The JS handle type that stands for a type in a generic position.
///
/// An `experimental_generic_mono` import's type parameter can be a Rust
/// element type, so one call yields `Promise<String>` where a non-mono API
/// yields `Promise<JsString>`. Both wrap the same JS promise, and [`Upcast`]
/// already relabels one to the other at zero cost, but only once the caller
/// has spelled out the target type. `JsCanon` names that target:
/// `<Promise<String> as JsCanon>::Canon` is `Promise<JsString>`, so
/// `promise.to_js_canon()` infers it.
///
/// Rust scalars map to their JS handle: `String`, `&str` and `char` to
/// [`JsString`], numbers to [`Number`], `bool` to [`Boolean`] and `()` to
/// [`Undefined`]. Every JS handle type maps to itself. Containers map
/// structurally through their elements: `Promise<T>` to `Promise<T::Canon>`,
/// `Array<T>` to `Array<T::Canon>`, `Map<K, V>` to `Map<K::Canon, V::Canon>`,
/// and the same for `Option<T>`, `Nullable<T>`, `JsOption<T>` and
/// `JsNullable<T>`, so nested types canonicalise all the way down.
///
/// Canonicalisation is a relabelling, so `Canon` is always an [`UpcastFrom`]
/// of `Self`. A handle container converts through `upcast_into` at zero
/// cost, a scalar converts by value (`JsString::from`), and `Option` and
/// `Nullable` map their payload.
///
/// For a JS handle type, `Canon` agrees with the identity `JsCanon` of its
/// [`IntoJsGeneric`] impl. `JsCanon` additionally covers Rust element types,
/// which have no `IntoJsGeneric` impl: `String` and the containers are
/// foreign to user crates, and `JsString` to `wasm-bindgen`, so the mapping
/// has to be owned here.
///
/// ```ignore
/// use js_sys::{JsCanon, JsString, Promise};
///
/// fn takes_js(promise: Promise<JsString>) { /* ... */ }
///
/// let promise: Promise<String> = make_string_promise();
/// takes_js(promise.to_js_canon());
/// ```
pub trait JsCanon: Sized {
    /// The canonical JS handle type for `Self`.
    type Canon: UpcastFrom<Self>;

    /// Convert `self` to its canonical JS handle type.
    fn to_js_canon(self) -> Self::Canon;
}

macro_rules! scalar {
    ($($t:ty => $canon:ident),* $(,)?) => ($(
        impl JsCanon for $t {
            type Canon = $canon;
            #[inline]
            fn to_js_canon(self) -> $canon {
                $canon::from(self)
            }
        }
    )*)
}

scalar! {
    String => JsString,
    &str => JsString,
    char => JsString,
    i8 => Number,
    u8 => Number,
    i16 => Number,
    u16 => Number,
    i32 => Number,
    u32 => Number,
    f32 => Number,
    f64 => Number,
    bool => Boolean,
}

impl JsCanon for () {
    type Canon = Undefined;
    #[inline]
    fn to_js_canon(self) -> Undefined {
        Undefined::UNDEFINED
    }
}

impl<T: JsCanon> JsCanon for Option<T> {
    type Canon = Option<T::Canon>;
    #[inline]
    fn to_js_canon(self) -> Option<T::Canon> {
        self.map(T::to_js_canon)
    }
}

impl<T: JsCanon> JsCanon for Nullable<T> {
    type Canon = Nullable<T::Canon>;
    #[inline]
    fn to_js_canon(self) -> Nullable<T::Canon> {
        self.map(T::to_js_canon)
    }
}

macro_rules! identity {
    ($($t:ty),* $(,)?) => ($(
        impl JsCanon for $t {
            type Canon = Self;
            #[inline]
            fn to_js_canon(self) -> Self {
                self
            }
        }
    )*)
}

// Handle containers relabel their element types at zero cost.
macro_rules! structural {
    ($($t:ident<$($p:ident),+>),* $(,)?) => ($(
        impl<$($p: JsCanon),+> JsCanon for $t<$($p),+> {
            type Canon = $t<$($p::Canon),+>;
            #[inline]
            fn to_js_canon(self) -> $t<$($p::Canon),+> {
                self.upcast_into()
            }
        }
    )*)
}

structural! {
    Array<T>,
    AsyncGenerator<T>,
    AsyncIterator<T>,
    Generator<T>,
    Iterator<T>,
    IteratorNext<T>,
    JsNullable<T>,
    JsOption<T>,
    Map<K, V>,
    Object<T>,
    Promise<T>,
    PromiseState<T>,
    PromiseWithResolvers<T>,
    PropertyDescriptor<T>,
    Set<T>,
    WeakMap<K, V>,
    WeakRef<T>,
    WeakSet<T>,
}

// A function signature's argument and return types are its ABI types, so
// `Function<T>` and `ArrayTuple<T>` keep their parameter as is.
impl<T: JsFunction + UpcastFrom<T>> JsCanon for Function<T> {
    type Canon = Self;
    #[inline]
    fn to_js_canon(self) -> Self {
        self
    }
}

impl<T: JsTuple + UpcastFrom<T>> JsCanon for ArrayTuple<T> {
    type Canon = Self;
    #[inline]
    fn to_js_canon(self) -> Self {
        self
    }
}

identity! {
    JsValue,
    Null,
    Undefined,
    AggregateError,
    ArrayBuffer,
    ArrayBufferOptions,
    AsyncDisposableStack,
    Base64Options,
    BigInt,
    BigInt64Array,
    BigUint64Array,
    Boolean,
    DataView,
    Date,
    DisposableStack,
    Error,
    ErrorOptions,
    EvalError,
    FinalizationRegistry,
    Float16Array,
    Float32Array,
    Float64Array,
    Int16Array,
    Int32Array,
    Int8Array,
    JsString,
    Number,
    Proxy,
    RangeError,
    ReferenceError,
    RegExp,
    RegExpMatchArray,
    SetFromResult,
    SharedArrayBuffer,
    SuppressedError,
    Symbol,
    SyntaxError,
    TypeError,
    Uint16Array,
    Uint32Array,
    Uint8Array,
    Uint8ClampedArray,
    UriError,
    Intl::Collator,
    Intl::CollatorOptions,
    Intl::DateTimeFormat,
    Intl::DateTimeFormatOptions,
    Intl::DateTimeFormatPart,
    Intl::DateTimeRangeFormatPart,
    Intl::DisplayNames,
    Intl::DisplayNamesOptions,
    Intl::Duration,
    Intl::DurationFormat,
    Intl::DurationFormatOptions,
    Intl::DurationFormatPart,
    Intl::ListFormat,
    Intl::ListFormatOptions,
    Intl::ListFormatPart,
    Intl::Locale,
    Intl::LocaleMatcherOptions,
    Intl::NumberFormat,
    Intl::NumberFormatOptions,
    Intl::NumberFormatPart,
    Intl::NumberRangeFormatPart,
    Intl::PluralRules,
    Intl::PluralRulesOptions,
    Intl::RelativeTimeFormat,
    Intl::RelativeTimeFormatOptions,
    Intl::RelativeTimeFormatPart,
    Intl::ResolvedCollatorOptions,
    Intl::ResolvedDateTimeFormatOptions,
    Intl::ResolvedDisplayNamesOptions,
    Intl::ResolvedDurationFormatOptions,
    Intl::ResolvedListFormatOptions,
    Intl::ResolvedNumberFormatOptions,
    Intl::ResolvedPluralRulesOptions,
    Intl::ResolvedRelativeTimeFormatOptions,
    Intl::ResolvedSegmenterOptions,
    Intl::SegmentData,
    Intl::Segmenter,
    Intl::SegmenterOptions,
    Intl::Segments,
    Intl::TextInfo,
    Intl::WeekInfo,
    WebAssembly::CompileError,
    WebAssembly::CompileOptions,
    WebAssembly::Exception,
    WebAssembly::Global,
    WebAssembly::Instance,
    WebAssembly::InstantiatedSource,
    WebAssembly::LinkError,
    WebAssembly::Memory,
    WebAssembly::Module,
    WebAssembly::RuntimeError,
    WebAssembly::Suspending,
    WebAssembly::Table,
    WebAssembly::Tag,
}

#[cfg(js_sys_unstable_apis)]
identity! {
    Temporal::ArithmeticOptions,
    Temporal::AssignmentOptions,
    Temporal::CalendarTypeToStringOptions,
    Temporal::DifferenceOptions,
    Temporal::Duration,
    Temporal::DurationArithmeticOptions,
    Temporal::DurationAssignmentOptions,
    Temporal::DurationRoundToOptions,
    Temporal::DurationTotalOptions,
    Temporal::Instant,
    Temporal::InstantToStringOptions,
    Temporal::OffsetDisambiguationOptions,
    Temporal::PlainDate,
    Temporal::PlainDateTime,
    Temporal::PlainMonthDay,
    Temporal::PlainTime,
    Temporal::PlainYearMonth,
    Temporal::RoundToOptions,
    Temporal::ShowCalendarOptions,
    Temporal::TimeZoneTransitionOptions,
    Temporal::ToInstantOptions,
    Temporal::ToStringPrecisionOptions,
    Temporal::ZonedDateTime,
    Temporal::ZonedDateTimeAssignmentOptions,
    Temporal::ZonedDateTimeToStringOptions,
}
