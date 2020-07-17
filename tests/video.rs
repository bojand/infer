use infer::{MatcherType, Type};

mod common;

test_format!(
    MatcherType::VIDEO,
    "video/mp4",
    "mp4",
    test_mp4,
    test_mp4_embed,
    "sample.mp4"
);

test_format!(
    MatcherType::VIDEO,
    "video/x-matroska",
    "mkv",
    test_mkv,
    test_mkv_embed,
    "sample.mkv"
);

test_format!(
    MatcherType::VIDEO,
    "video/webm",
    "webm",
    test_webm,
    test_webm_embed,
    "sample.webm"
);

test_format!(
    MatcherType::VIDEO,
    "video/quicktime",
    "mov",
    test_mov,
    test_mov_embed,
    "sample.mov"
);

test_format!(
    MatcherType::VIDEO,
    "video/x-msvideo",
    "avi",
    test_avi,
    test_avi_embed,
    "sample.avi"
);

test_format!(
    MatcherType::VIDEO,
    "video/x-flv",
    "flv",
    test_flv,
    test_flv_embed,
    "sample.flv"
);
