extern crate file_minidb;

use file_minidb::table::Table;
use file_minidb::types::ColumnType;
use file_minidb::column::Column;
use file_minidb::values::Value;

pub fn main() {
    let column1 = Column::key("First Name", ColumnType::String);
    let column2 = Column::key("Last Name", ColumnType::String);
    let column3 = Column::new("Age", ColumnType::Integer);
    let columns = vec![column1, column2, column3];

    let mut table = Table::new(&columns).unwrap();

    println!("{:?}", table);

    // Valid
    assert!(table.insert(vec![Value::String("Peter".to_string()), Value::String("Pan".to_string()), Value::Integer(15)]).is_ok());

    println!("Empty table");
    println!("{:?}", table);

    // Errors because table has two columns, but only one was given.
    assert!(table.insert(vec![Value::String("Petra".to_string())]).is_err());
    
    // Errors because table String as second type but Integer was given
    assert!(table.insert(vec![Value::String("Bill".to_string()), Value::Integer(16), Value::String("Jukes".to_string())]).is_err());


    // Add some more
    assert!(table.insert(vec![Value::String("Alf".to_string()), Value::String("Mason".to_string()), Value::Integer(25)]).is_ok());
    assert!(table.insert(vec![Value::String("Robert".to_string()), Value::String("Mullins".to_string()), Value::Integer(35)]).is_ok());
    assert!(table.insert(vec![Value::String("Alan".to_string()), Value::String("Herb".to_string()), Value::Integer(37)]).is_ok());
    assert!(table.insert(vec![Value::String("Canary".to_string()), Value::String("Robb".to_string()), Value::Integer(42)]).is_ok());

    println!("Filled table");
    println!("{:?}", table);

    // Remove item in table
    assert!(table.remove(vec![Value::String("Alf".to_string()), Value::String("Mason".to_string())]));

    println!("Removed item");
    println!("{:?}", table);

    // Remove non existant item
    assert!(!table.remove(vec![Value::String("Mary".to_string()), Value::String("Darling".to_string())]));

    println!("End table");
    println!("{:?}", table);
    
}