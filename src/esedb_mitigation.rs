use std::io;

#[deprecated(note="this function is a mitigation for some strange behaviour. see https://github.com/janstarke/ntdsextract2/pull/16")]
pub(crate) fn libesedb_count(count_fn: impl Fn() -> io::Result<i32>) -> io::Result<i32> {
    match count_fn() {
        Ok(val) => Ok(val),
        Err(_) => count_fn()
    }
}
