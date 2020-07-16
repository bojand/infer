use infer::{Infer, MatcherType, Type};

#[test]
fn test_ttf() {
    let info = Infer::new();

    assert_eq!(
        Type::new_for_test(MatcherType::FONT, "application/font-sfnt", "ttf",),
        info.get_from_path("testdata/sample.ttf").unwrap().unwrap()
    );
}
