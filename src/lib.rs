/*!
Small crate to infer file and MIME type by checking the
[magic number](https://en.wikipedia.org/wiki/Magic_number_(programming)) signature.

# Examples

### Get the type of a buffer

```rust
let buf = [0xFF, 0xD8, 0xFF, 0xAA];
let kind = infer::get(&buf).expect("file type is known");

assert_eq!(kind.mime_type(), "image/jpeg");
assert_eq!(kind.extension(), "jpg");
assert_eq!(kind.matcher_type(), infer::MatcherType::Image);
```

### Check file type by path

```rust
# #[cfg(feature = "std")]
# fn run() {
let kind = infer::get_from_path("testdata/sample.jpg")
    .expect("file read successfully")
    .expect("file type is known");

assert_eq!(kind.mime_type(), "image/jpeg");
assert_eq!(kind.extension(), "jpg");
# }
```

### Check for specific type

```rust
let buf = [0xFF, 0xD8, 0xFF, 0xAA];
assert!(infer::image::is_jpeg(&buf));
```

### Check for specific type class

```rust
let buf = [0xFF, 0xD8, 0xFF, 0xAA];
assert!(infer::is_image(&buf));
```

### Adds a custom file type matcher

Here we actually need to use the `Infer` struct to be able to declare custom matchers.

```rust
# #[cfg(feature = "alloc")]
# #[cfg(feature = "std")]
# fn run() {
use std::io::{Result, Read};

fn custom_matcher(buf: &[u8]) -> bool {
    return buf.len() >= 3 && buf[0] == 0x10 && buf[1] == 0x11 && buf[2] == 0x12;
}

fn custom_matcher_read(r: &mut dyn Read) -> Result<bool> {
    let mut buffer = [0; 4];
    r.read_exact(&mut buffer[..])?;
    Ok(custom_matcher(&buffer))
}

let mut info = infer::Infer::new();
info.add("custom/foo", "foo", custom_matcher, Some(custom_matcher_read));

let buf = [0x10, 0x11, 0x12, 0x13, 0x14];
let mut kind = info.get(&buf).unwrap();

assert_eq!(kind.mime_type(), "custom/foo");
assert_eq!(kind.extension(), "foo");

let mut f = std::io::Cursor::new(buf);
kind = info.get_read(&mut f).unwrap().expect("file type is known");

assert_eq!(kind.mime_type(), "custom/foo");
assert_eq!(kind.extension(), "foo");
# }
```
*/

#![crate_name = "infer"]
#![doc(html_root_url = "https://docs.rs/infer/latest")]
#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod map;
mod matchers;
mod matchtype;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::fs::File;
#[cfg(feature = "std")]
use std::io::{self, Read, Seek};
#[cfg(feature = "std")]
use std::path::Path;

pub use map::MatcherType;

#[cfg(feature = "std")]
use map::{WrapMatcher, WrapReadMatcher};

use map::MATCHER_MAP;

/// All the supported matchers categorized and exposed as functions
pub use matchers::*;

pub use matchtype::*;

/// Infer allows to use a custom set of `Matcher`s for infering a MIME type.
///
/// Most operations can be done by using the _top level functions_, but when custom matchers
/// are needed every call has to go through the `Infer` struct to be able
/// to see the custom matchers.
pub struct Infer {
    #[cfg(feature = "alloc")]
    mmap: Vec<Type>,
}

impl Infer {
    /// Initialize a new instance of the infer struct.
    pub const fn new() -> Infer {
        #[cfg(feature = "alloc")]
        return Infer { mmap: Vec::new() };

        #[cfg(not(feature = "alloc"))]
        return Infer {};
    }

    fn iter_matchers(&self) -> impl Iterator<Item = &Type> {
        let mmap = MATCHER_MAP.iter();

        #[cfg(feature = "alloc")]
        return self.mmap.iter().chain(mmap);

        #[cfg(not(feature = "alloc"))]
        return mmap;
    }

