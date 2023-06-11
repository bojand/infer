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
    buf.starts_with(&[0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00])
}

/// Returns whether a buffer is an EXE. DLL and EXE have the same magic number, so returns true also for a DLL.
///
/// # Example
///
/// ```rust
/// use std::fs;
/// assert!(infer::app::is_exe(&fs::read("testdata/sample.exe").unwrap()));
/// ```
pub fn is_exe(buf: &[u8]) -> bool {
    buf.starts_with(&[0x4D, 0x5A])
}

/// Returns whether a buffer is a DLL. DLL and EXE have the same magic number, so returns true also for an EXE.
pub fn is_dll(buf: &[u8]) -> bool {
    is_exe(buf)
}

/// Returns whether a buffer is an ELF.
pub fn is_elf(buf: &[u8]) -> bool {
    buf.len() > 52 && buf.starts_with(&[0x7F, 0x45, 0x4C, 0x46])
}

/// Returns whether a buffer is compiled Java bytecode.
pub fn is_java(buf: &[u8]) -> bool {
    buf.starts_with(&[0x43, 0x41, 0x76, 0x45])
        && (buf[4..].starts_with(&[0x42, 0x01, 0x42, 0x45])
            || buf[4..].starts_with(&[0x44, 0x30, 0x30, 0x44]))
}

/// Returns whether a buffer is LLVM Bitcode.
pub fn is_llvm(buf: &[u8]) -> bool {
    buf.starts_with(&[0x42, 0x43])
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

/// Returns whether a buffer is a Dalvik Executable (DEX).
pub fn is_dex(buf: &[u8]) -> bool {
    // https://source.android.com/devices/tech/dalvik/dex-format#dex-file-magic

    buf.len() > 36
    // magic
    && buf.starts_with(&[0x64, 0x65, 0x78, 0x0A])
    // file sise
    && buf[36] == 0x70
}

/// Returns whether a buffer is a Dey Optimized Dalvik Executable (ODEX).
pub fn is_dey(buf: &[u8]) -> bool {
    buf.len() > 100
    // magic
    && buf.starts_with(&[0x64, 0x65, 0x79, 0x0A])
    // file sise
    && is_dex(&buf[40..100])
}

/// Returns whether a buffer DER encoded X.509 certificate.
pub fn is_der(buf: &[u8]) -> bool {
    // https://en.wikipedia.org/wiki/List_of_file_signatures
    // https://github.com/ReFirmLabs/binwalk/blob/master/src/binwalk/magic/crypto#L25-L37
    // https://www.digitalocean.com/community/tutorials/openssl-essentials-working-with-ssl-certificates-private-keys-and-csrs
    // openssl req -newkey rsa:2048 -nodes -keyout domain.key -x509 -days 1 -out domain.crt
    // openssl x509 -in domain.crt -outform der -out domain.der

    buf.starts_with(&[0x30, 0x82])
}

/// Returns whether a buffer is a Common Object File Format for i386 architecture.
pub fn is_coff_i386(buf: &[u8]) -> bool {
    buf.starts_with(&[0x4C, 0x01])
}

/// Returns whether a buffer is a Common Object File Format for x64 architecture.
pub fn is_coff_x64(buf: &[u8]) -> bool {
    buf.starts_with(&[0x64, 0x86])
}

/// Returns whether a buffer is a Common Object File Format for Itanium architecture.
pub fn is_coff_ia64(buf: &[u8]) -> bool {
    buf.starts_with(&[0x00, 0x02])
}

/// Returns whether a buffer is a Common Object File Format.
pub fn is_coff(buf: &[u8]) -> bool {
    is_coff_x64(buf) || is_coff_i386(buf) || is_coff_ia64(buf)
}

/// Returns whether a buffer is pem
pub fn is_pem(buf: &[u8]) -> bool {
    // https://en.wikipedia.org/wiki/List_of_file_signatures
    buf.starts_with(b"-----BEGIN ".as_slice())
}
