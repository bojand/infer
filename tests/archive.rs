extern crate infer;

use infer::Infer;

#[test]
fn test_exe() {
    let info = Infer::new();

    assert_eq!(
        infer::Type {
            mime: String::from("application/vnd.microsoft.portable-executable"),
            ext: String::from("exe"),
        },
        info.get_from_path("testdata/sample.exe").unwrap().unwrap()
    );
}

#[test]
fn test_elf() {
    let info = Infer::new();

    assert_eq!(
        infer::Type {
            mime: String::from("application/x-executable"),
            ext: String::from("elf"),
        },
        info.get_from_path("testdata/sample_elf").unwrap().unwrap()
    );
}

#[test]
fn test_sqlite() {
    let info = Infer::new();

    assert_eq!(
        infer::Type {
            mime: String::from("application/vnd.sqlite3"),
            ext: String::from("sqlite"),
        },
        info.get_from_path("testdata/sample.db").unwrap().unwrap()
    );
}

#[test]
fn test_zst() {
    let info = Infer::new();

    assert_eq!(
        infer::Type {
            mime: String::from("application/zstd"),
            ext: String::from("zst"),
        },
        info.get_from_path("testdata/sample.tar.zst")
            .unwrap()
            .unwrap()
    );
}
