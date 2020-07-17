#[cfg(feature = "std")]
use infer::{Infer, MatcherType, Type};

#[cfg(feature = "std")]
fn matcher(_buf: &[u8]) -> bool {
    false
}

#[cfg(feature = "std")]
#[test]
fn test_sqlite() {
    let info = Infer::new();

    assert_eq!(
        Type::new(
            MatcherType::ARCHIVE,
            "application/vnd.sqlite3",
            "sqlite",
            matcher
        ),
        info.get_from_path("testdata/sample.db").unwrap().unwrap()
    );
}

#[cfg(feature = "std")]
#[test]
fn test_zst() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::ARCHIVE, "application/zstd", "zst", matcher),
        info.get_from_path("testdata/sample.tar.zst")
            .unwrap()
            .unwrap()
    );
}
