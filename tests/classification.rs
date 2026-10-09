use xj_cmath::*;

// Exercise both sides of each category boundary and sign bits independently
// of magnitude, including negative zero and explicitly signed quiet NaNs.
macro_rules! check_classification {
    ($ty:ty, $nan:expr, $negative_nan:expr,
     $classify:ident, $finite:ident, $infinite:ident, $isnan:ident,
     $normal:ident, $sign:ident) => {{
        let cases: &[(
            $ty,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        )] = &[
            (0.0, FP_ZERO, 1, 0, 0, 0, 0),
            (-0.0, FP_ZERO, 1, 0, 0, 0, 1),
            (<$ty>::from_bits(1), FP_SUBNORMAL, 1, 0, 0, 0, 0),
            (-<$ty>::from_bits(1), FP_SUBNORMAL, 1, 0, 0, 0, 1),
            (<$ty>::MIN_POSITIVE, FP_NORMAL, 1, 0, 0, 1, 0),
            (-<$ty>::MIN_POSITIVE, FP_NORMAL, 1, 0, 0, 1, 1),
            (<$ty>::MAX, FP_NORMAL, 1, 0, 0, 1, 0),
            (<$ty>::MIN, FP_NORMAL, 1, 0, 0, 1, 1),
            (<$ty>::INFINITY, FP_INFINITE, 0, 1, 0, 0, 0),
            (<$ty>::NEG_INFINITY, FP_INFINITE, 0, 1, 0, 0, 1),
            ($nan, FP_NAN, 0, 0, 1, 0, 0),
            ($negative_nan, FP_NAN, 0, 0, 1, 0, 1),
        ];
        for &(x, category, finite, infinite, nan, normal, sign) in cases {
            assert_eq!($classify(x), category, "classification of {x:?}");
            assert_eq!($finite(x), finite, "isfinite of {x:?}");
            assert_eq!($infinite(x), infinite, "isinf of {x:?}");
            assert_eq!($isnan(x), nan, "isnan of {x:?}");
            assert_eq!($normal(x), normal, "isnormal of {x:?}");
            assert_eq!($sign(x), sign, "signbit of {x:?}");
        }
    }};
}

#[test]
fn double_classification() {
    check_classification!(
        f64,
        f64::from_bits(0x7ff8_0000_0000_0001),
        f64::from_bits(0xfff8_0000_0000_0001),
        fpclassify,
        isfinite,
        isinf,
        isnan,
        isnormal,
        signbit
    );
}

#[test]
fn float_classification() {
    check_classification!(
        f32,
        f32::from_bits(0x7fc0_0001),
        f32::from_bits(0xffc0_0001),
        fpclassifyf,
        isfinitef,
        isinff,
        isnanf,
        isnormalf,
        signbitf
    );
}
