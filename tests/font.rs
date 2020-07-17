use infer::{MatcherType, Type};

mod common;

test_format!(
    MatcherType::FONT,
    "application/font-sfnt",
    "ttf",
    test_ttf,
    test_ttf_embed,
    "sample.ttf"
);
