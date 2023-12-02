use crate::cache::Value;

pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Null(()) => panic!("unreachable code executed"),
            Value::Bool(v) => format!("{v}"),
            Value::U8(v) => format!("{v}"),
            Value::U16(v) => format!("{v}"),
            Value::U32(v) => format!("{v}"),
            Value::I16(v) => format!("{v}"),
            Value::I32(v) => format!("{v}"),
            Value::I64(v) => format!("{v}"),
            Value::F32(v) => format!("{v}"),
            Value::F64(v) => format!("{v}"),
            Value::Currency(v) => format!("{v}"),
            Value::DateTime(v) => format!("{v}"),
            Value::Binary(v) => hex::encode(v.as_ref()).to_string(),
            Value::Text(v) => v.as_ref().to_owned(),
            Value::LargeBinary(v) => hex::encode(v.as_ref()).to_string(),
            Value::LargeText(v) => v.as_ref().to_owned(),
            Value::SuperLarge(v) => hex::encode(v.as_ref()).to_string(),
            Value::Guid(v) => hex::encode(v.as_ref()).to_string(),
        }
    }
}