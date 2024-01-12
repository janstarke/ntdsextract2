use std::fmt::{Display, Formatter};

#[derive(PartialEq)]
pub enum Value {
    Null(()),
    Bool(bool),
    U8(u8),
    I16(i16),
    I32(i32),
    Currency(i64),
    F32(f32),
    F64(f64),
    DateTime(u64),
    Binary(Box<Vec<u8>>),
    Text(Box<String>),
    LargeBinary(Box<Vec<u8>>),
    LargeText(Box<String>),
    SuperLarge(Box<Vec<u8>>),
    U32(u32),
    I64(i64),
    Guid(Box<Vec<u8>>),
    U16(u16),
    Long,
    Multi
}

impl Eq for Value {

}

impl From<libesedb::Value> for Value {
    fn from(value: libesedb::Value) -> Self {
        match value {
            libesedb::Value::Null(_) => Self::Null(()),
            libesedb::Value::Bool(v) => Self::Bool(v),
            libesedb::Value::U8(v) => Self::U8(v),
            libesedb::Value::I16(v) => Self::I16(v),
            libesedb::Value::I32(v) => Self::I32(v),
            libesedb::Value::Currency(v) => Self::Currency(v),
            libesedb::Value::F32(v) => Self::F32(v),
            libesedb::Value::F64(v) => Self::F64(v),
            libesedb::Value::DateTime(v) => Self::DateTime(v),
            libesedb::Value::Binary(v) => Self::Binary(Box::new(Vec::from(&v[..]))),
            libesedb::Value::Text(v) => Self::Text(Box::new(v)),
            libesedb::Value::LargeBinary(v) => Self::LargeBinary(Box::new(Vec::from(&v[..]))),
            libesedb::Value::LargeText(v) => Self::LargeText(Box::new(v)),
            libesedb::Value::SuperLarge(v) => Self::SuperLarge(Box::new(Vec::from(&v[..]))),
            libesedb::Value::U32(v) => Self::U32(v),
            libesedb::Value::I64(v) => Self::I64(v),
            libesedb::Value::Guid(v) => Self::Guid(Box::new(Vec::from(&v[..]))),
            libesedb::Value::U16(v) => Self::U16(v),
            libesedb::Value::Long => Self::Long,
            libesedb::Value::Multi => Self::Multi,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null(_) => write!(f, "Null(())"),
            Value::Bool(v) => write!(f, "Bool({v})"),
            Value::U8(v) => write!(f, "U8({v})"),
            Value::I16(v) => write!(f, "I16({v})"),
            Value::I32(v) => write!(f, "I32({v})"),
            Value::Currency(v) => write!(f, "Currency({v})"),
            Value::F32(v) => write!(f, "F32({v})"),
            Value::F64(v) => write!(f, "F64({v})"),
            Value::DateTime(v) => write!(f, "DateTime({v})"),
            Value::Binary(v) => write!(f, "Binary({v:?})"),
            Value::Text(v) => write!(f, "Text({v})"),
            Value::LargeBinary(v) => write!(f, "LargeBinary({v:?})"),
            Value::LargeText(v) => write!(f, "LargeText({v})"),
            Value::SuperLarge(v) => write!(f, "SuperLarge({v:?})"),
            Value::U32(v) => write!(f, "U32({v})"),
            Value::I64(v) => write!(f, "I64({v})"),
            Value::Guid(v) => write!(f, "Guid({v:?})"),
            Value::U16(v) => write!(f, "U16({v})"),
            Value::Long => write!(f, "Long"),
            Value::Multi => write!(f, "Multi"),
        }
    }
}