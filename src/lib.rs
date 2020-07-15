/*!
Small crate to infer file and MIME type by checking the
[magic number](https://en.wikipedia.org/wiki/Magic_number_(programming)) signature.

# Examples

### Get the type of a buffer

```rust
let v = vec![0xFF, 0xD8, 0xFF, 0xAA];
assert_eq!("image/jpeg", infer::get(&v).unwrap().mime);
assert_eq!("jpg", infer::get(&v).unwrap().ext);
```

### Check path

```rust
let res = infer::get_from_path("testdata/sample.jpg");
assert!(res.is_ok());
let o = res.unwrap();
assert!(o.is_some());
let typ = o.unwrap();
assert_eq!("image/jpeg", typ.mime);
assert_eq!("jpg", typ.ext);
```

### Check for specific type

Note individual matcher functions do not require an Infer struct instance.

```rust
let v = vec![0xFF, 0xD8, 0xFF, 0xAA];
assert!(infer::image::is_jpeg(&v));
```

### Check for specific type class

```rust
let v = vec![0xFF, 0xD8, 0xFF, 0xAA];
assert!(infer::is_image(&v));
```

### Adds a custom file type matcher

```rust
fn custom_matcher(buf: &[u8]) -> bool {
    return buf.len() >= 3 && buf[0] == 0x10 && buf[1] == 0x11 && buf[2] == 0x12;
}

let mut info = infer::Infer::new();
info.add("custom/foo", "foo", custom_matcher);

let v = vec![0x10, 0x11, 0x12, 0x13];
let res =  info.get(&v).unwrap();

assert_eq!("custom/foo", res.mime);
assert_eq!("foo", res.ext);
```
*/
#![crate_name = "infer"]

mod map;
mod matchers;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use map::{MatcherType, WrapMatcher, MATCHER_MAP};

/// All the supported matchers categorized and exposed as functions
pub use matchers::*;

/// Matcher function
pub type Matcher = fn(buf: &[u8]) -> bool;

/// Generic information for a type
#[derive(Debug, Eq, PartialEq)]
pub struct Type {
    /// The mime
    pub mime: String,

    /// The file extension
    pub ext: String,
}

/// Infer allows to use a custom set of `Matcher`s for infering a MIME type.
pub struct Infer {
    mmap: Vec<(map::MatcherType, String, String, WrapMatcher)>,
}

impl Infer {
    /// Initialize a new instance of the infer struct.
    pub const fn new() -> Infer {
        Infer { mmap: Vec::new() }
    }

    fn iter_matchers(&self) -> impl Iterator<Item = (&MatcherType, &str, &str, &WrapMatcher)> {
        let mmap = MATCHER_MAP
            .iter()
            .map(|(mt, mime, ext, matcher)| (mt, *mime, *ext, matcher));
        self.mmap
            .iter()
            .map(|(mt, mime, ext, matcher)| (mt, mime.as_str(), ext.as_str(), matcher))
            .chain(mmap)
    }

    /// Returns the file type of the buffer.
    ///
    /// See [`get`](./fn.get.html).
    pub fn get(&self, buf: &[u8]) -> Option<Type> {
        for (_, mime, ext, matcher) in self.iter_matchers() {
            if matcher.0(buf) {
                return Some(Type {
                    mime: mime.to_string(),
                    ext: ext.to_string(),
                });
            }
        }

        None
    }

