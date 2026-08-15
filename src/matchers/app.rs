/// Returns whether a buffer is a wasm.
///
/// # Examples
///
/// ```rust
/// use std::fs;
/// assert!(infer::app::is_wasm(&fs::read("testdata/sample.wasm").unwrap()));
/// ```
#[must_use]
pub fn is_wasm(buf: &[u8]) -> bool {
    // WASM has starts with `\0asm`, followed by the version.
    // http://webassembly.github.io/spec/core/binary/modules.html#binary-magic
    buf.starts_with(b"\0asm\x01\0\0\0")
}

/// Returns whether a buffer is an EXE. DLL and EXE have the same magic number, so returns true also for a DLL.
///
/// # Example
///
/// ```rust
/// use std::fs;
/// assert!(infer::app::is_exe(&fs::read("testdata/sample.exe").unwrap()));
/// ```
#[must_use]
pub fn is_exe(buf: &[u8]) -> bool {
    buf.starts_with(b"MZ")
}

/// Returns whether a buffer is a DLL. DLL and EXE have the same magic number, so returns true also for an EXE.
#[must_use]
pub fn is_dll(buf: &[u8]) -> bool {
    is_exe(buf)
}

/// Returns whether a buffer is an ELF.
#[must_use]
pub fn is_elf(buf: &[u8]) -> bool {
    buf.len() > 52 && buf.starts_with(b"\x7fELF")
}

/// Returns whether a buffer is compiled Java bytecode.
#[must_use]
pub fn is_java(buf: &[u8]) -> bool {
    let Some(&[0xca, 0xfe, 0xba, 0xbe, .., major_hi, major_lo]) = buf.first_chunk::<8>() else {
        return false;
    };

    // Checking the last 2 bytes are greater than or equal to 45 to distinguish from Mach-O binaries
    // Mach-O "Fat" binaries also use 0xCAFEBABE as magic bytes to start the file
    // Java are always Big Endian, after the magic bytes there are 2 bytes for the class file's
    // minor version and then 2 bytes for the major version
    // Java class files start at a major version of 45 and a minor of 0
    // So a value less than this shouldn't be a Java class file
    // https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html
    u16::from_be_bytes([major_hi, major_lo]) >= 45
}

/// Returns whether a buffer is LLVM Bitcode.
#[must_use]
pub fn is_llvm(buf: &[u8]) -> bool {
    buf.starts_with(b"BC")
}

/// Returns whether a buffer is a Mach-O binary.
#[must_use]
pub fn is_mach(buf: &[u8]) -> bool {
    // Mach-O binaries can be one of four variants: x86, x64, PowerPC, "Fat" (x86 + PowerPC)
    // https://ilostmynotes.blogspot.com/2014/05/mach-o-filetype-identification.html

    let Some(&magic) = buf.first_chunk::<4>() else {
        return false;
    };
    match magic {
        [0xcf | 0xce, 0xfa, 0xed, 0xfe] | [0xfe, 0xed, 0xfa, 0xcf | 0xce] => true,
        [0xca, 0xfe, 0xba, 0xbe] => {
            // Checking the next 4 bytes are less than 45 to distinguish from Java class files
            // which also use 0xCAFEBABE as magic bytes
            // Fat Mach-O binaries are always Big Endian
            matches!(buf.first_chunk::<8>(), Some(&[.., a, b, c, d]) if u32::from_be_bytes([a, b, c, d]) < 45)
        }
        _ => false,
    }
}

/// Returns whether a buffer is a Dalvik Executable (DEX).
#[must_use]
pub fn is_dex(buf: &[u8]) -> bool {
    // https://source.android.com/devices/tech/dalvik/dex-format#dex-file-magic
    buf.starts_with(b"dex\n") && buf.get(36) == Some(&0x70)
}

/// Returns whether a buffer is a Dey Optimized Dalvik Executable (ODEX).
#[must_use]
pub fn is_dey(buf: &[u8]) -> bool {
    buf.starts_with(b"dey\n") && buf.get(40..100).is_some_and(is_dex)
}

/// Returns whether a buffer DER encoded X.509 certificate.
#[must_use]
pub fn is_der(buf: &[u8]) -> bool {
    // https://en.wikipedia.org/wiki/List_of_file_signatures
    // https://github.com/ReFirmLabs/binwalk/blob/master/src/binwalk/magic/crypto#L25-L37
    // https://www.digitalocean.com/community/tutorials/openssl-essentials-working-with-ssl-certificates-private-keys-and-csrs
    // openssl req -newkey rsa:2048 -nodes -keyout domain.key -x509 -days 1 -out domain.crt
    // openssl x509 -in domain.crt -outform der -out domain.der
    buf.starts_with(&[0x30, 0x82])
}

/// Returns whether a buffer is a Common Object File Format for i386 architecture.
#[must_use]
pub fn is_coff_i386(buf: &[u8]) -> bool {
    buf.starts_with(&[0x4C, 0x01])
}

/// Returns whether a buffer is a Common Object File Format for x64 architecture.
#[must_use]
pub fn is_coff_x64(buf: &[u8]) -> bool {
    buf.starts_with(&[0x64, 0x86])
}

/// Returns whether a buffer is a Common Object File Format for Itanium architecture.
#[must_use]
pub fn is_coff_ia64(buf: &[u8]) -> bool {
    buf.starts_with(&[0x00, 0x02])
}

/// Returns whether a buffer is a Common Object File Format.
#[must_use]
pub fn is_coff(buf: &[u8]) -> bool {
    is_coff_x64(buf) || is_coff_i386(buf) || is_coff_ia64(buf)
}

/// Returns whether a buffer is pem
#[must_use]
pub fn is_pem(buf: &[u8]) -> bool {
    // https://en.wikipedia.org/wiki/List_of_file_signatures
    buf.starts_with(b"-----BEGIN ")
}

/// Returns whether a buffer is a QCOW2 disk.
#[must_use]
pub fn is_qcow2(buf: &[u8]) -> bool {
    // https://github.com/qemu/qemu/blob/master/docs/interop/qcow2.txt
    buf.starts_with(b"QFI\xfb")
}
