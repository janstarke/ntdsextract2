use std::path::Path;

use anyhow::Result;
use clap::Parser;
use libesedb::EseDb;
use libntdsextract2::{
    CDatabase, CsvSerialization, EntryId, EsedbInfo, JsonSerialization,
};
use libntdsextract2::cli::{Args, Commands, OutputOptions};
use simplelog::{Config, TermLogger};

mod progress_bar;

use cap::Cap;
use std::alloc;

macro_rules! do_with_serialization {
    ($cmd: expr, $db: expr, $function: ident, $options: expr) => {
        if $cmd.flat_serialization() {
            $db.$function::<CsvSerialization>($options)
        } else {
            $db.$function::<JsonSerialization>($options)
        }
    };
}

#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());

fn main() -> Result<()> {
    ALLOCATOR.set_limit(4096 * 1024 * 1024).unwrap();

    let cli = Args::parse();
    let _ = TermLogger::init(
        cli.verbose().log_level_filter(),
        Config::default(),
        simplelog::TerminalMode::Stderr,
        simplelog::ColorChoice::Auto,
    );

    let ntds_path = Path::new(cli.ntds_file());
    if !(ntds_path.exists() && ntds_path.is_file()) {
        eprintln!("unable to open '{}'", cli.ntds_file());
        std::process::exit(-1);
    }

    let esedb = EseDb::open(cli.ntds_file())?;
    let info = EsedbInfo::try_from(&esedb)?;
    let database = CDatabase::new(&info)?;

    let mut options = OutputOptions::default();
    options.set_display_all_attributes(cli.command().display_all_attributes());
    options.set_flat_serialization(cli.command().flat_serialization());
    options.set_format(cli.command().format());

    match cli.command() {
        Commands::Group { .. } => {
            do_with_serialization!(cli.command(), database, show_groups, &options)
        }
        Commands::User { .. } => {
            do_with_serialization!(cli.command(), database, show_users, &options)
        }
        Commands::Computer { .. } => {
            do_with_serialization!(cli.command(), database, show_computers, &options)
        }
        Commands::Types { .. } => {
            do_with_serialization!(cli.command(), database, show_type_names, &options)
        }
        Commands::Timeline {
            all_objects,
            include_deleted,
        } => {
            options.set_show_all_objects(*all_objects);
            database.show_timeline(&options, *include_deleted)
        }
        Commands::Tree { max_depth } => Ok(database.show_tree(*max_depth)?),
        Commands::Entry { entry_id, use_sid, entry_format } => {
            let id = if *use_sid {
                EntryId::Rid((*entry_id).try_into().unwrap())
            } else {
                EntryId::Id((*entry_id).into())
            };
            Ok(database.show_entry(id, *entry_format)?)
        }
        Commands::Search { regex, ignore_case } => {
            let regex = if *ignore_case {
                format!("(?i:{regex})")
            } else {
                regex.to_owned()
            };
            database.search_entries(&regex)
        }
    }
}
