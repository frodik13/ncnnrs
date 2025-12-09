use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let ncnn_include_dir = env::var("NCNN_INCLUDE_DIR")
        .map(|dir| PathBuf::from(dir))
        .expect(
            "ERROR: please set NCNN_INCLUDE_DIR,e.g. export NCNN_INCLUDE_DIR=/path/to/ncnn/include",
        );
    if !ncnn_include_dir.join("c_api.h").exists() {
        panic!(
            "ERROR: please set NCNN_INCLUDE_DIR,e.g2. export NCNN_INCLUDE_DIR=/path/to/ncnn/include"
        );
    }

    // println!("cargo:rerun-if-env-changed=NCNN_INCLUDE_DIR");
    let bindings = bindgen::Builder::default()
        // .header(format!("{}/gpu.h", ncnn_include_dir.display()))
        .header(format!("{}/c_api.h", ncnn_include_dir.display()))
        // .clang_arg(format!("-I{}", ncnn_include_dir.display())) 
        .clang_arg("-x")
        .clang_arg("c++")
        .allowlist_type("regex")
        .allowlist_function("ncnn.*")
        .allowlist_var("NCNN.*")
        .allowlist_type("ncnn.*")
        // .manually_drop_union(".*")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // === ПАТЧИМ ФАЙЛ ===
    let bindings_file = out_path.join("bindings.rs");
    let content = fs::read_to_string(&bindings_file)
        .expect("Failed to read bindings.rs");

    let patched = content.replace(
        "pub _M_val: _Tp,",
        "pub _M_val: ::core::mem::ManuallyDrop<_Tp>,"
    );

    fs::write(&bindings_file, patched)
        .expect("Failed to write patched bindings.rs");
}
