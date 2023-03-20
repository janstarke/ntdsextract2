use anyhow::{anyhow, bail, Result};

use chrono::{DateTime, Duration, NaiveDate, Utc};
use libesedb::Value;

pub(crate) trait FromValue
where
    Self: Sized,
{
    fn from_value(value: &Value, attribute_name: &str) -> Result<Self> {
        if matches!(value, Value::Null(())) {
            bail!("missing value in '{attribute_name}'");
        } else {
            match Self::from_value_opt(value, attribute_name)? {
                Some(x) => Ok(x),
                None => bail!("value of '{attribute_name}' has no content"),
            }
        }
    }
    fn from_value_opt(value: &Value, attribute_name: &str) -> Result<Option<Self>>;
}

impl FromValue for i32 {
    fn from_value_opt(value: &Value, attribute_name: &str) -> Result<Option<Self>> {
        match value {
            Value::I32(val) => Ok(Some(*val)),
            Value::Null(()) => Ok(None),
            _ => bail!("invalid value detected: {value:?} in attribute '{attribute_name}'"),
        }
    }
}

impl FromValue for u32 {
    fn from_value_opt(value: &Value, attribute_name: &str) -> Result<Option<Self>> {
        match value {
            Value::U8(val) => Ok(Some((*val).into())),
            Value::U16(val) => Ok(Some((*val).into())),
            Value::U32(val) => Ok(Some(*val)),
            Value::I16(val) => Ok(Some((*val).try_into()?)),
            Value::I32(val) => Ok(Some((*val).try_into()?)),
            Value::Null(()) => Ok(None),
            _ => bail!("invalid value detected: {value:?} in attribute '{attribute_name}'"),
        }
    }
}

impl FromValue for String {
    fn from_value_opt(value: &Value, attribute_name: &str) -> Result<Option<Self>> {
        match value {
            Value::Text(val) => Ok(Some(val.to_owned())),
            Value::LargeText(val) => Ok(Some(val.to_owned())),
            Value::Binary(val) | Value::LargeBinary(val) => Ok(Some(hex::encode(val))),
            Value::Null(()) => Ok(None),
            _ => bail!("invalid value detected: {value:?} in field {attribute_name}"),
        }
    }
}

pub(crate) type UtcDatetime = DateTime<Utc>;
impl FromValue for UtcDatetime {
    fn from_value_opt(value: &Value, attrib_name: &str) -> Result<Option<Self>> {
        match value {
            Value::Currency(val) => Ok(Some(currency_to_datetime(*val))),
            Value::Null(()) => Ok(None),
            _ => Err(anyhow!(
                "invalid value detected: {:?} in field {}",
                value,
                attrib_name
            )),
        }
    }
}

fn currency_to_datetime(val: i64) -> DateTime<Utc> {
    let dt_base = DateTime::<Utc>::from_utc(NaiveDate::from_ymd(1601, 1, 1).and_hms(0, 0, 0), Utc);
    let duration = Duration::microseconds(val / 10);
    dt_base + duration
}

