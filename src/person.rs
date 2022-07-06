use serde::Serialize;

use crate::{dbrecord::DbRecord, ColumnInfoMapping};
use anyhow::Result;

#[derive(Serialize)]
pub (crate) struct Person {
    sid: String,
    sam_account_name: String,
    user_principal_name: String
}

impl Person {
    pub fn from(dbrecord: DbRecord, mapping: &ColumnInfoMapping) -> Result<Self> {
        Ok(Self {
            sid: dbrecord.ds_sidindex(mapping)?,
            sam_account_name: dbrecord.ds_samaccount_name_index(mapping)?,
            user_principal_name: dbrecord.ds_user_principal_name_index(mapping)?,
        })
    }
}