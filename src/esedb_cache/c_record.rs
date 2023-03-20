use libesedb::{Record, Value};

pub struct CRecord {
    values: Vec<Value>
}

impl<'a> TryFrom<Record<'a>> for CRecord {
    type Error = std::io::Error;

    fn try_from(record: Record<'a>) -> Result<Self, Self::Error> {
        let mut values = Vec::new();
        for value in record.iter_values()? {
            let value = match value {
                Ok(v) => v,
                Err(why) => {
                    //log::warn!("unable to read value: {why}");
                    Value::Null(())
                }
            };
            values.push(value);
        }
        Ok(Self {
            values
        })
    }
}

impl IntoIterator for CRecord {
    type Item = Value;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl CRecord {
    pub fn count_values(&self) -> i32 {
        self.values.len().try_into().unwrap()
    }
    pub fn value(&self, entry: i32) -> Option<&Value> {
        self.values.get(usize::try_from(entry).unwrap())
    }
}