fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    println!("cargo:rustc-env=BUILD_TARGET_OS={}", target_os);
    
    // Check for Termux specifically via environment variables usually present in Termux
    let is_termux = std::env::var("TERMUX_VERSION").is_ok();
    if is_termux {
        println!("cargo:rustc-cfg=is_termux");
        println!("cargo:rustc-env=BUILD_ENVIRONMENT=termux");
    } else {
        println!("cargo:rustc-env=BUILD_ENVIRONMENT=standard");
    }

    // Re-run if build script changes
    println!("cargo:rerun-if-changed=build.rs");
}
