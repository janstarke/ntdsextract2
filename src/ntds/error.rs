use std::num::TryFromIntError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("this value has no data")]
    ValueIsMissing,

    #[error("invalid value detected: '{0:?}'")]
    InvalidValueDetected(String),

    #[error("unable to convert integer '{value:?}' to {self}: {why}")]
    IntegerConversionError {
        value: String,
        intended_type: &'static str,
        why: TryFromIntError,
    },

    #[error("unable to convert '{value:?}' to {self}: {why}")]
    MiscConversionError {
        value: String,
        intended_type: &'static str,
        why: anyhow::Error,
    },

    #[error("no schema record found")]
    MissingSchemaRecord,

    #[error("The schema record has no children")]
    SchemaRecordHasNoChildren,
}

pub type Result<T> = core::result::Result<T, Error>;
