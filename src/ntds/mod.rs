mod data_table;
mod link_table;
mod attribute_id;
mod link_table_builder;
mod object_type;
mod data_table_record;
mod error;
mod from_data_table;
mod object;
mod schema;
mod attribute_name;
mod attribute_value;

pub use data_table::*;
pub use link_table::*;
pub use attribute_id::*;
pub use object_type::*;
pub use data_table_record::*;
pub use error::*;
pub use from_data_table::*;
pub use object::*;
pub use schema::*;
pub use attribute_name::*;
pub use attribute_value::*;


pub type Person<T> = Object<T, types::Person>;
pub type Group<T> = Object<T, types::Group>;
pub type Computer<T> = Object<T, types::Computer>;