mod common;

test_format!(APP, "application/x-executable", "elf", elf, "sample_elf");

test_format!(
    APP,
    "application/vnd.microsoft.portable-executable",
    "exe",
    exe,
    "sample.exe"
);

test_format!(
    APP,
    "application/x-mach-binary",
    "mach",
    mach_x86,
    "sample_mach_x86"
);

test_format!(
    APP,
    "application/x-mach-binary",
    "mach",
    mach_x64,
    "sample_mach_x64"
);

test_format!(
    APP,
    "application/x-mach-binary",
    "mach",
    mach_ppc,
    "sample_mach_ppc"
);

test_format!(
    APP,
    "application/x-mach-binary",
    "mach",
    mach_fat,
    "sample_mach_fat"
);

test_format!(APP, "application/wasm", "wasm", wasm, "sample.wasm");

test_format!(APP, "application/vnd.android.dex", "dex", dex, "sample.dex");

test_format!(APP, "application/vnd.android.dey", "dey", dey, "sample.dey");

test_format!(APP, "application/x-x509-ca-cert", "der", der, "sample.der");
