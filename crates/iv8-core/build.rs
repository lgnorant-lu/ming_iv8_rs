// build.rs for iv8-core
//
// Compiles a small C++ wrapper file (cxx/iv8_v8_extra.cc) that adds two
// ObjectTemplate methods missing from upstream rusty_v8:
//   - v8__ObjectTemplate__MarkAsUndetectable
//   - v8__ObjectTemplate__SetCallAsFunctionHandler
//
// The wrapper uses V8 headers from the v8 crate's source directory and links
// against the prebuilt libv8 static library that the v8 crate already provides.

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=cxx/iv8_v8_extra.cc");
    println!("cargo:rerun-if-changed=build.rs");

    let v8_crate_dir = locate_v8_crate_dir();
    let v8_include = v8_crate_dir.join("v8").join("include");

    if !v8_include.exists() {
        panic!(
            "iv8-core build.rs: V8 headers not found at {}.\n\
             The v8 crate source must be available at build time.\n\
             Run `cargo fetch` first if needed.",
            v8_include.display()
        );
    }

    // Compile our extra wrapper file
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .file("cxx/iv8_v8_extra.cc")
        .include(&v8_include)
        .define("V8_COMPRESS_POINTERS", None)
        .define("V8_ENABLE_SANDBOX", None)
        .std("c++20")
        .flag_if_supported("-Wno-deprecated-declarations")
        .flag_if_supported("/wd4996");

    // Windows MSVC needs additional flags
    if cfg!(target_env = "msvc") {
        build.flag("/EHsc");
        // MSVC keeps __cplusplus at 199711L for legacy compat unless this flag is set;
        // V8 headers check __cplusplus to detect C++20.
        build.flag("/Zc:__cplusplus");
    }

    build.compile("iv8_v8_extra");

    println!("cargo:rustc-link-lib=static=iv8_v8_extra");
}

/// Locate the v8 crate's source directory in the cargo registry cache.
///
/// This is fragile — cargo doesn't expose dependency source paths directly.
/// We rely on the fact that the v8 crate is in a known parent of OUT_DIR.
/// As a fallback, we walk up from OUT_DIR looking for `registry/src/.../v8-*`.
fn locate_v8_crate_dir() -> PathBuf {
    // First check explicit override
    if let Ok(custom) = env::var("IV8_V8_CRATE_DIR") {
        return PathBuf::from(custom);
    }

    // OUT_DIR is target/.../build/iv8-core-<hash>/out
    // Cargo registry: <CARGO_HOME>/registry/src/<index>/v8-<version>
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));

    // Try CARGO_HOME-based paths first (most reliable)
    let cargo_home = env::var("CARGO_HOME")
        .ok()
        .map(PathBuf::from)
        .or_else(|| dirs_home_dir().map(|h| h.join(".cargo")))
        .expect("Cannot locate CARGO_HOME");

    let registry_src = cargo_home.join("registry").join("src");
    if registry_src.exists() {
        if let Some(v8_dir) = find_v8_in_registry(&registry_src) {
            return v8_dir;
        }
    }

    // Fallback: walk up from OUT_DIR looking for registry/src
    let mut current = out_dir.as_path();
    while let Some(parent) = current.parent() {
        let candidate = parent.join("registry").join("src");
        if candidate.exists() {
            if let Some(v8_dir) = find_v8_in_registry(&candidate) {
                return v8_dir;
            }
        }
        current = parent;
    }

    panic!(
        "iv8-core build.rs: could not locate v8 crate source directory.\n\
         Set IV8_V8_CRATE_DIR environment variable to override."
    );
}

fn find_v8_in_registry(registry_src: &std::path::Path) -> Option<PathBuf> {
    // registry/src/<index-dir>/v8-<version>
    if let Ok(entries) = std::fs::read_dir(registry_src) {
        for index_entry in entries.flatten() {
            let index_path = index_entry.path();
            if !index_path.is_dir() {
                continue;
            }
            if let Ok(crate_entries) = std::fs::read_dir(&index_path) {
                for crate_entry in crate_entries.flatten() {
                    let name = crate_entry.file_name();
                    let name_str = name.to_string_lossy();
                    if name_str.starts_with("v8-") && crate_entry.path().is_dir() {
                        // Verify it has the expected layout
                        let candidate = crate_entry.path();
                        if candidate.join("v8").join("include").exists() {
                            return Some(candidate);
                        }
                    }
                }
            }
        }
    }
    None
}

fn dirs_home_dir() -> Option<PathBuf> {
    // Minimal home dir lookup without the dirs crate
    if cfg!(windows) {
        env::var_os("USERPROFILE").map(PathBuf::from)
    } else {
        env::var_os("HOME").map(PathBuf::from)
    }
}
