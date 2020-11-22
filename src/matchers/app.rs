/// Returns whether a buffer is a wasm.
///
/// # Examples
///
/// ```rust
/// use std::fs;
/// assert!(infer::app::is_wasm(&fs::read("testdata/sample.wasm").unwrap()));
/// ```
pub fn is_wasm(buf: &[u8]) -> bool {
    // WASM has starts with `\0asm`, followed by the version.
    // http://webassembly.github.io/spec/core/binary/modules.html#binary-magic
    buf.len() >= 8
        && buf[0] == 0x00
        && buf[1] == 0x61
        && buf[2] == 0x73
        && buf[3] == 0x6D
        && buf[4] == 0x01
        && buf[5] == 0x00
        && buf[6] == 0x00
        && buf[7] == 0x00
}

/// Returns whether a buffer is an EXE.
///
/// # Example
///
/// ```rust
/// use std::fs;
/// assert!(infer::app::is_exe(&fs::read("testdata/sample.exe").unwrap()));
/// ```
pub fn is_exe(buf: &[u8]) -> bool {
    buf.len() > 1 && buf[0] == 0x4D && buf[1] == 0x5A
}

/// Returns whether a buffer is an ELF.
pub fn is_elf(buf: &[u8]) -> bool {
    buf.len() > 52 && buf[0] == 0x7F && buf[1] == 0x45 && buf[2] == 0x4C && buf[3] == 0x46
}

/// Returns whether a buffer is compiled Java bytecode.
pub fn is_java(buf: &[u8]) -> bool {
    buf.len() >= 8
        && buf[0] == 0x43
        && buf[1] == 0x41
        && buf[2] == 0x76
        && buf[3] == 0x45
        && ((buf[4] == 0x42 && buf[5] == 0x01 && buf[6] == 0x42 && buf[7] == 0x45)
            || (buf[4] == 0x44 && buf[5] == 0x30 && buf[6] == 0x30 && buf[7] == 0x44))
}

/// Returns whether a buffer is LLVM Bitcode.
pub fn is_llvm(buf: &[u8]) -> bool {
    buf.len() >= 2 && buf[0] == 0x42 && buf[1] == 0x43
}

/// Returns whether a buffer is a Mach-O binary.
pub fn is_mach(buf: &[u8]) -> bool {
    // Mach-O binaries can be one of four variants: x86, x64, PowerPC, "Fat" (x86 + PowerPC)
    // https://ilostmynotes.blogspot.com/2014/05/mach-o-filetype-identification.html

    if buf.len() < 4 {
        return false;
    }

    match buf[0..4] {
        [width, 0xfa, 0xed, 0xfe] if width == 0xcf || width == 0xce => true,
        [0xfe, 0xed, 0xfa, width] if width == 0xcf || width == 0xce => true,
        [0xca, 0xfe, 0xba, 0xbe] => true,
        _ => false,
    }
}
