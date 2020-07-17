use infer::{Infer, MatcherType, Type};

fn matcher(_buf: &[u8]) -> bool {
    false
}

#[test]
fn test_elf() {
    let info = Infer::new();

    assert_eq!(
        Type::new(MatcherType::APP, "application/x-executable", "elf", matcher),
        info.get_from_path("testdata/sample_elf").unwrap().unwrap()
    );
}

#[test]
fn test_exe() {
    let info = Infer::new();

    assert_eq!(
        Type::new(
            MatcherType::APP,
            "application/vnd.microsoft.portable-executable",
            "exe",
            matcher
        ),
        info.get_from_path("testdata/sample.exe").unwrap().unwrap()
    );
}

#[test]
fn test_wasm() {
    let info = Infer::new();
    assert_eq!(
        Type::new(MatcherType::APP, "application/wasm", "wasm", matcher),
        info.get_from_path("testdata/sample.wasm").unwrap().unwrap()
    );
}
