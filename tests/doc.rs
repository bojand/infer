mod common;

#[cfg(feature = "std")]
macro_rules! test_format_get_only {
    ($exp_matchert:ident, $exp_mimet:expr, $exp_ext:expr, $format:ident, $file:expr) => {
        mod $format {
            use infer::{MatcherType, Type};

            fn matcher(_buf: &[u8]) -> bool {
                false
            }

            #[test]
            fn get() {
                let expected_kind =
                    Type::new(MatcherType::$exp_matchert, $exp_mimet, $exp_ext, matcher);
                let buf = include_bytes!(concat!("../testdata/", $file));
                let kind = infer::get(buf).expect("test file matches");

                assert_eq!(expected_kind, kind);
            }
        }
    };
}

#[cfg(feature = "std")]
test_format_get_only!(Doc, "application/msword", "doc", doc, "sample.doc");

test_format!(
    Doc,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    "docx",
    docx,
    "sample.docx"
);

#[cfg(feature = "std")]
test_format_get_only!(Doc, "application/vnd.ms-excel", "xls", xls, "sample.xls");

test_format!(
    Doc,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    "xlsx",
    xlsx,
    "sample.xlsx"
);

#[cfg(feature = "std")]
test_format_get_only!(
    Doc,
    "application/vnd.ms-powerpoint",
    "ppt",
    ppt,
    "sample.ppt"
);

test_format!(
    Doc,
    "application/vnd.openxmlformats-officedocument.presentationml.presentation",
    "pptx",
    pptx,
    "sample.pptx"
);

fn append_zip_entry(
    buf: &mut Vec<u8>,
    file_name: &str,
    extra_field: &[u8],
    contents: &[u8],
    flags: u16,
) {
    let mut header = [0_u8; 30];
    header[..4].copy_from_slice(b"PK\x03\x04");
    header[4..6].copy_from_slice(&20_u16.to_le_bytes());
    header[6..8].copy_from_slice(&flags.to_le_bytes());
    header[18..22].copy_from_slice(
        &u32::try_from(contents.len())
            .expect("test contents fit in a ZIP local header")
            .to_le_bytes(),
    );
    header[22..26].copy_from_slice(
        &u32::try_from(contents.len())
            .expect("test contents fit in a ZIP local header")
            .to_le_bytes(),
    );
    header[26..28].copy_from_slice(
        &u16::try_from(file_name.len())
            .expect("test file name fits in a ZIP local header")
            .to_le_bytes(),
    );
    header[28..30].copy_from_slice(
        &u16::try_from(extra_field.len())
            .expect("test extra field fits in a ZIP local header")
            .to_le_bytes(),
    );
    buf.extend_from_slice(&header);
    buf.extend_from_slice(file_name.as_bytes());
    buf.extend_from_slice(extra_field);
    buf.extend_from_slice(contents);
}

fn append_empty_zip_entry(buf: &mut Vec<u8>, file_name: &str) {
    append_zip_entry(buf, file_name, &[], &[], 0);
}

fn zip_with_entries(entries: &[&str]) -> Vec<u8> {
    let mut buf = Vec::new();
    for entry in entries {
        append_empty_zip_entry(&mut buf, entry);
    }
    buf
}

#[test]
fn detects_ooxml_type_after_metadata_entries() {
    for (main_part, expected_extension) in [
        ("word/document.xml", "docx"),
        ("xl/workbook.xml", "xlsx"),
        ("ppt/presentation.xml", "pptx"),
    ] {
        let buf = zip_with_entries(&[
            "[Content_Types].xml",
            "_rels/.rels",
            "docProps/core.xml",
            "docProps/app.xml",
            main_part,
        ]);

        let kind = infer::get(&buf).expect("reordered OOXML file matches");
        assert_eq!(kind.extension(), expected_extension);
    }
}

#[test]
fn detects_ooxml_when_unrelated_part_is_first() {
    let buf = zip_with_entries(&[
        "customXml/item.xml",
        "ppt/presentation.xml",
        "docProps/core.xml",
        "_rels/.rels",
        "[Content_Types].xml",
    ]);

    let kind = infer::get(&buf).expect("order-independent OOXML file matches");
    assert_eq!(kind.extension(), "pptx");
}

