extern crate infer;

use std::fs;
use infer::Infer;

#[test]
fn test_mp4() {
    let info = Infer::new();

    assert_eq!(infer::Type { 
        mime: String::from("video/mp4"), 
        ext: String::from("mp4"),
    }, 
    info.get(&fs::read("src/testdata/sample.mp4").unwrap()).unwrap());
}

#[test]
fn test_mkv() {
    let info = Infer::new();

    assert_eq!(infer::Type { 
        mime: String::from("video/x-matroska"), 
        ext: String::from("mkv"),
    }, 
    info.get(&fs::read("src/testdata/sample.mkv").unwrap()).unwrap());
}


#[test]
fn test_webm() {
    let info = Infer::new();

    assert_eq!(infer::Type { 
        mime: String::from("video/webm"), 
        ext: String::from("webm"),
    }, 
    info.get(&fs::read("src/testdata/sample.webm").unwrap()).unwrap());
}

#[test]
fn test_mov() {
    let info = Infer::new();

    assert_eq!(infer::Type { 
        mime: String::from("video/quicktime"), 
        ext: String::from("mov"),
    }, 
    info.get(&fs::read("src/testdata/sample.mov").unwrap()).unwrap());
}

#[test]
fn test_avi() {
    let info = Infer::new();

    assert_eq!(infer::Type { 
        mime: String::from("video/x-msvideo"), 
        ext: String::from("avi"),
    }, 
    info.get(&fs::read("src/testdata/sample.avi").unwrap()).unwrap());
}

#[test]
fn test_flv() {
    let info = Infer::new();
    
    assert_eq!(infer::Type { 
        mime: String::from("video/x-flv"), 
        ext: String::from("flv"),
    }, 
    info.get(&fs::read("src/testdata/sample.flv").unwrap()).unwrap());
}