use infer::{Infer, MatcherType, Type};

#[test]
fn test_mp3() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::AUDIO, "audio/mpeg", "mp3",),
        info.get_from_path("testdata/sample.mp3").unwrap().unwrap()
    );
}
