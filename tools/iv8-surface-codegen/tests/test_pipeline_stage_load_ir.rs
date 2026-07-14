use std::path::PathBuf;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/minimal_ir.json")
}

#[test]
fn load_ir_stage_accepts_minimal_fixture() {
    let path = fixture_path();
    let (defs, stats) =
        iv8_surface_codegen::ir::load_ir(&path.to_string_lossy()).expect("load_ir fixture");

    assert_eq!(stats.interfaces, 1);
    assert_eq!(stats.definitions, 1);

    let node = defs
        .iter()
        .find(|d| d.name.as_deref() == Some("FixtureNode"))
        .expect("FixtureNode definition");
    assert_eq!(node.kind, "interface");
    assert_eq!(node.members.len(), 1);
    assert_eq!(node.members[0].kind, "attribute");
    assert_eq!(node.members[0].name.as_deref(), Some("nodeName"));
    assert_eq!(node.members[0].idl_type.as_deref(), Some("DOMString"));
    assert!(node.members[0].readonly);
}
