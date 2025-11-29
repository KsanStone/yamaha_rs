use crate::ResponseCode;
use serde::{Serialize, Serializer};
use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ResponseError(code) => write!(f, "Yamaha Device Error: {:?}", code),
            Error::InternalError(e) => write!(f, "Internal Error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InternalError::DeserializationError(e) => write!(f, "JSON Error: {}", e),
            InternalError::IoError(e) => write!(f, "IO Error: {}", e),
        }
    }
}

impl std::error::Error for InternalError {}

impl From<std::io::Error> for InternalError {
    fn from(err: std::io::Error) -> Self {
        InternalError::IoError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::InternalError(InternalError::DeserializationError(err))
    }
}

impl From<InternalError> for Error {
    fn from(err: InternalError) -> Self {
        Error::InternalError(err)
    }
}

#[derive(Debug)]
pub enum Error {
    ResponseError(ResponseCode),
    InternalError(InternalError),
}

/// Returned when the library fails to interface with the Yamaha device.
#[derive(Debug)]
pub enum InternalError {
    DeserializationError(serde_json::Error),
    IoError(std::io::Error),
}

impl Serialize for InternalError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(
            match self {
                InternalError::DeserializationError(e) => format!("DeserializationError: {}", e),
                InternalError::IoError(e) => format!("IoError: {}", e),
            }
            .as_str(),
        )
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(
            match self {
                Error::ResponseError(e) => format!("ResponseError: {}", e),
                Error::InternalError(e) => serde_json::to_string(e).unwrap(),
            }
            .as_str(),
        )
    }
}
