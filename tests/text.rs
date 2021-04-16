mod common;

test_format!(Text, "text/html", "html", html, "sample.html");

test_format!(Text, "text/xml", "xml", xml, "sample.xml");

test_format!(Text, "text/x-shellscript", "sh", sh, "sample.sh");
