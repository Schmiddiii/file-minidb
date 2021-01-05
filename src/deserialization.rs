use crate::column::Column;
use crate::table::Table;
use crate::types::ColumnType;
use crate::serializer::Serializable;
use crate::entry::Entry;
use crate::values::Value;

// The entire file needs major refactoring.

fn split_to_first_unescaped(str: &str, ch: char) -> Option<(String, String)> {
    let mut result = String::from("");
    let mut last: char = 'a';

    for c in str.chars() {
        if c == ch && last != '\\' {
            return Some((result.clone(), str.to_string().split_off(result.len())));
        }
        last = c;
        result.push(c);
    }

    None
}

// Returns, whether str starts with pattern.
// If it starts with the pattern, the pattern is removed at the front.
// Otherwise the whole string is returned
fn starts_with_and_remove(str: &str, pattern: &str) -> (bool, String) {
    if str.starts_with(pattern) {
        (true, String::from(str.split_at(pattern.len()).1))
    } else {
        (false, str.to_string())
    }
}

impl Table {
    pub fn deserialize(str: String) -> Result<Self, String> {
        let mut lines = str.split('\n');

        let first_line = lines.next();
        if first_line.is_none() {
            return Err("String is empty".to_string());
        }

        let columns = Column::deserialize_columns(first_line.unwrap().to_string());

        let mut table = Table::new(columns.clone().unwrap().0).unwrap();

        let mut next_line = lines.next();

        while next_line.is_some() {
            if !next_line.unwrap().is_empty() {
                let entry_opt = Entry::deserialize_data(next_line.unwrap().to_string(), columns.clone().unwrap().0);

                if entry_opt.is_err() {
                    return Err(entry_opt.err().unwrap());
                } else {
                    table.insert(entry_opt.unwrap().values.iter().map(|(_,v)| v.clone()).collect()).unwrap();
                }

                next_line = lines.next()
            } else {
                break;
            }

        }

        Ok(table)
    }

}

impl Entry {
    fn deserialize_data(str: String, columns: Vec<Column>) -> Result<Self, String> {
        let mut result: Vec<(Column, Value)> = vec![];
        let mut working_str = str;
        for column in columns {
            let split = split_to_first_unescaped(&working_str, ',');


            // Last column
            if split.is_none() {
                if !working_str.is_empty() {
                    working_str.remove(0);
                    working_str.remove(working_str.len() - 1);
                    working_str = working_str.replace("\\,", ",");
                }
                let value = Entry::deserialize_value(working_str, column.get_type());

                if value.is_err() {
                    return Err(value.err().unwrap());
                } else {
                    result.push((column, value.unwrap()));
                    return Ok(Entry::new(result));
                }
            } else {
                let (first, rest) = split.unwrap();

                let mut first_mut = first.clone();

                first_mut.remove(0);
                first_mut.remove(first_mut.len() - 1);
                first_mut = first_mut.replace("\\,", ",");

                let value = Entry::deserialize_value(first_mut, column.get_type());

                if value.is_err() {
                    return Err(value.err().unwrap());
                } else {
                    result.push((column, value.unwrap()));
                }

                working_str = rest;
                working_str.remove(0);
            }

        }

        Err("Cannot deserialize data".to_string())
    }

    fn deserialize_value(str: String, column_type: ColumnType) -> Result<Value, String> {
        match column_type{
            ColumnType::String => Ok(Value::String(str)),
            ColumnType::Integer => {
                let value = str.parse::<i32>();
                if value.is_err() {
                    Err("Cannot parse integer".to_string())
                } else {
                    Ok(Value::Integer(value.unwrap()))
                }

            }
        }

    }
}

impl Column {
    fn deserialize_columns(str: String) -> Result<(Vec<Self>, String), String> {
        let mut rest = str;
        let mut result = vec![];
        while rest != "" {
            if rest.starts_with(',') {
                rest = rest.split_off(1);
            }
            let next = Column::deserialize(rest);
            if next.is_err() {
                return Err(next.err().unwrap());
            }

            let (col, r) = next.unwrap();
            result.push(col.clone());
            rest = r;
        }

        Ok((result, rest))
    }

    fn deserialize(str: String) -> Result<(Self, String), String> {
        let mut is_key = false;
        let mut rest = str.clone();
        if let (true, r) = starts_with_and_remove(&str, "key ") {
            is_key = true;
            rest = r;
        }

        let column_type_result = ColumnType::deserialize(rest);
        if column_type_result.is_err() {
            return Err(column_type_result.err().unwrap());
        }

        let (column_type, rest) = column_type_result.unwrap();

       let (_, rest) = starts_with_and_remove(&rest, "\"");

       let name_option = split_to_first_unescaped(&rest, '\"');           // "

       if name_option.is_none() {
           return Err("Cannot find a name for the column".to_owned());
       }

       let (name, rest) = name_option.unwrap();

       let mut rest_mut = rest;
       rest_mut.remove(0); // Remove leading "

       let name_unescaped = name.replace("\\,", ",");       // Unescape the name.

       Ok((Column {
           is_key,
           name: name_unescaped,
           column_type
       }, rest_mut))
    }
}

impl ColumnType {
    fn deserialize(str: String) -> Result<(Self, String), String> {
        let mut rest;
        if let (true, r) = starts_with_and_remove(&str, &ColumnType::Integer.serialize()) {
            rest = r;
            rest.remove(0); // Remove leading space
            return Ok((ColumnType::Integer, rest));
        }
        if let (true, r) = starts_with_and_remove(&str, &ColumnType::String.serialize()) {
            rest = r;
            rest.remove(0);
            return Ok((ColumnType::String, rest));
        }

        Err("Unknown column type".to_string())
    }
}

#[cfg(test)]
mod test {

    use crate::serializer::Serializable;

    fn create_test_table() -> super::Table {
        let column1 = super::Column::key("C1", crate::types::ColumnType::Integer);
        let column2 = super::Column::new("C2", crate::types::ColumnType::String);
        let columns = vec![column1, column2];

        return super::Table::new(columns).unwrap();
    }

    fn create_test_table_with_comma() -> super::Table {
        let column1 = super::Column::key("C,1", crate::types::ColumnType::Integer);
        let column2 = super::Column::new("C2,", crate::types::ColumnType::String);
        let columns = vec![column1, column2];

        return super::Table::new(columns).unwrap();
    }

    fn deserialization_equal(table: super::Table) -> bool {
        let serialized = table.serialize();

        let deserialized = super::Table::deserialize(serialized);

        if deserialized.is_err() {
            return false;
        }

        table == deserialized.unwrap()

    }

    #[test]
    fn deserialize_no_data() {
        let table = create_test_table();

        assert!(deserialization_equal(table));
    }

    #[test]
    fn deserialize_data() {
        let mut table = create_test_table();

        table.insert(vec![10.into(), "Hello".into()]).unwrap();
        table.insert(vec![20.into(), "World".into()]).unwrap();

        assert!(deserialization_equal(table));

    }

    #[test]
    fn deserialize_with_comma_no_data() {
        let table = create_test_table_with_comma();

        assert!(deserialization_equal(table));
    }

    #[test]
    fn deserialize_with_comma_data() {
        let mut table = create_test_table_with_comma();

        table.insert(vec![10.into(), "He,llo".into()]).unwrap();
        table.insert(vec![20.into(), "Wor,ld".into()]).unwrap();

        assert!(deserialization_equal(table));
    }

}

