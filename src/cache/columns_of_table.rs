use std::{
    collections::HashMap,
    ops::{Deref, Index},
};

use libesedb::Table;

use super::{Column, ColumnIndex};

pub struct ColumnsOfTable {
    ids: Vec<Column>,
    names: HashMap<String, ColumnIndex>,
}

impl TryFrom<&Table<'_>> for ColumnsOfTable {
    type Error = std::io::Error;

    fn try_from(table: &Table) -> Result<Self, Self::Error> {
        let mut ids = Vec::new();
        let mut names = HashMap::new();
        for (column, index) in table.iter_columns()?.zip(0..) {
            let column = Column::new(column?, index.into())?;
            names.insert(column.name().to_owned(), index.into());
            ids.push(column);
        }
        Ok(Self { ids, names })
    }
}

impl ColumnsOfTable {
    pub fn iter(&self) -> impl Iterator<Item = &Column> {
        self.ids.iter()
    }
}

impl Index<ColumnIndex> for ColumnsOfTable {
    type Output = Column;

    fn index(&self, index: ColumnIndex) -> &Self::Output {
        self.ids.index(*index as usize)
    }
}

impl Index<&ColumnIndex> for ColumnsOfTable {
    type Output = Column;

    fn index(&self, index: &ColumnIndex) -> &Self::Output {
        self.ids.index(*(index.deref()) as usize)
    }
}

impl Index<&str> for ColumnsOfTable {
    type Output = Column;

    fn index(&self, name: &str) -> &Self::Output {
        let index = self.names[name];
        self.ids.index(*(index.deref()) as usize)
    }
}
