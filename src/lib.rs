//! Safe wrappers for C `math.h`, using the platform libm.
//!
//! Floating-point arguments use `f32`/`f64`; integer arguments and results use
//! the corresponding `core::ffi` types.
//!
//! The compatibility target is ordinary C math usage with the default
//! round-to-nearest environment: numerical results, signed zeros, infinities,
//! NaN classification, and domain/pole/range error reporting.
//!
//! # Behavioral compatibility details
//!
//! Numerical work uses the platform's math implementation or compiler builtins;
//! the crate does not impose a common accuracy or error-handling policy across
//! platforms.
//!
//! [`sqrt`] and [`sqrtf`] explicitly set `errno` to the target's `EDOM` for
//! negative arguments (excluding negative zero). [`fmod`] and [`fmodf`] do so
//! for an infinite dividend or a zero divisor, unless either argument is NaN.
//! These guarantees apply when the target advertises `MATH_ERRNO`, including
//! when the result is discarded. The C shims use the target's `errno` storage
//! and constants and do not clear incoming `errno` on success. These checks
//! preserve domain-error reporting even when Rust's compiler builtins supply
//! the numerical implementation.
//!
//! Differential tests of [`exp`], [`log`], and [`pow`] and their `f32` variants
//! on x86_64 Linux/glibc found matching non-NaN results and `errno` against
//! builtin-enabled, optimized GCC C code under default rounding. This is
//! evidence for the tested inputs and toolchains, not a guarantee of exact
//! native-libm behavior for every call. Outside the explicit checks above,
//! compiler folding or substitution can affect both results and error reporting
//! (for example, replacing `pow(x, 2)` with multiplication).
//!
//! The crate does not guarantee bitwise agreement with a separately compiled C
//! call, dynamic rounding-mode preservation, floating-point exception flags or
//! traps, or NaN signs, payloads, and signaling behavior. These limits also apply
//! to [`raw`]; calling a raw declaration does not disable compiler builtins.
//!
//! For context, conforming C implementations can differ in mathematical
//! accuracy, intermediate evaluation precision, NaN representations, and their
//! advertised `math_errhandling` policy; underflow reporting is also partly
//! implementation-defined. Constant folding and builtin substitutions can
//! change observable floating-point flags when environment access is disabled.
//! C's `FENV_ACCESS` and IEC 60559 (Annex F) requirements constrain those changes
//! when applicable; optimization does not license losing required `errno`
//! side effects. See [C11 draft N1570], sections 5.2.4.2.2, 7.6.1, 7.12.1,
//! and Annex F. The compatibility target above is therefore narrower than full
//! equivalence to C compiled for strict floating-point environment access.
//!
//! [C11 draft N1570]: https://open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf
//!
//! # API conventions
//!
//! Classification wrappers use Rust's built-in float methods and return `0` or
//! `1`. [`fpclassify`] and [`fpclassifyf`] return the native C `FP_*` values,
//! exposed here as immutable statics.
//!
//! Output-pointer functions take mutable references. Their unsafe `_ptr`
//! alternatives accept native pointers. The NaN constructors take `&mut CStr`
//! to guarantee NUL-terminated input.
//!
//! [`nexttoward`] and [`nexttowardf`] take [`LongDouble`], an opaque native C
//! value passed through a C shim to preserve precision and the platform ABI.
//!
//! Safe `lgamma`/`gamma` functions use the reentrant implementation and do not
//! update C's global `signgam`. Use [`lgamma_r`] to obtain the sign safely.
//! Original functions and the global are available in [`raw`] for synchronized
//! unsafe access; [`signgam`] also re-exports the C global at the crate root.
//!
//! Requires a C11 compiler and the platform libm. macOS uses compatibility
//! shims for missing GNU/BSD and obsolete symbols; single-precision Bessel
//! functions evaluate the native double-precision function and convert to `f32`.
//! Other platforms require libm to export the requested extensions.
//!
//! ```
//! let mut exponent = 0;
//! assert_eq!(xj_cmath::frexp(12.0, &mut exponent), 0.75);
//! assert_eq!(exponent, 4);
//! let direction = xj_cmath::LongDouble::from(2.0);
//! assert!(xj_cmath::nexttoward(1.0, direction) > 1.0);
//! ```

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

