use crate::ntds::{types, NoSpecificAttributes, Object};


pub type Person<T> = Object<T, types::Person, NoSpecificAttributes>;