use crate::types::ColumnType;

use std::fmt;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
            Value::Integer(_) => ColumnType::Integer, 
        }
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::String(v.to_string())
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::Integer(v)
    }
}

impl TryFrom<Value> for i32 {
    type Error = &'static str;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::Integer(i) = value {
            return Ok(i);
        } else {
            return Err("Cannot convert from String");
        }
    }
}

impl TryFrom<Value> for String {
    type Error = &'static str;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::String(str) = value {
            return Ok(str);
        } else {
            return Err("Cannot convert from Integer");
        }
    }

}
