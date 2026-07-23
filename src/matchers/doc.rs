use core::convert::TryInto;

use super::compare_bytes;

// ZIP local file header fields used by the structured OOXML scan. Unlike the
// legacy detector (which searches for the next `PK\x03\x04` in a byte window),
// this path advances by parsing each header's lengths and compressed size.
const ZIP_LOCAL_FILE_HEADER: &[u8] = b"PK\x03\x04";
const ZIP_LOCAL_FILE_HEADER_LEN: usize = 30;
const ZIP_FLAGS_OFFSET: usize = 6;
const ZIP_COMPRESSED_SIZE_OFFSET: usize = 18;
const ZIP_FILE_NAME_LENGTH_OFFSET: usize = 26;
const ZIP_EXTRA_FIELD_LENGTH_OFFSET: usize = 28;
// When set, the real sizes follow the file data in a data descriptor, so the
// compressed-size field in the local header cannot be trusted for seeking.
const ZIP_DATA_DESCRIPTOR_FLAG: u16 = 0x0008;
// ZIP64 stores the real size in the extra field; `u32::MAX` is only a marker.
const ZIP64_SIZE: u32 = u32::MAX;
// `get_from_path` reads at most 8 KiB. Deriving the iteration bound from the
// smallest possible local header covers every header parseable in that window
// while keeping direct buffer matching bounded as well.
const MAX_OOXML_ENTRIES: usize = 8192 / ZIP_LOCAL_FILE_HEADER_LEN;

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DocType {
    DOC,
    DOCX,
    XLS,
    XLSX,
    PPT,
    PPTX,
    OOXML,
}

// One ZIP local file header: the entry name used for classification, and the
// absolute offset of the next header when it can be computed safely.
struct LocalFileHeader<'a> {
    file_name: &'a [u8],
    next_offset: Option<usize>,
}

/// Returns whether a buffer is Microsoft Word Document (DOC) data.
#[must_use]
pub fn is_doc(buf: &[u8]) -> bool {
    ole2(buf) == Some(DocType::DOC)
}

/// Returns whether a buffer is Microsoft Word Open XML Format Document (DOCX) data.
#[must_use]
pub fn is_docx(buf: &[u8]) -> bool {
    msooxml(buf) == Some(DocType::DOCX)
}

/// Returns whether a buffer is Microsoft Excel 97-2003 Worksheet (XLS) data.
#[must_use]
pub fn is_xls(buf: &[u8]) -> bool {
    ole2(buf) == Some(DocType::XLS)
}

/// Returns whether a buffer is Microsoft Excel Open XML Format Spreadsheet (XLSX) data.
#[must_use]
pub fn is_xlsx(buf: &[u8]) -> bool {
    msooxml(buf) == Some(DocType::XLSX)
}

/// Returns whether a buffer is Microsoft PowerPoint 97-2003 Presentation (PPT) data.
#[must_use]
pub fn is_ppt(buf: &[u8]) -> bool {
    ole2(buf) == Some(DocType::PPT)
}

/// Returns whether a buffer is Microsoft `PowerPoint` Open XML Presentation (PPTX) data.
#[must_use]
pub fn is_pptx(buf: &[u8]) -> bool {
    msooxml(buf) == Some(DocType::PPTX)
}

