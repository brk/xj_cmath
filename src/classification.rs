use core::ffi::c_int;
use core::num::FpCategory;

unsafe extern "C" {
    /// Native C classification value for NaNs.
    #[link_name = "xj_cmath_FP_NAN"]
    pub safe static FP_NAN: c_int;
    /// Native C classification value for infinities.
    #[link_name = "xj_cmath_FP_INFINITE"]
    pub safe static FP_INFINITE: c_int;
    /// Native C classification value for positive and negative zero.
    #[link_name = "xj_cmath_FP_ZERO"]
    pub safe static FP_ZERO: c_int;
    /// Native C classification value for subnormal numbers.
    #[link_name = "xj_cmath_FP_SUBNORMAL"]
    pub safe static FP_SUBNORMAL: c_int;
    /// Native C classification value for normal numbers.
    #[link_name = "xj_cmath_FP_NORMAL"]
    pub safe static FP_NORMAL: c_int;
}

macro_rules! predicates {
    ($($name:ident($ty:ty) => $method:ident, $doc:literal;)*) => {$(
        #[doc = $doc]
        ///
        /// Returns `1` when true and `0` otherwise, using Rust's built-in float method.
        #[inline]
        pub fn $name(x: $ty) -> c_int {
            if x.$method() { 1 } else { 0 }
        }
    )*};
}

predicates! {
    isfinite(f64) => is_finite, "Tests whether a double is finite.";
    isfinitef(f32) => is_finite, "Tests whether a float is finite.";
    isinf(f64) => is_infinite, "Tests whether a double is positive or negative infinity.";
    isinff(f32) => is_infinite, "Tests whether a float is positive or negative infinity.";
    isnan(f64) => is_nan, "Tests whether a double is NaN.";
    isnanf(f32) => is_nan, "Tests whether a float is NaN.";
    isnormal(f64) => is_normal, "Tests whether a double is normal (finite, nonzero, and not subnormal).";
    isnormalf(f32) => is_normal, "Tests whether a float is normal (finite, nonzero, and not subnormal).";
    signbit(f64) => is_sign_negative, "Tests the sign bit of a double, including zeros and NaNs.";
    signbitf(f32) => is_sign_negative, "Tests the sign bit of a float, including zeros and NaNs.";
}

#[inline]
fn category_value(category: FpCategory) -> c_int {
    match category {
        FpCategory::Nan => FP_NAN,
        FpCategory::Infinite => FP_INFINITE,
        FpCategory::Zero => FP_ZERO,
        FpCategory::Subnormal => FP_SUBNORMAL,
        FpCategory::Normal => FP_NORMAL,
    }
}

/// Classifies a double using Rust's built-in method and returns the native C `FP_*` value.
#[inline]
pub fn fpclassify(x: f64) -> c_int {
    category_value(x.classify())
}

/// Classifies a float using Rust's built-in method and returns the native C `FP_*` value.
#[inline]
pub fn fpclassifyf(x: f32) -> c_int {
    category_value(x.classify())
}
