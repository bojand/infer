extern crate infer;

use std::fs;
use infer::Infer;

#[test]
fn test_ttf() {
    let info = Infer::new();

    assert_eq!(infer::Type { 
        mime: String::from("application/font-sfnt"), 
        ext: String::from("ttf"),
    }, 
    info.get(&fs::read("testdata/sample.ttf").unwrap()).unwrap());
}
