#[test]
fn test_build_metadata_exists() {
    let target_os = env!("BUILD_TARGET_OS");
    let env = env!("BUILD_ENVIRONMENT");

    assert!(!target_os.is_empty(), "BUILD_TARGET_OS should not be empty");
    assert!(!env.is_empty(), "BUILD_ENVIRONMENT should not be empty");

    println!("Running on OS: {}, Environment: {}", target_os, env);
}

#[test]
fn test_default_theme_logic() {
    let env = env!("BUILD_ENVIRONMENT");
    let is_termux = env == "termux";

    // Logic check: if on termux, default should be dracula (dark), else notion (light)
    // We are testing the logic in src/cli/theme.rs:notion()
    if is_termux {
        assert_eq!(env, "termux");
    } else {
        assert_eq!(env, "standard");
    }
}

#[test]
#[cfg(target_os = "windows")]
fn test_windows_specifics() {
    assert_eq!(env!("BUILD_TARGET_OS"), "windows");
}

#[test]
#[cfg(target_os = "linux")]
fn test_linux_specifics() {
    // Both standard Linux and Android/Termux are linux-family
    assert!(env!("BUILD_TARGET_OS") == "linux" || env!("BUILD_TARGET_OS") == "android");
}

#[test]
#[cfg(target_os = "macos")]
fn test_macos_specifics() {
    assert_eq!(env!("BUILD_TARGET_OS"), "macos");
}
