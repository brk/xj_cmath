use xj_cmath::*;

// Call every scalar wrapper so missing system symbols fail during linking.
#[test]
fn scalar_symbols_and_results() {
    assert_eq!(acos(1.0), 0.0, "acos");
    assert_eq!(acosf(1.0), 0.0, "acosf");
    assert_eq!(asin(0.0), 0.0, "asin");
    assert_eq!(asinf(0.0), 0.0, "asinf");
    assert_eq!(atan(0.0), 0.0, "atan");
    assert_eq!(atanf(0.0), 0.0, "atanf");
    assert_eq!(cos(0.0), 1.0, "cos");
    assert_eq!(cosf(0.0), 1.0, "cosf");
    assert_eq!(sin(0.0), 0.0, "sin");
    assert_eq!(sinf(0.0), 0.0, "sinf");
    assert_eq!(tan(0.0), 0.0, "tan");
    assert_eq!(tanf(0.0), 0.0, "tanf");
    assert_eq!(acosh(1.0), 0.0, "acosh");
    assert_eq!(acoshf(1.0), 0.0, "acoshf");
    assert_eq!(asinh(0.0), 0.0, "asinh");
    assert_eq!(asinhf(0.0), 0.0, "asinhf");
    assert_eq!(atanh(0.0), 0.0, "atanh");
    assert_eq!(atanhf(0.0), 0.0, "atanhf");
    assert_eq!(cosh(0.0), 1.0, "cosh");
    assert_eq!(coshf(0.0), 1.0, "coshf");
    assert_eq!(sinh(0.0), 0.0, "sinh");
    assert_eq!(sinhf(0.0), 0.0, "sinhf");
    assert_eq!(tanh(0.0), 0.0, "tanh");
    assert_eq!(tanhf(0.0), 0.0, "tanhf");
    assert_eq!(exp(0.0), 1.0, "exp");
    assert_eq!(expf(0.0), 1.0, "expf");
    assert_eq!(exp2(0.0), 1.0, "exp2");
    assert_eq!(exp2f(0.0), 1.0, "exp2f");
    assert_eq!(expm1(0.0), 0.0, "expm1");
    assert_eq!(expm1f(0.0), 0.0, "expm1f");
    assert_eq!(log(1.0), 0.0, "log");
    assert_eq!(logf(1.0), 0.0, "logf");
    assert_eq!(log10(1.0), 0.0, "log10");
    assert_eq!(log10f(1.0), 0.0, "log10f");
    assert_eq!(log1p(0.0), 0.0, "log1p");
    assert_eq!(log1pf(0.0), 0.0, "log1pf");
    assert_eq!(log2(1.0), 0.0, "log2");
    assert_eq!(log2f(1.0), 0.0, "log2f");
    assert_eq!(logb(8.0), 3.0, "logb");
    assert_eq!(logbf(8.0), 3.0, "logbf");
    assert_eq!(cbrt(8.0), 2.0, "cbrt");
    assert_eq!(cbrtf(8.0), 2.0, "cbrtf");
    assert_eq!(fabs(-2.0), 2.0, "fabs");
    assert_eq!(fabsf(-2.0), 2.0, "fabsf");
    assert_eq!(sqrt(4.0), 2.0, "sqrt");
    assert_eq!(sqrtf(4.0), 2.0, "sqrtf");
    assert_eq!(erf(0.0), 0.0, "erf");
    assert_eq!(erff(0.0), 0.0, "erff");
    assert_eq!(erfc(0.0), 1.0, "erfc");
    assert_eq!(erfcf(0.0), 1.0, "erfcf");
    assert_eq!(tgamma(1.0), 1.0, "tgamma");
    assert_eq!(tgammaf(1.0), 1.0, "tgammaf");
    assert_eq!(ceil(1.25), 2.0, "ceil");
    assert_eq!(ceilf(1.25), 2.0, "ceilf");
    assert_eq!(floor(1.25), 1.0, "floor");
    assert_eq!(floorf(1.25), 1.0, "floorf");
    assert_eq!(nearbyint(1.25), 1.0, "nearbyint");
    assert_eq!(nearbyintf(1.25), 1.0, "nearbyintf");
    assert_eq!(rint(1.25), 1.0, "rint");
    assert_eq!(rintf(1.25), 1.0, "rintf");
    assert_eq!(round(1.5), 2.0, "round");
    assert_eq!(roundf(1.5), 2.0, "roundf");
    assert_eq!(trunc(-1.25), -1.0, "trunc");
    assert_eq!(truncf(-1.25), -1.0, "truncf");
    assert_eq!(significand(12.0), 1.5, "significand");
    assert_eq!(significandf(12.0), 1.5, "significandf");
    assert_eq!(atan2(0.0, 1.0), 0.0, "atan2");
    assert_eq!(atan2f(0.0, 1.0), 0.0, "atan2f");
    assert_eq!(hypot(3.0, 4.0), 5.0, "hypot");
    assert_eq!(hypotf(3.0, 4.0), 5.0, "hypotf");
    assert_eq!(pow(2.0, 3.0), 8.0, "pow");
    assert_eq!(powf(2.0, 3.0), 8.0, "powf");
    assert_eq!(fmod(5.0, 2.0), 1.0, "fmod");
    assert_eq!(fmodf(5.0, 2.0), 1.0, "fmodf");
    assert_eq!(remainder(7.0, 2.0), -1.0, "remainder");
    assert_eq!(remainderf(7.0, 2.0), -1.0, "remainderf");
    assert_eq!(copysign(2.0, -1.0), -2.0, "copysign");
    assert_eq!(copysignf(2.0, -1.0), -2.0, "copysignf");
    assert_eq!(nextafter(1.0, 1.0), 1.0, "nextafter");
    assert_eq!(nextafterf(1.0, 1.0), 1.0, "nextafterf");
    assert_eq!(fdim(5.0, 2.0), 3.0, "fdim");
    assert_eq!(fdimf(5.0, 2.0), 3.0, "fdimf");
    assert_eq!(fmax(5.0, 2.0), 5.0, "fmax");
    assert_eq!(fmaxf(5.0, 2.0), 5.0, "fmaxf");
    assert_eq!(fmin(5.0, 2.0), 2.0, "fmin");
    assert_eq!(fminf(5.0, 2.0), 2.0, "fminf");
    assert_eq!(drem(7.0, 2.0), -1.0, "drem");
    assert_eq!(dremf(7.0, 2.0), -1.0, "dremf");
    assert_eq!(scalb(1.5, 3.0), 12.0, "scalb");
    assert_eq!(scalbf(1.5, 3.0), 12.0, "scalbf");
    assert_eq!(fma(2.0, 3.0, 4.0), 10.0, "fma");
    assert_eq!(fmaf(2.0, 3.0, 4.0), 10.0, "fmaf");
    assert_eq!(ilogb(8.0), 3, "ilogb");
    assert_eq!(ilogbf(8.0), 3, "ilogbf");
    assert_eq!(finite(1.0), 1, "finite");
    assert_eq!(finitef(1.0), 1, "finitef");
    assert_eq!(isinf(0.0), 0, "isinf");
    assert_eq!(isinff(0.0), 0, "isinff");
    assert_eq!(isnan(0.0), 0, "isnan");
    assert_eq!(isnanf(0.0), 0, "isnanf");
    assert_eq!(ldexp(1.5, 3), 12.0, "ldexp");
    assert_eq!(ldexpf(1.5, 3), 12.0, "ldexpf");
    assert_eq!(scalbn(1.5, 3), 12.0, "scalbn");
    assert_eq!(scalbnf(1.5, 3), 12.0, "scalbnf");
    assert_eq!(scalbln(1.5, 3), 12.0, "scalbln");
    assert_eq!(scalblnf(1.5, 3), 12.0, "scalblnf");
    assert_eq!(lrint(1.25), 1, "lrint");
    assert_eq!(lrintf(1.25), 1, "lrintf");
    assert_eq!(lround(-1.5), -2, "lround");
    assert_eq!(lroundf(-1.5), -2, "lroundf");
    assert_eq!(llrint(1.25), 1, "llrint");
    assert_eq!(llrintf(1.25), 1, "llrintf");
    assert_eq!(llround(-1.5), -2, "llround");
    assert_eq!(llroundf(-1.5), -2, "llroundf");
    assert_eq!(j0(0.0), 1.0, "j0");
    assert_eq!(j0f(0.0), 1.0, "j0f");
    assert_eq!(j1(0.0), 0.0, "j1");
    assert_eq!(j1f(0.0), 0.0, "j1f");
    assert!(((y0(1.0) as f64) - (0.08825696421567696)).abs() < 1e-14);
    assert!(((y0f(1.0) as f64) - (0.08825696421567696)).abs() < 1e-6);
    assert!(((y1(1.0) as f64) - (-0.7812128213002887)).abs() < 1e-14);
    assert!(((y1f(1.0) as f64) - (-0.7812128213002887)).abs() < 1e-6);
    assert_eq!(jn(2, 0.0), 0.0, "jn");
    assert_eq!(jnf(2, 0.0), 0.0, "jnf");
    assert!(((yn(0, 1.0) as f64) - (0.08825696421567696)).abs() < 1e-14);
    assert!(((ynf(0, 1.0) as f64) - (0.08825696421567696)).abs() < 1e-6);
}

