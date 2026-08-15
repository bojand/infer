/// Returns whether a buffer is WOFF font data.
#[must_use]
pub fn is_woff(buf: &[u8]) -> bool {
    buf.starts_with(b"wOFF\x00\x01\x00\x00")
}

/// Returns whether a buffer is WOFF2 font data.
#[must_use]
pub fn is_woff2(buf: &[u8]) -> bool {
    buf.starts_with(b"wOF2\x00\x01\x00\x00")
}

/// Returns whether a buffer is TTF font data.
#[must_use]
pub fn is_ttf(buf: &[u8]) -> bool {
    buf.starts_with(b"\x00\x01\x00\x00\x00")
}

/// Returns whether a buffer is OTF font data.
#[must_use]
pub fn is_otf(buf: &[u8]) -> bool {
    buf.starts_with(b"OTTO\x00")
}
