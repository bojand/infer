mod common;

#[cfg(feature = "std")]
macro_rules! test_format_get_only {
    ($exp_matchert:ident, $exp_mimet:expr, $exp_ext:expr, $format:ident, $file:expr) => {
        mod $format {
            use infer::{MatcherType, Type};

            fn matcher(_buf: &[u8]) -> bool {
                false
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

#[cfg(feature = "std")]
test_format_get_only!(Doc, "application/msword", "doc", doc, "sample.doc");

test_format!(
    Doc,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    "docx",
    docx,
    "sample.docx"
);

#[cfg(feature = "std")]
test_format_get_only!(Doc, "application/vnd.ms-excel", "xls", xls, "sample.xls");

test_format!(
    Doc,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    "xlsx",
    xlsx,
    "sample.xlsx"
);

#[cfg(feature = "std")]
test_format_get_only!(
    Doc,
    "application/vnd.ms-powerpoint",
    "ppt",
    ppt,
    "sample.ppt"
);

test_format!(
    Doc,
    "application/vnd.openxmlformats-officedocument.presentationml.presentation",
    "pptx",
    pptx,
    "sample.pptx"
);
