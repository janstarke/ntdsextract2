use getset::Getters;

use super::ColumnIndex;

#[derive(Getters)]
#[getset(get="pub")]
pub struct Column {
    index: ColumnIndex,
    name: String,
}

impl<'a> Column {
    pub fn new(col: libesedb::Column<'a>, index: ColumnIndex) -> Result<Self, std::io::Error> {
        // log::warn!("caching column {name}", name=col.name()?);
        Ok(Self{
            name: col.name()?,
            index
        })
    }
}
