mod common;

test_format!(
    Doc,
    "application/vnd.oasis.opendocument.text",
    "odt",
    odt,
    "sample.odt"
);

test_format!(
    Doc,
    "application/vnd.oasis.opendocument.spreadsheet",
    "ods",
    ods,
    "sample.ods"
);

test_format!(
    Doc,
    "application/vnd.oasis.opendocument.presentation",
    "odp",
    odp,
    "sample.odp"
);
