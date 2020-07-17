use infer::{MatcherType, Type};

mod common;

test_format!(
    MatcherType::IMAGE,
    "image/jpeg",
    "jpg",
    test_jpg,
    test_jpg_embed,
    "sample.jpg"
);

test_format!(
    MatcherType::IMAGE,
    "image/png",
    "png",
    test_png,
    test_png_embed,
    "sample.png"
);

test_format!(
    MatcherType::IMAGE,
    "image/gif",
    "gif",
    test_gif,
    test_gif_embed,
    "sample.gif"
);

test_format!(
    MatcherType::IMAGE,
    "image/tiff",
    "tif",
    test_tif,
    test_tif_embed,
    "sample.tif"
);

test_format!(
    MatcherType::IMAGE,
    "image/tiff",
    "tif",
    test_tif2,
    test_tif2_embed,
    "sample2.tif"
);

test_format!(
    MatcherType::IMAGE,
    "image/tiff",
    "tif",
    test_tif3,
    test_tif3_embed,
    "sample3.tif"
);

test_format!(
    MatcherType::IMAGE,
    "image/tiff",
    "tif",
    test_tif4,
    test_tif4_embed,
    "sample4.tif"
);

test_format!(
    MatcherType::IMAGE,
    "image/tiff",
    "tif",
    test_tif5,
    test_tif5_embed,
    "sample5.tif"
);

test_format!(
    MatcherType::IMAGE,
    "image/bmp",
    "bmp",
    test_bmp,
    test_bmp_embed,
    "sample.bmp"
);

test_format!(
    MatcherType::IMAGE,
    "image/vnd.adobe.photoshop",
    "psd",
    test_psd,
    test_psd_embed,
    "sample.psd"
);

test_format!(
    MatcherType::IMAGE,
    "image/vnd.microsoft.icon",
    "ico",
    test_ico,
    test_ico_embed,
    "sample.ico"
);

test_format!(
    MatcherType::IMAGE,
    "image/heif",
    "heif",
    test_heif,
    test_heif_embed,
    "sample.heic"
);

test_format!(
    MatcherType::IMAGE,
    "image/avif",
    "avif",
    test_avif,
    test_avif_embed,
    "sample.avif"
);
