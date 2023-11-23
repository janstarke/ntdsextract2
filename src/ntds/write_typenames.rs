pub trait WriteTypenames {
    fn write_typenames<'s, I>(&self, names: I) where I: Iterator<Item=String>;
}