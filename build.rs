fn main() {
    println!("cargo:rerun-if-changed=src/long_double.c");
    cc::Build::new()
        .file("src/long_double.c")
        .std("c11")
        .warnings(true)
        .compile("xj_cmath_long_double");
    println!("cargo:rustc-link-lib=m");
}
