mod common;

test_format!(Audio, "audio/mpeg", "mp3", mp3, "sample.mp3");
test_format!(Audio, "audio/x-dsf", "dsf", dsf, "sample.dsf");