    /// Returns the file type of the buffer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let info = infer::Infer::new();
    /// let buf = [0xFF, 0xD8, 0xFF, 0xAA];
    /// let kind = info.get(&buf).expect("file type is known");
    ///
    /// assert_eq!(kind.mime_type(), "image/jpeg");
    /// assert_eq!(kind.extension(), "jpg");
    /// ```
    pub fn get(&self, buf: &[u8]) -> Option<Type> {
        self.iter_matchers().find(|kind| kind.matches(buf)).copied()
    }

    /// Returns the file type of the data in the reader.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::fs;
    /// use std::io::prelude::*;
    /// use std::fs::File;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let info = infer::Infer::new();
    ///     let mut f = File::open("testdata/sample.jpg")?;
    ///     let kind = info.get_read(&mut f).unwrap().expect("file type is known");
    ///     assert_eq!(kind.mime_type(), "image/jpeg");
    ///     assert_eq!(kind.extension(), "jpg");
    ///     Ok(())
    /// }
    /// ```
    #[cfg(feature = "std")]
    pub fn get_read<R>(&self, r: &mut R) -> io::Result<Option<Type>>
    where
        R: Read + Seek,
    {
        let mut res_value: Option<Type> = None;

        for kind in self.iter_matchers() {
            let match_res = kind.matches_read(r)?;
            if match_res {
                res_value = Some(*kind);
                break;
            }

            r.rewind().ok();
        }

        Ok(res_value)
    }

    /// Returns the file type of the file given a path.
    ///
    /// # Examples
    ///
    /// See [`get_from_path`](./fn.get_from_path.html).
    #[cfg(feature = "std")]
    pub fn get_from_path<P: AsRef<Path>>(&self, path: P) -> io::Result<Option<Type>> {
        let file = File::open(path)?;

        let limit = file
            .metadata()
            .map(|m| std::cmp::min(m.len(), 8192) as usize + 1)
            .unwrap_or(0);
        let mut bytes = Vec::with_capacity(limit);
        file.take(8192).read_to_end(&mut bytes)?;

        Ok(self.get(&bytes))
    }

    /// Determines whether a buffer is of given extension.
    ///
    /// # Examples
    ///
    /// See [`is`](./fn.is.html).
    pub fn is(&self, buf: &[u8], extension: &str) -> bool {
        self.iter_matchers()
            .any(|kind| kind.extension() == extension && kind.matches(buf))
    }

    /// Determines whether data from read is of given extension.
    ///
    /// # Examples
    ///
    /// See [`is_read`](./fn.is_read.html).
    #[cfg(feature = "std")]
    pub fn is_read<R>(&self, r: &mut R, extension: &str) -> io::Result<bool>
    where
        R: Read + Seek,
    {
        let mut res_value: bool = false;

        for kind in self.iter_matchers() {
            if kind.extension() == extension {
                let match_res = kind.matches_read(r)?;
                if match_res {
                    res_value = true;
                    break;
                }

                r.rewind().ok();
            }
        }

        Ok(res_value)
    }

    /// Determines whether a buffer is of given mime type.
    ///
    /// # Examples
    ///
    /// See [`is_mime`](./fn.is_mime.html).
    pub fn is_mime(&self, buf: &[u8], mime_type: &str) -> bool {
        self.iter_matchers()
            .any(|kind| kind.mime_type() == mime_type && kind.matches(buf))
    }

    /// Determines whether data from reader is of given mime type.
    ///
    /// # Examples
    ///
    /// See [`is_mime_read`](./fn.is_mime_read.html).
    #[cfg(feature = "std")]
    pub fn is_mime_read<R>(&self, r: &mut R, mime_type: &str) -> io::Result<bool>
    where
        R: Read + Seek,
    {
        let mut res_value: bool = false;
        for kind in self.iter_matchers() {
            if kind.mime_type() == mime_type {
                let match_res = kind.matches_read(r)?;
                if match_res {
                    res_value = true;
                    break;
                }

                r.rewind().ok();
            }
        }

        Ok(res_value)
    }

    /// Returns whether an extension is supported.
    ///
    /// # Examples
    ///
    /// See [`is_supported`](./fn.is_supported.html).
    pub fn is_supported(&self, extension: &str) -> bool {
        self.iter_matchers()
            .any(|kind| kind.extension() == extension)
    }

