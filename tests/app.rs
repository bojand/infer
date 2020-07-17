use infer::{MatcherType, Type};

mod common;

test_format!(
    MatcherType::APP,
    "application/x-executable",
    "elf",
    test_elf,
    test_elf_embed,
    "sample_elf"
);

test_format!(
    MatcherType::APP,
    "application/vnd.microsoft.portable-executable",
    "exe",
    test_exe,
    test_exe_embed,
    "sample.exe"
);

test_format!(
    MatcherType::APP,
    "application/wasm",
    "wasm",
    test_wasm,
    test_wasm_embed,
    "sample.wasm"
);
