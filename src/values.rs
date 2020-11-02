use crate::types::ColumnType;

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    String(String),
    Integer(i32),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Value::String(v) => write!(f, "{}", v),
            Value::Integer(v) => write!(f, "{}", v),
        }
    }
}

impl Value {
    pub fn get_type(&self) -> ColumnType {
        match &self {
            Value::String(_) => ColumnType::String,
            Value::Integer(_) => ColumnType::Integer, // _ => {panic!("Unknown column type");}
        }
    }
}
