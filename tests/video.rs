use infer::{Infer, MatcherType, Type};

#[test]
fn test_mp4() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::VIDEO, "video/mp4", "mp4",),
        info.get_from_path("testdata/sample.mp4").unwrap().unwrap()
    );
}

#[test]
fn test_mkv() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::VIDEO, "video/x-matroska", "mkv",),
        info.get_from_path("testdata/sample.mkv").unwrap().unwrap()
    );
}

#[test]
fn test_webm() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::VIDEO, "video/webm", "webm",),
        info.get_from_path("testdata/sample.webm").unwrap().unwrap()
    );
}

#[test]
fn test_mov() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::VIDEO, "video/quicktime", "mov",),
        info.get_from_path("testdata/sample.mov").unwrap().unwrap()
    );
}

#[test]
fn test_avi() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::VIDEO, "video/x-msvideo", "avi",),
        info.get_from_path("testdata/sample.avi").unwrap().unwrap()
    );
}

#[test]
fn test_flv() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::VIDEO, "video/x-flv", "flv",),
        info.get_from_path("testdata/sample.flv").unwrap().unwrap()
    );
}