#[test]
fn floating_point_edge_cases() {
    assert!(sqrt(-1.0).is_nan());
    assert!(sqrtf(-1.0).is_nan());
    assert_eq!(log(0.0), f64::NEG_INFINITY);
    assert_eq!(logf(0.0), f32::NEG_INFINITY);
    assert_eq!(fabs(-0.0).to_bits(), 0.0f64.to_bits());
    assert_eq!(fabsf(-0.0).to_bits(), 0.0f32.to_bits());
    assert_eq!(copysign(0.0, -1.0).to_bits(), (-0.0f64).to_bits());
    assert_eq!(copysignf(0.0, -1.0).to_bits(), (-0.0f32).to_bits());
    assert_eq!(nextafter(1.0, 2.0).to_bits(), 1.0f64.to_bits() + 1);
    assert_eq!(nextafterf(1.0, 2.0).to_bits(), 1.0f32.to_bits() + 1);
    assert_eq!(nextafter(0.0, 1.0).to_bits(), 1);
    assert_eq!(nextafterf(0.0, 1.0).to_bits(), 1);
    assert_eq!(fmin(f64::NAN, 2.0), 2.0);
    assert_eq!(fmax(2.0, f64::NAN), 2.0);
    assert_eq!(fminf(f32::NAN, 2.0), 2.0);
    assert_eq!(fmaxf(2.0, f32::NAN), 2.0);
    assert_eq!(finite(f64::INFINITY), 0);
    assert_eq!(finitef(f32::NAN), 0);
    assert_ne!(isinf(f64::NEG_INFINITY), 0);
    assert_ne!(isinff(f32::INFINITY), 0);
    assert_ne!(isnan(f64::NAN), 0);
    assert_ne!(isnanf(f32::NAN), 0);
    // A separate multiply/add rounds away the residue; fma retains it.
    assert_eq!(
        fma(1.0 + f64::EPSILON, 1.0 - f64::EPSILON, -1.0),
        -f64::EPSILON * f64::EPSILON
    );
    assert_eq!(
        fmaf(1.0 + f32::EPSILON, 1.0 - f32::EPSILON, -1.0),
        -f32::EPSILON * f32::EPSILON
    );
}

