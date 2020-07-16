use infer::{Infer, MatcherType, Type};

#[test]
fn test_jpg() {
    let info = Infer::new();

    assert_eq!(
        Type::new_for_test(MatcherType::IMAGE, "image/jpeg", "jpg"),
        info.get_from_path("testdata/sample.jpg").unwrap().unwrap()
    );
}

#[test]
fn test_png() {
    let info = Infer::new();

    assert_eq!(
        Type::new_for_test(MatcherType::IMAGE, "image/png", "png"),
        info.get_from_path("testdata/sample.png").unwrap().unwrap()
    );
}

#[test]
fn test_gif() {
    let info = Infer::new();

    assert_eq!(
        Type::new_for_test(MatcherType::IMAGE, "image/gif", "gif"),
        info.get_from_path("testdata/sample.gif").unwrap().unwrap()
    );
}

#[test]
fn test_tif() {
    let info = Infer::new();

    assert_eq!(
        Type::new_for_test(MatcherType::IMAGE, "image/tiff", "tif"),
        info.get_from_path("testdata/sample.tif").unwrap().unwrap()
    );
}

#[test]
fn test_tif2() {
    let info = Infer::new();

    assert_eq!(
        Type::new_for_test(MatcherType::IMAGE, "image/tiff", "tif"),
        info.get_from_path("testdata/sample2.tif").unwrap().unwrap()
    );
}

#[test]
fn test_tif3() {
    let info = Infer::new();

    assert_eq!(
        Type::new_for_test(MatcherType::IMAGE, "image/tiff", "tif"),
        info.get_from_path("testdata/sample3.tif").unwrap().unwrap()
    );
}

#[test]
fn test_tif4() {
    let info = Infer::new();

    assert_eq!(
        Type::new_for_test(MatcherType::IMAGE, "image/tiff", "tif"),
        info.get_from_path("testdata/sample4.tif").unwrap().unwrap()
    );
}

#[test]
fn test_tif5() {
    let info = Infer::new();

    assert_eq!(
        Type::new_for_test(MatcherType::IMAGE, "image/tiff", "tif"),
        info.get_from_path("testdata/sample5.tif").unwrap().unwrap()
    );
}

#[test]
fn test_bmp() {
    let info = Infer::new();

    assert_eq!(
        Type::new_for_test(MatcherType::IMAGE, "image/bmp", "bmp"),
        info.get_from_path("testdata/sample.bmp").unwrap().unwrap()
    );
}

#[test]
fn test_psd() {
    let info = Infer::new();

    assert_eq!(
        Type::new_for_test(MatcherType::IMAGE, "image/vnd.adobe.photoshop", "psd"),
        info.get_from_path("testdata/sample.psd").unwrap().unwrap()
    );
}

#[test]
fn test_ico() {
    let info = Infer::new();

    assert_eq!(
        Type::new_for_test(MatcherType::IMAGE, "image/vnd.microsoft.icon", "ico"),
        info.get_from_path("testdata/sample.ico").unwrap().unwrap()
    );
}

#[test]
fn test_heif() {
    let info = Infer::new();

    assert_eq!(
        Type::new_for_test(MatcherType::IMAGE, "image/heif", "heif"),
        info.get_from_path("testdata/sample.heic").unwrap().unwrap()
    );
}

#[test]
fn test_avif() {
    let info = Infer::new();

    assert_eq!(
        Type::new_for_test(MatcherType::IMAGE, "image/avif", "avif"),
        info.get_from_path("testdata/sample.avif").unwrap().unwrap()
    );
}
