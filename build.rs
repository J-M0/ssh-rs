fn main() {
    // Dynamic libssh doesn't need initialization since 0.8,
    // so we require that to keep things easy.
    // pkg_config::Config::new()
    //     .atleast_version("0.8")
    //     .statik(false)
    //     .probe("libssh")
    //     .expect("dynamically linked libssh >= 0.8 is required");
    let dst = cmake::Config::new("libssh")
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=dylib=libssh");
}
