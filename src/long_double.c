#define _GNU_SOURCE
#include <float.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>

/* Rust stores the object representation but never interprets it. Using
 * memcpy avoids making any assumptions about the buffer's alignment.
 * The build fails rather than truncating an unsupported representation. */
_Static_assert(sizeof(long double) <= 16, "long double exceeds 16 bytes");

static void store(unsigned char *out, long double value) {
    memset(out, 0, 16);
    memcpy(out, &value, sizeof(value));
}

static long double load(const unsigned char *in) {
    long double value;
    memcpy(&value, in, sizeof(value));
    return value;
}

void xj_cmath_long_double_from_f64(double value, unsigned char *out) {
    store(out, (long double)value);
}

double xj_cmath_long_double_to_f64(const unsigned char *value) {
    return (double)load(value);
}

int xj_cmath_long_double_parse(const char *text, unsigned char *out) {
    char *end;
    long double value = strtold(text, &end);
    if (end == text || *end != '\0') {
        return 0;
    }
    store(out, value);
    return 1;
}

int xj_cmath_long_double_mant_dig(void) {
    return LDBL_MANT_DIG;
}

double xj_cmath_nexttoward(double x, const unsigned char *y) {
    return nexttoward(x, load(y));
}

float xj_cmath_nexttowardf(float x, const unsigned char *y) {
    return nexttowardf(x, load(y));
}
