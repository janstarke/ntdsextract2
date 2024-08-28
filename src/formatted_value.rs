use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FormattedValue<T: Display> {
    NoValue,
    Hide,
    Value(T)
}
