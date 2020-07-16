/*!
Small crate to infer file and MIME type by checking the
[magic number](https://en.wikipedia.org/wiki/Magic_number_(programming)) signature.

# Examples

### Get the type of a buffer

```rust
let v = vec![0xFF, 0xD8, 0xFF, 0xAA];
let info = infer::Infer::new();
assert_eq!("image/jpeg", info.get(&v).unwrap().mime_type());
assert_eq!("jpg", info.get(&v).unwrap().extension());
```

### Check path

```rust
let res = infer::get_from_path("testdata/sample.jpg");
assert!(res.is_ok());
let o = res.unwrap();
assert!(o.is_some());
let typ = o.unwrap();
assert_eq!("image/jpeg", typ.mime_type());
assert_eq!("jpg", typ.extension());
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

assert_eq!("custom/foo", res.mime_type());
assert_eq!("foo", res.extension());
```
*/
#![crate_name = "infer"]

mod map;
mod matchers;

use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub use map::MatcherType;
use map::{WrapMatcher, MATCHER_MAP};

/// All the supported matchers categorized and exposed as functions
pub use matchers::*;

/// Matcher function
pub type Matcher = fn(buf: &[u8]) -> bool;

/// Generic information for a type
#[derive(Copy, Clone)]
pub struct Type {
    matcher_type: MatcherType,
    mime_type: &'static str,
    extension: &'static str,
    matcher: WrapMatcher,
}

impl Type {
    pub(crate) const fn new(
        matcher_type: MatcherType,
        mime_type: &'static str,
        extension: &'static str,
        matcher: WrapMatcher,
    ) -> Self {
        Self {
            matcher_type,
            mime_type,
            extension,
            matcher,
        }
    }

    /// Only for tests
    #[doc(hidden)]
    pub fn new_for_test(
        matcher_type: MatcherType,
        mime_type: &'static str,
        extension: &'static str,
    ) -> Self {
        fn matcher(_buf: &[u8]) -> bool {
            false
        };

        Self::new(matcher_type, mime_type, extension, WrapMatcher(matcher))
    }

    /// Returns the type of matcher
    pub const fn matcher_type(&self) -> MatcherType {
        self.matcher_type
    }

    /// Returns the mime type
    pub const fn mime_type(&self) -> &'static str {
        self.mime_type
    }

    /// Returns the file extension
    pub const fn extension(&self) -> &'static str {
        self.extension
    }

    /// Checks if buf matches this Type
    fn matches(&self, buf: &[u8]) -> bool {
        (self.matcher.0)(buf)
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Kind")
            .field("matcher_type", &self.matcher_type)
            .field("mime_type", &self.mime_type)
            .field("extension", &self.extension)
            .finish()
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.matcher_type == other.matcher_type
            && self.mime_type == other.mime_type
            && self.extension == other.extension
    }
}

/// Infer allows to use a custom set of `Matcher`s for infering a MIME type.
pub struct Infer {
    mmap: Vec<Type>,
}

impl Infer {
    /// Initialize a new instance of the infer struct.
    pub const fn new() -> Infer {
        Infer { mmap: Vec::new() }
    }

    fn iter_matchers(&self) -> impl Iterator<Item = &Type> {
        let mmap = MATCHER_MAP.iter();
        self.mmap.iter().chain(mmap)
    }

    /// Returns the file type of the buffer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let info = infer::Infer::new();
    /// let v = vec![0xFF, 0xD8, 0xFF, 0xAA];
    /// assert_eq!("image/jpeg", info.get(&v).unwrap().mime_type());
    /// assert_eq!("jpg", info.get(&v).unwrap().extension());
    /// ```
    pub fn get(&self, buf: &[u8]) -> Option<Type> {
        for kind in self.iter_matchers() {
            if kind.matches(buf) {
                return Some(*kind);
            }
        }

        None
    }

    /// Returns the file type of the file given a path.
    ///
    /// # Examples
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
    /// # Examples
    ///
    /// See [`is`](./fn.is.html).
    pub fn is(&self, buf: &[u8], extension: &str) -> bool {
        if let Some(kind) = self
            .iter_matchers()
            .find(|kind| kind.extension() == extension)
        {
            if kind.matches(buf) {
                return true;
            }
        }

        false
    }

    /// Determines whether a buffer is of given mime type.
    ///
    /// # Examples
    ///
    /// See [`is_mime`](./fn.is_mime.html).
    pub fn is_mime(&self, buf: &[u8], mime_type: &str) -> bool {
        if let Some(kind) = self
            .iter_matchers()
            .find(|kind| kind.mime_type() == mime_type)
        {
            if kind.matches(buf) {
                return true;
            }
        }

        false
    }

