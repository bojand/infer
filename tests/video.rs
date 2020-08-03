mod common;

test_format!(VIDEO, "video/mp4", "mp4", mp4, "sample.mp4");

test_format!(VIDEO, "video/x-matroska", "mkv", mkv, "sample.mkv");

test_format!(VIDEO, "video/webm", "webm", webm, "sample.webm");

test_format!(VIDEO, "video/quicktime", "mov", mov, "sample.mov");

test_format!(VIDEO, "video/x-msvideo", "avi", avi, "sample.avi");

test_format!(VIDEO, "video/x-flv", "flv", flv, "sample.flv");
