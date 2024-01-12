use std::num::TryFromIntError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("this value has no data")]
    ValueIsMissing,

    #[error("invalid value detected: '{0:?}'")]
    InvalidValueDetected(String),

    #[error("unable to convert integer '{value:?}' to {intended_type}: {why}")]
    IntegerConversionError {
        value: String,
        intended_type: &'static str,
        why: TryFromIntError,
    },

    #[error("unable to convert '{value:?}' to {intended_type}: {why}")]
    MiscConversionError {
        value: String,
        intended_type: &'static str,
        why: anyhow::Error,
    },

    #[error("no schema record found")]
    MissingSchemaRecord,

    #[error("The schema record has no children")]
    SchemaRecordHasNoChildren,

    #[error("IO Error: {why}")]
    IoError{why: std::io::Error},

    #[error("Invalid UUID: {why}")]
    UuidError{why: uuid::Error}
}

pub type Result<T> = core::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError { why: value }
    }
}


impl From<uuid::Error> for Error {
    fn from(value: uuid::Error) -> Self {
        Self::UuidError { why: value }
    }
}