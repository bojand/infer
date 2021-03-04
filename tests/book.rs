mod common;

test_format!(BOOK, "application/epub+zip", "epub", epub, "sample.epub");
test_format!(
    BOOK,
    "application/x-mobipocket-ebook",
    "mobi",
    mobi,
    "sample.mobi"
);
