mod common;

test_format!(Video, "video/mp4", "mp4", mp4, "sample.mp4");

test_format!(Video, "video/x-matroska", "mkv", mkv, "sample.mkv");

test_format!(Video, "video/webm", "webm", webm, "sample.webm");
test_format!(Video, "video/webm", "webm", webm_vint2, "sample_vint2.webm");
test_format!(Video, "video/webm", "webm", webm_vint4, "sample_vint4.webm");
test_format!(Video, "video/webm", "webm", webm_vint8, "sample_vint8.webm");
test_format!(
    Video,
    "video/webm",
    "webm",
    webm_padded,
    "sample_padded.webm"
);
test_format!(
    Video,
    "video/webm",
    "webm",
    webm_decoy_bytes,
    "sample_decoy_bytes.webm"
);

test_format!(Video, "video/quicktime", "mov", mov, "sample.mov");
test_format!(Video, "video/quicktime", "mov", mov2, "sample2.mov");

test_format!(Video, "video/x-msvideo", "avi", avi, "sample.avi");

test_format!(Video, "video/x-flv", "flv", flv, "sample.flv");
