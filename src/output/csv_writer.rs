use super::Writer;

#[derive(Default)]
pub struct CsvWriter;

impl Writer for CsvWriter {
    fn write_typenames<I>(&self, names: I) -> anyhow::Result<()>
    where
        I: Iterator<Item = String>,
    {
        let mut csv_wtr = csv::Writer::from_writer(std::io::stdout());
        anyhow::Result::from_iter(
            names.map(|name| csv_wtr.serialize(name).map_err(|why| anyhow::anyhow!(why))),
        )
    }
}
