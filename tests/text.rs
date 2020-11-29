mod common;

test_format!(TEXT, "text/html", "html", html, "sample.html");

test_format!(TEXT, "text/xml", "xml", xml, "sample.xml");

test_format!(TEXT, "text/x-shellscript", "sh", sh, "sample.sh");
