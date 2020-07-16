use infer::{Infer, MatcherType, Type};

#[test]
fn test_sqlite() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::ARCHIVE, "application/vnd.sqlite3", "sqlite",),
        info.get_from_path("testdata/sample.db").unwrap().unwrap()
    );
}

#[test]
fn test_zst() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::ARCHIVE, "application/zstd", "zst",),
        info.get_from_path("testdata/sample.tar.zst")
            .unwrap()
            .unwrap()
    );
}
