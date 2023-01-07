extern crate bindgen;

fn main() {
    let dst = cmake::Config::new("gvox")
        .build_target("gvox")
        .configure_arg("-DGVOX_ENABLE_FILE_IO=true")
        .profile("Release")
        .build();

    println!("cargo:rustc-link-search=native={}/build/Release", dst.display());
    println!("cargo:rustc-link-lib=static=gvox");

    use std::env;
    use std::path::PathBuf;
    println!("cargo:rerun-if-changed=gvox/include/gvox/gvox.h");
    let bindings = bindgen::Builder::default()
        .clang_arg("--target=x86_64-pc-windows-msvc")
        .clang_arg("--language=c")
        .clang_arg("-DGVOX_ENABLE_FILE_IO=1")
        .header("gvox/include/gvox/gvox.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}