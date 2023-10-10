fn main() {
    // for tests7.rs
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env={}={}", "TEST_FOO", timestamp - 3);

    // for tests8.rs
    println!("cargo:rustc-cfg=feature=\"pass\"");
    println!("cargo:rustc-cfg=test");
}
