use std::path::Path;

use anyhow::Result;
use clap::Parser;
use libesedb::EseDb;
use libntdsextract2::{CDatabase, EntryId, EsedbInfo, OutputOptions};
use simplelog::{Config, TermLogger};

mod cli;
use cli::*;

fn main() -> Result<()> {
    let cli = Args::parse();
    let _ = TermLogger::init(
        cli.verbose.log_level_filter(),
        Config::default(),
        simplelog::TerminalMode::Stderr,
        simplelog::ColorChoice::Auto,
    );

    let ntds_path = Path::new(&cli.ntds_file);
    if !(ntds_path.exists() && ntds_path.is_file()) {
        eprintln!("unable to open '{}'", cli.ntds_file);
        std::process::exit(-1);
    }

    let esedb = EseDb::open(&cli.ntds_file)?;
    let info = EsedbInfo::try_from(&esedb)?;
    let database = CDatabase::new(&info)?;

    let mut options = OutputOptions::default();
    options.set_display_all_attributes(cli.command.display_all_attributes());
    options.set_flat_serialization(cli.command.flat_serialization());
    options.set_format(cli.command.format());

    match &cli.command {
        Commands::Group { .. } => database.data_table().show_groups(&options),
        Commands::User { .. } => database.data_table().show_users(&options),
        Commands::Computer { .. } => database.data_table().show_computers(&options),
        Commands::Types { .. } => database.data_table().show_type_names(&options),
        Commands::Timeline { all_objects } => {
            options.set_show_all_objects(*all_objects);
            database
                .data_table()
                .show_timeline(&options, database.link_table())
        }
        Commands::Tree { max_depth } => Ok(database.data_table().show_tree(*max_depth)?),
        Commands::Entry { entry_id, use_sid } => {
            let id = if *use_sid {
                EntryId::Rid((*entry_id).try_into().unwrap())
            } else {
                EntryId::Id(*entry_id)
            };
            Ok(database.data_table().show_entry(id)?)
        }
        Commands::Search { regex, ignore_case } => {
            let regex = if *ignore_case {
                format!("(?i:{regex})")
            } else {
                regex.to_owned()
            };
            database.data_table().search_entries(&regex)
        }
    }
}
