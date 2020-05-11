#[derive(Debug)]
pub enum FontError {
    RusttypeError(rusttype::Error),
    IoError(std::io::Error),
}

impl From<std::io::Error> for FontError {
    fn from(e: std::io::Error) -> FontError {
        FontError::IoError(e)
    }
}

impl From<rusttype::Error> for FontError {
    fn from(e: rusttype::Error) -> FontError {
        FontError::RusttypeError(e)
    }
}
