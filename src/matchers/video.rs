/// Returns whether a buffer is M4V video data.
#[must_use]
pub fn is_m4v(buf: &[u8]) -> bool {
    buf.get(4..11) == Some(b"ftypM4V")
}

/// Returns whether a buffer is MKV video data.
#[must_use]
pub fn is_mkv(buf: &[u8]) -> bool {
    is_ebml_doctype(buf, b"\x42\x82\x88matroska")
}

/// Returns whether a buffer is WEBM video data.
#[must_use]
pub fn is_webm(buf: &[u8]) -> bool {
    is_ebml_doctype(buf, b"\x42\x82\x84webm")
}

/// Returns whether a buffer is Quicktime MOV video data.
#[must_use]
pub fn is_mov(buf: &[u8]) -> bool {
    matches!(buf.get(4..12), Some(b"ftypqt  "))
        || matches!(buf.get(4..8), Some(b"moov" | b"mdat"))
        || buf.get(12..16) == Some(b"mdat")
}

/// Returns whether a buffer is AVI video data.
#[must_use]
pub fn is_avi(buf: &[u8]) -> bool {
    buf.starts_with(b"RIFF") && buf.get(8..11) == Some(b"AVI")
}

/// Returns whether a buffer is WMV video data.
#[must_use]
pub fn is_wmv(buf: &[u8]) -> bool {
    buf.starts_with(b"\x30\x26\xb2\x75\x8e\x66\xcf\x11\xa6\xd9")
}

/// Returns whether a buffer is MPEG video data.
#[must_use]
pub fn is_mpeg(buf: &[u8]) -> bool {
    matches!(
        buf.first_chunk::<4>(),
        Some([0x00, 0x00, 0x01, 0xb0..=0xbf])
    )
}

/// Returns whether a buffer is FLV video data.
#[must_use]
pub fn is_flv(buf: &[u8]) -> bool {
    buf.starts_with(b"FLV\x01")
}

/// Returns whether a buffer is MP4 video data.
#[must_use]
pub fn is_mp4(buf: &[u8]) -> bool {
    matches!(buf.get(4..8), Some(b"ftyp"))
        && matches!(
            buf.get(8..12),
            Some(
                b"avc1"
                    | b"dash"
                    | b"iso2"
                    | b"iso3"
                    | b"iso4"
                    | b"iso5"
                    | b"iso6"
                    | b"isom"
                    | b"mmp4"
                    | b"mp41"
                    | b"mp42"
                    | b"mp4v"
                    | b"mp71"
                    | b"MSNV"
                    | b"NDAS"
                    | b"NDSC"
                    | b"NSDC"
                    | b"NDSH"
                    | b"NDSM"
                    | b"NDSP"
                    | b"NDSS"
                    | b"NDXC"
                    | b"NDXH"
                    | b"NDXM"
                    | b"NDXP"
                    | b"NDXS"
                    | b"F4V "
                    | b"F4P "
            )
        )
}

fn is_ebml_doctype(buf: &[u8], doctype: &[u8]) -> bool {
    const EBML_MAGIC: &[u8; 4] = b"\x1a\x45\xdf\xa3";
    const EBML_SEARCH_LIMIT: usize = 256;

    buf.starts_with(EBML_MAGIC)
        && crate::match_bytes(&buf[..buf.len().min(EBML_SEARCH_LIMIT)], doctype)
}
