use getset::{Getters, Setters};

use crate::OutputFormat;

#[derive(Getters, Setters, Default)]
#[getset(get="pub", set="pub")]
pub struct OutputOptions {
    flat_serialization: bool,
    display_all_attributes: bool,
    show_all_objects: bool,
    format: OutputFormat
}