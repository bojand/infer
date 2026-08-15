use core::convert::TryFrom;

/// Returns whether a buffer is an ePub.
#[must_use]
pub fn is_epub(buf: &[u8]) -> bool {
    crate::book::is_epub(buf)
}

/// Returns whether a buffer is a zip archive.
#[must_use]
pub fn is_zip(buf: &[u8]) -> bool {
    buf.starts_with(b"PK\x03\x04")            // Local file header
        || buf.starts_with(b"PK\x05\x06")     // End of central directory
        || buf.starts_with(b"PK\x07\x08")     // Spanned archvie
        || buf.starts_with(b"PK00PK\x03\x04") // Winzip
}

/// Returns whether a buffer is a tar archive.
#[must_use]
pub fn is_tar(buf: &[u8]) -> bool {
    buf.get(257..262) == Some(b"ustar")
}

/// Returns whether a buffer is a PAR2 archive.
#[must_use]
pub fn is_par2(buf: &[u8]) -> bool {
    buf.starts_with(b"PAR2\0PKT")
}

/// Returns whether a buffer is a RAR archive.
#[must_use]
pub fn is_rar(buf: &[u8]) -> bool {
    matches!(
        buf.first_chunk::<7>(),
        Some(&[b'R', b'a', b'r', b'!', 0x1A, 0x07, 0x00 | 0x01])
    )
}

/// Returns whether a buffer is a gzip archive.
#[must_use]
pub fn is_gz(buf: &[u8]) -> bool {
    buf.starts_with(b"\x1f\x8b\x08")
}

/// Returns whether a buffer is a bzip2 archive.
#[must_use]
pub fn is_bz2(buf: &[u8]) -> bool {
    buf.starts_with(b"BZh")
}

/// Returns whether a buffer is a bzip3 archive.
#[must_use]
pub fn is_bz3(buf: &[u8]) -> bool {
    buf.starts_with(b"BZ3v1")
}

/// Returns whether a buffer is a 7z archive.
#[must_use]
pub fn is_7z(buf: &[u8]) -> bool {
    buf.starts_with(b"7z\xbc\xaf\x27\x1c")
}

/// Returns whether a buffer is a PDF.
#[must_use]
pub fn is_pdf(buf: &[u8]) -> bool {
    // Per PDF Reference 1.4 section H.3.4.1, Acrobat viewers require only
    // that the header appear somewhere within the first 1024 bytes of the file.
    const PDF_MAGIC: &[u8] = b"%PDF";
    const SEARCH_LIMIT: usize = 1024;

    let search_len = buf.len().min(SEARCH_LIMIT);
    buf[..search_len]
        .windows(4)
        .any(|window| window == PDF_MAGIC)
}

/// Returns whether a buffer is a SWF.
#[must_use]
pub fn is_swf(buf: &[u8]) -> bool {
    matches!(buf.first_chunk::<3>(), Some(&[b'C' | b'F', b'W', b'S']))
}

/// Returns whether a buffer is an RTF.
#[must_use]
pub fn is_rtf(buf: &[u8]) -> bool {
    buf.starts_with(b"{\\rtf")
}

/// Returns whether a buffer is a Nintendo NES ROM.
#[must_use]
pub fn is_nes(buf: &[u8]) -> bool {
    buf.starts_with(b"NES\x1a")
}

/// Returns whether a buffer is Google Chrome Extension
#[must_use]
pub fn is_crx(buf: &[u8]) -> bool {
    buf.starts_with(b"Cr24")
}

/// Returns whether a buffer is a CAB.
#[must_use]
pub fn is_cab(buf: &[u8]) -> bool {
    buf.starts_with(b"MSCF") || buf.starts_with(b"ISc(")
}

/// Returns whether a buffer is a eot octet stream.
#[must_use]
pub fn is_eot(buf: &[u8]) -> bool {
    buf.get(34..36) == Some(b"LP")
        && matches!(
            buf.get(8..11),
            Some([0x02, 0x00, 0x01] | [0x01, 0x00, 0x00] | [0x02, 0x00, 0x02])
        )
}

/// Returns whether a buffer is postscript.
#[must_use]
pub fn is_ps(buf: &[u8]) -> bool {
    buf.starts_with(b"%!")
}

