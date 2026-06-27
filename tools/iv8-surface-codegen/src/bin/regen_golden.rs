use std::path::PathBuf;

fn main() {
    let ir_path = PathBuf::from("tools/idl/output/unified_ir.json");
    let golden_dir = PathBuf::from("tools/iv8-surface-codegen/tests/golden");

    let (definitions, _stats) =
        iv8_surface_codegen::ir::load_ir(&ir_path.to_string_lossy()).expect("load ir");

    let (merged, topo_result) = iv8_surface_codegen::topo::merge_and_sort(&definitions);

    let (files, install_info) =
        iv8_surface_codegen::codegen::generate_all(&merged, &topo_result.sorted);

    let mut domain_names: Vec<String> = Vec::new();
    for file in &files {
        let mod_name = file.domain.replace('-', "_");
        let path = golden_dir.join(format!("{}.rs", mod_name));
        std::fs::write(&path, &file.content).expect("write golden");
        println!("  wrote {}", path.display());
        domain_names.push(mod_name.clone());
    }

    let mod_content = iv8_surface_codegen::codegen::generate_mod_rs(&domain_names);
    std::fs::write(golden_dir.join("mod.rs"), &mod_content).expect("write mod.rs");
    println!("  wrote mod.rs");

    let install_content = iv8_surface_codegen::codegen::generate_install_all(
        &merged,
        &topo_result.sorted,
        &install_info.domain_of,
    );
    std::fs::write(golden_dir.join("install_all.rs"), &install_content).expect("write install_all.rs");
    println!("  wrote install_all.rs");
}
