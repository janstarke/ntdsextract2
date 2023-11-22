use std::num::TryFromIntError;

use libesedb::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError<'v> {
    #[error("this value has no data")]
    ValueIsMissing,

    #[error("invalid value detected: '{0:?}'")]
    InvalidValueDetected(&'v Value),

    #[error("unable to convert integer '{value:?}' to {self}: {why}")]
    IntegerConversionError {
        value: &'v Value,
        intended_type: &'static str,
        why: TryFromIntError,
    },

    #[error("unable to convert '{value:?}' to {self}: {why}")]
    MiscConversionError {
        value: &'v Value,
        intended_type: &'static str,
        why: anyhow::Error,
    },
}

pub type ConversionResult<'v, T> = Result<T, ConversionError<'v>>;