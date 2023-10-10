fn main() {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 5;
    println!("cargo:rustc-env={}={}", "TEST_FOO", timestamp);
}
