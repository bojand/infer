/// Returns whether a buffer is html data.
///
/// Conforms to [whatwg](https://mimesniff.spec.whatwg.org/)
/// specification.
pub fn is_html(buf: &[u8]) -> bool {
    let values: &[&[u8]] = &[
        b"<!DOCTYPE HTML",
        b"<HTML",
        b"<HEAD",
        b"<SCRIPT",
        b"<IFRAME",
        b"<H1",
        b"<DIV",
        b"<FONT",
        b"<TABLE",
        b"<A",
        b"<STYLE",
        b"<TITLE",
        b"<B",
        b"<BODY",
        b"<BR",
        b"<P",
        b"<!--",
    ];
    let buf = trim_start_whitespaces(buf);

    for val in values {
        if buf.len() <= val.len() {
            continue;
        }
        let b = &buf[..val.len()];
        if b.eq_ignore_ascii_case(val) {
            match buf[val.len()] {
                // tag-terminitating byte
                0x20 | 0x3E => return true,
                _ => continue,
            }
        }
    }

    false
}

/// Returns whether a buffer is xml data.
///
/// Conforms to [whatwg](https://mimesniff.spec.whatwg.org/)
/// specification.
pub fn is_xml(buf: &[u8]) -> bool {
    let val: &[u8] = b"<?xml";
    let buf = trim_start_whitespaces(buf);
    if buf.len() <= val.len() {
        return false;
    }
    let b = &buf[..val.len()];
    b.eq_ignore_ascii_case(val)
}

/// Strip whitespaces at the beginning of the buffer.
///
/// Follows https://mimesniff.spec.whatwg.org
/// definition of whitespace.
fn trim_start_whitespaces(buf: &[u8]) -> &[u8] {
    for (i, b) in buf.iter().enumerate() {
        match b {
            0x09 | 0x0A | 0x0C | 0x0D | 0x20 => continue,
            _ => return &buf[i..],
        }
    }
    &[]
}

#[cfg(test)]
mod tests {
    use super::{is_html, trim_start_whitespaces};

    #[test]
    fn trim_whitespaces() {
        let got = trim_start_whitespaces(&[0x09, 0x0A, 0x0C, 0x0D, 0x20, b'A', b'B', b'C']);
        assert_eq!(got, b"ABC");

        let got = trim_start_whitespaces(b"abc");
        assert_eq!(got, b"abc");

        let got = trim_start_whitespaces(&[]);
        assert_eq!(got, &[]);
    }

    #[test]
    fn html() {
        assert_eq!(is_html(b"<"), false);
        assert_eq!(is_html(b"<HTML"), false);
        assert_eq!(is_html(b"<HTML "), true);
        assert_eq!(is_html(b"   <BODY>"), true);
    }
}