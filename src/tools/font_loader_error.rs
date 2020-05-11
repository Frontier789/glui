use std::io::Error as IoError;
use tools::FontError;

#[derive(Debug)]
pub enum FontLoaderError {
    IoError(IoError),
    NotFound,
    UnrecognizedFormat,
    IllFormed,
}

impl From<IoError> for FontLoaderError {
    fn from(e: IoError) -> FontLoaderError {
        FontLoaderError::IoError(e)
    }
}

impl From<FontError> for FontLoaderError {
    fn from(e: FontError) -> FontLoaderError {
        match e {
            FontError::IoError(e) => e.into(),
            FontError::RusttypeError(e) => e.into(),
        }
    }
}

impl From<rusttype::Error> for FontLoaderError {
    fn from(e: rusttype::Error) -> FontLoaderError {
        match e {
            rusttype::Error::IllFormed => FontLoaderError::IllFormed,
            rusttype::Error::UnrecognizedFormat => FontLoaderError::UnrecognizedFormat,
            _ => FontLoaderError::NotFound,
        }
    }
}
