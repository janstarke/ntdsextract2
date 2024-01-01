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
pub struct NameWithGuid {
    name: String,
    guid: Option<String>,
}

impl Display for NameWithGuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.guid {
            Some(guid) => write!(f, "{} (deleted, was in {guid}", self.name),
            None => self.name.fmt(f),
        }
    }
}

impl FromValue for NameWithGuid {
    fn from_value_opt(value: &Value) -> ntds::Result<Option<Self>>
    where
        Self: Sized,
    {
        match value {
            Value::Text(s) | Value::LargeText(s) => {
                match regex_captures!(r#"^(?P<name>.*)(\x0aDEL:(?P<guid>\d+))?$"#, s) {
                    None => Err(ntds::Error::InvalidValueDetected(value.to_string())),
                    Some((_, name, _, guid)) => {
                        if guid.is_empty() {
                            Ok(Some(Self {
                                name: name.to_owned(),
                                guid: None,
                            }))
                        } else {
                            Ok(Some(Self {
                                name: name.to_owned(),
                                guid: Some(guid.to_owned()),
                            }))
                        }
                    }
                }
            }
            Value::Null(()) => Ok(None),
            _ => Err(ntds::Error::InvalidValueDetected(value.to_string())),
        }
    }
}

impl TryFrom<&str> for NameWithGuid {
    type Error = anyhow::Error;

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        match regex_captures!(r#"^(?P<name>.*)( -- DEL:(?P<guid>\d+))?$"#, v) {
            None => bail!("invalid object name: '{v}'"),
            Some((_, name, _, guid)) => {
                if guid.is_empty() {
                    Ok(NameWithGuid {
                        name: name.to_owned(),
                        guid: None,
                    })
                } else {
                    Ok(NameWithGuid {
                        name: name.to_owned(),
                        guid: Some(guid.to_owned()),
                    })
                }
            }
        }
    }
}

impl TryFrom<String> for NameWithGuid{
    type Error = anyhow::Error;

    fn try_from(v: String) -> Result<Self, Self::Error> {
        Self::try_from(&v[..])
    }

}

impl Serialize for NameWithGuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(guid) = &self.guid {
            serializer.serialize_str(&format!("{} -- DEL:{guid}", self.name))
        } else {
            serializer.serialize_str(&self.name)
        }
    }
}

impl<'de> Deserialize<'de> for NameWithGuid {
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
    type Value = NameWithGuid;

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
        NameWithGuid::try_from(v).or(Err(E::custom(format!("invalid object name: '{v}'"))))
    }
}
