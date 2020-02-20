extern crate infer;

use infer::Infer;

#[test]
fn test_doc() {
    let info = Infer::new();

    assert_eq!(
        infer::Type {
            mime: String::from("application/msword"),
            ext: String::from("doc"),
        },
        info.get_from_path("testdata/sample.doc").unwrap().unwrap()
    );
}

#[test]
fn test_docx() {
    let info = Infer::new();

    assert_eq!(
        infer::Type {
            mime: String::from(
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
            ),
            ext: String::from("docx"),
        },
        info.get_from_path("testdata/sample.docx").unwrap().unwrap()
    );
}

#[test]
fn test_xlsx() {
    let info = Infer::new();

    assert_eq!(
        infer::Type {
            mime: String::from("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
            ext: String::from("xlsx"),
        },
        info.get_from_path("testdata/sample.xlsx").unwrap().unwrap()
    );
}

#[test]
fn test_pptx() {
    let info = Infer::new();

    assert_eq!(
        infer::Type {
            mime: String::from("application/application/vnd.openxmlformats-officedocument.presentationml.presentation"),
            ext: String::from("pptx"),
        },
        info.get_from_path("testdata/sample.pptx").unwrap().unwrap()
    );
}
