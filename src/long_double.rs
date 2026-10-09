use core::ffi::{CStr, c_char, c_int};
use core::{fmt, mem::MaybeUninit};

/// An opaque value with the precision and range of native C `long double`.
///
/// Construct from `f32`/`f64`, or parse a C string to retain precision beyond
/// `f64`. This is storage for a C shim, **not** a type that can be passed directly
/// to an `extern "C"` function expecting a by-value `long double`.
#[derive(Clone, Copy)]
pub struct LongDouble {
    // C long double can contain uninitialized padding. Never inspect these
    // bytes in Rust; MaybeUninit permits copying that padding without UB.
    bytes: [MaybeUninit<u8>; 16],
}

unsafe extern "C" {
    fn xj_cmath_long_double_from_f64(value: f64, out: *mut u8);
    fn xj_cmath_long_double_to_f64(value: *const u8) -> f64;
    fn xj_cmath_long_double_parse(text: *const c_char, out: *mut u8) -> c_int;
    fn xj_cmath_long_double_mant_dig() -> c_int;
    fn xj_cmath_nexttoward(x: f64, y: *const u8) -> f64;
    fn xj_cmath_nexttowardf(x: f32, y: *const u8) -> f32;
}

impl LongDouble {
    fn uninit() -> Self {
        Self {
            bytes: [MaybeUninit::uninit(); 16],
        }
    }

    /// Parses a complete C floating-point string using native `strtold`.
    ///
    /// Accepts decimal, hexadecimal, infinity, and NaN forms, and leading
    /// whitespace. Trailing characters (including whitespace) are rejected.
    /// Parsing follows the current C locale. Range errors have `strtold`
    /// semantics: overflow/underflow may produce infinity/zero and set `errno`.
    pub fn from_c_str(text: &CStr) -> Result<Self, ParseLongDoubleError> {
        let mut value = Self::uninit();
        // SAFETY: CStr supplies terminated input; the buffer has 16 writable
        // bytes. The shim verifies sizeof(long double) <= 16 at compile time.
        let success =
            unsafe { xj_cmath_long_double_parse(text.as_ptr(), value.bytes.as_mut_ptr().cast()) };
        if success != 0 {
            Ok(value)
        } else {
            Err(ParseLongDoubleError)
        }
    }

    /// Converts to `f64`, rounding according to the native C conversion.
    pub fn to_f64(self) -> f64 {
        // SAFETY: Only constructors that store valid C long doubles are public.
        unsafe { xj_cmath_long_double_to_f64(self.bytes.as_ptr().cast()) }
    }

    /// Returns the native `LDBL_MANT_DIG` (significand precision in radix digits).
    pub fn mantissa_digits() -> c_int {
        // SAFETY: No arguments or shared mutable state.
        unsafe { xj_cmath_long_double_mant_dig() }
    }
}

impl From<f64> for LongDouble {
    fn from(value: f64) -> Self {
        let mut result = Self::uninit();
        // SAFETY: The shim writes a valid representation into a 16-byte buffer.
        unsafe { xj_cmath_long_double_from_f64(value, result.bytes.as_mut_ptr().cast()) };
        result
    }
}

impl From<f32> for LongDouble {
    fn from(value: f32) -> Self {
        Self::from(f64::from(value))
    }
}

impl fmt::Debug for LongDouble {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // This approximation does not attempt to display the full C precision.
        f.debug_struct("LongDouble")
            .field("f64_approximation", &self.to_f64())
            .finish()
    }
}

/// A string was empty or was not consumed completely by `strtold`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseLongDoubleError;

impl fmt::Display for ParseLongDoubleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid long double string")
    }
}

impl core::error::Error for ParseLongDoubleError {}

/// Calls C's `nexttoward`, preserving the native `long double` direction.
#[inline]
pub fn nexttoward(x: f64, y: LongDouble) -> f64 {
    // SAFETY: y contains a valid long double; the shim handles its native ABI.
    unsafe { xj_cmath_nexttoward(x, y.bytes.as_ptr().cast()) }
}

/// Calls C's `nexttowardf`, preserving the native `long double` direction.
#[inline]
pub fn nexttowardf(x: f32, y: LongDouble) -> f32 {
    // SAFETY: y contains a valid long double; the shim handles its native ABI.
    unsafe { xj_cmath_nexttowardf(x, y.bytes.as_ptr().cast()) }
}
