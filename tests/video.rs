#[cfg(feature = "std")]
use infer::{Infer, MatcherType, Type};

#[cfg(feature = "std")]
fn matcher(_buf: &[u8]) -> bool {
    false
}

#[cfg(feature = "std")]
#[test]
fn test_mp4() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::VIDEO, "video/mp4", "mp4", matcher),
        info.get_from_path("testdata/sample.mp4").unwrap().unwrap()
    );
}

#[cfg(feature = "std")]
#[test]
fn test_mkv() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::VIDEO, "video/x-matroska", "mkv", matcher),
        info.get_from_path("testdata/sample.mkv").unwrap().unwrap()
    );
}

#[cfg(feature = "std")]
#[test]
fn test_webm() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::VIDEO, "video/webm", "webm", matcher),
        info.get_from_path("testdata/sample.webm").unwrap().unwrap()
    );
}

#[cfg(feature = "std")]
#[test]
fn test_mov() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::VIDEO, "video/quicktime", "mov", matcher),
        info.get_from_path("testdata/sample.mov").unwrap().unwrap()
    );
}

#[cfg(feature = "std")]
#[test]
fn test_avi() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::VIDEO, "video/x-msvideo", "avi", matcher),
        info.get_from_path("testdata/sample.avi").unwrap().unwrap()
    );
}

#[cfg(feature = "std")]
#[test]
fn test_flv() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::VIDEO, "video/x-flv", "flv", matcher),
        info.get_from_path("testdata/sample.flv").unwrap().unwrap()
    );
}
