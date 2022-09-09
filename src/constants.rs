macro_rules! str_attribute {
    ($const_id: ident, $internal_col_name: expr) => {
        pub (crate) const $const_id: &str = $internal_col_name;        
    };
}

str_attribute!(TYPENAME_PERSON, "Person");
str_attribute!(TYPENAME_COMPUTER, "Computer");