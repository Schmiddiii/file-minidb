extern crate file_minidb;

use file_minidb::column::Column;
use file_minidb::serializer::Serializable;
use file_minidb::table::Table;
use file_minidb::types::ColumnType;

pub fn main() {
    let column1 = Column::key("First Name", ColumnType::String);
    let column2 = Column::key("Last Name", ColumnType::String);
    let column3 = Column::new("Age", ColumnType::Integer);
    let columns = vec![column1, column2, column3];

    let mut table = Table::new(columns).unwrap();

    println!("{}", table);

    // Valid
    assert!(table
        .insert(vec!["Peter".into(), "Pan".into(), 15.into()])
        .is_ok());

    println!("Empty table");
    println!("{}", table);

    // Errors because table has two columns, but only one was given.
    assert!(table.insert(vec!["Petra".into()]).is_err());

    // Errors because table String as second type but Integer was given
    assert!(table
        .insert(vec!["Bill".into(), 16.into(), "Jukes".into()])
        .is_err());

    // Add some more
    assert!(table
        .insert(vec!["Alf".into(), "Mason".into(), 25.into()])
        .is_ok());
    assert!(table
        .insert(vec!["Robert".into(), "Mullins".into(), 35.into()])
        .is_ok());
    assert!(table
        .insert(vec!["Alan".into(), "Herb".into(), 37.into()])
        .is_ok());
    assert!(table
        .insert(vec!["Canary".into(), "Robb".into(), 42.into()])
        .is_ok());

    println!("Filled table");
    println!("{}", table);

    // Remove item in table
    assert!(table.remove(vec!["Alf".into(), "Mason".into()]));

    println!("Removed item");
    println!("{}", table);

    // Remove non existant item
    assert!(!table.remove(vec!["Mary".into(), "Darling".into()]));

    println!("End table");
    println!("{}", table);

    println!("Serialized table");
    println!("{}", table.serialize());
}
