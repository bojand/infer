#[cfg(feature = "std")]
use super::map::MatcherType;
#[cfg(feature = "std")]
use super::matchtype::*;
#[cfg(feature = "std")]
use super::Infer;
#[cfg(feature = "std")]
use super::INFER;
#[cfg(feature = "std")]
use std::io::{self, Read, Seek};

#[cfg(feature = "std")]
impl Infer {
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

    /// Determines whether data from read is of given extension.
    ///
    /// # Examples
    ///
    /// See [`is_read`](./fn.is_read.html).
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

    /// Determines whether data from reader is of given mime type.
    ///
    /// # Examples
    ///
    /// See [`is_mime_read`](./fn.is_mime_read.html).
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

    /// Determines whether data is an application type.
    ///
    /// # Examples
    ///
    /// See [`is_app_read`](./fn.is_app_read.html).
    pub fn is_app_read<R>(&self, r: &mut R) -> io::Result<bool>
    where
        R: Read + Seek,
    {
        self.is_type_read(r, MatcherType::App)
    }

    /// Determines whether data from reader is an archive type.
    ///
    /// # Examples
    ///
    /// See [`is_archive_read`](./fn.is_archive_read.html).
    pub fn is_archive_read<R>(&self, r: &mut R) -> io::Result<bool>
    where
        R: Read + Seek,
    {
        self.is_type_read(r, MatcherType::Archive)
    }

    /// Determines whether data from reader is an audio type.
    ///
    /// # Examples
    ///
    /// See [`is_audio_read`](./fn.is_audio_read.html).
    pub fn is_audio_read<R>(&self, r: &mut R) -> io::Result<bool>
    where
        R: Read + Seek,
    {
        self.is_type_read(r, MatcherType::Audio)
    }

    /// Determines whether data from reader is a book type.
    ///
    /// # Examples
    ///
    /// See [`is_book_read`](./fn.is_book_read.html).
    pub fn is_book_read<R>(&self, r: &mut R) -> io::Result<bool>
    where
        R: Read + Seek,
    {
        self.is_type_read(r, MatcherType::Book)
    }

    /// Determines whether data from reader is a document type.
    ///
    /// # Examples
    ///
    /// See [`is_document_read`](./fn.is_document_read.html).
    pub fn is_document_read<R>(&self, r: &mut R) -> io::Result<bool>
    where
        R: Read + Seek,
    {
        self.is_type_read(r, MatcherType::Doc)
    }

    /// Determines whether data from reader is a font type.
    ///
    /// # Examples
    ///
    /// See [`is_font_read`](./fn.is_font_read.html).
    pub fn is_font_read<R>(&self, r: &mut R) -> io::Result<bool>
    where
        R: Read + Seek,
    {
        self.is_type_read(r, MatcherType::Font)
    }

    /// Determines whether data from reader is an image type.
    ///
    /// # Examples
    ///
    /// See [`is_image_read`](./fn.is_image_read.html).
    pub fn is_image_read<R>(&self, r: &mut R) -> io::Result<bool>
    where
        R: Read + Seek,
    {
        self.is_type_read(r, MatcherType::Image)
    }

    /// Determines whether data from reader is a video type.
    ///
    /// # Examples
    ///
    /// See [`is_video_read`](./fn.is_video_read.html).
    pub fn is_video_read<R>(&self, r: &mut R) -> io::Result<bool>
    where
        R: Read + Seek,
    {
        self.is_type_read(r, MatcherType::Video)
    }

    /// Determines whether data from reader is one of the custom types added.
    pub fn is_custom_read<R>(&self, r: &mut R) -> io::Result<bool>
    where
        R: Read + Seek,
    {
        self.is_type_read(r, MatcherType::Custom)
    }

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
