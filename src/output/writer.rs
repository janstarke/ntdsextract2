pub trait Writer {
    fn write_typenames<I>(&self, names: I) -> anyhow::Result<()>
    where
        I: Iterator<Item = String>;
}
