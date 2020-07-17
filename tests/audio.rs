#[cfg(feature = "std")]
use infer::{Infer, MatcherType, Type};

#[cfg(feature = "std")]
fn matcher(_buf: &[u8]) -> bool {
    false
}

#[cfg(feature = "std")]
#[test]
fn test_mp3() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::AUDIO, "audio/mpeg", "mp3", matcher),
        info.get_from_path("testdata/sample.mp3").unwrap().unwrap()
    );
}