    /// Returns the type for the mime type if supported.
    ///
    /// # Examples
    ///
    /// See [`is_supported`](./fn.get_type_by_mime.html).
    pub fn get_type_by_mime(&self, mime_type: &str) -> Option<Type> {
        self.iter_matchers()
            .find(|kind| kind.mime_type() == mime_type)
            .copied()
    }

    /// Returns the type for the extension if supported.
    ///
    /// # Examples
    ///
    /// See [`is_supported`](./fn.get_type_by_extension.html).
    pub fn get_type_by_extension(&self, extension: &str) -> Option<Type> {
        self.iter_matchers()
            .find(|kind| kind.extension() == extension)
            .copied()
    }

    /// Returns whether a mime type is supported.
    ///
    /// # Examples
    ///
    /// See [`is_mime_supported`](./fn.is_mime_supported.html).
    pub fn is_mime_supported(&self, mime_type: &str) -> bool {
        self.iter_matchers()
            .any(|kind| kind.mime_type() == mime_type)
    }

    /// Determines whether a buffer is an application type.
    ///
    /// # Examples
    ///
    /// See [`is_app`](./fn.is_app.html).
    pub fn is_app(&self, buf: &[u8]) -> bool {
        self.is_type(buf, MatcherType::App)
    }

    /// Determines whether data is an application type.
    ///
    /// # Examples
    ///
    /// See [`is_app_read`](./fn.is_app_read.html).
    #[cfg(feature = "std")]
    pub fn is_app_read<R>(&self, r: &mut R) -> io::Result<bool>
    where
        R: Read + Seek,
    {
        self.is_type_read(r, MatcherType::App)
    }

    /// Determines whether a buffer is an archive type.
    ///
    /// # Examples
    ///
    /// See [`is_archive`](./fn.is_archive.html).
    pub fn is_archive(&self, buf: &[u8]) -> bool {
        self.is_type(buf, MatcherType::Archive)
    }

    /// Determines whether data from reader is an archive type.
    ///
    /// # Examples
    ///
    /// See [`is_archive_read`](./fn.is_archive_read.html).
    #[cfg(feature = "std")]
    pub fn is_archive_read<R>(&self, r: &mut R) -> io::Result<bool>
    where
        R: Read + Seek,
    {
        self.is_type_read(r, MatcherType::Archive)
    }

    /// Determines whether a buffer is an audio type.
    ///
    /// # Examples
    ///
    /// See [`is_audio`](./fn.is_audio.html).
    pub fn is_audio(&self, buf: &[u8]) -> bool {
        self.is_type(buf, MatcherType::Audio)
    }

    /// Determines whether data from reader is an audio type.
    ///
    /// # Examples
    ///
    /// See [`is_audio_read`](./fn.is_audio_read.html).
    #[cfg(feature = "std")]
    pub fn is_audio_read<R>(&self, r: &mut R) -> io::Result<bool>
    where
        R: Read + Seek,
    {
        self.is_type_read(r, MatcherType::Audio)
    }

    /// Determines whether a buffer is a book type.
    ///
    /// # Examples
    ///
    /// See [`is_book`](./fn.is_book.html).
    pub fn is_book(&self, buf: &[u8]) -> bool {
        self.is_type(buf, MatcherType::Book)
    }

    /// Determines whether data from reader is a book type.
    ///
    /// # Examples
    ///
    /// See [`is_book_read`](./fn.is_book_read.html).
    #[cfg(feature = "std")]
    pub fn is_book_read<R>(&self, r: &mut R) -> io::Result<bool>
    where
        R: Read + Seek,
    {
        self.is_type_read(r, MatcherType::Book)
    }

    /// Determines whether a buffer is a document type.
    ///
    /// # Examples
    ///
    /// See [`is_document`](./fn.is_document.html).
    pub fn is_document(&self, buf: &[u8]) -> bool {
        self.is_type(buf, MatcherType::Doc)
    }

    /// Determines whether data from reader is a document type.
    ///
    /// # Examples
    ///
    /// See [`is_document_read`](./fn.is_document_read.html).
    #[cfg(feature = "std")]
    pub fn is_document_read<R>(&self, r: &mut R) -> io::Result<bool>
    where
        R: Read + Seek,
    {
        self.is_type_read(r, MatcherType::Doc)
    }

