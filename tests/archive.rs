extern crate infer;

use std::fs;
use infer::Infer;

#[test]
fn test_exe() {
    let info = Infer::new();

    assert_eq!(infer::Type { 
        mime: String::from("application/x-msdownload"), 
        ext: String::from("exe"),
    }, 
    info.get(&fs::read("testdata/sample.exe").unwrap()).unwrap());
}

#[test]
fn test_elf() {
    let info = Infer::new();

    assert_eq!(infer::Type { 
        mime: String::from("application/x-executable"), 
        ext: String::from("elf"),
    }, 
    info.get(&fs::read("testdata/sample_elf").unwrap()).unwrap());
}

#[test]
fn test_sqlite() {
    let info = Infer::new();

    assert_eq!(infer::Type { 
        mime: String::from("application/x-sqlite3"), 
        ext: String::from("sqlite"),
    }, 
    info.get(&fs::read("testdata/sample.db").unwrap()).unwrap());
}