use infer::{MatcherType, Type};

mod common;

test_format!(
    MatcherType::DOC,
    "application/msword",
    "doc",
    test_doc,
    test_doc_embed,
    "sample.doc"
);

test_format!(
    MatcherType::DOC,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    "docx",
    test_docx,
    test_docx_embed,
    "sample.docx"
);

test_format!(
    MatcherType::DOC,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    "xlsx",
    test_xlsx,
    test_xlsx_embed,
    "sample.xlsx"
);

test_format!(
    MatcherType::DOC,
    "application/application/vnd.openxmlformats-officedocument.presentationml.presentation",
    "pptx",
    test_pptx,
    test_pptx_embed,
    "sample.pptx"
);
