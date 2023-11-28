use strum::{IntoStaticStr, Display, EnumString};

#[derive(IntoStaticStr, EnumString, Debug, Display, Eq, PartialEq, Hash)]
pub enum ObjectType {
    Person,
    Group,
    Computer
}