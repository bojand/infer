#[cfg(feature = "std")]
use std::io::{self, Read};

use core::fmt;

use super::map::{MatcherType, WrapMatcher};

#[cfg(feature = "std")]
use super::map::WrapReadMatcher;

/// Matcher function
pub type Matcher = fn(&[u8]) -> bool;

#[cfg(feature = "std")]
pub type ReadMatcher = fn(&mut dyn Read) -> io::Result<bool>;

/// Generic information for a type
#[cfg(feature = "std")]
#[derive(Copy, Clone)]
pub struct Type {
    matcher_type: MatcherType,
    mime_type: &'static str,
    extension: &'static str,
    matcher: WrapMatcher,
    read_matcher: Option<WrapReadMatcher>,
    read_size: Option<usize>,
}

/// Generic information for a type
#[cfg(not(feature = "std"))]
#[derive(Copy, Clone)]
pub struct Type {
    matcher_type: MatcherType,
    mime_type: &'static str,
    extension: &'static str,
    matcher: WrapMatcher,
}

impl Type {
    #[cfg(feature = "std")]
    pub(crate) const fn new_static(
        matcher_type: MatcherType,
        mime_type: &'static str,
        extension: &'static str,
        matcher: WrapMatcher,
        read_matcher: Option<WrapReadMatcher>,
    ) -> Self {
        Self {
            matcher_type,
            mime_type,
            extension,
            matcher,
            read_matcher,
            read_size: None,
        }
    }

    #[cfg(not(feature = "std"))]
    pub(crate) const fn new_static(
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

    /// Returns a new `Type` with matcher and extension.
    #[cfg(feature = "std")]
    pub fn new(
        matcher_type: MatcherType,
        mime_type: &'static str,
        extension: &'static str,
        matcher: Matcher,
        read_matcher: Option<ReadMatcher>,
    ) -> Self {
        Self::new_static(
            matcher_type,
            mime_type,
            extension,
            WrapMatcher(matcher),
            read_matcher.map(WrapReadMatcher),
        )
    }

    /// Returns a new `Type` with matcher and extension.
    #[cfg(not(feature = "std"))]
    pub fn new(
        matcher_type: MatcherType,
        mime_type: &'static str,
        extension: &'static str,
        matcher: Matcher,
    ) -> Self {
        Self::new_static(matcher_type, mime_type, extension, WrapMatcher(matcher))
    }

    /// Returns the type of matcher
    ///
    /// # Examples
    ///
    /// ```rust
    /// let info = infer::Infer::new();
    /// let buf = [0xFF, 0xD8, 0xFF, 0xAA];
    /// let kind = info.get(&buf).expect("file type is known");
    ///
    /// assert_eq!(kind.matcher_type(), infer::MatcherType::Image);
    /// ```
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
    pub(crate) fn matches(&self, buf: &[u8]) -> bool {
        (self.matcher.0)(buf)
    }

    /// Checks if reader matches this Type
    #[cfg(feature = "std")]
    pub(crate) fn matches_read(&self, r: &mut impl Read) -> io::Result<bool> {
        match self.read_matcher {
            Some(m) => m.0(r),
            None => Ok(false),
        }
    }

    /// Returns the file extension
    #[cfg(feature = "std")]
    pub fn read_size(&self) -> usize {
        self.read_size.unwrap_or(0)
    }

    /// Returns whether the type supports matching by Read.
    #[cfg(feature = "std")]
    pub fn supports_read_match(&self) -> bool {
        self.read_matcher.is_some()
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Type")
            .field("matcher_type", &self.matcher_type)
            .field("mime_type", &self.mime_type)
            .field("extension", &self.extension)
            .finish()
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.mime_type, f)
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.matcher_type == other.matcher_type
            && self.mime_type == other.mime_type
            && self.extension == other.extension
    }
}
