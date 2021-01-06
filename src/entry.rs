use crate::column::Column;
use crate::values::Value;

use std::collections::HashSet;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Entry {
    pub(crate) values: Vec<(Column, Value)>,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for v in &self.values {
            write!(f, "| {}\t", v.1).unwrap();
        }
        write!(f, "|")
    }
}

impl Entry {
    pub fn new(values: Vec<(Column, Value)>) -> Entry {
        Entry { values }
    }

    fn get_key_columns(&self) -> Vec<Column> {
        self.values
            .iter()
            .filter(|v| v.0.is_key)
            .map(|v| v.0.clone())
            .collect()
    }

    fn get_key_values(&self) -> Vec<Value> {
        self.values
            .iter()
            .filter(|v| v.0.is_key)
            .map(|v| v.1.clone())
            .collect()
    }

    /// Get saved data of this entry.
    pub fn get_values(&self) -> Vec<Value> {
        self.values.iter().map(|v| v.1.clone()).collect()
    }

    /// Two entries are key equivalent if
    ///     - All (not just key) columns must be equivalent
    ///     - All values from key columns must be equivalent
    /// Two entries are definitly no key equivalent if
    /// no key columns exist.
    pub fn key_eq(&self, other: &Self) -> bool {
        let num_key_columns = self.get_key_columns().len();

        // Check for the same amount of columns
        if num_key_columns != other.get_key_columns().len() {
            return false;
        }

        if num_key_columns == 0 {
            return false;
        }

        // Check that columns are the same
        for (self_col, other_col) in self
            .get_key_columns()
            .into_iter()
            .zip(other.get_key_columns().into_iter())
        {
            if self_col != other_col {
                return false;
            }
        }

        // Check that values are the same
        for (self_val, other_val) in self
            .get_key_values()
            .into_iter()
            .zip(other.get_key_values().into_iter())
        {
            if self_val != other_val {
                return false;
            }
        }

        true
    }

