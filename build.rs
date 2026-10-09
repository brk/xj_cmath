fn main() {
    println!("cargo:rerun-if-changed=src/long_double.c");
    println!("cargo:rerun-if-changed=src/macos.c");
    let mut build = cc::Build::new();
    build.file("src/long_double.c").std("c11").warnings(true);
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        build.file("src/macos.c");
        // cc defaults to the SDK's minimum, which can exceed Rust's minimum.
        // Ask rustc so explicit MACOSX_DEPLOYMENT_TARGET settings also apply.
        println!("cargo:rerun-if-env-changed=MACOSX_DEPLOYMENT_TARGET");
        let output = std::process::Command::new(std::env::var_os("RUSTC").unwrap())
            .args(["--print", "deployment-target", "--target"])
            .arg(std::env::var_os("TARGET").unwrap())
            .output()
            .expect("failed to query rustc's macOS deployment target");
        assert!(
            output.status.success(),
            "rustc deployment target query failed"
        );
        let output = String::from_utf8(output.stdout).unwrap();
        let version = output
            .trim()
            .strip_prefix("MACOSX_DEPLOYMENT_TARGET=")
            .unwrap();
        build.flag(format!("-mmacosx-version-min={version}"));
    }
    build.compile("xj_cmath_long_double");
    println!("cargo:rustc-link-lib=m");
}
