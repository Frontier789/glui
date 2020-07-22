#![cfg(feature = "serializable")]
extern crate serde;
extern crate serde_json;

use self::serde::de::DeserializeOwned;
use self::serde::Serialize;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

pub trait SerdeJsonQuick {
    fn save_json<P: AsRef<Path>>(&self, file: P) -> Result<(), SerdeError>;
    fn load_json<P: AsRef<Path>>(file: P) -> Result<Self, SerdeError>
    where
        Self: Sized;
}

#[derive(Debug)]
pub enum SerdeError {
    IoError(io::Error),
    SerderError(serde_json::Error),
}

impl From<io::Error> for SerdeError {
    fn from(e: io::Error) -> Self {
        SerdeError::IoError(e)
    }
}
impl From<serde_json::Error> for SerdeError {
    fn from(e: serde_json::Error) -> Self {
        SerdeError::SerderError(e)
    }
}

impl<T> SerdeJsonQuick for T
where
    T: Serialize + DeserializeOwned,
{
    fn save_json<P: AsRef<Path>>(&self, file: P) -> Result<(), SerdeError> {
        let str = serde_json::to_string(&self)?;
        let mut file = File::create(file)?;
        write!(file, "{}", str)?;
        Ok(())
    }

    fn load_json<P: AsRef<Path>>(file: P) -> Result<Self, SerdeError>
    where
        Self: Sized,
    {
        let file_str = std::fs::read_to_string(file)?;
        let object: Self = serde_json::from_str(&file_str)?;
        Ok(object)
    }
}
