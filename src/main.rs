use std::{path::Path};

use clap::{Parser, Subcommand};
use libesedb::{EseDb};
use simplelog::{Config, TermLogger};
use anyhow::{Result};

use crate::{column_info_mapping::*, data_table_ext::DataTableExt};

mod dbrecord;
mod person;
mod computer;
mod constants;
mod column_information;
mod column_info_mapping;
mod data_table_ext;

/// this needs to be a global variable, 
/// because it is read by serialization code, which has no state by default
static mut DISPLAY_ALL_ATTRIBUTES: bool = false;

pub (crate) fn display_all_attributes() -> bool {
    unsafe {
        DISPLAY_ALL_ATTRIBUTES
    }
}

pub (crate) fn skip_all_attributes<T>(_t: &T) -> bool {
    ! display_all_attributes()
}

fn set_display_all_attributes(val: bool) {
    unsafe {
        DISPLAY_ALL_ATTRIBUTES = val
    }
}

#[derive(clap::ArgEnum, Clone)]
enum OutputFormat{
    Csv,
    Json,
    JsonLines
}

#[derive(Subcommand)]
enum Commands {
    /// Display user accounts
    User {
        /// Output format
        #[clap(arg_enum, short('F'), long("format"), default_value_t = OutputFormat::Csv)]
        format: OutputFormat,

        /// show all non-empty values. This option is ignored when CSV-Output is selected
        #[clap(short('A'), long("show-all"))]
        show_all: bool
    },

    /// display computer accounts
    Computer {
        /// Output format
        #[clap(arg_enum, short('F'), long("format"), default_value_t = OutputFormat::Csv)]
        format: OutputFormat,

        /// show all non-empty values. This option is ignored when CSV-Output is selected
        #[clap(short('A'), long("show-all"))]
        show_all: bool
    },

    /// create a timeline (in bodyfile format)
    Timeline
}


#[derive(Parser)]
#[clap(name="ntdsextract2", author, version, about, long_about = None)]
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

    set_display_all_attributes(
       match &cli.command {
        Commands::User{format: OutputFormat::Json, show_all} |
        Commands::User{format: OutputFormat::JsonLines, show_all} |
        Commands::Computer{format: OutputFormat::Json, show_all} |
        Commands::Computer{format: OutputFormat::JsonLines, show_all} => *show_all,
        _ => false,
       } 
    );

    match &cli.command {
        Commands::User { format, ..} => data_table.show_users(format),
        Commands::Computer { format, .. } => data_table.show_computers(format),
        Commands::Timeline => data_table.show_timeline(),
    }
}

