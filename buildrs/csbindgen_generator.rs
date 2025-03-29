use std::{
    fs,
    path::Path,
};

// 定数としてターゲットファイルパスを定義
const TARGET_FILE_PATH: &str = "target/ZenohNative.g.cs";

pub fn generate_csharp_binding() {
    let mut rust_files = Vec::new();
    find_rust_files_recursive(Path::new("src"), &mut rust_files);
    find_rust_files_recursive(Path::new("src/opaque_types"), &mut rust_files);
    find_rust_files_recursive(Path::new("src/closures"), &mut rust_files);
    find_rust_files_recursive(Path::new("src/platform"), &mut rust_files);

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
        .csharp_use_function_pointer(false)
        .csharp_generate_const_filter(|_|true)
        .generate_csharp_file(TARGET_FILE_PATH)
        .unwrap();
    
    replace_enum_type_in_generated_file();
}

// find all .rs files in a directory recursively
fn find_rust_files_recursive(dir: &Path, files: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                // don't process child folders
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

// fix z_consolidation_mode_t error, where uint type is generated but it has -1 value.
pub fn replace_enum_type_in_generated_file() {
    if let Ok(content) = fs::read_to_string(TARGET_FILE_PATH) {
        let updated_content = content.replace(
            "enum z_consolidation_mode_t : uint",
            "enum z_consolidation_mode_t : int",
        );
        if let Err(e) = fs::write(TARGET_FILE_PATH, updated_content) {
            eprintln!("Failed to write updated file: {}", e);
        }
    } else {
        eprintln!("Failed to read the file: {}", TARGET_FILE_PATH);
    }
}