#[cfg(target_os = "macos")]
#[test]
fn macos_compatibility_edge_cases() {
    assert_eq!(significand(f64::from_bits(1)), 1.0);
    assert_eq!(significandf(f32::from_bits(1)), 1.0);
    assert_eq!(significand(-12.0), -1.5);
    assert_eq!(significandf(-12.0), -1.5);
    assert_eq!(significand(-0.0).to_bits(), (-0.0f64).to_bits());
    assert_eq!(significandf(-0.0).to_bits(), (-0.0f32).to_bits());
    assert_eq!(significand(f64::INFINITY), f64::INFINITY);
    assert!(significandf(f32::NAN).is_nan());

    assert_eq!(scalbf(1.0, -149.0).to_bits(), 1);
    assert_eq!(scalbf(-1.0, -f32::MAX).to_bits(), (-0.0f32).to_bits());
    assert_eq!(scalbf(1.0, f32::MAX), f32::INFINITY);
    assert_eq!(scalbf(1.0, f32::INFINITY), f32::INFINITY);
    assert_eq!(scalbf(1.0, f32::NEG_INFINITY), 0.0);
    assert!(scalbf(1.0, 0.5).is_nan());
    assert!(scalbf(1.0, f32::NAN).is_nan());
    assert_eq!(scalbf(-0.0, f32::MAX).to_bits(), (-0.0f32).to_bits());

    let (mut sine, mut cosine) = (1.0, 0.0);
    sincos(-0.0, &mut sine, &mut cosine);
    assert_eq!(sine.to_bits(), (-0.0f64).to_bits());
    assert_eq!(cosine, 1.0);
    let (mut sine, mut cosine) = (0.0, 0.0);
    sincosf(f32::INFINITY, &mut sine, &mut cosine);
    assert!(sine.is_nan());
    assert!(cosine.is_nan());
}