    /// Determines whether a buffer is a font type.
    ///
    /// # Examples
    ///
    /// See [`is_font`](./fn.is_font.html).
    pub fn is_font(&self, buf: &[u8]) -> bool {
        self.is_type(buf, MatcherType::Font)
    }

    /// Determines whether data from reader is a font type.
    ///
    /// # Examples
    ///
    /// See [`is_font_read`](./fn.is_font_read.html).
    #[cfg(feature = "std")]
    pub fn is_font_read<R>(&self, r: &mut R) -> io::Result<bool>
    where
        R: Read + Seek,
    {
        self.is_type_read(r, MatcherType::Font)
    }

    /// Determines whether a buffer is an image type.
    ///
    /// # Examples
    ///
    /// See [`is_image`](./fn.is_image.html).
    pub fn is_image(&self, buf: &[u8]) -> bool {
        self.is_type(buf, MatcherType::Image)
    }

    /// Determines whether data from reader is an image type.
    ///
    /// # Examples
    ///
    /// See [`is_image_read`](./fn.is_image_read.html).
    #[cfg(feature = "std")]
    pub fn is_image_read<R>(&self, r: &mut R) -> io::Result<bool>
    where
        R: Read + Seek,
    {
        self.is_type_read(r, MatcherType::Image)
    }

    /// Determines whether a buffer is a video type.
    ///
    /// # Examples
    ///
    /// See [`is_video`](./fn.is_video.html).
    pub fn is_video(&self, buf: &[u8]) -> bool {
        self.is_type(buf, MatcherType::Video)
    }

    /// Determines whether data from reader is a video type.
    ///
    /// # Examples
    ///
    /// See [`is_video_read`](./fn.is_video_read.html).
    #[cfg(feature = "std")]
    pub fn is_video_read<R>(&self, r: &mut R) -> io::Result<bool>
    where
        R: Read + Seek,
    {
        self.is_type_read(r, MatcherType::Video)
    }

    /// Determines whether a buffer is one of the custom types added.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "alloc")]
    /// # #[cfg(feature = "std")]
    /// # fn run() {
    /// fn custom_matcher(buf: &[u8]) -> bool {
    ///     return buf.len() >= 3 && buf[0] == 0x10 && buf[1] == 0x11 && buf[2] == 0x12;
    /// }
    ///
    /// let mut info = infer::Infer::new();
    /// info.add("custom/foo", "foo", custom_matcher, None);
    /// let buf = [0x10, 0x11, 0x12, 0x13];
    /// assert!(info.is_custom(&buf));
    /// # }
    /// ```
    pub fn is_custom(&self, buf: &[u8]) -> bool {
        self.is_type(buf, MatcherType::Custom)
    }

    /// Determines whether data from reader is one of the custom types added.
    #[cfg(feature = "std")]
    pub fn is_custom_read<R>(&self, r: &mut R) -> io::Result<bool>
    where
        R: Read + Seek,
    {
        self.is_type_read(r, MatcherType::Custom)
    }

    /// Adds a custom matcher.
    ///
    /// Custom matchers are matched in order of addition and before
    /// the default set of matchers.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "alloc")]
    /// # #[cfg(feature = "std")]
    /// # fn run() {
    /// fn custom_matcher(buf: &[u8]) -> bool {
    ///     return buf.len() >= 3 && buf[0] == 0x10 && buf[1] == 0x11 && buf[2] == 0x12;
    /// }
    ///
    /// let mut info = infer::Infer::new();
    /// info.add("custom/foo", "foo", custom_matcher, None);
    /// let buf = [0x10, 0x11, 0x12, 0x13];
    /// let kind =  info.get(&buf).expect("file type is known");
    ///
    /// assert_eq!(kind.mime_type(), "custom/foo");
    /// assert_eq!(kind.extension(), "foo");
    /// # }
    /// ```
    #[cfg(feature = "alloc")]
    #[cfg(feature = "std")]
    pub fn add(
        &mut self,
        mime_type: &'static str,
        extension: &'static str,
        m: Matcher,
        rm: Option<ReadMatcher>,
    ) {
        self.mmap.push(Type::new_static(
            MatcherType::Custom,
            mime_type,
            extension,
            WrapMatcher(m),
            rm.map(WrapReadMatcher),
        ));
    }

