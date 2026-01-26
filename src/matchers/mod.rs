pub mod app;
pub mod archive;
pub mod audio;
pub mod book;
pub mod doc;
pub mod font;
pub mod image;
pub mod odf;
pub mod text;
pub mod video;

pub(crate) fn compare_bytes(slice: &[u8], sub_slice: &[u8], start_offset: usize) -> bool {
    let sl = sub_slice.len();

    if start_offset + sl > slice.len() {
        return false;
    }

    for (i, v) in slice.iter().skip(start_offset).take(sl).enumerate() {
        let v2 = sub_slice[i];

        if *v != v2 {
            return false;
        }
    }

    true
}

pub(crate) fn match_bytes(slice: &[u8], sub_slice: &[u8]) -> bool {
    slice.windows(sub_slice.len())
         .find(|window| window == &sub_slice)
         .is_some()
}