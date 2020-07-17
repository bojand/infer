use infer::{Infer, MatcherType, Type};

fn matcher(_buf: &[u8]) -> bool {
    false
}

#[test]
fn test_mp3() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::AUDIO, "audio/mpeg", "mp3", matcher),
        info.get_from_path("testdata/sample.mp3").unwrap().unwrap()
    );
}
