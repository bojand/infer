mod common;

test_format!(
    Archive,
    "application/vnd.bzip3",
    "bz3",
    bz3,
    "sample.tar.bz3"
);
test_format!(
    Archive,
    "application/vnd.sqlite3",
    "sqlite",
    sqlite,
    "sample.db"
);

test_format!(Archive, "application/zstd", "zst", zst, "sample.tar.zst");
test_format!(Archive, "application/x-cpio", "cpio", cpio, "sample.cpio");
test_format!(
    Archive,
    "application/zstd",
    "zst",
    zst_skip,
    "sample.skippable.zst"
);
