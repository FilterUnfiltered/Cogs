use proptest::prelude::*;

proptest! {
    // Update when new tests are added
    #[test]
    fn test_cogs(test_index in 1..=1) {
        crate::init_tracing().unwrap();
        let file = std::fs::read_to_string(format!("tests/{}.cog", test_index)).unwrap();
        let ast = crate::parse_cog(file).unwrap();
        insta::with_settings!({ snapshot_suffix => format!("{test_index}") }, {
            insta::assert_debug_snapshot!(ast);
            insta::assert_snapshot!(cogs_codegen::generate(&ast).unwrap());
        });
    }
}
