use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "android" {
        android();
    }
}

fn android() {
    println!("cargo:rustc-link-lib=c++_shared");
    let out_dir = env::var("OUT_DIR").unwrap();
    let output_path = env::var("CARGO_NDK_OUTPUT_PATH").unwrap_or(out_dir);
    let sysroot_libs_path = PathBuf::from(env::var_os("CARGO_NDK_SYSROOT_LIBS_PATH").unwrap());
    let lib_path = sysroot_libs_path.join("libc++_shared.so");
    let to_path = Path::new(&output_path).join("libc++_shared.so");
    std::fs::copy(lib_path, to_path).unwrap();
}
