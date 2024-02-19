use std::env;
use std::path::{Path, PathBuf};

use cmake::Config;

fn bindgen_xlink(xlink_root: &Path) {
    println!("cargo:rustc-link-search={}/lib", xlink_root.to_string_lossy());
    println!("cargo:rustc-link-lib=static=XLink");

    let bindings = bindgen::Builder::default()
        .header("src/bindgen/wrapper.h")
        .clang_arg(format!("-I{}/include", xlink_root.to_string_lossy()))
        .clang_arg(format!("-Ideps/xlink/src/pc/MacOS"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn build_xlink() -> PathBuf {
    let dst = Config::new("deps/xlink")
        .define("CMAKE_WARN_DEPRECATED", "FALSE")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("XLINK_LIBUSB_SYSTEM", "FALSE")
        .build_arg("-Wno-dev")
        .build();
    let dst_str = dst.to_str().unwrap();

    println!("cargo:rustc-link-search=native={}/lib", dst_str);
    println!("cargo:rustc-link-search=native={}", dst_str);
    println!("cargo:rustc-link-lib=static=XLink");
    println!("cargo:rustc-link-lib=dylib=usb-1.0");
    println!("cargo:rustc-link-lib=dylib=c++");
    dst
}

fn main() {
    let xlink_root = build_xlink();
    bindgen_xlink(&xlink_root);
}