    /// Returns whether an extension is supported.
    ///
    /// # Examples
    ///
    /// See [`is_supported`](./fn.is_supported.html).
    pub fn is_supported(&self, extension: &str) -> bool {
        for kind in self.iter_matchers() {
            if kind.extension() == extension {
                return true;
            }
        }

        false
    }

    /// Returns whether a mime type is supported.
    ///
    /// # Examples
    ///
    /// See [`is_mime_supported`](./fn.is_mime_supported.html).
    pub fn is_mime_supported(&self, mime_type: &str) -> bool {
        for kind in self.iter_matchers() {
            if kind.mime_type() == mime_type {
                return true;
            }
        }

        false
    }

    /// Determines whether a buffer is an application type.
    ///
    /// # Examples
    ///
    /// See [`is_app`](./fn.is_app.html).
    pub fn is_app(&self, buf: &[u8]) -> bool {
        self.is_type(buf, MatcherType::APP)
    }

    /// Determines whether a buffer is an archive type.
    ///
    /// # Examples
    ///
    /// See [`is_archive`](./fn.is_archive.html).
    pub fn is_archive(&self, buf: &[u8]) -> bool {
        self.is_type(buf, MatcherType::ARCHIVE)
    }

    /// Determines whether a buffer is an audio type.
    ///
    /// # Examples
    ///
    /// See [`is_audio`](./fn.is_audio.html).
    pub fn is_audio(&self, buf: &[u8]) -> bool {
        self.is_type(buf, MatcherType::AUDIO)
    }

    /// Determines whether a buffer is a document type.
    ///
    /// # Examples
    ///
    /// See [`is_document`](./fn.is_document.html).
    pub fn is_document(&self, buf: &[u8]) -> bool {
        self.is_type(buf, MatcherType::DOC)
    }

    /// Determines whether a buffer is a font type.
    ///
    /// # Examples
    ///
    /// See [`is_font`](./fn.is_font.html).
    pub fn is_font(&self, buf: &[u8]) -> bool {
        self.is_type(buf, MatcherType::FONT)
    }

    /// Determines whether a buffer is an image type.
    ///
    /// # Examples
    ///
    /// See [`is_image`](./fn.is_image.html).
    pub fn is_image(&self, buf: &[u8]) -> bool {
        self.is_type(buf, MatcherType::IMAGE)
    }

    /// Determines whether a buffer is a video type.
    ///
    /// # Examples
    ///
    /// See [`is_video`](./fn.is_video.html).
    pub fn is_video(&self, buf: &[u8]) -> bool {
        self.is_type(buf, MatcherType::VIDEO)
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
        self.is_type(buf, MatcherType::CUSTOM)
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
    /// assert_eq!("custom/foo", res.mime_type());
    /// assert_eq!("foo", res.extension());
    /// ```
    pub fn add(&mut self, mime_type: &'static str, extension: &'static str, m: Matcher) {
        self.mmap.push(Type::new(
            MatcherType::CUSTOM,
            mime_type,
            extension,
            WrapMatcher(m),
        ));
    }

    fn is_type(&self, buf: &[u8], matcher_type: MatcherType) -> bool {
        for kind in self
            .iter_matchers()
            .filter(|kind| kind.matcher_type() == matcher_type)
        {
            if kind.matches(buf) {
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
/// assert_eq!("image/jpeg", infer::get(&v).unwrap().mime_type());
/// assert_eq!("jpg", infer::get(&v).unwrap().extension());
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
/// assert_eq!("image/jpeg", typ.mime_type());
/// assert_eq!("jpg", typ.extension());
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
pub fn is(buf: &[u8], extension: &str) -> bool {
    INFER.is(buf, extension)
}

/// Determines whether a buffer is of given mime type.
///
/// # Examples
///
/// ```rust
/// let v = vec![0xFF, 0xD8, 0xFF, 0xAA];
/// assert!(infer::is_mime(&v, "image/jpeg"));
/// ```
pub fn is_mime(buf: &[u8], mime_type: &str) -> bool {
    INFER.is_mime(buf, mime_type)
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
                assert_eq!(info.extension(), "jpg");
                assert_eq!(info.mime_type(), "image/jpeg");
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
        assert_eq!(typ.mime_type(), "custom/foo");
        assert_eq!(typ.extension(), "foo");

        let buf_bar = &[0x89, 0x50, 0x4E, 0x47];
        let typ = info.get(buf_bar).expect("type is matched");
        assert_eq!(typ.mime_type(), "custom/bar");
        assert_eq!(typ.extension(), "bar");
    }
}