fn msooxml(buf: &[u8]) -> Option<DocType> {
    // Preserve every positive result from the original positional heuristic.
    // The structured scan below only adds conservative matches for layouts the
    // legacy detector could not classify (e.g. metadata entries before the
    // office namespace, or a main part that is not among the first few files).
    let legacy_type = legacy_msooxml(buf);
    if let Some(doc_type) = legacy_type {
        // Concrete types win immediately; a bare `OOXML` result is kept so the
        // structured scan can still try to refine it.
        if doc_type != DocType::OOXML {
            return Some(doc_type);
        }
    }

    // Walk ZIP local headers by parsing each entry instead of scanning for the
    // next `PK` signature. This is order-independent and ignores `PK\x03\x04`
    // sequences that happen to appear inside file data or extra fields.
    let mut header_offset = 0;
    let mut saw_content_types = false;
    let mut saw_package_relationships = false;
    let mut main_part_type = None;
    let mut ambiguous_type = false;

    for _ in 0..MAX_OOXML_ENTRIES {
        let header = match local_file_header(buf, header_offset) {
            Some(header) => header,
            None => break,
        };

        // Package-level OPC files required by every OOXML document.
        if header.file_name == b"[Content_Types].xml" {
            saw_content_types = true;
        } else if header.file_name == b"_rels/.rels" {
            saw_package_relationships = true;
        }

        // Record the first canonical main part; conflicting main parts make the
        // archive ambiguous (e.g. both word/ and xl/ present).
        if let Some(doc_type) = check_msooxml_main_part(header.file_name) {
            match main_part_type {
                None => main_part_type = Some(doc_type),
                Some(current) if current == doc_type => {}
                Some(_) => ambiguous_type = true,
            }
        }

        // Stop when the next header cannot be located safely (short buffer,
        // data descriptor, or ZIP64 sizes).
        header_offset = match header.next_offset {
            Some(next_offset) => next_offset,
            None => break,
        };
    }

    // The package-level files and canonical main part are independent of
    // physical ZIP entry order and together provide a conservative OOXML
    // signal for archives that the legacy detector could not classify.
    if !ambiguous_type && saw_content_types && saw_package_relationships && main_part_type.is_some()
    {
        return main_part_type;
    }

    // Fall back to a generic OOXML hit when we saw package markers (or legacy
    // already did) but could not pin down a single concrete office type.
    if legacy_type.is_some() || saw_content_types || saw_package_relationships {
        Some(DocType::OOXML)
    } else {
        None
    }
}

fn legacy_msooxml(buf: &[u8]) -> Option<DocType> {
    let signature = [b'P', b'K', 0x03, 0x04];

    // start by checking for ZIP local file header signature
    if !compare_bytes(buf, &signature, 0) {
        return None;
    }

    let v = check_msooml(buf, 0x1E);
    if v.is_some() {
        return v;
    }

    if !compare_bytes(buf, b"[Content_Types].xml", 0x1E)
        && !compare_bytes(buf, b"_rels/.rels", 0x1E)
        && !compare_bytes(buf, b"docProps", 0x1E)
    {
        return None;
    }

    // skip to the second local file header
    // since some documents include a 520-byte extra field following the file
    // header, we need to scan for the next header
    let mut start_offset =
        u32::from_le_bytes(buf[18..22].try_into().unwrap()).checked_add(49)? as usize;

    let idx = search(buf, start_offset, 6000)?;

    // now skip to the *third* local file header; again, we need to scan due to a
    // 520-byte extra field following the file header
    start_offset += idx + 4 + 26;
    let idx = search(buf, start_offset, 6000)?;

    // and check the subdirectory name to determine which type of OOXML
    // file we have.  Correct the mimetype with the registered ones:
    // http://technet.microsoft.com/en-us/library/cc179224.aspx
    start_offset += idx + 4 + 26;
    check_msooml(buf, start_offset)?;

    // OpenOffice/Libreoffice orders ZIP entry differently, so check the 4th file
    start_offset += 26;
    let idx = search(buf, start_offset, 6000);
    match idx {
        Some(idx) => start_offset += idx + 4 + 26,
        None => return Some(DocType::OOXML),
    }

    let typo = check_msooml(buf, start_offset);
    if typo.is_some() {
        return typo;
    }

    Some(DocType::OOXML)
}

#[cfg(feature = "std")]
fn ole2(buf: &[u8]) -> Option<DocType> {
    use std::io::Cursor;

    if !compare_bytes(buf, &[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1], 0) {
        return None;
    }
    if let Ok(file) = cfb::CompoundFile::open(Cursor::new(buf)) {
        return match file.root_entry().clsid().to_string().as_str() {
            "00020810-0000-0000-c000-000000000046" | "00020820-0000-0000-c000-000000000046" => {
                Some(DocType::XLS)
            }
            "00020906-0000-0000-c000-000000000046" => Some(DocType::DOC),
            "64818d10-4f9b-11cf-86ea-00aa00b929e8" => Some(DocType::PPT),
            _ => None,
        };
    }
    None
}

