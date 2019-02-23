extern crate infer;

use std::fs;
use infer::Infer;

#[test]
fn test_wasm() {
    let info = Infer::new();
    assert_eq!(infer::Type { 
        mime: String::from("application/wasm"), 
        ext: String::from("wasm"),
    }, 
    info.get(&fs::read("testdata/sample.wasm").unwrap()).unwrap());
}
