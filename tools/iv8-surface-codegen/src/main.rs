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
        eprintln!("  --check          generate in-memory and exit 1 if differs from --output");
        eprintln!("  --diff           like --check but print first differing file paths");
        return;
    }

    let check_mode = args.contains(&"--check".to_string()) || args.contains(&"--diff".to_string());
    let show_diff = args.contains(&"--diff".to_string());

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
    eprintln!(
        "{} code {} ...",
        if check_mode {
            "Checking generated"
        } else {
            "Generating"
        },
        output_dir
    );
    let (files, install_info) = codegen::generate_all(&merged, &topo_result.sorted);

    let mut total_ifaces = 0;
    let mut domain_names: Vec<String> = Vec::new();
    let mut planned: Vec<(String, String)> = Vec::new();

    for file in &files {
        total_ifaces += file.interface_count;
        let mod_name = file.domain.replace('-', "_");
        domain_names.push(mod_name.clone());
        planned.push((format!("{}.rs", mod_name), file.content.clone()));
        if !check_mode {
            eprintln!(
                "  {}: {} interfaces -> {}.rs",
                file.domain, file.interface_count, mod_name
            );
        }
    }

    let mod_content = codegen::generate_mod_rs(&domain_names);
    planned.push(("mod.rs".into(), mod_content));

    let install_content =
        codegen::generate_install_all(&merged, &topo_result.sorted, &install_info.domain_of);
    planned.push(("install_all.rs".into(), install_content));

    if check_mode {
        let mut mismatches = 0usize;
        for (name, content) in &planned {
            let path = format!("{}/{}", output_dir, name);
            match std::fs::read_to_string(&path) {
                Ok(existing) => {
                    if existing != *content {
                        mismatches += 1;
                        if show_diff {
                            eprintln!("DIFF {}", path);
                        } else {
                            eprintln!("OUT_OF_DATE {}", path);
                        }
                    }
                }
                Err(_) => {
                    mismatches += 1;
                    eprintln!("MISSING {}", path);
                }
            }
        }
        if mismatches == 0 {
            eprintln!(
                "CHECK PASS: {} files match ({} interfaces)",
                planned.len(),
                total_ifaces
            );
            std::process::exit(0);
        } else {
            eprintln!(
                "CHECK FAIL: {} of {} generated files out of date",
                mismatches,
                planned.len()
            );
            std::process::exit(1);
        }
    }

    std::fs::create_dir_all(&output_dir).unwrap_or_else(|e| {
        eprintln!("Failed to create output dir: {}", e);
        std::process::exit(1);
    });

    for (name, content) in &planned {
        let file_path = format!("{}/{}", output_dir, name);
        std::fs::write(&file_path, content).unwrap();
    }
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
