use libesedb::{Value, Record};

pub trait IsRecord: for<'a> TryFrom<Record<'a>, Error = std::io::Error> {
    fn count_values(&self) -> i32;
    
    fn with_value<F>(&self, index: i32, action: F) where F: Fn(&Value) {
        self.with_value_mut(index, action);
    }

    fn with_value_mut<F>(&self, index: i32, action: F) where F: FnMut(&Value);
}