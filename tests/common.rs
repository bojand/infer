#[macro_export]
macro_rules! test_format {
    ($exp_matchert:ident, $exp_mimet:expr, $exp_ext:expr, $format:ident, $file:expr) => {
        mod $format {
            use infer::{MatcherType, Type};

            fn matcher(_buf: &[u8]) -> bool {
                false
            }

            #[cfg(feature = "std")]
            #[test]
            fn get_from_path() {
                let expected_kind =
                    Type::new(MatcherType::$exp_matchert, $exp_mimet, $exp_ext, matcher);
                let kind = infer::get_from_path(concat!("testdata/", $file))
                    .expect("test file read")
                    .expect("test file matches");

                assert_eq!(expected_kind, kind);
            }

            #[test]
            fn get() {
                let expected_kind =
                    Type::new(MatcherType::$exp_matchert, $exp_mimet, $exp_ext, matcher);
                let buf = include_bytes!(concat!("../testdata/", $file));
                let kind = infer::get(buf).expect("test file matches");

                assert_eq!(expected_kind, kind);
            }
        }
    };
}
