use std::fmt::Display;

use serde::Deserialize;

#[derive(Deserialize)]
pub enum FormattedValue<T: Display> {
    NoValue,
    Hide,
    Value(T)
}
