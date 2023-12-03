pub struct Column {
    name: String,
}

impl<'a> TryFrom<libesedb::Column<'a>> for Column {
    type Error = std::io::Error;

    fn try_from(col: libesedb::Column<'a>) -> Result<Self, Self::Error> {
        // log::warn!("caching column {name}", name=col.name()?);
        Ok(Self{
            name: col.name()?,
        })
    }
}

impl Column {
    pub fn name(&self) -> &str {
        &self.name[..]
    }
}