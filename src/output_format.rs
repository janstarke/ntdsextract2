use crate::output::{CsvWriter, JsonLinesWriter, JsonWriter, Writer};

#[derive(clap::ValueEnum, Clone, Copy)]
pub enum OutputFormat {
    Csv,
    Json,
    JsonLines,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Csv
    }
}

impl Writer for OutputFormat {
    fn write_typenames<I>(&self, names: I) -> anyhow::Result<()>
    where
        I: Iterator<Item = String>,
    {
        match self {
            OutputFormat::Csv => CsvWriter.write_typenames(names),
            OutputFormat::Json => JsonWriter.write_typenames(names),
            OutputFormat::JsonLines => JsonLinesWriter.write_typenames(names),
        }
    }
}
