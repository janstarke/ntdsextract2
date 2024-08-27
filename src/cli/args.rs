use clap::Parser;
use getset::Getters;

use super::Commands;


#[derive(Parser, Getters)]
#[getset(get="pub")]
#[clap(name="ntdsextract2", author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub(crate) command: Commands,

    /// name of the file to analyze
    pub(crate) ntds_file: String,

    #[clap(flatten)]
    pub(crate) verbose: clap_verbosity_flag::Verbosity,
}
