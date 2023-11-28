use serde::Serialize;

use crate::OutputOptions;

use super::DataTableRecord;

pub trait Object: Serialize + Sized {
    fn new(dbrecord: DataTableRecord, options: &OutputOptions) -> Result<Self, anyhow::Error>;
}
