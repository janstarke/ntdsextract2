use strum::Display;

#[derive(clap::ValueEnum, Clone, Copy, Display)]
pub enum EntryFormat {
    /// use JSON format
    #[strum(serialize = "json")]
    Json,

    /// display a formatted table
    #[strum(serialize = "table")]
    Table,

    /// use a simple key-values based format
    #[strum(serialize = "simple")]
    Simple,
}