    /// Returns the values of this entry in the given order.
    /// Will error if the given columns is not a subset of the columns of this entry.
    pub fn get_values_in_order(&self, columns: &[Column]) -> Result<Vec<Value>, String> {
        let current_columns: HashSet<_> = self.values.iter().map(|(c, _)| c).cloned().collect();
        let new_columns: HashSet<_> = columns.iter().cloned().collect();

        if !new_columns.is_subset(&current_columns) {
            return Err("The current columns are not a superset of the given ones".to_string());
        }

        let mut result = vec![];
        for column in columns {
            for value in self.values.clone() {
                if column.clone() == value.0 {
                    result.push(value.1.clone());
                }
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn equivalent_entries_are_key_equivalent() {
        let column1 = crate::column::Column::key("C1", crate::types::ColumnType::String);
        let column2 = crate::column::Column::new("C2", crate::types::ColumnType::String);

        let entry1_values = vec![
            (column1, crate::values::Value::String("Hello".to_owned())),
            (column2, crate::values::Value::String("World".to_owned())),
        ];
        let entry2_values = entry1_values.clone();

        let entry1 = super::Entry::new(entry1_values);
        let entry2 = super::Entry::new(entry2_values);

        assert!(entry1.key_eq(&entry2));
    }

    #[test]
    fn key_equivalent_entries_are_key_equivalent() {
        let column1 = crate::column::Column::key("C1", crate::types::ColumnType::String);
        let column2 = crate::column::Column::new("C2", crate::types::ColumnType::String);

        let entry1_values = vec![
            (
                column1.clone(),
                crate::values::Value::String("Hello".to_owned()),
            ),
            (
                column2.clone(),
                crate::values::Value::String("World".to_owned()),
            ),
        ];
        let entry2_values = vec![
            (
                column1.clone(),
                crate::values::Value::String("Hello".to_owned()),
            ),
            (
                column2.clone(),
                crate::values::Value::String("Moon".to_owned()),
            ),
        ];

        let entry1 = super::Entry::new(entry1_values);
        let entry2 = super::Entry::new(entry2_values);

        assert!(entry1.key_eq(&entry2));
    }

    #[test]
    fn no_keys_are_key_equivalent() {
        let column1 = crate::column::Column::new("C1", crate::types::ColumnType::String);
        let column2 = crate::column::Column::new("C2", crate::types::ColumnType::String);

        let entry1_values = vec![
            (
                column1.clone(),
                crate::values::Value::String("Hello".to_owned()),
            ),
            (
                column2.clone(),
                crate::values::Value::String("World".to_owned()),
            ),
        ];
        let entry2_values = vec![
            (
                column1.clone(),
                crate::values::Value::String("Hello".to_owned()),
            ),
            (
                column2.clone(),
                crate::values::Value::String("World".to_owned()),
            ),
        ];

        let entry1 = super::Entry::new(entry1_values);
        let entry2 = super::Entry::new(entry2_values);

        assert!(!entry1.key_eq(&entry2));
    }

    #[test]
    fn different_columns_entries_are_not_key_equivalent() {
        let column1 = crate::column::Column::key("C1", crate::types::ColumnType::String);
        let column2 = crate::column::Column::key("C2", crate::types::ColumnType::String);
        let column3 = crate::column::Column::new("C3", crate::types::ColumnType::Integer);

        let entry1_values = vec![
            (
                column1.clone(),
                crate::values::Value::String("Hello".to_owned()),
            ),
            (
                column2.clone(),
                crate::values::Value::String("World".to_owned()),
            ),
        ];
        let entry2_values = vec![
            (
                column1.clone(),
                crate::values::Value::String("Hello".to_owned()),
            ),
            (column3.clone(), crate::values::Value::Integer(16)),
        ];

        let entry1 = super::Entry::new(entry1_values);
        let entry2 = super::Entry::new(entry2_values);

        assert!(!entry1.key_eq(&entry2));
    }

    #[test]
    fn key_different_entries_are_key_equivalent() {
        let column1 = crate::column::Column::key("C1", crate::types::ColumnType::String);
        let column2 = crate::column::Column::new("C2", crate::types::ColumnType::String);

        let entry1_values = vec![
            (
                column1.clone(),
                crate::values::Value::String("Hello".to_owned()),
            ),
            (
                column2.clone(),
                crate::values::Value::String("World".to_owned()),
            ),
        ];
        let entry2_values = vec![
            (
                column1.clone(),
                crate::values::Value::String("Bye".to_owned()),
            ),
            (
                column2.clone(),
                crate::values::Value::String("World".to_owned()),
            ),
        ];

        let entry1 = super::Entry::new(entry1_values);
        let entry2 = super::Entry::new(entry2_values);

        assert!(!entry1.key_eq(&entry2));
    }

    #[test]
    fn get_key_values() {
        let column1 = crate::column::Column::key("C1", crate::types::ColumnType::String);
        let column2 = crate::column::Column::new("C2", crate::types::ColumnType::String);

        let entry_values = vec![
            (
                column1.clone(),
                crate::values::Value::String("Hello".to_owned()),
            ),
            (
                column2.clone(),
                crate::values::Value::String("World".to_owned()),
            ),
        ];
        let entry = super::Entry::new(entry_values);

        assert_eq!(
            entry.get_key_values(),
            vec![crate::values::Value::String("Hello".to_owned())]
        )
    }

    #[test]
    fn get_key_values_no_key() {
        let column1 = crate::column::Column::new("C1", crate::types::ColumnType::String);
        let column2 = crate::column::Column::new("C2", crate::types::ColumnType::String);

        let entry_values = vec![
            (
                column1.clone(),
                crate::values::Value::String("Hello".to_owned()),
            ),
            (
                column2.clone(),
                crate::values::Value::String("World".to_owned()),
            ),
        ];
        let entry = super::Entry::new(entry_values);

        assert_eq!(entry.get_key_values(), vec![])
    }
}