    fn is_type(&self, buf: &[u8], matcher_type: MatcherType) -> bool {
        self.iter_matchers()
            .any(|kind| kind.matcher_type() == matcher_type && kind.matches(buf))
    }

    #[cfg(feature = "std")]
    fn is_type_read<R>(&self, r: &mut R, matcher_type: MatcherType) -> io::Result<bool>
    where
        R: Read + Seek,
    {
        let mut res_value: bool = false;

        for kind in self.iter_matchers() {
            if kind.matcher_type() == matcher_type && kind.supports_read_match() {
                let match_res = kind.matches_read(r)?;
                if match_res {
                    res_value = true;
                    break;
                }

                r.rewind().ok();
            }
        }

        Ok(res_value)
    }
}

impl Default for Infer {
    fn default() -> Self {
        Infer::new()
    }
}

static INFER: Infer = Infer::new();

/// Returns the file type of the buffer.
///
/// # Examples
///
/// ```rust
/// let buf = [0xFF, 0xD8, 0xFF, 0xAA];
/// let kind = infer::get(&buf).expect("file type is known");
///
/// assert_eq!(kind.mime_type(), "image/jpeg");
/// assert_eq!(kind.extension(), "jpg");
/// ```
pub fn get(buf: &[u8]) -> Option<Type> {
    INFER.get(buf)
}

/// Returns the file type of the data in the reader.
///
/// # Examples
///
/// ```rust
/// use std::fs;
/// use std::io::prelude::*;
/// use std::fs::File;
///
/// fn main() -> std::io::Result<()> {
///     let mut f = File::open("testdata/sample.jpg")?;
///     let kind = infer::get_read(&mut f).unwrap().expect("file type is known");
///     assert_eq!(kind.mime_type(), "image/jpeg");
///     assert_eq!(kind.extension(), "jpg");
///     Ok(())
/// }
/// ```
#[cfg(feature = "std")]
pub fn get_read<R>(r: &mut R) -> io::Result<Option<Type>>
where
    R: Read + Seek,
{
    INFER.get_read(r)
}

/// Returns the file type of the file given a path.
///
/// # Errors
///
/// Returns an error if we fail to read the path.
///
/// # Examples
///
/// ```rust
/// let kind = infer::get_from_path("testdata/sample.jpg")
///     .expect("file read successfully")
///     .expect("file type is known");
///
/// assert_eq!(kind.mime_type(), "image/jpeg");
/// assert_eq!(kind.extension(), "jpg");
/// ```
#[cfg(feature = "std")]
pub fn get_from_path<P: AsRef<Path>>(path: P) -> io::Result<Option<Type>> {
    INFER.get_from_path(path)
}

/// Determines whether a buffer is of given extension.
///
/// # Examples
///
/// ```rust
/// let buf = [0xFF, 0xD8, 0xFF, 0xAA];
/// assert!(infer::is(&buf, "jpg"));
/// ```
pub fn is(buf: &[u8], extension: &str) -> bool {
    INFER.is(buf, extension)
}

/// Determines whether a buffer is of given extension.
///
/// # Examples
///
/// ```rust
/// use std::fs;
/// use std::io::prelude::*;
/// use std::fs::File;
///
/// fn main() -> std::io::Result<()> {
///     let mut f = File::open("testdata/sample.jpg")?;
///     let jpg = infer::is_read(&mut f, "jpg").unwrap();
///     assert!(jpg);
///     Ok(())
/// }
/// ```
#[cfg(feature = "std")]
pub fn is_read<R>(r: &mut R, extension: &str) -> io::Result<bool>
where
    R: Read + Seek,
{
    INFER.is_read(r, extension)
}

/// Determines whether a buffer is of given mime type.
///
/// # Examples
///
/// ```rust
/// let buf = [0xFF, 0xD8, 0xFF, 0xAA];
/// assert!(infer::is_mime(&buf, "image/jpeg"));
/// ```
pub fn is_mime(buf: &[u8], mime_type: &str) -> bool {
    INFER.is_mime(buf, mime_type)
}