    /// Returns the file type of the file given a path.
    ///
    /// See [`get_from_path`](./fn.get_from_path.html).
    pub fn get_from_path<P: AsRef<Path>>(&self, path: P) -> Result<Option<Type>, std::io::Error> {
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
    /// See [`is`](./fn.is.html).
    pub fn is(&self, buf: &[u8], ext: &str) -> bool {
        if let Some((_mt, _mi, _e, matcher)) = self
            .iter_matchers()
            .find(|(_mt, _mime, ex, _matcher)| *ex == ext)
        {
            if matcher.0(buf) {
                return true;
            }
        }

        false
    }

    /// Determines whether a buffer is of given mime type.
    ///
    /// See [`is_mime`](./fn.is_mime.html).
    pub fn is_mime(&self, buf: &[u8], mime: &str) -> bool {
        if let Some((_mt, _mi, _e, matcher)) = self
            .iter_matchers()
            .find(|(_mt, mi, _ext, _matcher)| *mi == mime)
        {
            if matcher.0(buf) {
                return true;
            }
        }

        false
    }

    /// Returns whether an extension is supported.
    ///
    /// See [`is_supported`](./fn.is_supported.html).
    pub fn is_supported(&self, ext: &str) -> bool {
        for (_mt, _mime, type_ext, _matcher) in self.iter_matchers() {
            if ext == type_ext {
                return true;
            }
        }

        false
    }

    /// Returns whether a mime type is supported.
    ///
    /// See [`is_mime_supported`](./fn.is_mime_supported.html).
    pub fn is_mime_supported(&self, mime: &str) -> bool {
        for (_mt, type_mime, _ext, _matcher) in self.iter_matchers() {
            if mime == type_mime {
                return true;
            }
        }

        false
    }

    /// Determines whether a buffer is an application type.
    ///
    /// See [`is_app`](./fn.is_app.html).
    pub fn is_app(&self, buf: &[u8]) -> bool {
        self.is_type(buf, map::MatcherType::APP)
    }

    /// Determines whether a buffer is an archive type.
    /// # Examples
    ///
    /// See [`is_archive`](./fn.is_archive.html).
    pub fn is_archive(&self, buf: &[u8]) -> bool {
        self.is_type(buf, map::MatcherType::ARCHIVE)
    }

    /// Determines whether a buffer is an audio type.
    ///
    /// # Examples
    ///
    /// See [`is_audio`](./fn.is_audio.html).
    pub fn is_audio(&self, buf: &[u8]) -> bool {
        self.is_type(buf, map::MatcherType::AUDIO)
    }

    /// Determines whether a buffer is a document type.
    ///
    /// See [`is_document`](./fn.is_document.html).
    pub fn is_document(&self, buf: &[u8]) -> bool {
        self.is_type(buf, map::MatcherType::DOC)
    }

    /// Determines whether a buffer is a font type.
    ///
    /// See [`is_font`](./fn.is_font.html).
    pub fn is_font(&self, buf: &[u8]) -> bool {
        self.is_type(buf, map::MatcherType::FONT)
    }

    /// Determines whether a buffer is an image type.
    ///
    /// See [`is_image`](./fn.is_image.html).
    pub fn is_image(&self, buf: &[u8]) -> bool {
        self.is_type(buf, map::MatcherType::IMAGE)
    }

    /// Determines whether a buffer is a video type.
    ///
    /// See [`is_video`](./fn.is_video.html).
    pub fn is_video(&self, buf: &[u8]) -> bool {
        self.is_type(buf, map::MatcherType::VIDEO)
    }

    /// Determines whether a buffer is one of the custom types added.
    ///
    /// # Examples
    ///
    /// ```rust
    /// fn custom_matcher(buf: &[u8]) -> bool {
    ///     return buf.len() >= 3 && buf[0] == 0x10 && buf[1] == 0x11 && buf[2] == 0x12;
    /// }
    ///
    /// let mut info = infer::Infer::new();
    /// info.add("custom/foo", "foo", custom_matcher);
    /// let v = vec![0x10, 0x11, 0x12, 0x13];
    /// assert!(info.is_custom(&v));
    /// ```
    pub fn is_custom(&self, buf: &[u8]) -> bool {
        self.is_type(buf, map::MatcherType::CUSTOM)
    }

    /// Adds a custom matcher.
    ///
    /// Custom matchers are matched in order of addition and before
    /// the default set of matchers.
    ///
    /// # Examples
    ///
    /// ```rust
    /// fn custom_matcher(buf: &[u8]) -> bool {
    ///     return buf.len() >= 3 && buf[0] == 0x10 && buf[1] == 0x11 && buf[2] == 0x12;
    /// }
    ///
    /// let mut info = infer::Infer::new();
    /// info.add("custom/foo", "foo", custom_matcher);
    /// let v = vec![0x10, 0x11, 0x12, 0x13];
    /// let res =  info.get(&v).unwrap();
    /// assert_eq!("custom/foo", res.mime);
    /// assert_eq!("foo", res.ext);
    /// ```
    pub fn add(&mut self, mime: &str, ext: &str, m: Matcher) {
        self.mmap.push((
            map::MatcherType::CUSTOM,
            mime.to_string(),
            ext.to_string(),
            WrapMatcher(m),
        ));
    }

    fn is_type(&self, buf: &[u8], typ: map::MatcherType) -> bool {
        for (_mt, _mi, _ex, matcher) in self
            .iter_matchers()
            .filter(|(mt, _mime, _e, _matcher)| **mt == typ)
        {
            if matcher.0(buf) {
                return true;
            }
        }

        false
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
/// let v = vec![0xFF, 0xD8, 0xFF, 0xAA];
/// assert_eq!("image/jpeg", infer::get(&v).unwrap().mime);
/// assert_eq!("jpg", infer::get(&v).unwrap().ext);
/// ```
pub fn get(buf: &[u8]) -> Option<Type> {
    INFER.get(buf)
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
/// let res = infer::get_from_path("testdata/sample.jpg");
/// assert!(res.is_ok());
/// let o = res.unwrap();
/// assert!(o.is_some());
/// let typ = o.unwrap();
/// assert_eq!("image/jpeg", typ.mime);
/// assert_eq!("jpg", typ.ext);
/// ```
pub fn get_from_path<P: AsRef<Path>>(path: P) -> Result<Option<Type>, std::io::Error> {
    INFER.get_from_path(path)
}

/// Determines whether a buffer is of given extension.
///
/// # Examples
///
/// ```rust
/// let v = vec![0xFF, 0xD8, 0xFF, 0xAA];
/// assert!(infer::is(&v, "jpg"));
/// ```
pub fn is(buf: &[u8], ext: &str) -> bool {
    INFER.is(buf, ext)
}

/// Determines whether a buffer is of given mime type.
///
/// # Examples
///
/// ```rust
/// let v = vec![0xFF, 0xD8, 0xFF, 0xAA];
/// assert!(infer::is_mime(&v, "image/jpeg"));
/// ```
pub fn is_mime(buf: &[u8], mime: &str) -> bool {
    INFER.is_mime(buf, mime)
}

/// Returns whether an extension is supported.
///
/// # Examples
///
/// ```rust
/// assert!(infer::is_supported("jpg"));
/// ```
pub fn is_supported(ext: &str) -> bool {
    INFER.is_supported(ext)
}

/// Returns whether a mime type is supported.
///
/// # Examples
///
/// ```rust
/// assert!(infer::is_mime_supported("image/jpeg"));
/// ```
pub fn is_mime_supported(mime: &str) -> bool {
    INFER.is_mime_supported(mime)
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

/// Determines whether a buffer is an audio type.
///
/// # Examples
///
/// ```rust
/// // mp3
/// let v = vec![0xff, 0xfb, 0x90, 0x44, 0x00];
/// assert!(infer::is_audio(&v));
/// ```
pub fn is_audio(buf: &[u8]) -> bool {
    INFER.is_audio(buf)
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

/// Determines whether a buffer is an image type.
///
/// # Examples
///
/// ```rust
/// let v = vec![0xFF, 0xD8, 0xFF, 0xAA];
/// assert!(infer::is_image(&v));
/// ```
pub fn is_image(buf: &[u8]) -> bool {
    INFER.is_image(buf)
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

#[cfg(test)]
mod tests {
    use super::Infer;

    #[test]
    fn test_get_unknown() {
        let v = Vec::new();
        let info = Infer::new();
        assert!(info.get(&v).is_none());
    }

    #[test]
    fn test_get_jpeg() {
        let v = vec![0xFF, 0xD8, 0xFF, 0xAA];
        match crate::get(&v) {
            Some(info) => {
                assert_eq!(info.ext, "jpg");
                assert_eq!(info.mime, "image/jpeg");
            }
            None => panic!("type info expected"),
        }
    }

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

        let mut info = Infer::new();
        info.add("custom/foo", "foo", foo_matcher);
        info.add("custom/bar", "bar", bar_matcher);

        let buf_foo = &[0xFF, 0xD8, 0xFF];
        let typ = info.get(buf_foo).expect("type is matched");
        assert_eq!(typ.mime, "custom/foo");
        assert_eq!(typ.ext, "foo");

        let buf_bar = &[0x89, 0x50, 0x4E, 0x47];
        let typ = info.get(buf_bar).expect("type is matched");
        assert_eq!(typ.mime, "custom/bar");
        assert_eq!(typ.ext, "bar");
    }
}
