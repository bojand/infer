#[macro_export]
macro_rules! test_format {
    ($exp_matchert:ident, $exp_mimet:expr, $exp_ext:expr, $format:ident, $file:expr) => {
        mod $format {
            use infer::{MatcherType, Type};

            fn matcher(_buf: &[u8]) -> bool {
                false
            }

            fn matcher_read(_r: &mut dyn std::io::Read) -> std::io::Result<bool> {
                Ok(false)
            }

            #[cfg(feature = "std")]
            #[test]
            fn get_from_path() {
                let expected_kind = Type::new(
                    MatcherType::$exp_matchert,
                    $exp_mimet,
                    $exp_ext,
                    matcher,
                    Some(matcher_read),
                );
                let kind = infer::get_from_path(concat!("testdata/", $file))
                    .expect("test file read")
                    .expect("test file matches");

                assert_eq!(expected_kind, kind);
            }

            #[test]
            fn get() {
                let expected_kind = Type::new(
                    MatcherType::$exp_matchert,
                    $exp_mimet,
                    $exp_ext,
                    matcher,
                    Some(matcher_read),
                );
                let buf = include_bytes!(concat!("../testdata/", $file));
                let kind = infer::get(buf).expect("test file matches");

                assert_eq!(expected_kind, kind);
            }

            #[test]
            fn get_read() {
                let expected_kind = Type::new(
                    MatcherType::$exp_matchert,
                    $exp_mimet,
                    $exp_ext,
                    matcher,
                    Some(matcher_read),
                );

                let mut f = std::fs::File::open(concat!("./testdata/", $file)).unwrap();
                let tp = infer::get_type_by_extension($exp_ext).unwrap();

                if tp.supports_read_match() {
                    let kind = infer::get_read(&mut f).unwrap().expect("test file matches");
                    assert_eq!(expected_kind, kind);
                }
            }
        }
    };
}
