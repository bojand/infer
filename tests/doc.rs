use infer::{Infer, MatcherType, Type};

fn matcher(_buf: &[u8]) -> bool {
    false
}

#[test]
fn test_doc() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::DOC, "application/msword", "doc", matcher),
        info.get_from_path("testdata/sample.doc").unwrap().unwrap()
    );
}

#[test]
fn test_docx() {
    let info = Infer::new();

    assert_eq!(
        Type::new(
            MatcherType::DOC,
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            "docx",
            matcher
        ),
        info.get_from_path("testdata/sample.docx").unwrap().unwrap()
    );
}

#[test]
fn test_xlsx() {
    let info = Infer::new();

    assert_eq!(
        Type::new(
            MatcherType::DOC,
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            "xlsx",
            matcher
        ),
        info.get_from_path("testdata/sample.xlsx").unwrap().unwrap()
    );
}

#[test]
fn test_pptx() {
    let info = Infer::new();

    assert_eq!(
        Type::new(
            MatcherType::DOC,
            "application/application/vnd.openxmlformats-officedocument.presentationml.presentation",
            "pptx",
            matcher
        ),
        info.get_from_path("testdata/sample.pptx").unwrap().unwrap()
    );
}
