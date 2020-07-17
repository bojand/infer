use infer::{MatcherType, Type};

mod common;

test_format!(
    MatcherType::ARCHIVE,
    "application/vnd.sqlite3",
    "sqlite",
    test_sqlite,
    test_sqlite_embed,
    "sample.db"
);

test_format!(
    MatcherType::ARCHIVE,
    "application/zstd",
    "zst",
    test_zst,
    test_zst_embed,
    "sample.tar.zst"
);
