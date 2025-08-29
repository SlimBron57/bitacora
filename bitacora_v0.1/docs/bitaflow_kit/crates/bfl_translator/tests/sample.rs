use bfl_translator::{validate_alias, md_to_bt, bt_to_md};

#[test]
fn alias_is_valid() {
    assert!(validate_alias("BITA-TPL-DOD-v1").is_ok());
}

#[test]
fn roundtrip_md_bt() {
    let md = "# Hola\nContenido.";
    let bt = md_to_bt(md, "BITA-TPL-DEMO-v1").unwrap();
    let md2 = bt_to_md(&bt).unwrap();
    assert!(md2.contains("Hola"));
}
