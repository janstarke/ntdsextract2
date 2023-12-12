use strum::{Display, EnumString, IntoStaticStr};

#[derive(IntoStaticStr, EnumString, Debug, Display, Eq, PartialEq, Hash, Clone, Copy)]
pub enum ObjectType {
    Person,
    Group,
    Computer,
}

pub trait HasObjectType {
    fn object_type() -> ObjectType;
}

pub mod types {
    use super::{HasObjectType, ObjectType};

    pub struct Person;
    pub struct Group;
    pub struct Computer;

    impl HasObjectType for Person {
        fn object_type() -> ObjectType {
            ObjectType::Person
        }
    }

    impl HasObjectType for Group {
        fn object_type() -> ObjectType {
            ObjectType::Group
        }
    }

    impl HasObjectType for Computer {
        fn object_type() -> ObjectType {
            ObjectType::Computer
        }
    }
}
