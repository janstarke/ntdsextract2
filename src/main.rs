use std::{path::Path};

use clap::{Parser, Subcommand};
use libesedb::{EseDb};
use simplelog::{Config, TermLogger};
use anyhow::{Result};

use crate::{column_info_mapping::*, data_table_ext::DataTableExt};

mod dbrecord;
mod person;
mod constants;
mod column_information;
mod column_info_mapping;
mod data_table_ext;

#[derive(clap::ArgEnum, Clone)]
enum OutputFormat{
    CSV
}

#[derive(Subcommand)]
enum Commands {
    /// Display user accounts
    User {
        #[clap(arg_enum, short('F'), long("format"), default_value_t = OutputFormat::CSV)]
        format: OutputFormat
    },

    /// display computer accounts
    Computer {
        #[clap(arg_enum, short('F'), long("format"), default_value_t = OutputFormat::CSV)]
        format: OutputFormat
    },

    /// create a timeline (in bodyfile format)
    Timeline
}


#[derive(Parser)]
#[clap(name="ntds", author, version, about, long_about = None)]
struct Args {

    #[clap(subcommand)]
    pub (crate) command: Commands,

    /// name of the file to analyze
    pub (crate) ntds_file: String,

    #[clap(flatten)]
    pub (crate) verbose: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<()> {
    let cli = Args::parse();
    let _ = TermLogger::init(
        cli.verbose.log_level_filter(), 
        Config::default(),
        simplelog::TerminalMode::Stderr,
        simplelog::ColorChoice::Auto);

    let ntds_path = Path::new(&cli.ntds_file);
    if ! (ntds_path.exists() && ntds_path.is_file()) {
        eprintln!("unable to open '{}'", cli.ntds_file);
        std::process::exit(-1);
    }

    let esedb = EseDb::open(&cli.ntds_file)?;
    log::info!("Db load finished");

    let data_table  = DataTableExt::from(esedb.table_by_name("datatable")?)?;

    match &cli.command {
        Commands::User { format} => data_table.show_users(format),
        Commands::Computer { format } => data_table.show_computers(format),
        Commands::Timeline => data_table.show_timeline(),
    }
}

