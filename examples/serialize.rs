extern crate file_minidb;

use file_minidb::column::Column;
use file_minidb::serializer::Serializable;
use file_minidb::table::Table;
use file_minidb::types::ColumnType;

fn main() {
    let column1 = Column::key("First, Name", ColumnType::String);
    let column2 = Column::key("Last Name", ColumnType::String);
    let column3 = Column::new("Age", ColumnType::Integer);
    let columns = vec![column1, column2, column3];

    let mut table = Table::new(columns).unwrap();

    // Add data
    assert!(table
        .insert(vec!["Peter".into(), "Pan".into(), 15.into()])
        .is_ok());

    assert!(table
        .insert(vec!["Alf".into(), "Ma,son".into(), 25.into()])
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

    println!("Original table:");
    println!("{}", table);

    println!("Serialized");
    let serialization = table.serialize();
    println!("{}", serialization);

    println!("Deserialized");
    let deserialization = Table::deserialize(serialization);
    println!("{}", deserialization.clone().unwrap());

    assert!(table == deserialization.unwrap());
}
