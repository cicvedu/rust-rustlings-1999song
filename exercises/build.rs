fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Skip drive3.rs exercise
    if cfg!(feature = "pass") {
        return;
    }
    if std::env::var("SKIP_DRIVE3").is_ok() {
        return;
    }



}
