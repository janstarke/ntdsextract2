use libesedb::Column;

pub struct CColumn {
    name: String,
}

impl<'a> TryFrom<Column<'a>> for CColumn {
    type Error = std::io::Error;

    fn try_from(col: Column<'a>) -> Result<Self, Self::Error> {
        Ok(Self{
            name: col.name()?,
        })
    }
}

impl CColumn {
    pub fn name(&self) -> &str {
        &self.name[..]
    }
}