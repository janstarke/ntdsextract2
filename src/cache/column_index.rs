use std::ops::Deref;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct ColumnIndex(i32);

impl From<i32> for ColumnIndex {
    fn from(v: i32) -> Self {
        Self(v)
    }
}

impl From<&i32> for ColumnIndex {
    fn from(v: &i32) -> Self {
        Self(*v)
    }
}

impl From<&ColumnIndex> for ColumnIndex {
    fn from(v: &ColumnIndex) -> Self {
        Self(v.0)
    }
}

impl Deref for ColumnIndex {
    type Target=i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}