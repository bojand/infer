use std::io::Read;
use std::io;
use super::compare_bytes;

#[derive(Debug, Eq, PartialEq)]
enum DocType {
    Text,
    Spreadsheet,
    Presentation,
}

/// Returns whether a buffer is OpenDocument Text.
pub fn is_odt(buf: &[u8]) -> bool {
    odf(buf) == Some(DocType::Text)
}

super::build_fn_read! {
    /// Returns whether data from reader is OpenDocument Text.
    (is_odt_read, is_odt, 72)
}

/// Returns whether a buffer is OpenDocument Spreadsheet.
pub fn is_ods(buf: &[u8]) -> bool {
    odf(buf) == Some(DocType::Spreadsheet)
}

super::build_fn_read! {
    /// Returns whether data from reader is OpenDocument Spreadsheet.
    (is_ods_read, is_ods, 72)
}

/// Returns whether a buffer is OpenDocument Presentation.
pub fn is_odp(buf: &[u8]) -> bool {
    odf(buf) == Some(DocType::Presentation)
}

super::build_fn_read! {
    /// Returns whether data from reader is OpenDocument Presentation.
    (is_odp_read, is_odp, 72)
}

fn odf(buf: &[u8]) -> Option<DocType> {
    let signature = [b'P', b'K', 0x03, 0x04];

    // start by checking for ZIP local file header signature
    if !compare_bytes(buf, &signature, 0) {
        return None;
    }

    // Check mimetype
    if !compare_bytes(buf, b"mimetype", 0x1E) {
        return None;
    }

    if compare_bytes(buf, b"vnd.oasis.opendocument.text", 0x32) {
        return Some(DocType::Text);
    }
    if compare_bytes(buf, b"vnd.oasis.opendocument.spreadsheet", 0x32) {
        return Some(DocType::Spreadsheet);
    }
    if compare_bytes(buf, b"vnd.oasis.opendocument.presentation", 0x32) {
        return Some(DocType::Presentation);
    }
    None
}
