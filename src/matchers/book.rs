/// Returns whether a buffer is an ePub.
#[must_use]
pub fn is_epub(buf: &[u8]) -> bool {
    buf.starts_with(b"PK\x03\x04") && buf.get(30..58) == Some(b"mimetypeapplication/epub+zip")
}

/// Returns whether a buffer is a mobi.
#[must_use]
pub fn is_mobi(buf: &[u8]) -> bool {
    buf.get(60..68) == Some(b"BOOKMOBI")
}
