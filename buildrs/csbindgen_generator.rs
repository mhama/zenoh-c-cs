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

    // src/opaque_types/mod.rs is just `include!(env!("OUT_DIR")/opaque_types.rs)`,
    // and csbindgen's static parser can't follow include!, so feed the generated
    // file directly.
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let generated_opaque_types = Path::new(&out_dir).join("opaque_types.rs");
    builder = builder.input_extern_file(
        generated_opaque_types
            .to_str()
            .expect("OUT_DIR contains non-UTF8 path"),
    );

    builder
        .input_extern_file("csbindgen/result.rs")// special z_result_t handling
        .csharp_dll_name("zenohc")
        .csharp_class_name("ZenohNative")
        .csharp_namespace("Zenoh.Plugins")
        .csharp_dll_name_if("UNITY_IOS && !UNITY_EDITOR", "__Internal")
        .csharp_use_function_pointer(false)
        .csharp_generate_const_filter(|_|true)
        .generate_csharp_file(TARGET_FILE_PATH)
        .unwrap();
    
    replace_enum_type_in_generated_file();
    strip_shm_dll_imports_if_feature_disabled();
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

// csbindgen ignores #[cfg(feature = "shared-memory")] and always emits these
// symbols. When the feature is off, drop them to avoid iOS link errors.
fn strip_shm_dll_imports_if_feature_disabled() {
    if std::env::var_os("CARGO_FEATURE_SHARED_MEMORY").is_some() {
        return;
    }

    let content = match fs::read_to_string(TARGET_FILE_PATH) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read {}: {}", TARGET_FILE_PATH, e);
            return;
        }
    };

    let lines: Vec<&str> = content.lines().collect();
    let mut out = String::with_capacity(content.len());
    let mut pending_docs: Vec<&str> = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim_start();

        if trimmed.starts_with("///") {
            pending_docs.push(line);
            i += 1;
            continue;
        }

        if trimmed.starts_with("[DllImport")
            && entry_point_contains_shm(line)
            && i + 1 < lines.len()
        {
            pending_docs.clear();
            i += 2;
            continue;
        }

        for doc in pending_docs.drain(..) {
            out.push_str(doc);
            out.push('\n');
        }
        out.push_str(line);
        out.push('\n');
        i += 1;
    }

    if let Err(e) = fs::write(TARGET_FILE_PATH, out) {
        eprintln!("Failed to write updated file: {}", e);
    }
}

fn entry_point_contains_shm(line: &str) -> bool {
    let key = "EntryPoint = \"";
    let Some(start) = line.find(key) else {
        return false;
    };
    let rest = &line[start + key.len()..];
    let Some(end) = rest.find('"') else {
        return false;
    };
    rest[..end].contains("shm")
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