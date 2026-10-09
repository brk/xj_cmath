#include <errno.h>
#include <math.h>

/* Rust's compiler_builtins can satisfy these libm symbols without their errno
 * side effects. Use distinct shim names and supply domain-error reporting
 * explicitly. The target headers provide errno's ABI and EDOM, and targets
 * that do not advertise MATH_ERRNO retain their native error-handling policy.
 * Keep the numerical calls so rounding and floating-point exceptions still
 * follow the C implementation. Successful calls never clear incoming errno. */
double xj_cmath_sqrt(double x) {
    double result = sqrt(x);
    if ((math_errhandling & MATH_ERRNO) && isless(x, 0.0)) {
        errno = EDOM;
    }
    return result;
}

float xj_cmath_sqrtf(float x) {
    float result = sqrtf(x);
    if ((math_errhandling & MATH_ERRNO) && isless(x, 0.0f)) {
        errno = EDOM;
    }
    return result;
}

double xj_cmath_fmod(double x, double y) {
    double result = fmod(x, y);
    if ((math_errhandling & MATH_ERRNO) && !isnan(x) && !isnan(y)
        && (isinf(x) || y == 0.0)) {
        errno = EDOM;
    }
    return result;
}

float xj_cmath_fmodf(float x, float y) {
    float result = fmodf(x, y);
    if ((math_errhandling & MATH_ERRNO) && !isnan(x) && !isnan(y)
        && (isinf(x) || y == 0.0f)) {
        errno = EDOM;
    }
    return result;
}
