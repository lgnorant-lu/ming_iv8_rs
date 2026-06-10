//! iv8-surface-codegen — IDL-to-Rust code generator.
//!
//! Reads unified_ir.json (v0.8.18 output) and generates Rust source files
//! for the iv8-surface crate. Each Web IDL interface becomes a FunctionTemplate
//! factory function with stub getters/setters/methods.
//!
//! Usage: cargo run -p iv8-surface-codegen -- --input <ir> --output <dir>

fn main() {
    eprintln!("iv8-surface-codegen v0.1.0");
    eprintln!("Usage: cargo run -p iv8-surface-codegen -- --input unified_ir.json --output crates/iv8-surface/src/generated/");
    eprintln!("v0.8.19: stub — IR reader and code generator not yet implemented.");
}
