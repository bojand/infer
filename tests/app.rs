mod common;

test_format!(App, "application/x-executable", "elf", elf, "sample_elf");

test_format!(
    App,
    "application/vnd.microsoft.portable-executable",
    "exe",
    exe,
    "sample.exe"
);

test_format!(
    App,
    "application/x-mach-binary",
    "mach",
    mach_x86,
    "sample_mach_x86"
);

test_format!(
    App,
    "application/x-mach-binary",
    "mach",
    mach_x64,
    "sample_mach_x64"
);

test_format!(
    App,
    "application/x-mach-binary",
    "mach",
    mach_ppc,
    "sample_mach_ppc"
);

test_format!(
    App,
    "application/x-mach-binary",
    "mach",
    mach_fat,
    "sample_mach_fat"
);

test_format!(App, "application/java", "class", java, "sample.class");

test_format!(App, "application/wasm", "wasm", wasm, "sample.wasm");

test_format!(App, "application/x-x509-ca-cert", "der", der, "sample.der");

test_format!(App, "application/x-x509-ca-cert", "pem", pem, "sample.pem");

test_format!(
    App,
    "application/x-qemu-disk",
    "qcow2",
    qcow2,
    "sample.qcow2"
);

#[test]
fn llvm_needs_full_magic() {
    assert!(infer::app::is_llvm(&[0x42, 0x43, 0xC0, 0xDE, 0x35, 0x14]));

    // Text that starts with "BC"
    let text = b"BC,code\n1,2\n";
    assert!(!infer::app::is_llvm(text));
    assert_eq!(infer::get(text), None);

    // A tar archive whose first member is named "BC.txt"
    let mut tar = [0u8; 512];
    tar[..6].copy_from_slice(b"BC.txt");
    tar[257..263].copy_from_slice(b"ustar\0");
    assert_eq!(
        infer::get(&tar).map(|kind| kind.mime_type()),
        Some("application/x-tar")
    );
}
