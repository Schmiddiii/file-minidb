
use crate::values::Value;
use crate::column::Column;


#[derive(Debug)]
pub struct Entry<'a> {
    values: Vec<(&'a Column, Value)>
}

impl<'a> Entry<'a> {
    pub fn new(values: Vec<(&'a Column, Value)>) -> Entry<'a> {
        Entry {
            values: values
        }
    }

    fn get_columns(&self) -> Vec<&'a Column> {
        self.values.iter().map(|v| v.0).collect()
    }

    // fn get_values(&self) -> Vec<Value> {
    //     self.values.iter().map(|v| v.1.clone()).collect()
    // }

    fn get_key_values(&self) -> Vec<Value> {
        self.values.iter().filter(|v| v.0.is_key).map(|v| v.1.clone()).collect()
    }

    /// Two values are key equivalent if
    ///     - All (not just key) columns must be equivalent
    ///     - All values from key columns must be equivalent
    pub fn key_eq(&self, other: &Self) -> bool {
        // Check for the same amount of columns
        if self.get_columns().len() != other.get_columns().len() {
            return false;
        }

        // Check that columns are the same
        for (self_col, other_col) in self.get_columns().into_iter().zip(other.get_columns().into_iter()) {
            if self_col != other_col {
                return false;
            }
        }

        // Check that values are the same
        for(self_val, other_val) in self.get_key_values().into_iter().zip(other.get_key_values().into_iter()) {
            if self_val != other_val {
                return false;
            }

        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn equivalent_entries_are_key_equivalent() {
        let column1 = crate::column::Column::key("C1", crate::types::ColumnType::String); 
        let column2 = crate::column::Column::new("C2", crate::types::ColumnType::String); 

        let entry1_values = vec![(&column1, crate::values::Value::String("Hello".to_owned())), (&column2, crate::values::Value::String("World".to_owned()))];
        let entry2_values = entry1_values.clone();

        let entry1 = super::Entry::new(entry1_values);
        let entry2 = super::Entry::new(entry2_values);

        assert!(entry1.key_eq(&entry2));
    }
    
    #[test]
    fn key_equivalent_entries_are_key_equivalent() {
        let column1 = crate::column::Column::key("C1", crate::types::ColumnType::String); 
        let column2 = crate::column::Column::new("C2", crate::types::ColumnType::String); 

        let entry1_values = vec![(&column1, crate::values::Value::String("Hello".to_owned())), (&column2, crate::values::Value::String("World".to_owned()))];
        let entry2_values = vec![(&column1, crate::values::Value::String("Hello".to_owned())), (&column2, crate::values::Value::String("Moon".to_owned()))];

        let entry1 = super::Entry::new(entry1_values);
        let entry2 = super::Entry::new(entry2_values);

        assert!(entry1.key_eq(&entry2));
    }

    
    #[test]
    fn different_columns_entries_are_not_key_equivalent() {
        let column1 = crate::column::Column::key("C1", crate::types::ColumnType::String); 
        let column2 = crate::column::Column::new("C2", crate::types::ColumnType::String); 
        let column3 = crate::column::Column::new("C3", crate::types::ColumnType::Integer); 

        let entry1_values = vec![(&column1, crate::values::Value::String("Hello".to_owned())), (&column2, crate::values::Value::String("World".to_owned()))];
        let entry2_values = vec![(&column1, crate::values::Value::String("Hello".to_owned())), (&column3, crate::values::Value::Integer(16))];

        let entry1 = super::Entry::new(entry1_values);
        let entry2 = super::Entry::new(entry2_values);

        assert!(!entry1.key_eq(&entry2));
    }

    
    #[test]
    fn key_different_entries_are_key_equivalent() {
        let column1 = crate::column::Column::key("C1", crate::types::ColumnType::String); 
        let column2 = crate::column::Column::new("C2", crate::types::ColumnType::String); 

        let entry1_values = vec![(&column1, crate::values::Value::String("Hello".to_owned())), (&column2, crate::values::Value::String("World".to_owned()))];
        let entry2_values = vec![(&column1, crate::values::Value::String("Bye".to_owned())), (&column2, crate::values::Value::String("World".to_owned()))];

        let entry1 = super::Entry::new(entry1_values);
        let entry2 = super::Entry::new(entry2_values);

        assert!(!entry1.key_eq(&entry2));
    }

    #[test]
    fn get_key_values() {
        let column1 = crate::column::Column::key("C1", crate::types::ColumnType::String); 
        let column2 = crate::column::Column::new("C2", crate::types::ColumnType::String); 

        let entry_values = vec![(&column1, crate::values::Value::String("Hello".to_owned())), (&column2, crate::values::Value::String("World".to_owned()))];
        let entry = super::Entry::new(entry_values);

        assert_eq!(entry.get_key_values(), vec![crate::values::Value::String("Hello".to_owned())])
        
    }
}