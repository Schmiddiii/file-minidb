use crate::types::ColumnType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    String(String),
    Integer(i32)
}

impl Value {
    pub fn get_type(&self) -> ColumnType {
        match &self {
            Value::String(_) => ColumnType::String,
            Value::Integer(_) => ColumnType::Integer
            // _ => {panic!("Unknown column type");}
        }

    }
}
