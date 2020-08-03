mod common;

test_format!(APP, "application/x-executable", "elf", elf, "sample_elf");

test_format!(
    APP,
    "application/vnd.microsoft.portable-executable",
    "exe",
    exe,
    "sample.exe"
);

test_format!(APP, "application/wasm", "wasm", wasm, "sample.wasm");
