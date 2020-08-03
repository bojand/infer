mod common;

test_format!(IMAGE, "image/jpeg", "jpg", jpg, "sample.jpg");

test_format!(IMAGE, "image/png", "png", png, "sample.png");

test_format!(IMAGE, "image/gif", "gif", gif, "sample.gif");

test_format!(IMAGE, "image/tiff", "tif", tif, "sample.tif");

test_format!(IMAGE, "image/tiff", "tif", tif2, "sample2.tif");

test_format!(IMAGE, "image/tiff", "tif", tif3, "sample3.tif");

test_format!(IMAGE, "image/tiff", "tif", tif4, "sample4.tif");

test_format!(IMAGE, "image/tiff", "tif", tif5, "sample5.tif");

test_format!(IMAGE, "image/bmp", "bmp", bmp, "sample.bmp");

test_format!(IMAGE, "image/vnd.adobe.photoshop", "psd", psd, "sample.psd");

test_format!(IMAGE, "image/vnd.microsoft.icon", "ico", ico, "sample.ico");

test_format!(IMAGE, "image/heif", "heif", heif, "sample.heic");

test_format!(IMAGE, "image/avif", "avif", avif, "sample.avif");
