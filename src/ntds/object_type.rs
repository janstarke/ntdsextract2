use strum::{IntoStaticStr, Display, EnumString};

#[derive(IntoStaticStr, EnumString, Debug, Display, Eq, PartialEq, Hash, Clone, Copy)]
pub enum ObjectType {
    Person,
    Group,
    Computer
}