use core::ffi::{CStr, c_char, c_int, c_long, c_longlong};

mod classification;
mod long_double;
pub mod raw;
pub use classification::*;
pub use long_double::{LongDouble, ParseLongDoubleError, nexttoward, nexttowardf};
pub use raw::signgam;

macro_rules! scalar_wrappers {
    ($(fn $name:ident($($arg:ident: $ty:ty),*) -> $ret:ty;)*) => {$(
        #[doc = concat!("Calls C's `", stringify!($name), "` function.")]
        #[inline]
        pub fn $name($($arg: $ty),*) -> $ret {
            // SAFETY: These C functions accept all values of their scalar types.
            unsafe { raw::$name($($arg),*) }
        }
    )*};
}

scalar_wrappers! {
    fn acos(x: f64) -> f64;
    fn acosf(x: f32) -> f32;
    fn asin(x: f64) -> f64;
    fn asinf(x: f32) -> f32;
    fn atan(x: f64) -> f64;
    fn atanf(x: f32) -> f32;
    fn cos(x: f64) -> f64;
    fn cosf(x: f32) -> f32;
    fn sin(x: f64) -> f64;
    fn sinf(x: f32) -> f32;
    fn tan(x: f64) -> f64;
    fn tanf(x: f32) -> f32;
    fn acosh(x: f64) -> f64;
    fn acoshf(x: f32) -> f32;
    fn asinh(x: f64) -> f64;
    fn asinhf(x: f32) -> f32;
    fn atanh(x: f64) -> f64;
    fn atanhf(x: f32) -> f32;
    fn cosh(x: f64) -> f64;
    fn coshf(x: f32) -> f32;
    fn sinh(x: f64) -> f64;
    fn sinhf(x: f32) -> f32;
    fn tanh(x: f64) -> f64;
    fn tanhf(x: f32) -> f32;
    fn exp(x: f64) -> f64;
    fn expf(x: f32) -> f32;
    fn exp2(x: f64) -> f64;
    fn exp2f(x: f32) -> f32;
    fn expm1(x: f64) -> f64;
    fn expm1f(x: f32) -> f32;
    fn log(x: f64) -> f64;
    fn logf(x: f32) -> f32;
    fn log10(x: f64) -> f64;
    fn log10f(x: f32) -> f32;
    fn log1p(x: f64) -> f64;
    fn log1pf(x: f32) -> f32;
    fn log2(x: f64) -> f64;
    fn log2f(x: f32) -> f32;
    fn logb(x: f64) -> f64;
    fn logbf(x: f32) -> f32;
    fn cbrt(x: f64) -> f64;
    fn cbrtf(x: f32) -> f32;
    fn fabs(x: f64) -> f64;
    fn fabsf(x: f32) -> f32;
    fn sqrt(x: f64) -> f64;
    fn sqrtf(x: f32) -> f32;
    fn erf(x: f64) -> f64;
    fn erff(x: f32) -> f32;
    fn erfc(x: f64) -> f64;
    fn erfcf(x: f32) -> f32;
    fn tgamma(x: f64) -> f64;
    fn tgammaf(x: f32) -> f32;
    fn ceil(x: f64) -> f64;
    fn ceilf(x: f32) -> f32;
    fn floor(x: f64) -> f64;
    fn floorf(x: f32) -> f32;
    fn nearbyint(x: f64) -> f64;
    fn nearbyintf(x: f32) -> f32;
    fn rint(x: f64) -> f64;
    fn rintf(x: f32) -> f32;
    fn round(x: f64) -> f64;
    fn roundf(x: f32) -> f32;
    fn trunc(x: f64) -> f64;
    fn truncf(x: f32) -> f32;
    fn significand(x: f64) -> f64;
    fn significandf(x: f32) -> f32;
    fn atan2(y: f64, x: f64) -> f64;
    fn atan2f(y: f32, x: f32) -> f32;
    fn hypot(x: f64, y: f64) -> f64;
    fn hypotf(x: f32, y: f32) -> f32;
    fn pow(x: f64, y: f64) -> f64;
    fn powf(x: f32, y: f32) -> f32;
    fn fmod(x: f64, y: f64) -> f64;
    fn fmodf(x: f32, y: f32) -> f32;
    fn remainder(x: f64, y: f64) -> f64;
    fn remainderf(x: f32, y: f32) -> f32;
    fn copysign(x: f64, y: f64) -> f64;
    fn copysignf(x: f32, y: f32) -> f32;
    fn nextafter(x: f64, y: f64) -> f64;
    fn nextafterf(x: f32, y: f32) -> f32;
    fn fdim(x: f64, y: f64) -> f64;
    fn fdimf(x: f32, y: f32) -> f32;
    fn fmax(x: f64, y: f64) -> f64;
    fn fmaxf(x: f32, y: f32) -> f32;
    fn fmin(x: f64, y: f64) -> f64;
    fn fminf(x: f32, y: f32) -> f32;
    fn drem(x: f64, y: f64) -> f64;
    fn dremf(x: f32, y: f32) -> f32;
    fn scalb(x: f64, exp: f64) -> f64;
    fn scalbf(x: f32, exp: f32) -> f32;
    fn fma(x: f64, y: f64, z: f64) -> f64;
    fn fmaf(x: f32, y: f32, z: f32) -> f32;
    fn ilogb(x: f64) -> c_int;
    fn ilogbf(x: f32) -> c_int;
    fn finite(x: f64) -> c_int;
    fn finitef(x: f32) -> c_int;
    fn ldexp(x: f64, exp: c_int) -> f64;
    fn ldexpf(x: f32, exp: c_int) -> f32;
    fn scalbn(x: f64, n: c_int) -> f64;
    fn scalbnf(x: f32, n: c_int) -> f32;
    fn scalbln(x: f64, n: c_long) -> f64;
    fn scalblnf(x: f32, n: c_long) -> f32;
    fn lrint(x: f64) -> c_long;
    fn lrintf(x: f32) -> c_long;
    fn lround(x: f64) -> c_long;
    fn lroundf(x: f32) -> c_long;
    fn llrint(x: f64) -> c_longlong;
    fn llrintf(x: f32) -> c_longlong;
    fn llround(x: f64) -> c_longlong;
    fn llroundf(x: f32) -> c_longlong;
    fn j0(x: f64) -> f64;
    fn j0f(x: f32) -> f32;
    fn j1(x: f64) -> f64;
    fn j1f(x: f32) -> f32;
    fn y0(x: f64) -> f64;
    fn y0f(x: f32) -> f32;
    fn y1(x: f64) -> f64;
    fn y1f(x: f32) -> f32;
    fn jn(n: c_int, x: f64) -> f64;
    fn jnf(n: c_int, x: f32) -> f32;
    fn yn(n: c_int, x: f64) -> f64;
    fn ynf(n: c_int, x: f32) -> f32;
}

