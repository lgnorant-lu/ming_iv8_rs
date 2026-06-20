mod codegen;
mod ea_handler;
mod ir;
mod topo;
mod type_mapper;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = get_arg(&args, "--input")
        .unwrap_or_else(|| "../../tools/idl/output/unified_ir.json".to_string());
    let output_dir = get_arg(&args, "--output")
        .unwrap_or_else(|| "../../crates/iv8-surface/src/generated".to_string());

    if args.contains(&"--stats".to_string()) {
        print_stats(&input_path);
        return;
    }
    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        eprintln!("iv8-surface-codegen v0.1.0");
        eprintln!("Usage: cargo run -p iv8-surface-codegen -- [OPTIONS]");
        eprintln!("  --input <path>   unified_ir.json path");
        eprintln!("  --output <dir>   output directory for generated Rust files");
        eprintln!("  --stats          print IR statistics");
        return;
    }

    // Load IR
    eprintln!("Loading {} ...", input_path);
    let (definitions, stats) = match ir::load_ir(&input_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    eprintln!(
        "Loaded {} definitions ({} interfaces, {} dicts, {} enums)",
        stats.definitions, stats.interfaces, stats.dictionaries, stats.enums
    );

    // Run topological sort
    let (merged, topo_result) = topo::merge_and_sort(&definitions);
    eprintln!(
        "Topo: {} sorted, {} cycles, {} merged",
        topo_result.sorted.len(),
        topo_result.cycles.len(),
        merged.len()
    );
    if !topo_result.cycles.is_empty() {
        eprintln!("WARNING: {} cycles detected!", topo_result.cycles.len());
    }

    // Generate code
    eprintln!("Generating code to {} ...", output_dir);
    let (files, install_info) = codegen::generate_all(&merged, &topo_result.sorted);

    std::fs::create_dir_all(&output_dir).unwrap_or_else(|e| {
        eprintln!("Failed to create output dir: {}", e);
        std::process::exit(1);
    });

    let mut total_ifaces = 0;
    let mut domain_names: Vec<String> = Vec::new();

    for file in &files {
        total_ifaces += file.interface_count;
        let mod_name = file.domain.replace('-', "_");
        domain_names.push(mod_name.clone());

        let file_path = format!("{}/{}.rs", output_dir, mod_name);
        std::fs::write(&file_path, &file.content).unwrap();
        eprintln!(
            "  {}: {} interfaces -> {}.rs",
            file.domain, file.interface_count, mod_name
        );
    }

    // Generate mod.rs
    let mod_content = codegen::generate_mod_rs(&domain_names);
    std::fs::write(format!("{}/mod.rs", output_dir), &mod_content).unwrap();

    // Generate install_all.rs
    let install_content =
        codegen::generate_install_all(&merged, &topo_result.sorted, &install_info.domain_of);
    std::fs::write(format!("{}/install_all.rs", output_dir), &install_content).unwrap();
    eprintln!(
        "  install_all.rs: {} interfaces in topological order",
        install_info.sorted.len()
    );
    eprintln!(
        "\nGenerated {} interfaces across {} files.",
        total_ifaces,
        files.len()
    );
    eprintln!("Done.");
}

fn print_stats(path: &str) {
    let (_, stats) = match ir::load_ir(path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    println!("Total definitions: {}", stats.definitions);
    println!("Interfaces:        {}", stats.interfaces);
    println!("Dictionaries:      {}", stats.dictionaries);
    println!("Enums:             {}", stats.enums);
    println!("Typedefs:          {}", stats.typedefs);
    println!("Callbacks:         {}", stats.callbacks);
    println!("Namespaces:        {}", stats.namespaces);
}

fn get_arg(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1).cloned())
}
