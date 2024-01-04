use std::fmt::Display;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct AttributeName(String);

impl From<String> for AttributeName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for AttributeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}