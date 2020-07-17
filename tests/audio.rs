use infer::{MatcherType, Type};

mod common;

test_format!(
    MatcherType::AUDIO,
    "audio/mpeg",
    "mp3",
    test_mp3,
    test_mp3_embed,
    "sample.mp3"
);
