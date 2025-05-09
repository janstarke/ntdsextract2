use crate::cache::Value;

pub trait WithValue<I> {
    fn with_value<T>(
        &self,
        index: I,
        function: impl FnMut(Option<&Value>) -> crate::ntds::Result<T>,
    ) -> crate::ntds::Result<T>;
}
