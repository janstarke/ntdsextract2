pub trait HasSerializableFields {
    fn field_count() -> usize {
        Self::fields().len()
    }
    fn fields() -> &'static Vec<&'static str>;
}