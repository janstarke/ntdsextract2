use std::fmt::{Display, Formatter};
use std::str::FromStr;

use anyhow::bail;
use getset::Getters;
use lazy_regex::regex_captures;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::cache::Value;
use crate::ntds;
use crate::value::FromValue;

use super::Guid;

#[derive(Getters, Eq, PartialEq, Clone, Hash)]
#[getset(get = "pub")]
pub struct Rdn {
    name: String,
    deleted_from_container: Option<Guid>, //TODO: should by a UUID
    conflicting_objects: Vec<Guid>,       //TODO: should be UUIDs
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
                let mut deleted_from_container = None;
                let mut conflicting_objects = Vec::new();

                for line in lines {
                    if let Some(guid) = line.strip_prefix("DEL:") {
                        if deleted_from_container.is_some() {
                            return Err(ntds::Error::InvalidValueDetected(value.to_string(), "Rdn (text)"));
                        }
                        deleted_from_container = Some(Guid::from_str(guid)?);
                    } else if let Some(guid) = line.strip_prefix("CNF:") {
                        conflicting_objects.push(Guid::from_str(guid)?);
                    } else {
                        log::warn!("unexpected value in Rdn field: '{line}'");
                    }
                }

                Ok(Some(Self {
                    name,
                    deleted_from_container,
                    conflicting_objects,
                }))
            }
            Value::Null(()) => Ok(None),
            Value::Long(_) => {
                log::warn!("no support for LONG columns yet, generating a random value");
                Ok(Some(Self{
                    name: uuid::Uuid::new_v4().to_string(),
                    deleted_from_container: None,
                    conflicting_objects: Vec::new(),
                }))
            }
            _ => Err(ntds::Error::InvalidValueDetected(value.to_string(), "Rdn (text)")),
        }
    }
}

impl TryFrom<&str> for Rdn {
    type Error = anyhow::Error;

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        match regex_captures!(r#"^(?P<name>.*)( -- DEL:(?P<guid>\d+))?$"#, v) {
            None => bail!("invalid object name: '{v}'"),
            Some((_, name, _, deleted_from_container)) => {
                if deleted_from_container.is_empty() {
                    Ok(Rdn {
                        name: name.to_owned(),
                        deleted_from_container: None,
                        conflicting_objects: Vec::new(),
                    })
                } else {
                    Ok(Rdn {
                        name: name.to_owned(),
                        deleted_from_container: Some(Guid::from_str(deleted_from_container)?),
                        conflicting_objects: Vec::new(),
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
