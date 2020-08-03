mod common;

test_format!(
    ARCHIVE,
    "application/vnd.sqlite3",
    "sqlite",
    sqlite,
    "sample.db"
);

test_format!(ARCHIVE, "application/zstd", "zst", zst, "sample.tar.zst");
