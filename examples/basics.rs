extern crate file_minidb;

use file_minidb::table::Table;
use file_minidb::types::ColumnType;
use file_minidb::column::Column;
use file_minidb::values::Value;

pub fn main() {
    let column1 = Column::new("First Name", ColumnType::String);
    let column2 = Column::new("Last Name", ColumnType::String);
    let column3 = Column::new("Age", ColumnType::Integer);
    let columns = vec![column1, column2, column3];

    let mut table = Table::new(&columns).unwrap();

    println!("{:?}", table);

    // Valid
    assert!(table.insert(vec![Value::String("Peter".to_string()), Value::String("Pan".to_string()), Value::Integer(15)]).is_ok());

    println!("{:?}", table);

    // Errors because table has two columns, but only one was given.
    assert!(table.insert(vec![Value::String("Petra".to_string())]).is_err());
    
    // Errors because table String as second type but Integer was given
    assert!(table.insert(vec![Value::String("Bill".to_string()), Value::Integer(16), Value::String("Jukes".to_string())]).is_err());
}