/// Determines whether data from reader is of given mime type.
///
/// # Examples
///
/// ```rust
/// use std::fs;
/// use std::io::prelude::*;
/// use std::fs::File;
///
/// fn main() -> std::io::Result<()> {
///     let mut f = File::open("testdata/sample.jpg")?;
///     assert!(infer::is_mime_read(&mut f, "image/jpeg").unwrap());
///     Ok(())
/// }
/// ```
#[cfg(feature = "std")]
pub fn is_mime_read<R>(r: &mut R, mime_type: &str) -> io::Result<bool>
where
    R: Read + Seek,
{
    INFER.is_mime_read(r, mime_type)
}

/// Returns whether an extension is supported.
///
/// # Examples
///
/// ```rust
/// assert!(infer::is_supported("jpg"));
/// ```
pub fn is_supported(extension: &str) -> bool {
    INFER.is_supported(extension)
}

/// Returns whether a mime type is supported.
///
/// # Examples
///
/// ```rust
/// assert!(infer::is_mime_supported("image/jpeg"));
/// ```
pub fn is_mime_supported(mime_type: &str) -> bool {
    INFER.is_mime_supported(mime_type)
}

/// Determines whether a buffer is an application type.
///
/// # Examples
///
/// ```rust
/// use std::fs;
/// assert!(infer::is_app(&fs::read("testdata/sample.wasm").unwrap()));
/// ```
pub fn is_app(buf: &[u8]) -> bool {
    INFER.is_app(buf)
}

/// Determines whether data from reader is an application type.
///
/// # Examples
///
/// ```rust
/// use std::fs;
/// use std::io::prelude::*;
/// use std::fs::File;
///
/// fn main() -> std::io::Result<()> {
///     let mut f = File::open("testdata/sample.wasm")?;
///     assert!(infer::is_app_read(&mut f).unwrap());
///     Ok(())
/// }
/// ```
#[cfg(feature = "std")]
pub fn is_app_read<R>(r: &mut R) -> io::Result<bool>
where
    R: Read + Seek,
{
    INFER.is_app_read(r)
}

/// Determines whether a buffer is an archive type.
/// # Examples
///
/// ```rust
/// use std::fs;
/// assert!(infer::is_archive(&fs::read("testdata/sample.pdf").unwrap()));
/// ```
pub fn is_archive(buf: &[u8]) -> bool {
    INFER.is_archive(buf)
}

/// Determines whether data from reader is an archive type.
/// # Examples
///
/// ```rust
/// use std::fs;
/// use std::io::prelude::*;
/// use std::fs::File;
///
/// fn main() -> std::io::Result<()> {
///     let mut f = File::open("testdata/sample.pdf")?;
///     assert!(infer::is_archive_read(&mut f).unwrap());
///     Ok(())
/// }
/// ```
#[cfg(feature = "std")]
pub fn is_archive_read<R>(r: &mut R) -> io::Result<bool>
where
    R: Read + Seek,
{
    INFER.is_archive_read(r)
}

/// Determines whether a buffer is an audio type.
///
/// # Examples
///
/// ```rust
/// // mp3
/// let v = [0xff, 0xfb, 0x90, 0x44, 0x00];
/// assert!(infer::is_audio(&v));
/// ```
pub fn is_audio(buf: &[u8]) -> bool {
    INFER.is_audio(buf)
}

/// Determines whether data from reader is an audio type.
///
/// # Examples
///
/// ```rust
/// use std::fs;
/// use std::io::prelude::*;
/// use std::fs::File;
///
/// fn main() -> std::io::Result<()> {
///     let mut f = File::open("testdata/sample.mp3")?;
///     assert!(infer::is_audio_read(&mut f).unwrap());
///     Ok(())
/// }
/// ```
#[cfg(feature = "std")]
pub fn is_audio_read<R>(r: &mut R) -> io::Result<bool>
where
    R: Read + Seek,
{
    INFER.is_audio_read(r)
}

