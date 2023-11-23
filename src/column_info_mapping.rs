use std::{ops::Index, collections::HashMap};

use crate::{column_information::ColumnInformation, ntds::NtdsAttributeId};
use anyhow::Result;
use libesedb::Table;

pub struct ColumnInfoMapping {
    mapping: HashMap<NtdsAttributeId, ColumnInformation>,
}

impl Index<NtdsAttributeId> for ColumnInfoMapping {
    type Output = ColumnInformation;

    fn index(&self, index: NtdsAttributeId) -> &Self::Output {
        self.mapping.index(&index)
    }
}

impl TryFrom<&Table<'_>> for ColumnInfoMapping {
    type Error = anyhow::Error;
    fn try_from(data_table: &Table) -> Result<Self, Self::Error> {
        let mut mapping = HashMap::new();
        for index in 0..data_table.count_columns()? {
            let column = data_table.column(index)?;
            if let Ok(column_id) = NtdsAttributeId::try_from(&column.name()?[..]) {
                let col_info = ColumnInformation::new(
                    index,
                    // column_res.name()?,
                    // column_res.variant()?
                );
                mapping.insert(column_id, col_info);
            }
            //log::info!("found column with name {name}", name=column_res.name());
        }

        Ok(Self { mapping })
    }
}

/*
impl<'a> RecordToBodyfile for DbRecord<'a> {
    fn to_bodyfile(
        &self,
        mapping: &ColumnInfoMapping,
        type_name: &str,
    ) -> Result<Vec<Bodyfile3Line>> {
        let mut res = Vec::new();

        let object_name = self
            .ds_object_name(mapping)?
            .or(self.ds_object_name2(mapping)?)
            .unwrap_or_else(|| "unknown".to_owned());

        add_bodyfile_timestamp!(
            res,
            self.ds_record_time(mapping)?,
            object_name,
            type_name,
            "record creation time"
        );
        add_bodyfile_timestamp!(
            res,
            self.ds_when_created(mapping)?,
            object_name,
            type_name,
            "object created"
        );
        add_bodyfile_timestamp!(
            res,
            self.ds_when_changed(mapping)?,
            object_name,
            type_name,
            "object changed"
        );
        add_bodyfile_timestamp!(
            res,
            self.ds_last_logon(mapping)?,
            object_name,
            type_name,
            "last logon on this DC"
        );
        add_bodyfile_timestamp!(
            res,
            self.ds_last_logon_time_stamp(mapping)?,
            object_name,
            type_name,
            "last logon on any DC"
        );
        add_bodyfile_timestamp!(
            res,
            self.ds_bad_pwd_time(mapping)?,
            object_name,
            type_name,
            "bad pwd time"
        );
        add_bodyfile_timestamp!(
            res,
            self.ds_password_last_set(mapping)?,
            object_name,
            type_name,
            "password last set"
        );

        Ok(res)
    }
}

pub trait IsMemberOf {
    fn member_of(&self) -> Vec<Box<dyn HasMembers>>;
}

pub trait HasMembers {
    fn members(&self) -> Vec<Box<dyn IsMemberOf>>;
}

impl<'a> IsMemberOf for DbRecord<'a> {
    fn member_of(&self) -> Vec<Box<dyn HasMembers>> {
        todo!()
    }
}
 */
