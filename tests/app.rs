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

test_format!(App, "application/wasm", "wasm", wasm, "sample.wasm");

test_format!(App, "application/x-x509-ca-cert", "der", der, "sample.der");
