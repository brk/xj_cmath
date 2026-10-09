// These errno expectations describe glibc's MATH_ERRNO behavior.
#![cfg(all(target_os = "linux", target_env = "gnu"))]

use core::ffi::c_int;
use std::hint::black_box;
use xj_cmath::{fmod, fmodf, raw, sqrt, sqrtf};

fn set_errno(value: c_int) {
    // SAFETY: glibc returns writable storage for the current thread's errno.
    unsafe { *libc::__errno_location() = value };
}

fn errno() -> c_int {
    // SAFETY: glibc returns readable storage for the current thread's errno.
    unsafe { *libc::__errno_location() }
}

#[test]
fn domain_errors_overwrite_incoming_errno() {
    for initial in [0, libc::E2BIG, libc::ERANGE] {
        for x in [-1.0, f64::NEG_INFINITY] {
            set_errno(initial);
            let result = sqrt(black_box(x));
            let error = errno();
            assert!(result.is_nan());
            assert_eq!(error, libc::EDOM, "sqrt({x})");
        }
        for x in [-1.0, f32::NEG_INFINITY] {
            set_errno(initial);
            let result = sqrtf(black_box(x));
            let error = errno();
            assert!(result.is_nan());
            assert_eq!(error, libc::EDOM, "sqrtf({x})");
        }
        for (x, y) in [
            (1.0, 0.0),
            (1.0, -0.0),
            (0.0, 0.0),
            (f64::INFINITY, 1.0),
            (f64::NEG_INFINITY, 1.0),
            (f64::INFINITY, f64::INFINITY),
        ] {
            set_errno(initial);
            let result = fmod(black_box(x), black_box(y));
            let error = errno();
            assert!(result.is_nan());
            assert_eq!(error, libc::EDOM, "fmod({x}, {y})");

            set_errno(initial);
            let result = fmodf(black_box(x as f32), black_box(y as f32));
            let error = errno();
            assert!(result.is_nan());
            assert_eq!(error, libc::EDOM, "fmodf({x}, {y})");
        }
    }
}

#[test]
fn successful_calls_preserve_errno_and_signed_zero() {
    macro_rules! check_precision {
        ($ty:ty, $sqrt:ident, $fmod:ident) => {
            for initial in [0, libc::E2BIG, libc::ERANGE] {
                for (x, expected) in [
                    (0.0, 0.0),
                    (-0.0, -0.0),
                    (1.0, 1.0),
                    (4.0, 2.0),
                    (<$ty>::INFINITY, <$ty>::INFINITY),
                ] {
                    set_errno(initial);
                    let result = $sqrt(black_box(x));
                    let error = errno();
                    assert_eq!(result.to_bits(), expected.to_bits());
                    assert_eq!(error, initial);
                }
                for (x, y, expected) in [
                    (5.0, 2.0, 1.0),
                    (-5.0, 2.0, -1.0),
                    (5.0, -2.0, 1.0),
                    (0.0, 1.0, 0.0),
                    (-0.0, 1.0, -0.0),
                    (-4.0, 2.0, -0.0),
                    (1.0, <$ty>::INFINITY, 1.0),
                    (<$ty>::from_bits(1), 1.0, <$ty>::from_bits(1)),
                ] {
                    set_errno(initial);
                    let result = $fmod(black_box(x), black_box(y));
                    let error = errno();
                    assert_eq!(result.to_bits(), expected.to_bits());
                    assert_eq!(error, initial);
                }
            }
        };
    }
    check_precision!(f64, sqrt, fmod);
    check_precision!(f32, sqrtf, fmodf);
}

#[test]
fn nan_inputs_do_not_introduce_domain_errors() {
    macro_rules! check_precision {
        ($ty:ty, $sqrt:ident, $fmod:ident) => {
            for initial in [0, libc::E2BIG, libc::ERANGE] {
                set_errno(initial);
                let result = $sqrt(black_box(<$ty>::NAN));
                let error = errno();
                assert!(result.is_nan());
                assert_eq!(error, initial);

                for (x, y) in [
                    (<$ty>::NAN, 0.0),
                    (<$ty>::NAN, 1.0),
                    (1.0, <$ty>::NAN),
                    (<$ty>::INFINITY, <$ty>::NAN),
                    (<$ty>::NAN, <$ty>::INFINITY),
                    (<$ty>::NAN, <$ty>::NAN),
                ] {
                    set_errno(initial);
                    let result = $fmod(black_box(x), black_box(y));
                    let error = errno();
                    assert!(result.is_nan());
                    assert_eq!(error, initial);
                }
            }
        };
    }
    check_precision!(f64, sqrt, fmod);
    check_precision!(f32, sqrtf, fmodf);
}

#[test]
fn raw_function_pointers_and_discarded_results_set_errno() {
    let sqrt_pointer: unsafe extern "C" fn(f64) -> f64 = raw::sqrt;
    let sqrtf_pointer: unsafe extern "C" fn(f32) -> f32 = raw::sqrtf;
    let fmod_pointer: unsafe extern "C" fn(f64, f64) -> f64 = raw::fmod;
    let fmodf_pointer: unsafe extern "C" fn(f32, f32) -> f32 = raw::fmodf;
    for _ in 0..2 {
        set_errno(0);
        // SAFETY: All scalar floating-point arguments are valid.
        unsafe { black_box(sqrt_pointer)(black_box(-1.0)) };
        assert_eq!(errno(), libc::EDOM);

        set_errno(0);
        // SAFETY: All scalar floating-point arguments are valid.
        unsafe { black_box(sqrtf_pointer)(black_box(-1.0)) };
        assert_eq!(errno(), libc::EDOM);

        set_errno(0);
        // SAFETY: All scalar floating-point arguments are valid.
        unsafe { black_box(fmod_pointer)(black_box(1.0), black_box(0.0)) };
        assert_eq!(errno(), libc::EDOM);

        set_errno(0);
        // SAFETY: All scalar floating-point arguments are valid.
        unsafe { black_box(fmodf_pointer)(black_box(1.0), black_box(0.0)) };
        assert_eq!(errno(), libc::EDOM);

        set_errno(0);
        sqrt(black_box(-1.0));
        assert_eq!(errno(), libc::EDOM);
        set_errno(0);
        sqrtf(black_box(-1.0));
        assert_eq!(errno(), libc::EDOM);
        set_errno(0);
        fmod(black_box(1.0), black_box(0.0));
        assert_eq!(errno(), libc::EDOM);
        set_errno(0);
        fmodf(black_box(1.0), black_box(0.0));
        assert_eq!(errno(), libc::EDOM);
    }
}