#[test]
fn output_references_and_raw_pointers() {
    {
        let mut exp = 0;
        assert_eq!(frexp(12.0, &mut exp), 0.75);
        assert_eq!(ldexp(0.75, exp), 12.0);
        let mut raw_exp = core::mem::MaybeUninit::uninit();
        // SAFETY: as_mut_ptr points to writable aligned c_int storage.
        assert_eq!(unsafe { frexp_ptr(12.0, raw_exp.as_mut_ptr()) }, 0.75);
        // SAFETY: frexp wrote the exponent.
        assert_eq!(unsafe { raw_exp.assume_init() }, exp);

        let mut integer = 0.0;
        assert_eq!(modf(-3.25, &mut integer), -0.25);
        assert_eq!(integer, -3.0);
        // SAFETY: integer is a valid writable output.
        assert_eq!(unsafe { modf_ptr(3.25, &mut integer) }, 0.25);
        assert_eq!(integer, 3.0);

        let mut quotient = 0;
        assert_eq!(remquo(7.0, 2.0, &mut quotient), -1.0);
        assert_eq!(quotient & 7, 4);
        // SAFETY: quotient is a valid writable output.
        assert_eq!(unsafe { remquo_ptr(-7.0, 2.0, &mut quotient) }, 1.0);
        assert!(quotient < 0);
        assert_eq!((-quotient) & 7, 4);

        let (mut sine, mut cosine) = (0.0, 0.0);
        sincos(0.75, &mut sine, &mut cosine);
        assert_eq!(sine, sin(0.75));
        assert_eq!(cosine, cos(0.75));
        // SAFETY: two distinct valid outputs.
        unsafe { sincos_ptr(-0.75, &mut sine, &mut cosine) };
        assert_eq!(sine, sin(-0.75));
        assert_eq!(cosine, cos(-0.75));

        let mut sign = 0;
        let log_gamma = lgamma_r(-0.5, &mut sign);
        assert_eq!(sign, -1);
        assert!((log_gamma as f64 - (2.0 * std::f64::consts::PI.sqrt()).ln()).abs() < 1e-14);
        assert_eq!(lgamma(-0.5), log_gamma);
        assert_eq!(gamma(-0.5), log_gamma);
        // SAFETY: sign is a valid writable output.
        assert_eq!(unsafe { lgamma_r_ptr(-0.5, &mut sign) }, log_gamma);
        assert_eq!(sign, -1);
    }
    {
        let mut exp = 0;
        assert_eq!(frexpf(12.0, &mut exp), 0.75);
        assert_eq!(ldexpf(0.75, exp), 12.0);
        let mut raw_exp = core::mem::MaybeUninit::uninit();
        // SAFETY: as_mut_ptr points to writable aligned c_int storage.
        assert_eq!(unsafe { frexpf_ptr(12.0, raw_exp.as_mut_ptr()) }, 0.75);
        // SAFETY: frexp wrote the exponent.
        assert_eq!(unsafe { raw_exp.assume_init() }, exp);

        let mut integer = 0.0;
        assert_eq!(modff(-3.25, &mut integer), -0.25);
        assert_eq!(integer, -3.0);
        // SAFETY: integer is a valid writable output.
        assert_eq!(unsafe { modff_ptr(3.25, &mut integer) }, 0.25);
        assert_eq!(integer, 3.0);

        let mut quotient = 0;
        assert_eq!(remquof(7.0, 2.0, &mut quotient), -1.0);
        assert_eq!(quotient & 7, 4);
        // SAFETY: quotient is a valid writable output.
        assert_eq!(unsafe { remquof_ptr(-7.0, 2.0, &mut quotient) }, 1.0);
        assert!(quotient < 0);
        assert_eq!((-quotient) & 7, 4);

        let (mut sine, mut cosine) = (0.0, 0.0);
        sincosf(0.75, &mut sine, &mut cosine);
        assert_eq!(sine, sinf(0.75));
        assert_eq!(cosine, cosf(0.75));
        // SAFETY: two distinct valid outputs.
        unsafe { sincosf_ptr(-0.75, &mut sine, &mut cosine) };
        assert_eq!(sine, sinf(-0.75));
        assert_eq!(cosine, cosf(-0.75));

        let mut sign = 0;
        let log_gamma = lgammaf_r(-0.5, &mut sign);
        assert_eq!(sign, -1);
        assert!((log_gamma as f64 - (2.0 * std::f64::consts::PI.sqrt()).ln()).abs() < 1e-6);
        assert_eq!(lgammaf(-0.5), log_gamma);
        assert_eq!(gammaf(-0.5), log_gamma);
        // SAFETY: sign is a valid writable output.
        assert_eq!(unsafe { lgammaf_r_ptr(-0.5, &mut sign) }, log_gamma);
        assert_eq!(sign, -1);
    }
}

