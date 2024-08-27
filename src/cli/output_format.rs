use strum::Display;

use crate::cli::output::{CsvWriter, JsonLinesWriter, JsonWriter, Writer};


#[derive(clap::ValueEnum, Clone, Copy, Display, Eq, PartialEq)]
pub enum OutputFormat {
    #[strum(serialize = "csv")]
    Csv,

    #[strum(serialize = "json")]
    Json,

    #[strum(serialize = "json-lines")]
    JsonLines,
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
