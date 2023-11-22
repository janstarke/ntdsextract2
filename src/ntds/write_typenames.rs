pub trait WriteTypenames {
    fn write_typenames<'s, I>(names: I) where I: Iterator<Item=&'s str>;
}