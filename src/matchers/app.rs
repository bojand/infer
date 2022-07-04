use std::io::Read;
use std::io;

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

super::build_fn_read! {
    /// Returns whether data from a reader is a wasm.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::fs;
    /// use std::io::prelude::*;
    /// use std::fs::File;
    /// 
    /// fn main() -> std::io::Result<()> {
    ///     let mut f = File::open("testdata/sample.wasm")?;
    ///     assert!(infer::app::is_wasm_read(&mut f).unwrap());
    ///     Ok(())
    /// }
    /// ```
    (is_wasm_read, is_wasm, 8)
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
    buf.len() > 1 && buf[0] == 0x4D && buf[1] == 0x5A
}

super::build_fn_read! {
    /// Returns whether a data from reader is an EXE. 
    /// DLL and EXE have the same magic number, so returns true also for a DLL.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::fs;
    /// use std::io::prelude::*;
    /// use std::fs::File;
    /// 
    /// fn main() -> std::io::Result<()> {
    ///     let mut f = File::open("testdata/sample.exe")?;
    ///     assert!(infer::app::is_exe_read(&mut f).unwrap());
    ///     Ok(())
    /// }
    /// ```
    (is_exe_read, is_exe, 2)
}

/// Returns whether a buffer is a DLL. DLL and EXE have the same magic number, so returns true also for an EXE.
pub fn is_dll(buf: &[u8]) -> bool {
    is_exe(buf)
}

super::build_fn_read! {
    /// Returns whether data from a reader is a DLL. 
    /// DLL and EXE have the same magic number, so returns true also for an EXE.
    (is_dll_read, is_dll, 2)
}

/// Returns whether a buffer is an ELF.
pub fn is_elf(buf: &[u8]) -> bool {
    buf.len() > 52 && buf[0] == 0x7F && buf[1] == 0x45 && buf[2] == 0x4C && buf[3] == 0x46
}

super::build_fn_read! {
    /// Returns whether data from reader is an ELF.
    (is_elf_read, is_elf, 53)
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

super::build_fn_read! {
    /// Returns whether data from reader is compiled Java bytecode.
    (is_java_read, is_java, 8)
}

/// Returns whether a buffer is LLVM Bitcode.
pub fn is_llvm(buf: &[u8]) -> bool {
    buf.len() >= 2 && buf[0] == 0x42 && buf[1] == 0x43
}

super::build_fn_read! {
    /// Returns whether data from reader is LLVM Bitcode.
    (is_llvm_read, is_llvm, 2)
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

super::build_fn_read! {
    /// Returns whether data from reader is a Mach-O binary.
    (is_mach_read, is_mach, 4)
}

/// Returns whether a buffer is a Dalvik Executable (DEX).
pub fn is_dex(buf: &[u8]) -> bool {
    // https://source.android.com/devices/tech/dalvik/dex-format#dex-file-magic

    buf.len() > 36
    // magic
    && buf[0] == 0x64 && buf[1] == 0x65 && buf[2] == 0x78 && buf[3] == 0x0A
    // file sise
    && buf[36] == 0x70
}

super::build_fn_read! {
    /// Returns whether data from reader is a Dalvik Executable (DEX).
    (is_dex_read, is_dex, 4)
}

/// Returns whether a buffer is a Dey Optimized Dalvik Executable (ODEX).
pub fn is_dey(buf: &[u8]) -> bool {
    buf.len() > 100
    // magic
    && buf[0] == 0x64 && buf[1] == 0x65 && buf[2] == 0x79 && buf[3] == 0x0A
    // file sise
    && is_dex(&buf[40..100])
}

super::build_fn_read! {
    /// Returns whether data from reader is a Dey Optimized Dalvik Executable (ODEX).
    (is_dey_read, is_dey, 101)
}

/// Returns whether a buffer is DER encoded X.509 certificate.
pub fn is_der(buf: &[u8]) -> bool {
    // https://en.wikipedia.org/wiki/List_of_file_signatures
    // https://github.com/ReFirmLabs/binwalk/blob/master/src/binwalk/magic/crypto#L25-L37
    // https://www.digitalocean.com/community/tutorials/openssl-essentials-working-with-ssl-certificates-private-keys-and-csrs
    // openssl req -newkey rsa:2048 -nodes -keyout domain.key -x509 -days 1 -out domain.crt
    // openssl x509 -in domain.crt -outform der -out domain.der

    buf.len() > 2 && buf[0] == 0x30 && buf[1] == 0x82
}

super::build_fn_read! {
    /// Returns whether data from reader is DER encoded X.509 certificate.
    (is_der_read, is_der, 101)
}

/// Returns whether a buffer is a Common Object File Format for i386 architecture.
pub fn is_coff_i386(buf: &[u8]) -> bool {
    buf.len() > 2 && buf[0] == 0x4C && buf[1] == 0x01
}

super::build_fn_read! {
    /// Returns whether data from reader is a Common Object File Format for i386 architecture.
    (is_coff_i386_read, is_coff_i386, 3)
}

/// Returns whether a buffer is a Common Object File Format for x64 architecture.
pub fn is_coff_x64(buf: &[u8]) -> bool {
    buf.len() > 2 && buf[0] == 0x64 && buf[1] == 0x86
}

super::build_fn_read! {
    /// Returns whether data from reader is a Common Object File Format for x64 architecture.
    (is_coff_x64_read, is_coff_x64, 3)
}

/// Returns whether a buffer is a Common Object File Format for Itanium architecture.
pub fn is_coff_ia64(buf: &[u8]) -> bool {
    buf.len() > 2 && buf[0] == 0x00 && buf[1] == 0x02
}

super::build_fn_read! {
    /// Returns whether data from reader is a Common Object File Format for Itanium architecture.
    (is_coff_ia64_read, is_coff_ia64, 3)
}

/// Returns whether a buffer is a Common Object File Format.
pub fn is_coff(buf: &[u8]) -> bool {
    is_coff_x64(buf) || is_coff_i386(buf) || is_coff_ia64(buf)
}

super::build_fn_read! {
    /// Returns whether data from reader is a Common Object File Format.
    (is_coff_read, is_coff, 3)
}
