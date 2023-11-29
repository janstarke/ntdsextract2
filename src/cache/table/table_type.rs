pub trait TableType {}

pub struct LinkTable;
pub struct DataTable;

impl TableType for LinkTable {}
impl TableType for DataTable {}