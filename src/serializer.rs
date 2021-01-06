use crate::column::Column;
use crate::entry::Entry;
use crate::table::Table;
use crate::types::ColumnType;
use crate::values::Value;

pub trait Serializable {
    fn serialize(&self) -> String;
}

// Escapes all "," in the given string
fn escape(str: &str) -> String {
    String::from(str).replace(",", "\\,")
}

// Will escape the string and surround it with "
fn escape_and_surround(str: String) -> String {
    let mut result = String::from("\"");
    result.push_str(&escape(&str));
    result.push('\"'); // " Fix syntax highlighting

    result
}

impl Serializable for Table {
    fn serialize(&self) -> String {
        let mut result = String::from("");
        result.push_str(&self.columns.serialize());
        result.push('\n');
        result.push_str(&self.entries.serialize());
        result
    }
}

impl Serializable for Vec<Column> {
    fn serialize(&self) -> String {
        let mut result = String::from("");
        for c in self {
            result.push_str(&c.serialize());
            result.push(',');
        }
        result.pop();

        result
    }
}

impl Serializable for Column {
    fn serialize(&self) -> String {
        let mut result = String::from("");
        if self.is_key {
            result.push_str("key ");
        }
        result.push_str(&self.column_type.serialize());
        result.push(' ');

        result.push('"');
        result.push_str(&escape(&self.name));
        result.push('"');

        result
    }
}

impl Serializable for ColumnType {
    fn serialize(&self) -> String {
        match self {
            ColumnType::Integer => "int".to_string(),
            ColumnType::String => "str".to_string(),
        }
    }
}

impl Serializable for Vec<Entry> {
    fn serialize(&self) -> String {
        let mut result = String::from("");
        for e in self {
            result.push_str(&e.serialize());
            result.push('\n');
        }

        result
    }
}

impl Serializable for Entry {
    fn serialize(&self) -> String {
        let mut result = String::from("");
        for (_type, v) in &self.values {
            result.push_str(&v.serialize());
            result.push(',');
        }
        result.pop();
        result
    }
}

impl Serializable for Value {
    fn serialize(&self) -> String {
        match self {
            Value::String(s) => escape_and_surround(s.clone()),
            Value::Integer(i) => escape_and_surround(i.clone().to_string()),
        }
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

    #[test]
    fn serialize_no_data() {
        let table = create_test_table();

        let serialized = r#"key int "C1",str "C2""#.to_owned() + "\n";

        assert!(table.serialize() == serialized);
    }

    #[test]
    fn serialize_data() {
        let mut table = create_test_table();

        table.insert(vec![10.into(), "Hello".into()]).unwrap();
        table.insert(vec![20.into(), "World".into()]).unwrap();

        let serialized = r#"key int "C1",str "C2"
"10","Hello"
"20","World"
"#;

        assert!(table.serialize() == serialized);
    }

    #[test]
    fn serialize_with_comma_no_data() {
        let table = create_test_table_with_comma();

        let serialized = r#"key int "C\,1",str "C2\,""#.to_owned() + "\n";

        assert!(table.serialize() == serialized);
    }

    #[test]
    fn serialize_with_comma_data() {
        let mut table = create_test_table_with_comma();

        table.insert(vec![10.into(), "H,ello".into()]).unwrap();
        table.insert(vec![20.into(), "Wor,ld".into()]).unwrap();

        let serialized = r#"key int "C\,1",str "C2\,"
"10","H\,ello"
"20","Wor\,ld"
"#;

        assert!(table.serialize() == serialized);
    }
}
