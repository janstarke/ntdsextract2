pub trait UnixTimestamp {
    fn timestamp(&self) -> i64;
}