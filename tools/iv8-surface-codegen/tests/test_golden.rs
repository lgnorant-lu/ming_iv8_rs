use std::path::PathBuf;

fn ir_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../idl/output/unified_ir.json")
}

fn golden_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/golden")
}

#[test]
fn golden_output_matches_committed() {
    let (definitions, _stats) =
        iv8_surface_codegen::ir::load_ir(&ir_path().to_string_lossy()).expect("load ir");

    let (merged, topo_result) = iv8_surface_codegen::topo::merge_and_sort(&definitions);

    let (files, install_info) =
        iv8_surface_codegen::codegen::generate_all(&merged, &topo_result.sorted);

    let mut domain_names: Vec<String> = Vec::new();
    for file in &files {
        let mod_name = file.domain.replace('-', "_");
        let expected_path = golden_dir().join(format!("{}.rs", mod_name));
        let expected = std::fs::read_to_string(&expected_path).unwrap_or_else(|e| {
            panic!("golden file missing: {}: {}", expected_path.display(), e);
        });
        if file.content != expected {
            eprintln!("MISMATCH in {}", file.domain);
            eprintln!("  generated: {} lines", file.content.lines().count());
            eprintln!("  golden:    {} lines", expected.lines().count());
            // Show first differing line
            for (i, (g, e)) in file.content.lines().zip(expected.lines()).enumerate() {
                if g != e {
                    eprintln!("  first diff at line {}", i + 1);
                    eprintln!("  generated: {}", g);
                    eprintln!("  golden:    {}", e);
                    break;
                }
            }
            panic!(
                "generated output differs from golden for domain: {}",
                file.domain
            );
        }
        domain_names.push(mod_name.clone());
    }

    let mod_content = iv8_surface_codegen::codegen::generate_mod_rs(&domain_names);
    let mod_path = golden_dir().join("mod.rs");
    let mod_expected = std::fs::read_to_string(&mod_path).unwrap_or_else(|e| {
        panic!("golden mod.rs missing: {}: {}", mod_path.display(), e);
    });
    assert_eq!(
        mod_content, mod_expected,
        "generated mod.rs differs from golden"
    );

    let install_content = iv8_surface_codegen::codegen::generate_install_all(
        &merged,
        &topo_result.sorted,
        &install_info.domain_of,
    );
    let install_path = golden_dir().join("install_all.rs");
    let install_expected = std::fs::read_to_string(&install_path).unwrap_or_else(|e| {
        panic!("golden install_all.rs missing: {}: {}", install_path.display(), e);
    });
    assert_eq!(
        install_content, install_expected,
        "generated install_all.rs differs from golden"
    );
}
