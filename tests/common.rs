#[macro_export]
macro_rules! test_format {
    ($exp_matchert:expr, $exp_mimet:expr, $exp_ext:expr, $path_name:ident, $path_embed:ident, $file:expr) => {
        #[cfg(feature = "std")]
        #[test]
        fn $path_name() {
            fn matcher(_buf: &[u8]) -> bool {
                false
            }

            let expected_kind = Type::new($exp_matchert, $exp_mimet, $exp_ext, matcher);
            let kind = infer::get_from_path(concat!("testdata/", $file))
                .expect("test file read")
                .expect("test file matches");

            assert_eq!(expected_kind, kind);
        }

        #[test]
        fn $path_embed() {
            fn matcher(_buf: &[u8]) -> bool {
                false
            }

            let expected_kind = Type::new($exp_matchert, $exp_mimet, $exp_ext, matcher);
            let buf = include_bytes!(concat!("../testdata/", $file));
            let kind = infer::get(buf).expect("test file matches");

            assert_eq!(expected_kind, kind);
        }
    };
}
