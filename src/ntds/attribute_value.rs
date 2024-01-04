use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AttributeValue(String);

impl From<String> for AttributeValue {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for AttributeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AttributeValue {
    pub fn value(&self) -> &str {
        &self.0[..]
    }
}
