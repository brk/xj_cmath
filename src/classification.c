#include <math.h>

/* The numeric category values are implementation-defined. Export immutable
 * values from the target C compiler, including when cross-compiling. */
const int xj_cmath_FP_NAN = FP_NAN;
const int xj_cmath_FP_INFINITE = FP_INFINITE;
const int xj_cmath_FP_ZERO = FP_ZERO;
const int xj_cmath_FP_SUBNORMAL = FP_SUBNORMAL;
const int xj_cmath_FP_NORMAL = FP_NORMAL;
