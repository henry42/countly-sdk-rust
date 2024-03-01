use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::fs::File;

fn main() {

    let ssl_lib = pkg_config::probe_library("libssl").unwrap();
    let crypto_lib = pkg_config::probe_library("libcrypto").unwrap();
    let curl_lib = pkg_config::probe_library("libcurl").unwrap();

    for lib in vec![&ssl_lib,&crypto_lib,&curl_lib] {
        lib.link_paths.iter().for_each(|p| {
            println!("cargo:rustc-link-search={}",p.to_string_lossy());
        });

        lib.libs.iter().for_each(|f|{
            println!("cargo:rustc-link-lib={}",f);
        });
    }
    
    cc::Build::new()
        .file("countly-cpp/src/adapter.cpp")
        .file("countly-cpp/src/countly.cpp")
        .file("countly-cpp/src/crash_module.cpp")
        .file("countly-cpp/src/event.cpp")
        .file("countly-cpp/src/logger_module.cpp")
        .file("countly-cpp/src/request_builder.cpp")
        .file("countly-cpp/src/request_module.cpp")
        // .file("countly-cpp/src/storage_module_db.cpp")
        .file("countly-cpp/src/storage_module_memory.cpp")
        .file("countly-cpp/src/views_module.cpp")
        .cpp(true)
        .std("c++14")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-function")
        .flag("-Wno-deprecated-declarations")
        .flag("-Wno-format-security")
        .flag("-Wno-sign-compare")
        .include("countly-cpp/include")
        .includes(&ssl_lib.include_paths)
        .includes(&crypto_lib.include_paths)
        .includes(&curl_lib.include_paths)
        .compile("countly-cpp");

    let bindings = bindgen::Builder::default()
        
        .header("wrapper.h")

        .allowlist_function("CLY_.*")

        // .allowlist_recursively(true)

        .clang_arg("-xc++")

        .clang_arg("-std=c++14")

        .clang_arg("-I./countly-cpp/include")

        .layout_tests(false)

        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))

        .generate()
        
        .expect("Unable to generate bindings");


    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let rustc_meta = rustc_version::version_meta().expect("Unable to get rustc version meta");

    let rustc_v = format!(r#"
    pub static RUSTC_HOST: &str = "{}";
    pub static RUSTC_VERSION: &str = "{}";
    pub static PKG_VERSION: &str = "{}";
    "#,
    rustc_meta.host,
    rustc_meta.semver,
    env!("CARGO_PKG_VERSION"));

    let mut file = File::create(out_path.join("rustc_ver.rs")).expect("Unable to generate rustc v");
    file.write_all(rustc_v.as_bytes()).expect("Unable to write rustc v");

}