#[cfg(not(feature = "std"))]
fn ole2(buf: &[u8]) -> Option<DocType> {
    if !compare_bytes(buf, &[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1], 0) {
        return None;
    }
    Some(DocType::DOC)
}

fn check_msooml(buf: &[u8], offset: usize) -> Option<DocType> {
    if compare_bytes(buf, b"word/", offset) {
        Some(DocType::DOCX)
    } else if compare_bytes(buf, b"ppt/", offset) {
        Some(DocType::PPTX)
    } else if compare_bytes(buf, b"xl/", offset) {
        Some(DocType::XLSX)
    } else {
        None
    }
}

fn search(buf: &[u8], start: usize, range: usize) -> Option<usize> {
    let length = buf.len();
    let mut end = start + range;
    let signature: &[_] = &[b'P', b'K', 0x03, 0x04];

    if end > length {
        end = length;
    }

    if start >= end {
        return None;
    }

    buf[start..end]
        .windows(signature.len())
        .position(|window| window == signature)
}

// Exact main-part names from the OOXML / OPC packages. Prefer these over a
// bare `word/` / `ppt/` / `xl/` prefix so ordinary ZIPs that happen to contain
// similarly named paths are not classified as Office documents.
fn check_msooxml_main_part(file_name: &[u8]) -> Option<DocType> {
    match file_name {
        b"word/document.xml" => Some(DocType::DOCX),
        b"ppt/presentation.xml" => Some(DocType::PPTX),
        b"xl/workbook.xml" => Some(DocType::XLSX),
        _ => None,
    }
}

// Parse one ZIP local file header at `header_offset`.
//
// Layout: signature (4) | ... | flags (2 @ +6) | ... | compressed size (4 @ +18)
//       | ... | file name length (2 @ +26) | extra length (2 @ +28)
//       | file name | extra field | file data
fn local_file_header(buf: &[u8], header_offset: usize) -> Option<LocalFileHeader<'_>> {
    if !compare_bytes(buf, ZIP_LOCAL_FILE_HEADER, header_offset) {
        return None;
    }

    let flags = read_u16(buf, header_offset.checked_add(ZIP_FLAGS_OFFSET)?)?;
    let compressed_size = read_u32(buf, header_offset.checked_add(ZIP_COMPRESSED_SIZE_OFFSET)?)?;
    let name_length_offset = header_offset.checked_add(ZIP_FILE_NAME_LENGTH_OFFSET)?;
    let name_length = usize::from(read_u16(buf, name_length_offset)?);
    let extra_length_offset = header_offset.checked_add(ZIP_EXTRA_FIELD_LENGTH_OFFSET)?;
    let extra_length = usize::from(read_u16(buf, extra_length_offset)?);
    let name_start = header_offset.checked_add(ZIP_LOCAL_FILE_HEADER_LEN)?;
    let name_end = name_start.checked_add(name_length)?;
    let file_name = buf.get(name_start..name_end)?;
    let data_start = name_end.checked_add(extra_length)?;

    // Only compute the next entry offset when the local header's compressed
    // size is authoritative. Otherwise refuse to seek rather than guess.
    let next_offset = if flags & ZIP_DATA_DESCRIPTOR_FLAG != 0 || compressed_size == ZIP64_SIZE {
        None
    } else {
        data_start.checked_add(compressed_size as usize)
    };

    Some(LocalFileHeader {
        file_name,
        next_offset,
    })
}

fn read_u16(buf: &[u8], offset: usize) -> Option<u16> {
    let end = offset.checked_add(2)?;
    Some(u16::from_le_bytes(buf.get(offset..end)?.try_into().ok()?))
}

fn read_u32(buf: &[u8], offset: usize) -> Option<u32> {
    let end = offset.checked_add(4)?;
    Some(u32::from_le_bytes(buf.get(offset..end)?.try_into().ok()?))
}
