/// Returns whether a buffer is an ePub.
pub fn is_epub(buf: &[u8]) -> bool {
    crate::book::is_epub(buf)
}

/// Returns whether a buffer is a zip archive.
pub fn is_zip(buf: &[u8]) -> bool {
    buf.len() > 3
        && buf[0] == 0x50
        && buf[1] == 0x4B
        && (((buf[2] == 0x3 && buf[3] == 0x4)
            || (buf[2] == 0x5 && buf[3] == 0x6)
            || (buf[2] == 0x7 && buf[3] == 0x8))
            || (
                // winzip
                buf[2..].starts_with(&[0x30, 0x30, 0x50, 0x4B, 0x3, 0x4])
            ))
}

/// Returns whether a buffer is a tar archive.
pub fn is_tar(buf: &[u8]) -> bool {
    buf.len() > 261 && buf.starts_with(&[0x75, 0x73, 0x74, 0x61, 0x72])
}

/// Returns whether a buffer is a RAR archive.
pub fn is_rar(buf: &[u8]) -> bool {
    buf.len() > 6
        && buf.starts_with(&[0x52, 0x61, 0x72, 0x21, 0x1A])
        && buf[5] == 0x7
        && (buf[6] == 0x0 || buf[6] == 0x1)
}

/// Returns whether a buffer is a gzip archive.
pub fn is_gz(buf: &[u8]) -> bool {
    buf.len() > 2 && buf.starts_with(&[0x1F, 0x8B]) && buf[2] == 0x8
}

/// Returns whether a buffer is a bzip archive.
pub fn is_bz2(buf: &[u8]) -> bool {
    buf.starts_with(&[0x42, 0x5A, 0x68])
}

/// Returns whether a buffer is a 7z archive.
pub fn is_7z(buf: &[u8]) -> bool {
    buf.starts_with(&[0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C])
}

/// Returns whether a buffer is a PDF.
pub fn is_pdf(buf: &[u8]) -> bool {
    buf.starts_with(&[0x25, 0x50, 0x44, 0x46])
}

/// Returns whether a buffer is a SWF.
pub fn is_swf(buf: &[u8]) -> bool {
    buf.len() > 2 && (buf[0] == 0x43 || buf[0] == 0x46) && buf[1] == 0x57 && buf[2] == 0x53
}

/// Returns whether a buffer is an RTF.
pub fn is_rtf(buf: &[u8]) -> bool {
    buf.starts_with(&[0x7B, 0x5C, 0x72, 0x74, 0x66])
}

/// Returns whether a buffer is a Nintendo NES ROM.
pub fn is_nes(buf: &[u8]) -> bool {
    buf.starts_with(&[0x4E, 0x45, 0x53, 0x1A])
}

/// Returns whether a buffer is Google Chrome Extension
pub fn is_crx(buf: &[u8]) -> bool {
    buf.starts_with(&[0x43, 0x72, 0x32, 0x34])
}

/// Returns whether a buffer is a CAB.
pub fn is_cab(buf: &[u8]) -> bool {
    buf.starts_with(&[0x4D, 0x53, 0x43, 0x46]) || buf.starts_with(&[0x49, 0x53, 0x63, 0x28])
}

/// Returns whether a buffer is a eot octet stream.
pub fn is_eot(buf: &[u8]) -> bool {
    buf.len() > 35
        && buf[34] == 0x4C
        && buf[35] == 0x50
        && ((buf[8] == 0x02 && buf[9] == 0x00 && buf[10] == 0x01)
            || (buf[8] == 0x01 && buf[9] == 0x00 && buf[10] == 0x00)
            || (buf[8] == 0x02 && buf[9] == 0x00 && buf[10] == 0x02))
}

/// Returns whether a buffer is postscript.
pub fn is_ps(buf: &[u8]) -> bool {
    buf.starts_with(&[0x25, 0x21])
}

/// Returns whether a buffer is xz archive.
pub fn is_xz(buf: &[u8]) -> bool {
    buf.starts_with(&[0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00])
}

/// Returns whether a buffer is a sqlite3 database.
///
/// # Example
///
/// ```rust
/// use std::fs;
/// assert!(infer::archive::is_sqlite(&fs::read("testdata/sample.db").unwrap()));
/// ```
pub fn is_sqlite(buf: &[u8]) -> bool {
    buf.starts_with(&[0x53, 0x51, 0x4C, 0x69])
}

/// Returns whether a buffer is a deb archive.
pub fn is_deb(buf: &[u8]) -> bool {
    buf.starts_with(&[
        0x21, 0x3C, 0x61, 0x72, 0x63, 0x68, 0x3E, 0x0A, 0x64, 0x65, 0x62, 0x69, 0x61, 0x6E, 0x2D,
        0x62, 0x69, 0x6E, 0x61, 0x72, 0x79,
    ])
}

/// Returns whether a buffer is a ar archive.
pub fn is_ar(buf: &[u8]) -> bool {
    buf.starts_with(&[0x21, 0x3C, 0x61, 0x72, 0x63, 0x68, 0x3E])
}

/// Returns whether a buffer is a z archive.
pub fn is_z(buf: &[u8]) -> bool {
    buf.len() > 1 && buf[0] == 0x1F && (buf[1] == 0xA0 || buf[1] == 0x9D)
}

/// Returns whether a buffer is a lzip archive.
pub fn is_lz(buf: &[u8]) -> bool {
    buf.starts_with(&[0x4C, 0x5A, 0x49, 0x50])
}

/// Returns whether a buffer is an RPM.
pub fn is_rpm(buf: &[u8]) -> bool {
    buf.len() > 96 && buf.starts_with(&[0xED, 0xAB, 0xEE, 0xDB])
}

/// Returns whether a buffer is a dcm archive.
pub fn is_dcm(buf: &[u8]) -> bool {
    buf.len() > 131 && buf.starts_with(&[0x44, 0x49, 0x43, 0x4D])
}

/// Returns whether a buffer is a Zstd archive.
pub fn is_zst(buf: &[u8]) -> bool {
    buf.starts_with(&[0x28, 0xB5, 0x2F, 0xFD])
}

/// Returns whether a buffer is a MSI Windows Installer archive.
pub fn is_msi(buf: &[u8]) -> bool {
    buf.starts_with(&[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1])
}

/// Returns whether a buffer is a CPIO archive.
pub fn is_cpio(buf: &[u8]) -> bool {
    (buf.len() > 1
        && ((buf[0] == 0xC7 && buf[1] == 0x71) // little endian, old format
        || (buf[0] == 0x71 && buf[1] == 0xC7))) // big endian, old format
    || buf.starts_with(&[0x30, 0x37, 0x30, 0x37, 0x30, 0x31]) // newc format
}
