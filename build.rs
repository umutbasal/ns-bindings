use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindgen::builder()
        .header("NSWorkspace.h")
        .clang_arg(format!("-F{}/System/Library/Frameworks", show_sdk_path())) // -F<directory> Add framework to the search path
        .derive_default(true)
        .derive_debug(true)
        .generate_comments(false)
		.block_extern_crate(true)
        .objc_extern_crate(true)
        .clang_arg("-ObjC")
        .blocklist_item("objc_object")
        .generate()
        .expect("Failed to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings file");

    println!("cargo:rustc-link-lib=framework=NSWorkspace")
}

/// Execute `xcrun --sdk macosx --show-sdk-path` to locate MacOS SDK
fn show_sdk_path() -> String {
    let output = Command::new("xcrun")
        .arg("--sdk")
        .arg("macosx")
        .arg("--show-sdk-path")
        .output()
        .expect("Failed to execute xcrun");

    if !output.stderr.is_empty() {
        panic!("ERROR: {}", String::from_utf8(output.stderr).unwrap());
    }

    let mut path = output.stdout;

    // Remove new line character ('\n' == 0x0A == 10)
    if path.ends_with(&[10]) {
        path.swap_remove(path.len() - 1);
    }

    String::from_utf8(path).unwrap()
}