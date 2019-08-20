extern crate infer;

use infer::Infer;
use std::fs;

#[test]
fn test_wasm() {
    let info = Infer::new();
    assert_eq!(
        infer::Type {
            mime: String::from("application/wasm"),
            ext: String::from("wasm"),
        },
        info.get(&fs::read("testdata/sample.wasm").unwrap())
            .unwrap()
    );
}
