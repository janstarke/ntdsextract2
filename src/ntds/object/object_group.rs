use crate::ntds::{types, NoSpecificAttributes, Object};


pub type Group<T> = Object<T, types::Group, NoSpecificAttributes>;