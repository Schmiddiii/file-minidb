use crate::column::Column;
use crate::table::Table;
use crate::types::ColumnType;
use crate::entry::Entry;
use crate::values::Value;

pub trait Serializable {
    fn serialize(&self) -> String;
}


// Escapes all "," in the given string
fn escape(str: &str) -> String {
    String::from(str).replace("\"", "\\\"")
}

// Will escape the string and surround it with "
fn escape_and_surround(str: String) -> String {
    let mut result = String::from("\"");
    result.push_str(&escape(&str));
    result.push('\"'); // " <- for syntax highlighting

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
            Value::Integer(i) => escape_and_surround(i.clone().to_string())
        }
    }

}