/// Determines whether a buffer is a book type.
///
/// # Examples
///
/// ```rust
/// use std::fs;
/// assert!(infer::is_book(&fs::read("testdata/sample.epub").unwrap()));
/// ```
pub fn is_book(buf: &[u8]) -> bool {
    INFER.is_book(buf)
}

/// Determines whether data from buffer is a book type.
///
/// # Examples
///
/// ```rust
/// use std::fs;
/// use std::io::prelude::*;
/// use std::fs::File;
///
/// fn main() -> std::io::Result<()> {
///     let mut f = File::open("testdata/sample.epub")?;
///     assert!(infer::is_book_read(&mut f).unwrap());
///     Ok(())
/// }
/// ```
#[cfg(feature = "std")]
pub fn is_book_read<R>(r: &mut R) -> io::Result<bool>
where
    R: Read + Seek,
{
    INFER.is_book_read(r)
}

/// Determines whether a buffer is a document type.
///
/// # Examples
///
/// ```rust
/// use std::fs;
/// assert!(infer::is_document(&fs::read("testdata/sample.docx").unwrap()));
/// ```
pub fn is_document(buf: &[u8]) -> bool {
    INFER.is_document(buf)
}

/// Determines whether data from reader is a document type.
///
/// # Examples
///
/// ```rust
/// use std::fs;
/// use std::io::prelude::*;
/// use std::fs::File;
///
/// fn main() -> std::io::Result<()> {
///     let mut f = File::open("testdata/sample.docx")?;
///     assert!(infer::is_document_read(&mut f).unwrap());
///     Ok(())
/// }
/// ```
#[cfg(feature = "std")]
pub fn is_document_read<R>(r: &mut R) -> io::Result<bool>
where
    R: Read + Seek,
{
    INFER.is_document_read(r)
}

/// Determines whether a buffer is a font type.
///
/// # Examples
///
/// ```rust
/// use std::fs;
/// assert!(infer::is_font(&fs::read("testdata/sample.ttf").unwrap()));
/// ```
pub fn is_font(buf: &[u8]) -> bool {
    INFER.is_font(buf)
}

/// Determines whether data from reader is a font type.
///
/// # Examples
///
/// ```rust
/// use std::fs;
/// use std::io::prelude::*;
/// use std::fs::File;
///
/// fn main() -> std::io::Result<()> {
///     let mut f = File::open("testdata/sample.ttf")?;
///     assert!(infer::is_font_read(&mut f).unwrap());
///     Ok(())
/// }
/// ```
#[cfg(feature = "std")]
pub fn is_font_read<R>(r: &mut R) -> io::Result<bool>
where
    R: Read + Seek,
{
    INFER.is_font_read(r)
}

/// Determines whether a buffer is an image type.
///
/// # Examples
///
/// ```rust
/// let v = [0xFF, 0xD8, 0xFF, 0xAA];
/// assert!(infer::is_image(&v));
/// ```
pub fn is_image(buf: &[u8]) -> bool {
    INFER.is_image(buf)
}

/// Determines whether data from reader is an image type.
///
/// # Examples
///
/// ```rust
/// use std::fs;
/// use std::io::prelude::*;
/// use std::fs::File;
///
/// fn main() -> std::io::Result<()> {
///     let mut f = File::open("testdata/sample.png")?;
///     assert!(infer::is_image_read(&mut f).unwrap());
///     Ok(())
/// }
/// ```
#[cfg(feature = "std")]
pub fn is_image_read<R>(r: &mut R) -> io::Result<bool>
where
    R: Read + Seek,
{
    INFER.is_image_read(r)
}

/// Determines whether a buffer is a video type.
///
/// # Examples
///
/// ```rust
/// use std::fs;
/// assert!(infer::is_video(&fs::read("testdata/sample.mov").unwrap()));
/// ```
pub fn is_video(buf: &[u8]) -> bool {
    INFER.is_video(buf)
}

/// Determines whether data from reader is a video type.
///
/// # Examples
///
/// ```rust
/// use std::fs;
/// use std::io::prelude::*;
/// use std::fs::File;
///
/// fn main() -> std::io::Result<()> {
///     let mut f = File::open("testdata/sample.mov")?;
///     assert!(infer::is_video_read(&mut f).unwrap());
///     Ok(())
/// }
/// ```
#[cfg(feature = "std")]
pub fn is_video_read<R>(r: &mut R) -> io::Result<bool>
where
    R: Read + Seek,
{
    INFER.is_video_read(r)
}

