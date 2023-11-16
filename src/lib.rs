
mod person;
mod serialization;
mod computer;
mod group;
mod constants;
mod column_information;
mod column_info_mapping;
mod data_table_ext;
mod link_table_ext;
mod win32_types;
mod esedb_utils;
mod object_tree_entry;
mod entry_id;
mod esedb_cache;

pub use entry_id::*;
pub use person::*;
pub use esedb_cache::*;
pub use data_table_ext::DataTableExt;

#[derive(clap::ValueEnum, Clone)]
pub enum OutputFormat{
    Csv,
    Json,
    JsonLines
}

/// this needs to be global, 
/// because it is read by serialization code, which has no state by default
static mut DISPLAY_ALL_ATTRIBUTES: bool = false;
static mut FLAT_SERIALIZATION: bool = true;

pub fn display_all_attributes() -> bool {
    unsafe {
        DISPLAY_ALL_ATTRIBUTES
    }
}

pub fn skip_all_attributes<T>(_t: &T) -> bool {
    ! display_all_attributes()
}

pub fn set_display_all_attributes(val: bool) {
    unsafe {
        DISPLAY_ALL_ATTRIBUTES = val
    }
}

pub fn do_flat_serialization() -> bool {
    unsafe {
        FLAT_SERIALIZATION
    }
}

pub fn serde_flat_serialization<T>(_t: &T) -> bool {
    do_flat_serialization()
}

pub fn set_do_flat_serialization(val: bool) {
    unsafe {
        FLAT_SERIALIZATION = val
    }
}