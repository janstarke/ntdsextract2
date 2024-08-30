use crate::object_tree::ObjectTree;

pub trait IsMemberOf {
    fn update_membership_dn(&mut self, tree: &ObjectTree);
}