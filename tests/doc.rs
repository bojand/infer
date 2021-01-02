mod common;

test_format!(DOC, "application/msword", "doc", doc, "sample.doc");

test_format!(
    DOC,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    "docx",
    docx,
    "sample.docx"
);

test_format!(
    DOC,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    "xlsx",
    xlsx,
    "sample.xlsx"
);

test_format!(
    DOC,
    "application/vnd.openxmlformats-officedocument.presentationml.presentation",
    "pptx",
    pptx,
    "sample.pptx"
);
