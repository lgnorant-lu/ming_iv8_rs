mod ir;
mod topo;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = get_arg(&args, "--input")
        .unwrap_or_else(|| "../../tools/idl/output/unified_ir.json".to_string());
    let output_dir = get_arg(&args, "--output");

    if args.contains(&"--stats".to_string()) || args.contains(&"--validate".to_string()) {
        print_stats(&input_path);
        return;
    }
    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        eprintln!("iv8-surface-codegen v0.1.0");
        eprintln!("Usage: cargo run -p iv8-surface-codegen -- [OPTIONS]");
        eprintln!("  --input <path>   unified_ir.json path");
        eprintln!("  --output <dir>   output directory");
        eprintln!("  --stats          print IR statistics");
        eprintln!("  --topo           run topological sort");
        return;
    }

    eprintln!("Loading {} ...", input_path);
    let (definitions, stats) = match ir::load_ir(&input_path) {
        Ok(v) => v,
        Err(e) => { eprintln!("Error: {}", e); std::process::exit(1); }
    };

    eprintln!("Loaded {} definitions: {} interfaces, {} dicts, {} enums, {} typedefs, {} callbacks, {} namespaces",
        stats.definitions, stats.interfaces, stats.dictionaries,
        stats.enums, stats.typedefs, stats.callbacks, stats.namespaces);

    if args.contains(&"--topo".to_string()) {
        let (merged, topo_result) = topo::merge_and_sort(&definitions);
        eprintln!("Topo: {} sorted, {} cycles, {} missing parents",
            topo_result.sorted.len(), topo_result.cycles.len(), topo_result.missing_parents.len());
        if !topo_result.cycles.is_empty() {
            eprintln!("Cycles: {:?}", topo_result.cycles);
        }
        eprintln!("Merged: {} definitions", merged.len());

        let mut domain_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for def in &merged {
            if let Some(name) = &def.name {
                *domain_counts.entry(topo::classify_domain(name).to_string()).or_insert(0) += 1;
            }
        }
        eprintln!("Domain distribution:");
        let mut domains: Vec<_> = domain_counts.iter().collect();
        domains.sort_by_key(|(_, c)| std::cmp::Reverse(**c));
        for (domain, count) in domains {
            eprintln!("  {}: {}", domain, count);
        }
    }

    if let Some(out_dir) = output_dir {
        eprintln!("Code generation not yet implemented. Output to: {}", out_dir);
    }

    eprintln!("Done.");
}

fn print_stats(path: &str) {
    let (_, stats) = match ir::load_ir(path) {
        Ok(v) => v,
        Err(e) => { eprintln!("Error: {}", e); return; }
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
    args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1).cloned())
}