macro_rules! output_wrappers {
    ($(fn $name:ident / $ptr:ident ($($arg:ident: $ty:ty),*; $($out:ident: $out_ty:ty),+) -> $ret:ty;)*) => {$(
        #[doc = concat!("Calls C's `", stringify!($name), "` with valid output references.")]
        #[inline]
        pub fn $name($($arg: $ty,)* $($out: &mut $out_ty),+) -> $ret {
            // SAFETY: Mutable references provide valid, distinct output locations.
            unsafe { $ptr($($arg,)* $($out),+) }
        }

        #[doc = concat!("Calls C's `", stringify!($name), "` with raw output pointers.")]
        ///
        /// # Safety
        /// Each output pointer must be non-null, properly aligned, writable, and
        /// valid for one value of its pointee type during the call. No other
        /// thread may access those output locations during the call.
        #[inline]
        pub unsafe fn $ptr($($arg: $ty,)* $($out: *mut $out_ty),+) -> $ret {
            // SAFETY: The caller supplies valid output pointers.
            unsafe { raw::$name($($arg,)* $($out),+) }
        }
    )*};
}

output_wrappers! {
    fn frexp / frexp_ptr (value: f64; exp: c_int) -> f64;
    fn modf / modf_ptr (value: f64; iptr: f64) -> f64;
    fn remquo / remquo_ptr (x: f64, y: f64; quo: c_int) -> f64;
    fn sincos / sincos_ptr (x: f64; sin: f64, cos: f64) -> ();
    fn lgamma_r / lgamma_r_ptr (x: f64; signp: c_int) -> f64;
    fn frexpf / frexpf_ptr (value: f32; exp: c_int) -> f32;
    fn modff / modff_ptr (value: f32; iptr: f32) -> f32;
    fn remquof / remquof_ptr (x: f32, y: f32; quo: c_int) -> f32;
    fn sincosf / sincosf_ptr (x: f32; sin: f32, cos: f32) -> ();
    fn lgammaf_r / lgammaf_r_ptr (x: f32; signp: c_int) -> f32;
}

