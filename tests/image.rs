#[cfg(feature = "std")]
use infer::{Infer, MatcherType, Type};

#[cfg(feature = "std")]
fn matcher(_buf: &[u8]) -> bool {
    false
}

#[cfg(feature = "std")]
#[test]
fn test_jpg() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::IMAGE, "image/jpeg", "jpg", matcher),
        info.get_from_path("testdata/sample.jpg").unwrap().unwrap()
    );
}

#[cfg(feature = "std")]
#[test]
fn test_png() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::IMAGE, "image/png", "png", matcher),
        info.get_from_path("testdata/sample.png").unwrap().unwrap()
    );
}

#[cfg(feature = "std")]
#[test]
fn test_gif() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::IMAGE, "image/gif", "gif", matcher),
        info.get_from_path("testdata/sample.gif").unwrap().unwrap()
    );
}

#[cfg(feature = "std")]
#[test]
fn test_tif() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::IMAGE, "image/tiff", "tif", matcher),
        info.get_from_path("testdata/sample.tif").unwrap().unwrap()
    );
}

#[cfg(feature = "std")]
#[test]
fn test_tif2() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::IMAGE, "image/tiff", "tif", matcher),
        info.get_from_path("testdata/sample2.tif").unwrap().unwrap()
    );
}

#[cfg(feature = "std")]
#[test]
fn test_tif3() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::IMAGE, "image/tiff", "tif", matcher),
        info.get_from_path("testdata/sample3.tif").unwrap().unwrap()
    );
}

#[cfg(feature = "std")]
#[test]
fn test_tif4() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::IMAGE, "image/tiff", "tif", matcher),
        info.get_from_path("testdata/sample4.tif").unwrap().unwrap()
    );
}

#[cfg(feature = "std")]
#[test]
fn test_tif5() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::IMAGE, "image/tiff", "tif", matcher),
        info.get_from_path("testdata/sample5.tif").unwrap().unwrap()
    );
}

#[cfg(feature = "std")]
#[test]
fn test_bmp() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::IMAGE, "image/bmp", "bmp", matcher),
        info.get_from_path("testdata/sample.bmp").unwrap().unwrap()
    );
}

#[cfg(feature = "std")]
#[test]
fn test_psd() {
    let info = Infer::new();

    assert_eq!(
        Type::new(
            MatcherType::IMAGE,
            "image/vnd.adobe.photoshop",
            "psd",
            matcher
        ),
        info.get_from_path("testdata/sample.psd").unwrap().unwrap()
    );
}

#[cfg(feature = "std")]
#[test]
fn test_ico() {
    let info = Infer::new();

    assert_eq!(
        Type::new(
            MatcherType::IMAGE,
            "image/vnd.microsoft.icon",
            "ico",
            matcher
        ),
        info.get_from_path("testdata/sample.ico").unwrap().unwrap()
    );
}

#[cfg(feature = "std")]
#[test]
fn test_heif() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::IMAGE, "image/heif", "heif", matcher),
        info.get_from_path("testdata/sample.heic").unwrap().unwrap()
    );
}

#[cfg(feature = "std")]
#[test]
fn test_avif() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::IMAGE, "image/avif", "avif", matcher),
        info.get_from_path("testdata/sample.avif").unwrap().unwrap()
    );
}
