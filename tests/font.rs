extern crate infer;

use infer::Infer;
use std::fs;

#[test]
fn test_ttf() {
    let info = Infer::new();

    assert_eq!(
        infer::Type {
            mime: String::from("application/font-sfnt"),
            ext: String::from("ttf"),
        },
        info.get(&fs::read("testdata/sample.ttf").unwrap()).unwrap()
    );
}
