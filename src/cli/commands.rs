use clap::{Subcommand, ValueEnum};
use strum::Display;

use super::{EntryFormat, MemberOfAttribute, OutputFormat};

#[derive(Subcommand)]
pub enum Commands {
    /// Display user accounts
    User {
        /// Output format
        #[clap(value_enum, short('F'), long("format"), default_value_t = OutputFormat::Csv)]
        format: OutputFormat,

        /// show all non-empty values. This option is ignored when CSV-Output is selected
        #[clap(short('A'), long("show-all"))]
        show_all: bool,

        /// include the distinguished name (DN) in the output.
        ///
        /// Note that this
        /// property is not an attribute of the AD entry iself; instead it is
        /// constructed from the relative DN (RDN) of the entry and
        /// all of its parents. That's why this property is normally not shown.
        #[clap(short('D'), long("include-dn"))]
        include_dn: bool,

        /// include the security descriptor in hte output
        /// 
        /// Note the not the raw value is show. Instead, an SDDL string is shown.
        #[clap(short('S'), long("include-sd"))]
        include_sd: bool,


        /// specify which attribute shall be used to display group memberships
        #[clap(long("member-of"), default_value_t=MemberOfAttribute::Rdn)]
        member_of_attribute: MemberOfAttribute,
    },

    /// Display groups
    Group {
        /// Output format
        #[clap(value_enum, short('F'), long("format"), default_value_t = OutputFormat::Csv)]
        format: OutputFormat,

        /// show all non-empty values. This option is ignored when CSV-Output is selected
        #[clap(short('A'), long("show-all"))]
        show_all: bool,

        /// include the distinguished name (DN) in the output.
        ///
        /// Note that this
        /// property is not an attribute of the AD entry iself; instead it is
        /// constructed from the relative DN (RDN) of the entry and
        /// all of its parents. That's why this property is normally not shown.
        #[clap(short('D'), long("include-dn"))]
        include_dn: bool,

        /// include the security descriptor in hte output
        /// 
        /// Note the not the raw value is show. Instead, an SDDL string is shown.
        #[clap(short('S'), long("include-sd"))]
        include_sd: bool,

        /// specify which attribute shall be used to display group memberships
        #[clap(long("member-of"), default_value_t=MemberOfAttribute::Rdn)]
        member_of_attribute: MemberOfAttribute,
    },

    /// display computer accounts
    Computer {
        /// Output format
        #[clap(value_enum, short('F'), long("format"), default_value_t = OutputFormat::Csv)]
        format: OutputFormat,

        /// show all non-empty values. This option is ignored when CSV-Output is selected
        #[clap(short('A'), long("show-all"))]
        show_all: bool,

        /// include the distinguished name (DN) in the output.
        ///
        /// Note that this
        /// property is not an attribute of the AD entry iself; instead it is
        /// constructed from the relative DN (RDN) of the entry and
        /// all of its parents. That's why this property is normally not shown.
        #[clap(short('D'), long("include-dn"))]
        include_dn: bool,

        /// include the security descriptor in hte output
        /// 
        /// Note the not the raw value is show. Instead, an SDDL string is shown.
        #[clap(short('S'), long("include-sd"))]
        include_sd: bool,

        /// specify which attribute shall be used to display group memberships
        #[clap(long("member-of"), default_value_t=MemberOfAttribute::Rdn)]
        member_of_attribute: MemberOfAttribute,
    },

    /// create a timeline (in flow-record format)
    Timeline {
        /// show objects of any type (this might be a lot)
        #[clap(long("all-objects"))]
        all_objects: bool,

        /// include also deleted objects (which don't have an AttObjectCategory attribute)
        #[clap(long("include-deleted"))]
        include_deleted: bool,

        /// output format
        #[clap(short('F'), long("format"), default_value_t=TimelineFormat::Record)]
        format: TimelineFormat,
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
        #[clap(long("max-depth"), default_value_t = 4)]
        max_depth: u8,
    },

