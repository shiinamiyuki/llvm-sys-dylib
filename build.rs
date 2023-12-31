use std::env;
use std::path::{Path, PathBuf};

use bindgen::Builder;

fn generate(llvm_path: &Path) {
    // let bindings = bindgen::Builder::default()
    //     // The input header we would like to generate
    //     // bindings for.
    //     .header("C:/")
    //     // Tell cargo to invalidate the built crate whenever any of the
    //     // included header files changed.
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    //     // Finish the builder and generate the bindings.
    //     .generate()
    //     // Unwrap the Result and panic on failure.
    //     .expect("Unable to generate bindings");

    // // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");
    let mut builder = bindgen::Builder::default();
    dbg!(&llvm_path);
    builder = builder
        .allowlist_function("LLVM.*")
        .dynamic_library_name("LLVM")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .clang_arg(&format!("-I{}/include", llvm_path.to_str().unwrap()))
        .clang_arg(&format!("-I{}/build/include", llvm_path.to_str().unwrap()));
    let mut dirs_to_read = vec![llvm_path.join("include/llvm-c")];
    while let Some(dir) = dirs_to_read.pop() {
        for entry in dir.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                dirs_to_read.push(path);
            } else if path.is_file() {
                builder = builder.header(path.to_str().unwrap());
            }
        }
    }

    let bindings = builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
    let bindings = std::fs::read_to_string("src/bindings.rs").unwrap();
    // replace Err: LLVMErrorRef with err: LLVMErrorRef
    let bindings = bindings
        .replace("Err: LLVMErrorRef", "err: LLVMErrorRef")
        .replace("Err)", "err)");
    std::fs::write("src/bindings.rs", bindings).unwrap();
}
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // retrieve path from LLVM_PATH environment variable
    // if not set, do not regenerate bindings
    // rerun if env var changes
    println!("cargo:rerun-if-env-changed=LLVM_PATH");
    if let Ok(llvm_path) = env::var("LLVM_PATH") {
        if llvm_path.is_empty() {
            return;
        }
        let llvm_path = Path::new(&llvm_path);
        generate(llvm_path);
    }
}
