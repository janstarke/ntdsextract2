use strum::IntoStaticStr;

#[derive(IntoStaticStr, Debug)]
pub enum ObjectType {
    Person,
    Group,
    Computer
}