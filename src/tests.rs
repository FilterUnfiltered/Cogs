#[test]
fn test_cogs() {
    let _ = crate::init_tracing();
    let tests = 1..=1;
    for test_index in tests {
        let name = format!("tests/{}.cog", test_index);
        let file = std::fs::read_to_string(&name).unwrap();
        let ast = crate::parse_cog(file, &name).unwrap();
        insta::with_settings!({ snapshot_suffix => format!("{test_index}") }, {
            insta::assert_debug_snapshot!(ast);
            insta::assert_snapshot!("codegen", cogs_codegen::generate(&ast).unwrap());
        });
    }
}
