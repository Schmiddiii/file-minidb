use crate::column::Column;
use crate::entry::Entry;
use crate::serializer::Serializable;
use crate::values::Value;

use std::collections::HashSet;
use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Table {
    pub(crate) columns: Vec<Column>,
    pub(crate) entries: Vec<Entry>,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in &self.columns {
            write!(f, "| {:?}\t", c.name).unwrap();
        }
        writeln!(f, "|").unwrap();
        for e in &self.entries {
            writeln!(f, "{}", e).unwrap();
        }

        writeln!(f)
    }
}

impl Table {
    /// Creates a new table. Returns an error if two columns have the same name.
    pub fn new(columns: Vec<Column>) -> Result<Table, String> {
        let mut names = HashSet::new();

        if columns.iter().map(|v| names.insert(&v.name)).any(|v| !v) {
            return Err("At least two columns have the same name".to_string());
        }

        Ok(Table {
            columns,
            entries: vec![],
        })
    }

    /// Load table from file.
    pub fn from_file(path: &Path) -> Result<Table, String> {
        let file_content = std::fs::read_to_string(path);

        if let Ok(content) = file_content {
            Table::deserialize(content)
        } else {
            Err(file_content.err().unwrap().to_string())
        }
    }

    /// Write contents of the table to a file.
    /// Will overwrite the file.
    pub fn write_file(&self, path: &Path) -> Result<(), String> {
        let file = OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .open(path);

        if file.is_err() {
            Err(file.err().unwrap().to_string())
        } else {
            let serialized = self.serialize();
            write!(&mut file.unwrap(), "{}", serialized).unwrap();

            Ok(())
        }
    }

    /// Gets the data saved in the table.
    pub fn get_entries(&self) -> Vec<Entry> {
        self.entries.clone()
    }

    /// Insert data into the table.
    /// The types of the data must be equal to the data in the table.
    /// Furthermore, the keys must not already exist in the table
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
                let error_message = format!(
                    "Column types do not match in column: {}, expected: {:?}, got {:?}",
                    i, c_type, v_type
                );
                return Err(error_message);
            }
        }

        // Create new Entry
        let values_iter = entry.into_iter();
        let columns_iter = self.columns.iter().cloned();

        let zip_vec: Vec<(Column, Value)> = columns_iter.zip(values_iter).collect();
        // let values = values_iter.enumerate().map(|(i, v)| (&self.columns[i], v)).collect();
        let new_entry: Entry = Entry::new(zip_vec);

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

    /// Remove the element with the key values from the table.
    /// The given keys must be in the same order as saved in the table.
    /// Returns a success value.
    pub fn remove(&mut self, keys: Vec<Value>) -> bool {
        let key_columns = self.columns.iter().cloned().filter(|c| c.is_key);
        if keys.len() != key_columns.clone().count() {
            return false;
        }
        let ziped = key_columns.zip(keys.into_iter());
        let to_remove = Entry::new(ziped.collect());
        let old_entries = self.entries.clone();
        self.entries = self
            .entries
            .clone()
            .into_iter()
            .filter(|entry| !entry.key_eq(&to_remove))
            .collect();

        old_entries != self.entries
    }

    /// Project, which columns in the table to keep.
    /// Will return a clone of the current table and keep the current table
    /// unmodified.
    pub fn project(&self, columns: Vec<Column>) -> Result<Table, String> {
        let current_columns: HashSet<_> = self.columns.iter().cloned().collect();
        let new_columns: HashSet<_> = columns.iter().cloned().collect();

        if !new_columns.is_subset(&current_columns) {
            return Err("The given columns are not a subset of the current ones".to_string());
        }

        let mut new_table = Table::new(columns.clone()).unwrap();

        for entry in &self.entries {
            // Ignore errors as a non-key column might have duplicates.
            let _ = new_table.insert(entry.get_values_in_order(&columns).unwrap());
        }

        Ok(new_table)
    }
}

#[cfg(test)]
mod test {

    use std::convert::TryFrom;

    #[test]
    fn normal_new() {
        let column1 = super::Column::new("Test1", crate::types::ColumnType::Integer);
        let column2 = super::Column::new("Test2", crate::types::ColumnType::String);

        assert!(super::Table::new(vec![column1, column2]).is_ok());
    }

    #[test]
    fn new_with_duplicate_columns() {
        let column1 = super::Column::new("Test1", crate::types::ColumnType::Integer);
        let column2 = super::Column::new("Test1", crate::types::ColumnType::String);

        assert!(super::Table::new(vec![column1, column2]).is_err());
    }

    #[test]
    fn normal_insert() {
        let column1 = super::Column::key("Test1", crate::types::ColumnType::Integer);
        let column2 = super::Column::new("Test2", crate::types::ColumnType::String);
        let columns = vec![column1, column2];

        let mut table = super::Table::new(columns).unwrap();

        assert!(table
            .insert(vec![
                crate::values::Value::Integer(10),
                crate::values::Value::String("Hello".to_string())
            ])
            .is_ok());
        assert!(table
            .insert(vec![
                crate::values::Value::Integer(12),
                crate::values::Value::String("World".to_string())
            ])
            .is_ok());
    }

