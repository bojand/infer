/// Returns whether a buffer is MIDI data.
#[must_use]
pub fn is_midi(buf: &[u8]) -> bool {
    buf.starts_with(b"MThd")
}

/// Returns whether a buffer is MP3 data.
#[must_use]
pub fn is_mp3(buf: &[u8]) -> bool {
    buf.starts_with(b"ID3") // ID3v2
        // ID3V1 Support. Final bit (has crc32) may be or may not be set.
        || matches!(buf.first_chunk::<2>(), Some([0xff, 0xfb | 0xf3 | 0xf2]))
}

/// Returns whether a buffer is M4A data.
#[must_use]
pub fn is_m4a(buf: &[u8]) -> bool {
    buf.get(4..11) == Some(b"ftypM4A") || buf.starts_with(b"M4A ")
}

/// Returns whether a buffer is OGG data.
#[must_use]
pub fn is_ogg(buf: &[u8]) -> bool {
    buf.starts_with(b"OggS")
}

/// Returns whether a buffer is OGG Opus data.
#[must_use]
pub fn is_ogg_opus(buf: &[u8]) -> bool {
    is_ogg(buf) && buf.get(28..36) == Some(b"OpusHead")
}

/// Returns whether a buffer is FLAC data.
#[must_use]
pub fn is_flac(buf: &[u8]) -> bool {
    buf.starts_with(b"fLaC")
}

/// Returns whether a buffer is WAV data.
#[must_use]
pub fn is_wav(buf: &[u8]) -> bool {
    buf.starts_with(b"RIFF") && buf.get(8..12) == Some(b"WAVE")
}

/// Returns whether a buffer is AMR data.
#[must_use]
pub fn is_amr(buf: &[u8]) -> bool {
    buf.starts_with(b"#!AMR\n")
}

/// Returns whether a buffer is AAC data.
#[must_use]
pub fn is_aac(buf: &[u8]) -> bool {
    matches!(buf.first_chunk::<2>(), Some([0xff, 0xf1 | 0xf9]))
}

/// Returns whether a buffer is AIFF data.
#[must_use]
pub fn is_aiff(buf: &[u8]) -> bool {
    buf.starts_with(b"FORM") && buf.get(8..12) == Some(b"AIFF")
}

/// Returns whether a buffer is DSF data.
#[must_use]
pub fn is_dsf(buf: &[u8]) -> bool {
    // ref: https://dsd-guide.com/sites/default/files/white-papers/DSFFileFormatSpec_E.pdf
    buf.starts_with(b"DSD ")
}

/// Returns whether a buffer is APE (Monkey's Audio) data.
#[must_use]
pub fn is_ape(buf: &[u8]) -> bool {
    // ref: https://github.com/fernandotcl/monkeys-audio/blob/master/src/MACLib/APEHeader.h
    buf.starts_with(b"MAC ")
}
