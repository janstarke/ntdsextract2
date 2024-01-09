use std::fmt::{Display, Formatter};

use anyhow::bail;
use getset::Getters;
use lazy_regex::regex_captures;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::cache::Value;
use crate::ntds;
use crate::value::FromValue;

#[derive(Getters, Eq, PartialEq, Clone, Hash)]
#[getset(get = "pub")]
pub struct Rdn {
    name: String,
    deleted_from_container: Option<String>,
}

impl Display for Rdn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.deleted_from_container {
            Some(_guid) => write!(f, "{} (DELETED)", self.name),
            None => self.name.fmt(f),
        }
    }
}

impl FromValue for Rdn {
    fn from_value_opt(value: &Value) -> ntds::Result<Option<Self>>
    where
        Self: Sized,
    {
        match value {
            Value::Text(s) | Value::LargeText(s) => {
                let mut lines = s.lines();
                let name = lines.next().unwrap().to_string();
                let guid = match lines.next() {
                    None => None,
                    Some(part) => {
                        if let Some(stripped) = part.strip_prefix("DEL:") {
                            Some(stripped.to_string())
                        } else {
                            return Err(ntds::Error::InvalidValueDetected(value.to_string()));
                        }
                    }
                };
                Ok(Some(Self { name, deleted_from_container: guid }))
            }
            Value::Null(()) => Ok(None),
            _ => Err(ntds::Error::InvalidValueDetected(value.to_string())),
        }
    }
}

impl TryFrom<&str> for Rdn {
    type Error = anyhow::Error;

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        match regex_captures!(r#"^(?P<name>.*)( -- DEL:(?P<guid>\d+))?$"#, v) {
            None => bail!("invalid object name: '{v}'"),
            Some((_, name, _, guid)) => {
                if guid.is_empty() {
                    Ok(Rdn {
                        name: name.to_owned(),
                        deleted_from_container: None,
                    })
                } else {
                    Ok(Rdn {
                        name: name.to_owned(),
                        deleted_from_container: Some(guid.to_owned()),
                    })
                }
            }
        }
    }
}

impl TryFrom<String> for Rdn {
    type Error = anyhow::Error;

    fn try_from(v: String) -> Result<Self, Self::Error> {
        Self::try_from(&v[..])
    }
}

impl Serialize for Rdn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(guid) = &self.deleted_from_container {
            serializer.serialize_str(&format!("{} -- DEL:{guid}", self.name))
        } else {
            serializer.serialize_str(&self.name)
        }
    }
}

impl<'de> Deserialize<'de> for Rdn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(NameWithGuidVisitor::default())
    }
}

#[derive(Default)]
pub struct NameWithGuidVisitor {}

impl<'de> Visitor<'de> for NameWithGuidVisitor {
    type Value = Rdn;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "a string, possibly containing information about the original container"
        )
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Rdn::try_from(v).or(Err(E::custom(format!("invalid object name: '{v}'"))))
    }
}
