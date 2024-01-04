use std::str::FromStr;

use getset::Getters;

use crate::ntds::{AttributeName, NtdsAttributeId};

use super::ColumnIndex;

#[derive(Getters)]
#[getset(get = "pub")]
pub struct Column {
    index: ColumnIndex,
    name: String,
    attribute_id: Option<NtdsAttributeId>,
    attribute_name: Option<AttributeName>,
}

impl<'a> Column {
    pub fn new(
        col: libesedb::Column<'a>,
        index: ColumnIndex,
    ) -> Result<Self, std::io::Error> {
        // log::warn!("caching column {name}", name=col.name()?);
        let name = col.name()?;
        if let Ok(attribute_id) = NtdsAttributeId::from_str(&name) {
            let attribute_name: &str = attribute_id.into();
            Ok(Self {
                name,
                index,
                attribute_id: Some(attribute_id),
                attribute_name: Some(attribute_name.to_string().into()),
            })
        } else {
            Ok(Self {
                name,
                index,
                attribute_id: None,
                attribute_name: None,
            })
        }
    }
}
