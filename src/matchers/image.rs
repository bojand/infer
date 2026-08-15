use core::convert::TryInto;

/// Returns whether a buffer is JPEG image data.
#[must_use]
pub fn is_jpeg(buf: &[u8]) -> bool {
    buf.starts_with(b"\xff\xd8\xff")
}

/// Returns whether a buffer is jpg2 image data.
#[must_use]
pub fn is_jpeg2000(buf: &[u8]) -> bool {
    buf.starts_with(b"\x00\x00\x00\x0cjP  \x0d\x0a\x87\x0a\x00")
}

/// Returns whether a buffer is PNG image data.
#[must_use]
pub fn is_png(buf: &[u8]) -> bool {
    buf.starts_with(b"\x89PNG")
}

/// Returns whether a buffer is GIF image data.
#[must_use]
pub fn is_gif(buf: &[u8]) -> bool {
    buf.starts_with(b"GIF")
}

/// Returns whether a buffer is WEBP image data.
#[must_use]
pub fn is_webp(buf: &[u8]) -> bool {
    buf.starts_with(b"RIFF") && buf.get(8..12) == Some(b"WEBP")
}

/// Returns whether a buffer is Canon CR2 image data.
#[must_use]
pub fn is_cr2(buf: &[u8]) -> bool {
    is_tiff_header(buf) && buf.get(8..11) == Some(b"CR\x02") // CR2 major version
}

/// Returns whether a buffer is TIFF image data.
#[must_use]
pub fn is_tiff(buf: &[u8]) -> bool {
    is_tiff_header(buf) && !is_cr2(buf) // To avoid conflicts differentiate Tiff from CR2
}

/// Returns whether a buffer is BMP image data.
#[must_use]
pub fn is_bmp(buf: &[u8]) -> bool {
    buf.starts_with(b"BM")
}

/// Returns whether a buffer is jxr image data.
#[must_use]
pub fn is_jxr(buf: &[u8]) -> bool {
    buf.starts_with(b"II\xbc")
}

/// Returns whether a buffer is Photoshop PSD image data.
#[must_use]
pub fn is_psd(buf: &[u8]) -> bool {
    buf.starts_with(b"8BPS")
}

/// Returns whether a buffer is ICO icon image data.
#[must_use]
pub fn is_ico(buf: &[u8]) -> bool {
    buf.starts_with(b"\x00\x00\x01\x00")
}

/// Returns whether a buffer is JPEG XL (JXL) image data.
#[must_use]
pub fn is_jxl(buf: &[u8]) -> bool {
    buf.starts_with(b"\xff\x0a") || buf.starts_with(b"\x00\x00\x00\x0cJXL \x0d\x0a\x87\x0a")
}

/// Returns whether a buffer is HEIF image data.
#[must_use]
pub fn is_heif(buf: &[u8]) -> bool {
    if !is_isobmff(buf) {
        return false;
    }

    if let Some((major, _minor, compatible)) = get_ftyp(buf) {
        if major == b"heic" || major == b"heix" {
            return true;
        }

        if major == b"mif1" || major == b"msf1" && compatible.into_iter().any(|b| b == b"heic") {
            return true;
        }
    }

    false
}

/// Returns whether a buffer is AVIF image data.
#[must_use]
pub fn is_avif(buf: &[u8]) -> bool {
    if !is_isobmff(buf) {
        return false;
    }

    if let Some((major, _minor, compatible)) = get_ftyp(buf) {
        if major == b"avif" || major == b"avis" {
            return true;
        }

        if compatible
            .into_iter()
            .any(|b| matches!(b, b"avif" | b"avis"))
        {
            return true;
        }
    }

    false
}

// IsISOBMFF checks whether the given buffer represents ISO Base Media File Format data
fn is_isobmff(buf: &[u8]) -> bool {
    if buf.len() < 16 {
        return false;
    }

    if &buf[4..8] != b"ftyp" {
        return false;
    }

    let ftyp_length = u32::from_be_bytes(buf[0..4].try_into().unwrap()) as usize;
    buf.len() >= ftyp_length
}

/// Returns whether a buffer is `ORA` image data.
#[must_use]
pub fn is_ora(buf: &[u8]) -> bool {
    buf.starts_with(b"PK\x03\x04") && buf.get(30..54) == Some(b"mimetypeimage/openraster")
}

/// Returns whether a buffer is `DjVu` image data.
#[must_use]
pub fn is_djvu(buf: &[u8]) -> bool {
    buf.starts_with(b"AT&TFORM") && buf.get(12..15) == Some(b"DJV")
}

/// Returns whether a buffer is an AutoCAD Drawing (DWG).
#[must_use]
pub fn is_dwg(buf: &[u8]) -> bool {
    let Some(magic) = buf.first_chunk::<6>() else {
        return false;
    };

    matches!(
        magic,
        b"MC0.0\0"
            | b"AC1.2\0"
            | b"AC1.3\0"
            | b"AC1.40"
            | b"AC1.50"
            | b"AC2.10"
            | b"AC2.21"
            | b"AC2.22"
            | b"AC1001"
            | b"AC1002"
            | b"AC1003"
            | b"AC1004"
            | b"AC1006"
            | b"AC1009"
            | b"AC1012"
            | b"AC1013"
            | b"AC1014"
            | b"AC1015"
            | b"AC1018"
            | b"AC1021"
            | b"AC1024"
            | b"AC1027"
            | b"AC1032"
            | b"AC1035"
    )
}

fn is_tiff_header(buf: &[u8]) -> bool {
    const TIFF_LE: &[u8; 4] = b"II\x2a\x00";
    const TIFF_BE: &[u8; 4] = b"MM\x00\x2a";
    matches!(buf.first_chunk::<4>(), Some(TIFF_LE | TIFF_BE))
}

// GetFtyp returns the major brand, minor version and compatible brands of the ISO-BMFF data
fn get_ftyp(buf: &[u8]) -> Option<(&[u8], &[u8], impl Iterator<Item = &[u8]>)> {
    if buf.len() < 16 {
        return None;
    }

    let ftyp_length = u32::from_be_bytes(buf[0..4].try_into().unwrap()) as usize;

    let major = &buf[8..12];
    let minor = &buf[12..16];
    let compatible = buf[16..]
        .chunks_exact(4)
        .take((ftyp_length / 4).saturating_sub(16 / 4));

    Some((major, minor, compatible))
}
