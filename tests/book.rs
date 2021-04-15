mod common;

test_format!(Book, "application/epub+zip", "epub", epub, "sample.epub");
test_format!(
    Book,
    "application/x-mobipocket-ebook",
    "mobi",
    mobi,
    "sample.mobi"
);
