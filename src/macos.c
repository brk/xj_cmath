#include <errno.h>
#include <fenv.h>
#include <limits.h>
#include <math.h>

/* Keep compatibility symbols private to this crate's bindings rather than
 * defining missing libm names that could collide with another library. */
double xj_cmath_significand(double x) {
    int exponent;
    return 2.0 * frexp(x, &exponent);
}

float xj_cmath_significandf(float x) {
    int exponent;
    return 2.0f * frexpf(x, &exponent);
}

void xj_cmath_sincos(double x, double *sine, double *cosine) {
    *sine = sin(x);
    *cosine = cos(x);
}

void xj_cmath_sincosf(float x, float *sine, float *cosine) {
    *sine = sinf(x);
    *cosine = cosf(x);
}

int xj_cmath_finite(double x) { return isfinite(x); }
int xj_cmath_finitef(float x) { return isfinite(x); }
int xj_cmath_isinff(float x) { return isinf(x); }
int xj_cmath_isnanf(float x) { return isnan(x); }

float xj_cmath_j0f(float x) { return (float)j0((double)x); }
float xj_cmath_j1f(float x) { return (float)j1((double)x); }
float xj_cmath_jnf(int n, float x) { return (float)jn(n, (double)x); }
float xj_cmath_y0f(float x) { return (float)y0((double)x); }
float xj_cmath_y1f(float x) { return (float)y1((double)x); }
float xj_cmath_ynf(int n, float x) { return (float)yn(n, (double)x); }

float xj_cmath_scalbf(float x, float exponent) {
    if (isnan(x) || isnan(exponent)) {
        return x * exponent;
    }
    if (!isfinite(exponent)) {
        return (float)scalb((double)x, (double)exponent);
    }
    // Apple's scalb accepts fractional exponents; GNU scalbf rejects them.
    if (exponent != truncf(exponent)) {
        errno = EDOM;
        feraiseexcept(FE_INVALID);
        return nanf("");
    }
    // Clamp before converting: scalbnf handles float overflow/underflow itself.
    int n = exponent >= (float)INT_MAX ? INT_MAX
          : exponent <= (float)INT_MIN ? INT_MIN : (int)exponent;
    return scalbnf(x, n);
}
