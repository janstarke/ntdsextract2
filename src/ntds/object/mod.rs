mod object_base;
mod specific_object_attribute;
mod no_specific_attributes;
mod has_serializable_fields;

mod object_computer;
mod object_group;
mod object_person;

pub use object_base::*;
pub use specific_object_attribute::*;
pub use no_specific_attributes::*;
pub use has_serializable_fields::*;

pub use object_computer::*;
pub use object_group::*;
pub use object_person::*;