    #[test]
    fn duplicate_insert() {
        let column1 = super::Column::key("Test1", crate::types::ColumnType::Integer);
        let column2 = super::Column::new("Test2", crate::types::ColumnType::String);
        let columns = vec![column1, column2];

        let mut table = super::Table::new(columns).unwrap();

        assert!(table
            .insert(vec![
                crate::values::Value::Integer(10),
                crate::values::Value::String("Hello".to_string())
            ])
            .is_ok());
        assert!(table
            .insert(vec![
                crate::values::Value::Integer(10),
                crate::values::Value::String("World".to_string())
            ])
            .is_err());
    }

    #[test]
    fn wrong_type_insert() {
        let column1 = super::Column::key("Test1", crate::types::ColumnType::Integer);
        let column2 = super::Column::new("Test2", crate::types::ColumnType::String);
        let columns = vec![column1, column2];

        let mut table = super::Table::new(columns).unwrap();

        assert!(table
            .insert(vec![
                crate::values::Value::String("Hello".to_string()),
                crate::values::Value::Integer(10)
            ])
            .is_err());
        assert!(table
            .insert(vec![
                crate::values::Value::String("Bye".to_string()),
                crate::values::Value::String("World".to_string())
            ])
            .is_err());
    }

    #[test]
    fn wrong_column_num_insert() {
        let column1 = super::Column::key("Test1", crate::types::ColumnType::Integer);
        let column2 = super::Column::new("Test2", crate::types::ColumnType::String);
        let columns = vec![column1, column2];

        let mut table = super::Table::new(columns).unwrap();

        assert!(table
            .insert(vec![crate::values::Value::Integer(10)])
            .is_err());
        assert!(table
            .insert(vec![
                crate::values::Value::Integer(12),
                crate::values::Value::String("World".to_string()),
                crate::values::Value::String("Hello".to_string())
            ])
            .is_err());
    }

    #[test]
    fn normal_remove() {
        let column1 = super::Column::key("Test1", crate::types::ColumnType::Integer);
        let column2 = super::Column::new("Test2", crate::types::ColumnType::String);
        let columns = vec![column1, column2];

        let mut table = super::Table::new(columns).unwrap();

        assert!(table
            .insert(vec![
                crate::values::Value::Integer(10),
                crate::values::Value::String("Hello".to_string())
            ])
            .is_ok());
        assert!(table
            .insert(vec![
                crate::values::Value::Integer(12),
                crate::values::Value::String("World".to_string())
            ])
            .is_ok());

        assert!(table.remove(vec![crate::values::Value::Integer(10)]));

        assert_eq!(table.entries.len(), 1);
        assert_eq!(
            table.entries[0]
                .values
                .clone()
                .into_iter()
                .map(|v| v.1)
                .collect::<Vec<crate::values::Value>>(),
            vec![
                crate::values::Value::Integer(12),
                crate::values::Value::String("World".to_string())
            ]
        );
    }

    #[test]
    fn unknown_remove() {
        let column1 = super::Column::key("Test1", crate::types::ColumnType::Integer);
        let column2 = super::Column::new("Test2", crate::types::ColumnType::String);
        let columns = vec![column1, column2];

        let mut table = super::Table::new(columns).unwrap();

        assert!(table
            .insert(vec![
                crate::values::Value::Integer(10),
                crate::values::Value::String("Hello".to_string())
            ])
            .is_ok());
        assert!(table
            .insert(vec![
                crate::values::Value::Integer(12),
                crate::values::Value::String("World".to_string())
            ])
            .is_ok());

        let table_clone = table.clone();

        assert!(!table.remove(vec![crate::values::Value::Integer(1)]));

        assert_eq!(table, table_clone);
    }

    #[test]
    fn project() {
        let column1 = super::Column::key("Test1", crate::types::ColumnType::Integer);
        let column2 = super::Column::new("Test2", crate::types::ColumnType::String);
        let columns = vec![column1.clone(), column2.clone()];

        let mut table = super::Table::new(columns).unwrap();

        assert!(table
            .insert(vec![
                crate::values::Value::Integer(10),
                crate::values::Value::String("Hello".to_string())
            ])
            .is_ok());
        assert!(table
            .insert(vec![
                crate::values::Value::Integer(12),
                crate::values::Value::String("World".to_string())
            ])
            .is_ok());

        let table_sub1_opt = table.project(vec![column1]);
        let table_sub2_opt = table.project(vec![column2]);

        assert!(table_sub1_opt.is_ok());
        assert!(table_sub2_opt.is_ok());

        let table_sub1 = table_sub1_opt.unwrap();
        let table_sub2 = table_sub2_opt.unwrap();

        let table_sub1_values: Vec<i32> = table_sub1
            .get_entries()
            .iter()
            .map(|e| i32::try_from(e.get_values().get(0).unwrap().clone()).unwrap())
            .collect();
        let table_sub2_values: Vec<String> = table_sub2
            .get_entries()
            .iter()
            .map(|e| String::try_from(e.get_values().get(0).unwrap().clone()).unwrap())
            .collect();

        println!("{}", table);
        println!("{:?}", table_sub2_values);

        assert!(table_sub1_values == vec![10, 12]);
        assert!(table_sub2_values == vec!["Hello".to_string(), "World".to_string()]);
    }
}
