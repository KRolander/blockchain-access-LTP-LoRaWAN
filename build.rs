fn main() {
    let path = "./libs";
    let lib = "fabric";
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-flags=-l framework=CoreFoundation -l framework=Security");
    }
    println!("cargo:rustc-link-search=native={}", path);
    println!("cargo:rustc-link-lib=static={}", lib);

    // TODO: Uncomment to generate Rust files from .proto file
    // use std::env;
    // use std::fs;
    // use std::path::Path;
    // use protobuf_codegen_pure::Customize;
    //
    // let out_dir = env::var("OUT_DIR").unwrap();
    // let generated_with_pure_dir = format!("{}/generated_with_pure", out_dir);
    //
    //
    // if Path::new(&generated_with_pure_dir).exists() {
    //     fs::remove_dir_all(&generated_with_pure_dir).unwrap();
    // }
    // fs::create_dir(&generated_with_pure_dir).unwrap();
    //
    // protobuf_codegen_pure::Codegen::new()
    //     .customize(Customize {
    //         gen_mod_rs: Some(true),
    //         ..Default::default()
    //     })
    //     .out_dir("src/payload")
    //     .input("resources/protobuf/payload.proto")
    //     .include("resources/protobuf")
    //     .run().unwrap();
}