    /// display one single entry from the directory information tree
    Entry {
        /// id of the entry to show
        entry_id: i32,

        /// search for SID instead for NTDS.DIT entry id.
        /// <ENTRY_ID> will be interpreted as RID, wich is the last part of the SID;
        /// e.g. 500 will return the Administrator account
        #[clap(long("sid"))]
        use_sid: bool,

        #[clap(short('F'), long("format"), default_value_t = EntryFormat::Simple)]
        entry_format: EntryFormat,
    },

    /// search for entries whose values match to some regular expression
    Search {
        /// regular expression to match against
        regex: String,

        /// case-insensitive search (ignore case)
        #[clap(short('i'), long("ignore-case"))]
        ignore_case: bool,
    },
}

impl Commands {
    pub fn display_all_attributes(&self) -> bool {
        match self {
            Commands::User {
                format: OutputFormat::Json,
                show_all,
                include_dn: _,
                include_sd: _,
                member_of_attribute: _,
            }
            | Commands::User {
                format: OutputFormat::JsonLines,
                show_all,
                include_dn: _,
                include_sd: _,
                member_of_attribute: _,
            }
            | Commands::Computer {
                format: OutputFormat::Json,
                show_all,
                include_dn: _,
                include_sd: _,
                member_of_attribute: _,
            }
            | Commands::Computer {
                format: OutputFormat::JsonLines,
                show_all,
                include_dn: _,
                include_sd: _,
                member_of_attribute: _,
            } => *show_all,
            _ => false,
        }
    }

    pub fn include_dn(&self) -> bool {
        match self {
            Commands::User {
                format: _,
                show_all: _,
                include_dn,
                include_sd: _,
                member_of_attribute: _,
            }
            | Commands::Group {
                format: _,
                show_all: _,
                include_dn,
                include_sd: _,
                member_of_attribute: _,
            } => *include_dn,
            Commands::Computer {
                format: _,
                show_all: _,
                include_dn,
                include_sd: _,
                member_of_attribute: _,
            } => *include_dn,
            _ => false,
        }
    }


    pub fn include_security_descriptor(&self) -> bool {
        match self {
            Commands::User {
                format: _,
                show_all: _,
                include_dn: _,
                include_sd,
                member_of_attribute: _,
            }
            | Commands::Group {
                format: _,
                show_all: _,
                include_dn: _,
                include_sd,
                member_of_attribute: _,
            } => *include_sd,
            Commands::Computer {
                format: _,
                show_all: _,
                include_dn: _,
                include_sd,
                member_of_attribute: _,
            } => *include_sd,
            _ => false,
        }
    }

    pub fn member_of_attribute(&self) -> MemberOfAttribute {
        match self {
            Commands::User {
                format: _,
                show_all: _,
                include_dn: _,
                include_sd: _,
                member_of_attribute,
            } => *member_of_attribute,
            Commands::Group {
                format: _,
                show_all: _,
                include_dn: _,
                include_sd: _,
                member_of_attribute,
            } => *member_of_attribute,
            Commands::Computer {
                format: _,
                show_all: _,
                include_dn: _,
                include_sd: _,
                member_of_attribute,
            } => *member_of_attribute,
            _ => MemberOfAttribute::Rdn,
        }
    }

    pub fn flat_serialization(&self) -> bool {
        matches!(
            &self,
            Commands::User {
                format: OutputFormat::Csv,
                ..
            } | Commands::Computer {
                format: OutputFormat::Csv,
                ..
            } | Commands::Group {
                format: OutputFormat::Csv,
                ..
            } | Commands::Timeline { .. }
        )
    }

    pub fn format(&self) -> Option<OutputFormat> {
        match self {
            Commands::User { format, .. } => Some(*format),
            Commands::Group { format, .. } => Some(*format),
            Commands::Computer { format, .. } => Some(*format),
            Commands::Types { format } => Some(*format),
            _ => None,
        }
    }
}

#[derive(ValueEnum, Clone, Display)]
pub enum TimelineFormat {

    /// bodyfile format
    #[strum(serialize = "bodyfile")]
    Bodyfile,

    /// flow record format (<https://docs.rs/flow-record>)
    #[strum(serialize = "record")]
    Record
}