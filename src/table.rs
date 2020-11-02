
use crate::column::Column;
use crate::entry::Entry;
use crate::values::Value;

use std::collections::HashSet;

#[derive(Debug)]
pub struct Table<'a> {
    columns: &'a Vec<Column>,
    entries: Vec<Entry<'a>>
}

// impl<'a> fmt::Debug for Table<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Table")
//          .field("columns", &(*self.columns).as_ref())
//          .field("entries", &self.entries)
//          .finish()
//     }
// }

impl<'a> Table<'a> {
    /// Creates a new table. Returns an error if two columns have the same name.
    pub fn new(columns: &'a Vec<Column>) -> Result<Table<'a>, String> {
        let mut names = HashSet::new();

        if columns.iter().map(|v| names.insert((*v.name).as_ref())).any(|v| !v) {
            return Err("At least two columns have the same name".to_string());
        }

        Ok(Table {
            columns: columns,
            entries: vec![]
        })
    }

    pub fn insert(&mut self, entry: Vec<Value>) -> Result<(), String> {
        // Check if all columns are given
        if entry.len() != self.columns.len() {
            return Err("Not all columns are given".to_owned());
        }

        let values_type_iter = entry.iter().map(|v| v.get_type());
        let columns_type_iter = self.columns.iter().map(|v| v.get_type());

        // Check if types from new entry is equivalent to the columns of the table
        for (i, (v_type, c_type)) in values_type_iter.zip(columns_type_iter).enumerate() {
            if v_type != c_type {
                let error_message = format!("Column types do not match in column: {}, expected: {:?}, got {:?}", i, c_type, v_type);
                return Err(error_message);
            }
        }


        // Create new Entry
        let values_iter = entry.into_iter();
        let columns_iter = self.columns.iter();

        let zip_vec: Vec<(&Column, Value)> = columns_iter.zip(values_iter).collect();
        let new_entry: Entry<'a> = Entry::new(zip_vec);

        // Check if key already exists
        for e in &self.entries {
            if e.key_eq(&new_entry) {
                return Err("Key already exists".to_string());
            }
        }
        
        // Ok to insert
        self.entries.push(new_entry);

        Ok(())
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn normal_new() {
        let column1 = super::Column::new("Test1", crate::types::ColumnType::Integer);
        let column2 = super::Column::new("Test2", crate::types::ColumnType::String);

        assert!(super::Table::new(&vec![column1, column2]).is_ok());
    }

    #[test]
    fn new_with_duplicate_columns() {
        let column1 = super::Column::new("Test1", crate::types::ColumnType::Integer);
        let column2 = super::Column::new("Test1", crate::types::ColumnType::String);

        assert!(super::Table::new(&vec![column1, column2]).is_err());
    }

    #[test]
    fn normal_insert() {
        let column1 = super::Column::key("Test1", crate::types::ColumnType::Integer);
        let column2 = super::Column::new("Test2", crate::types::ColumnType::String);
        let columns = vec![column1, column2];

        let mut table = super::Table::new(&columns).unwrap();

        assert!(table.insert(vec![crate::values::Value::Integer(10), crate::values::Value::String("Hello".to_string())]).is_ok());
        assert!(table.insert(vec![crate::values::Value::Integer(12), crate::values::Value::String("World".to_string())]).is_ok());
    }
    
    #[test]
    fn duplicate_insert() {
        let column1 = super::Column::key("Test1", crate::types::ColumnType::Integer);
        let column2 = super::Column::new("Test2", crate::types::ColumnType::String);
        let columns = vec![column1, column2];

        let mut table = super::Table::new(&columns).unwrap();

        assert!(table.insert(vec![crate::values::Value::Integer(10), crate::values::Value::String("Hello".to_string())]).is_ok());
        assert!(table.insert(vec![crate::values::Value::Integer(10), crate::values::Value::String("World".to_string())]).is_err());
    }

    #[test]
    fn wrong_type_insert() {
        let column1 = super::Column::key("Test1", crate::types::ColumnType::Integer);
        let column2 = super::Column::new("Test2", crate::types::ColumnType::String);
        let columns = vec![column1, column2];

        let mut table = super::Table::new(&columns).unwrap();

        assert!(table.insert(vec![crate::values::Value::String("Hello".to_string()), crate::values::Value::Integer(10)]).is_err());
        assert!(table.insert(vec![crate::values::Value::String("Bye".to_string()), crate::values::Value::String("World".to_string())]).is_err());
    }

    #[test]
    fn wrong_column_num_insert() {
        let column1 = super::Column::key("Test1", crate::types::ColumnType::Integer);
        let column2 = super::Column::new("Test2", crate::types::ColumnType::String);
        let columns = vec![column1, column2];

        let mut table = super::Table::new(&columns).unwrap();

        assert!(table.insert(vec![crate::values::Value::Integer(10)]).is_err());
        assert!(table.insert(vec![crate::values::Value::Integer(12), crate::values::Value::String("World".to_string()), crate::values::Value::String("Hello".to_string())]).is_err());
        
    }
}