use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FormattedValue<T: Display> {
    NoValue,
    Hide,
    Value(T)
}
