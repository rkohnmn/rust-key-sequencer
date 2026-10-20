fn main() {
    // Only hide console window in release builds on Windows
    #[cfg(all(target_os = "windows", not(debug_assertions)))]
    println!("cargo:rustc-link-arg=/SUBSYSTEM:WINDOWS");
    #[cfg(all(target_os = "windows", not(debug_assertions)))]
    println!("cargo:rustc-link-arg=/ENTRY:mainCRTStartup");
}
