#[cfg(feature = "std")]
use infer::{Infer, MatcherType, Type};

#[cfg(feature = "std")]
fn matcher(_buf: &[u8]) -> bool {
    false
}

#[cfg(feature = "std")]
#[test]
fn test_ttf() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::FONT, "application/font-sfnt", "ttf", matcher),
        info.get_from_path("testdata/sample.ttf").unwrap().unwrap()
    );
}
