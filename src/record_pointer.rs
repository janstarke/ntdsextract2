use std::rc::Rc;

use libesedb::Record;

#[derive(Clone)]
pub struct RecordPointer {
    pointer: i32
}

impl<'r> From<RecordPointer> for Record<'r> {

}