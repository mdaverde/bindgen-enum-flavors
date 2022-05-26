use bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=enum.h");

    let bindings = bindgen::Builder::default()
        .header("enum.h")
        // .default_enum_style(bindgen::EnumVariation::Rust {
        //     non_exhaustive: false,
        // })
        // .prepend_enum_name(default) // default: true
        // .translate_enum_integer_types(true)
        // .constified_enum_module("game")
        // .rustified_non_exhaustive_enum("game")
        // .rustified_enum("game")
        .constified_enum("game")
        // .newtype_enum("game")
        // .bitfield_enum("game")
        .generate()
        .expect("Unable to generate bindings");

    let src_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    bindings
        .write_to_file(src_dir.join("src/bindings.rs"))
        .expect("Couldn't write bindings!");
}
