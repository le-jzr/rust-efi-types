extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .constified_enum("*")
        .ignore_functions()

        // variadic functions aren't supported
        .hide_type("EFI_INSTALL_MULTIPLE_PROTOCOL_INTERFACES")
        .hide_type("EFI_UNINSTALL_MULTIPLE_PROTOCOL_INTERFACES")

        .unstable_rust(true)
        .use_core()
        .ctypes_prefix("::ctypes")
        .clang_arg("-I ./include")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(&out_path)
        .expect("Couldn't write bindings!");

    // Bindgen puts Copy on stuff that shouldn't be. For now, just strip Copy from everything.
    // We also remove Debug because `extern "win64"` messes it up for some reason.

    let mut cont = String::new();
    File::open(&out_path).unwrap().read_to_string(&mut cont).unwrap();

    cont = cont.replace("#[derive(Debug, Copy)]", "");
    cont = cont.replace("#[derive(Copy)]", "");
    // I don't have the strength to deal with this properly.
    cont = cont.replace("fn clone(&self) -> Self { *self }", "fn clone(&self) -> Self { unimplemented!() }");

    // We also need to fix calling convention for function pointers.
    cont = cont.replace("extern \"C\"", "extern \"win64\"");



    File::create(&out_path).unwrap().write_all(cont.as_bytes()).unwrap();
}