/// Returns the file type for the mime type if supported.
///
/// # Examples
///
/// ```rust
/// let kind = infer::get_type_by_mime("image/jpeg").expect("mime type is known");
///
/// assert_eq!(kind.mime_type(), "image/jpeg");
/// assert_eq!(kind.extension(), "jpg");
/// ```
pub fn get_type_by_mime(mime_type: &str) -> Option<Type> {
    INFER.get_type_by_mime(mime_type)
}

/// Returns the type for the extension if supported.
///
/// # Examples
///
/// ```rust
/// let kind = infer::get_type_by_extension("jpg").expect("extension is known");
///
/// assert_eq!(kind.mime_type(), "image/jpeg");
/// assert_eq!(kind.extension(), "jpg");
/// ```
pub fn get_type_by_extension(extension: &str) -> Option<Type> {
    INFER.get_type_by_extension(extension)
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "alloc")]
    #[cfg(feature = "std")]
    use super::Infer;
    #[cfg(feature = "std")]
    use std::fs::File;
    #[cfg(feature = "std")]
    use std::io::{self, Cursor, Read};

    #[test]
    fn test_get_unknown() {
        let buf = [];
        assert!(crate::get(&buf).is_none());
    }

    #[test]
    fn test_get_jpeg() {
        let buf = [0xFF, 0xD8, 0xFF, 0xAA];
        let kind = crate::get(&buf).expect("file type is known");
        assert_eq!(kind.extension(), "jpg");
        assert_eq!(kind.mime_type(), "image/jpeg");
    }

    #[test]
    fn test_matcher_type() {
        let buf = [0xFF, 0xD8, 0xFF, 0xAA];
        let kind = crate::get(&buf).expect("file type is known");
        assert_eq!(kind.matcher_type(), crate::MatcherType::Image);
    }

    #[cfg(feature = "alloc")]
    #[cfg(feature = "std")]
    #[test]
    fn test_custom_matcher_ordering() {
        // overrides jpeg matcher
        fn foo_matcher(buf: &[u8]) -> bool {
            buf.len() > 2 && buf[0] == 0xFF && buf[1] == 0xD8 && buf[2] == 0xFF
        }

        // overrides png matcher
        fn bar_matcher(buf: &[u8]) -> bool {
            buf.len() > 3 && buf[0] == 0x89 && buf[1] == 0x50 && buf[2] == 0x4E && buf[3] == 0x47
        }

        fn bar_matcher_read(r: &mut dyn Read) -> io::Result<bool> {
            let mut buffer = [0; 4];
            r.read_exact(&mut buffer[..])?;
            Ok(bar_matcher(&buffer))
        }

        let mut info = Infer::new();
        info.add("custom/foo", "foo", foo_matcher, None);
        info.add("custom/bar", "bar", bar_matcher, Some(bar_matcher_read));

        let buf_foo = &[0xFF, 0xD8, 0xFF];
        let typ = info.get(buf_foo).expect("type is matched");
        assert_eq!(typ.mime_type(), "custom/foo");
        assert_eq!(typ.extension(), "foo");

        let buf_bar = &[0x89, 0x50, 0x4E, 0x47, 0x12];
        let typ = info.get(buf_bar).expect("type is matched");
        assert_eq!(typ.mime_type(), "custom/bar");
        assert_eq!(typ.extension(), "bar");

        let mut f = Cursor::new(buf_bar);
        let kind = info.get_read(&mut f).unwrap().expect("type is matched");

        assert_eq!(kind.mime_type(), "custom/bar");
        assert_eq!(kind.extension(), "bar");
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_is_wasm_read() {
        let fr = File::open("testdata/sample.wasm");
        if fr.is_err() {
            assert!(fr.is_err(), "{:?}", fr.unwrap_err());
        }
        let mut f = fr.unwrap();
        let result = crate::app::is_wasm_read(&mut f).unwrap();
        assert!(result);
    }
}
