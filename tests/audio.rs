extern crate infer;

use infer::Infer;
use std::fs;

#[test]
fn test_mp3() {
    let info = Infer::new();

    assert_eq!(
        infer::Type {
            mime: String::from("audio/mpeg"),
            ext: String::from("mp3"),
        },
        info.get(&fs::read("testdata/sample.mp3").unwrap()).unwrap()
    );
}
