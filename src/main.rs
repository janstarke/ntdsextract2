use std::path::Path;

use clap::{Parser, Subcommand};
use libesedb::EseDb;
use libntdsextract2::{OutputFormat, CTable, DataTableExt, set_display_all_attributes, set_do_flat_serialization, EntryId, CDatabase};
use simplelog::{Config, TermLogger};
use anyhow::Result;

#[derive(Subcommand)]
enum Commands {
    /// Display user accounts
    User {
        /// Output format
        #[clap(value_enum, short('F'), long("format"), default_value_t = OutputFormat::Csv)]
        format: OutputFormat,

        /// show all non-empty values. This option is ignored when CSV-Output is selected
        #[clap(short('A'), long("show-all"))]
        show_all: bool
    },

    /// Display groups
    Group {
        /// Output format
        #[clap(value_enum, short('F'), long("format"), default_value_t = OutputFormat::Csv)]
        format: OutputFormat,

        /// show all non-empty values. This option is ignored when CSV-Output is selected
        #[clap(short('A'), long("show-all"))]
        show_all: bool
    },

    /// display computer accounts
    Computer {
        /// Output format
        #[clap(value_enum, short('F'), long("format"), default_value_t = OutputFormat::Csv)]
        format: OutputFormat,

        /// show all non-empty values. This option is ignored when CSV-Output is selected
        #[clap(short('A'), long("show-all"))]
        show_all: bool
    },

    /// create a timeline (in bodyfile format)
    Timeline {
        /// show objects of any type (this might be a lot)
        #[clap(long("all-objects"))]
        all_objects: bool
    },

    /// list all defined types
    Types {
        /// Output format
        #[clap(value_enum, short('F'), long("format"), default_value_t = OutputFormat::Csv)]
        format: OutputFormat,
    },

    /// display the directory information tree
    Tree {
        /// maximum recursion depth 
        #[clap(long("max-depth"), default_value_t=4)]
        max_depth: u8
    },

    /// display one single entry from the directory information tree
    Entry {
        /// id of the entry to show
        entry_id: i32,

        /// search for SID instead for NTDS.DIT entry id.
        /// <ENTRY_ID> will be interpreted as RID, wich is the last part of the SID;
        /// e.g. 500 will return the Administrator account
        #[clap(long("sid"))]
        use_sid: bool
    },

    /// search for entries whose values match to some regular expression
    Search {
        /// regular expression to match against
        regex: String,

        /// case-insensitive search (ignore case)
        #[clap(short('i'), long("ignore-case"))]
        ignore_case: bool
    }
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
    let database = CDatabase::try_from(&esedb)?;

    set_display_all_attributes (
       match &cli.command {
        Commands::User{format: OutputFormat::Json, show_all} |
        Commands::User{format: OutputFormat::JsonLines, show_all} |
        Commands::Computer{format: OutputFormat::Json, show_all} |
        Commands::Computer{format: OutputFormat::JsonLines, show_all} => *show_all,
        _ => false,
       } 
    );

    set_do_flat_serialization(
        matches!(&cli.command, 
            Commands::User{format: OutputFormat::Csv, ..} |
            Commands::Computer{format: OutputFormat::Csv, ..} |
            Commands::Group{format: OutputFormat::Csv, ..} |
            Commands::Timeline { .. })  
    );

    match &cli.command {
        Commands::Group { format, .. } => database.data_table().show_groups(format),
        Commands::User { format, ..} => database.data_table().show_users(format),
        Commands::Computer { format, .. } => database.data_table().show_computers(format),
        Commands::Types { format, .. } => database.data_table().show_type_names(format),
        Commands::Timeline {all_objects} => database.data_table().show_timeline(*all_objects),
        Commands::Tree { max_depth } => database.data_table().show_tree(*max_depth),
        Commands::Entry { entry_id , use_sid} => {
            let id = if *use_sid {
                EntryId::Rid((*entry_id).try_into().unwrap())
            } else {
                EntryId::Id(*entry_id)
            };
            database.data_table().show_entry(id)
        }
        Commands::Search { regex , ignore_case} => {
            let regex = if *ignore_case {
                format!("(?i:{regex})")
            } else {
                regex.to_owned()
            };
            database.data_table().search_entries(&regex)
        }
    }
}