#[test]
fn nan_tags() {
    let mut tag = std::ffi::CString::new("123").unwrap().into_boxed_c_str();
    assert!(nan(&mut tag).is_nan());
    assert!(nanf(&mut tag).is_nan());
    // SAFETY: literals supply terminated readable strings.
    unsafe {
        assert!(nan_ptr(c"".as_ptr()).is_nan());
        assert!(nanf_ptr(c"123".as_ptr()).is_nan());
    }
}

#[test]
fn native_long_double_direction() {
    assert_eq!(LongDouble::from(1.25f64).to_f64(), 1.25);
    assert_eq!(LongDouble::from(1.25f32).to_f64(), 1.25);
    let towards_two = LongDouble::from(2.0);
    assert_eq!(nexttoward(1.0, towards_two).to_bits(), 1.0f64.to_bits() + 1);
    assert_eq!(
        nexttowardf(1.0, towards_two).to_bits(),
        1.0f32.to_bits() + 1
    );

    // These values round to 1.0 in f64 but are distinct in extended precision.
    if LongDouble::mantissa_digits() > 53 {
        let above = LongDouble::from_c_str(c"0x1.000000000000001p0").unwrap();
        let below = LongDouble::from_c_str(c"0x0.fffffffffffffffp0").unwrap();
        assert_eq!(above.to_f64(), 1.0);
        assert_eq!(below.to_f64(), 1.0);
        let copied = above;
        assert_eq!(nexttoward(1.0, copied).to_bits(), 1.0f64.to_bits() + 1);
        assert_eq!(nexttoward(1.0, below).to_bits(), 1.0f64.to_bits() - 1);
        assert_eq!(nexttowardf(1.0, above).to_bits(), 1.0f32.to_bits() + 1);
        assert_eq!(nexttowardf(1.0, below).to_bits(), 1.0f32.to_bits() - 1);
    }
    let negative_zero = LongDouble::from(-0.0);
    assert_eq!(
        nexttoward(0.0, negative_zero).to_bits(),
        (-0.0f64).to_bits()
    );
    assert_eq!(
        nexttowardf(0.0, negative_zero).to_bits(),
        (-0.0f32).to_bits()
    );
    let not_a_number = LongDouble::from_c_str(c"nan").unwrap();
    assert!(nexttoward(1.0, not_a_number).is_nan());
    assert!(nexttowardf(1.0, not_a_number).is_nan());
    assert!(LongDouble::from_c_str(c"").is_err());
    assert!(LongDouble::from_c_str(c"1.0garbage").is_err());
    assert!(LongDouble::from_c_str(c"1.0 ").is_err());
}

// The sole test accessing native signgam. All other tests use reentrant calls.
#[test]
fn original_gamma_global_and_reentrant_safety() {
    // SAFETY: No other test accesses signgam or calls non-reentrant gamma.
    unsafe {
        signgam = 42;
        assert_eq!(lgamma(1.0), 0.0);
        assert_eq!(lgammaf(1.0), 0.0);
        assert_eq!(gamma(1.0), 0.0);
        assert_eq!(gammaf(1.0), 0.0);
        assert_eq!(core::ptr::addr_of!(signgam).read(), 42);
        raw::lgamma(-0.5);
        assert_eq!(core::ptr::addr_of!(signgam).read(), -1);
        raw::lgammaf(-0.5);
        assert_eq!(core::ptr::addr_of!(signgam).read(), -1);
        raw::gamma(0.5);
        assert_eq!(core::ptr::addr_of!(signgam).read(), 1);
        raw::gammaf(0.5);
        assert_eq!(core::ptr::addr_of!(signgam).read(), 1);
    }
}
