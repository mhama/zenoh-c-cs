use std::{
    fs,
    path::Path,
};

// find all .rs files in a directory recursively
fn find_rust_files_recursive(dir: &Path, files: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                find_rust_files_recursive(&path, files);
            } else if let Some(extension) = path.extension() {
                if extension == "rs" {
                    if let Some(path_str) = path.to_str() {
                        files.push(path_str.to_string());
                    }
                }
            }
        }
    }
}

pub fn generate_csharp_binding() {
    let mut rust_files = Vec::new();
    let src_dir = Path::new("src");
    find_rust_files_recursive(src_dir, &mut rust_files);

    let mut builder = csbindgen::Builder::default();
    // input files other than result.rs
    for file in rust_files {
        if !file.ends_with("/result.rs") && !file.ends_with("\\result.rs") {
            builder = builder.input_extern_file(&file);
        }
    }
    builder
        .input_extern_file("csbindgen/result.rs")// special z_result_t handling
        .csharp_dll_name("libzenohc")
        .csharp_class_name("ZenohNative")
        .csharp_namespace("Zenoh.Plugins")
        .csharp_dll_name_if("UNITY_IOS && !UNITY_EDITOR", "__Internal")
        .csharp_generate_const_filter(|_|true)
        .csharp_type_rename(|rust_type_name| {
            println!("cargo:warning=type: {}", rust_type_name);
            return rust_type_name;
        })
        .generate_csharp_file("target/ZenohNative.g.cs")
        .unwrap();
}