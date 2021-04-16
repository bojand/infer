mod common;

test_format!(Video, "video/mp4", "mp4", mp4, "sample.mp4");

test_format!(Video, "video/x-matroska", "mkv", mkv, "sample.mkv");

test_format!(Video, "video/webm", "webm", webm, "sample.webm");

test_format!(Video, "video/quicktime", "mov", mov, "sample.mov");

test_format!(Video, "video/x-msvideo", "avi", avi, "sample.avi");

test_format!(Video, "video/x-flv", "flv", flv, "sample.flv");