#[test]
fn preserves_legacy_namespace_pair_detection() {
    let buf = zip_with_entries(&[
        "[Content_Types].xml",
        "_rels/.rels",
        "ppt/slides/_rels/slide1.xml.rels",
        "ppt/slides/_rels/slide2.xml.rels",
    ]);

    let kind = infer::get(&buf).expect("legacy OOXML layout matches");
    assert_eq!(kind.extension(), "pptx");
}

#[test]
fn incomplete_local_header_does_not_panic() {
    let kind = infer::get(b"PK\x03\x04").expect("ZIP signature matches");
    assert_eq!(kind.extension(), "zip");
}

#[test]
fn ordinary_zip_remains_zip() {
    let buf = zip_with_entries(&["README.md", "assets/presentation.xml"]);
    let kind = infer::get(&buf).expect("ZIP signature matches");
    assert_eq!(kind.extension(), "zip");
}

#[test]
fn ooxml_metadata_with_unrelated_office_namespace_remains_zip() {
    let buf = zip_with_entries(&["[Content_Types].xml", "_rels/.rels", "ppt/assets.bin"]);
    let kind = infer::get(&buf).expect("ZIP signature matches");
    assert_eq!(kind.extension(), "zip");
}

#[test]
fn structured_scan_ignores_local_header_signature_in_payload() {
    let mut false_header = Vec::new();
    append_empty_zip_entry(&mut false_header, "word/document.xml");

    let mut buf = Vec::new();
    append_zip_entry(&mut buf, "[Content_Types].xml", &[], &false_header, 0);
    append_empty_zip_entry(&mut buf, "_rels/.rels");

    let kind = infer::get(&buf).expect("ZIP signature matches");
    assert_eq!(kind.extension(), "zip");
}

#[test]
fn structured_scan_ignores_local_header_signature_in_extra_field() {
    let mut false_header = Vec::new();
    append_empty_zip_entry(&mut false_header, "xl/workbook.xml");

    let mut buf = Vec::new();
    append_zip_entry(&mut buf, "[Content_Types].xml", &false_header, &[], 0);
    append_empty_zip_entry(&mut buf, "_rels/.rels");

    let kind = infer::get(&buf).expect("ZIP signature matches");
    assert_eq!(kind.extension(), "zip");
}

#[test]
fn data_descriptor_entry_uses_conservative_fallback() {
    let mut false_header = Vec::new();
    append_empty_zip_entry(&mut false_header, "ppt/presentation.xml");

    let mut buf = Vec::new();
    append_zip_entry(&mut buf, "[Content_Types].xml", &[], &false_header, 0x0008);
    append_empty_zip_entry(&mut buf, "_rels/.rels");

    let kind = infer::get(&buf).expect("ZIP signature matches");
    assert_eq!(kind.extension(), "zip");
}

#[test]
fn zip64_entry_uses_conservative_fallback() {
    let mut buf = zip_with_entries(&["[Content_Types].xml", "_rels/.rels", "ppt/presentation.xml"]);
    buf[18..22].copy_from_slice(&u32::MAX.to_le_bytes());

    let kind = infer::get(&buf).expect("ZIP signature matches");
    assert_eq!(kind.extension(), "zip");
}

#[test]
fn structured_scan_entry_limit_covers_the_path_read_window() {
    // A ZIP local header is at least 30 bytes, so infer's 8 KiB path read can
    // contain at most 273 complete headers. Keep the success case exactly at
    // the limit and document the conservative direct-buffer boundary after it.
    let mut within_limit = vec!["misc/part.xml"; 270];
    within_limit.extend(["[Content_Types].xml", "_rels/.rels", "ppt/presentation.xml"]);
    let kind = infer::get(&zip_with_entries(&within_limit)).expect("OOXML at limit matches");
    assert_eq!(kind.extension(), "pptx");

    let mut beyond_limit = vec!["misc/part.xml"; 271];
    beyond_limit.extend(["[Content_Types].xml", "_rels/.rels", "ppt/presentation.xml"]);
    let kind = infer::get(&zip_with_entries(&beyond_limit)).expect("ZIP signature matches");
    assert_eq!(kind.extension(), "zip");
}

#[test]
fn truncated_first_namespace_preserves_fast_path() {
    let mut buf = vec![0_u8; 30];
    buf[..4].copy_from_slice(b"PK\x03\x04");
    buf[26..28].copy_from_slice(&100_u16.to_le_bytes());
    buf.extend_from_slice(b"word/");

    let kind = infer::get(&buf).expect("legacy prefix matches");
    assert_eq!(kind.extension(), "docx");
}