/// Returns whether a buffer is xz archive.
#[must_use]
pub fn is_xz(buf: &[u8]) -> bool {
    buf.starts_with(b"\xfd7zXZ\x00")
}

/// Returns whether a buffer is a sqlite3 database.
///
/// # Example
///
/// ```rust
/// use std::fs;
/// assert!(infer::archive::is_sqlite(&fs::read("testdata/sample.db").unwrap()));
/// ```
#[must_use]
pub fn is_sqlite(buf: &[u8]) -> bool {
    buf.starts_with(b"SQLi")
}

/// Returns whether a buffer is a deb archive.
#[must_use]
pub fn is_deb(buf: &[u8]) -> bool {
    buf.starts_with(b"!<arch>\ndebian-binary")
}

/// Returns whether a buffer is a ar archive.
#[must_use]
pub fn is_ar(buf: &[u8]) -> bool {
    buf.starts_with(b"!<arch>")
}

/// Returns whether a buffer is a z archive.
#[must_use]
pub fn is_z(buf: &[u8]) -> bool {
    matches!(buf.first_chunk::<2>(), Some([0x1f, 0xa0 | 0x9d]))
}

/// Returns whether a buffer is a lzip archive.
#[must_use]
pub fn is_lz(buf: &[u8]) -> bool {
    buf.starts_with(b"LZIP")
}

/// Returns whether a buffer is an RPM.
#[must_use]
pub fn is_rpm(buf: &[u8]) -> bool {
    buf.len() > 96 && buf.starts_with(b"\xed\xab\xee\xdb")
}

/// Returns whether a buffer is a dcm archive.
#[must_use]
pub fn is_dcm(buf: &[u8]) -> bool {
    buf.get(128..132) == Some(b"DICM")
}

/// Returns whether a buffer is a Zstd archive.
// Zstandard compressed data is made of one or more frames.
// There are two frame formats defined by Zstandard: Zstandard frames and Skippable frames.
// See more details from https://tools.ietf.org/id/draft-kucherawy-dispatch-zstd-00.html#rfc.section.2
#[must_use]
pub fn is_zst(buf: &[u8]) -> bool {
    starts_with_frame(buf, b"\x28\xb5\x2f\xfd")
}

/// Returns whether a buffer is a LZ4 archive.
// LZ4 compressed data is made of one or more frames.
// There are two frame formats defined by LZ4: LZ4 Frame format and Skippable frames.
// See more details from https://github.com/lz4/lz4/blob/v1.9.4/doc/lz4_Frame_format.md
#[must_use]
pub fn is_lz4(buf: &[u8]) -> bool {
    starts_with_frame(buf, b"\x04\x22\x4d\x18")
}

/// Returns whether a buffer is a MSI Windows Installer archive.
#[must_use]
pub fn is_msi(buf: &[u8]) -> bool {
    buf.starts_with(b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1")
}

/// Returns whether a buffer is a CPIO archive.
#[must_use]
pub fn is_cpio(buf: &[u8]) -> bool {
    matches!(buf.first_chunk::<2>(), Some([0xc7, 0x71] | [0x71, 0xc7])) // little/big endian, old format
        || buf.starts_with(b"070701") // nwc format
}

/// Skips leading skippable frames and reports whether a real frame with `magic` follows.
fn starts_with_frame(buf: &[u8], magic: &[u8; 4]) -> bool {
    const ZSTD_SKIP_START: u32 = 0x184D_2A50;
    const ZSTD_SKIP_MASK: u32 = 0xFFFF_FFF0;

    let mut frame = buf;
    loop {
        if frame.starts_with(magic) {
            return true;
        }

        let Some(&[m0, m1, m2, m3, l0, l1, l2, l3]) = frame.first_chunk::<8>() else {
            return false;
        };

        if u32::from_le_bytes([m0, m1, m2, m3]) & ZSTD_SKIP_MASK != ZSTD_SKIP_START {
            return false;
        }

        let Ok(data_len) = usize::try_from(u32::from_le_bytes([l0, l1, l2, l3])) else {
            return false;
        };

        let Some(rest) = data_len.checked_add(8).and_then(|end| frame.get(end..)) else {
            return false;
        };
        frame = rest;
    }
}
