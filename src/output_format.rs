#[derive(clap::ValueEnum, Clone, Copy)]
pub enum OutputFormat{
    Csv,
    Json,
    JsonLines
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Csv
    }
}