/// Computes the logarithm of the absolute gamma function using C's reentrant implementation.
///
/// Returns the numerical result of `lgamma` without writing `signgam`.
/// Use [`lgamma_r`] for the sign or [`raw::lgamma`] for the original global side effect.
#[inline]
pub fn lgamma(x: f64) -> f64 {
    let mut sign = 0;
    lgamma_r(x, &mut sign)
}

/// Computes the logarithm of the absolute gamma function using C's reentrant implementation.
///
/// Returns the numerical result of `lgammaf` without writing `signgam`.
/// Use [`lgammaf_r`] for the sign or [`raw::lgammaf`] for the original global side effect.
#[inline]
pub fn lgammaf(x: f32) -> f32 {
    let mut sign = 0;
    lgammaf_r(x, &mut sign)
}

/// Computes the logarithm of the absolute gamma function using C's reentrant implementation.
///
/// Returns the numerical result of `gamma` without writing `signgam`.
/// Use [`lgamma_r`] for the sign or [`raw::gamma`] for the original global side effect.
#[inline]
pub fn gamma(x: f64) -> f64 {
    let mut sign = 0;
    lgamma_r(x, &mut sign)
}

/// Computes the logarithm of the absolute gamma function using C's reentrant implementation.
///
/// Returns the numerical result of `gammaf` without writing `signgam`.
/// Use [`lgammaf_r`] for the sign or [`raw::gammaf`] for the original global side effect.
#[inline]
pub fn gammaf(x: f32) -> f32 {
    let mut sign = 0;
    lgammaf_r(x, &mut sign)
}

/// Constructs a NaN from a NUL-terminated C payload tag.
#[inline]
pub fn nan(tagp: &mut CStr) -> f64 {
    // SAFETY: CStr guarantees readable, terminated input; C does not modify it.
    unsafe { nan_ptr(tagp.as_ptr()) }
}

/// Constructs a NaN from a raw C payload tag.
///
/// # Safety
/// `tagp` must be non-null and point to a readable NUL-terminated string
/// that remains valid and is not modified during the call.
#[inline]
pub unsafe fn nan_ptr(tagp: *const c_char) -> f64 {
    // SAFETY: The caller supplies a valid C string.
    unsafe { raw::nan(tagp) }
}

/// Constructs a NaN from a NUL-terminated C payload tag.
#[inline]
pub fn nanf(tagp: &mut CStr) -> f32 {
    // SAFETY: CStr guarantees readable, terminated input; C does not modify it.
    unsafe { nanf_ptr(tagp.as_ptr()) }
}

/// Constructs a NaN from a raw C payload tag.
///
/// # Safety
/// `tagp` must be non-null and point to a readable NUL-terminated string
/// that remains valid and is not modified during the call.
#[inline]
pub unsafe fn nanf_ptr(tagp: *const c_char) -> f32 {
    // SAFETY: The caller supplies a valid C string.
    unsafe { raw::nanf(tagp) }
}
