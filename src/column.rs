use crate::types::ColumnType;
// use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Column {
    pub(crate) is_key: bool,
    pub(crate) name: String,
    pub(crate) column_type: ColumnType,
}

// impl fmt::Debug for Column {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Column")
//             .field("is_key", &self.is_key)
//             .field("name", &(*self.name).as_ref())
//             .field("column_type", &self.column_type)
//             .finish()
//     }
// }

// impl Clone for Column {
//     fn clone(&self) -> Self {
//         Column {
//             is_key: self.is_key,
//             name: Box::new((*self.name).as_ref().clone()),
//             column_type: self.column_type.clone()
//         }
//     }
// }

// impl PartialEq for Column {
//     fn eq(&self, other: &Self) -> bool {
//         return self.is_key == other.is_key
//             && self.column_type == other.column_type
//             && self.name = other.name;
//     }
// }

// impl Eq for Column {}

impl Column {
    pub fn new<T: 'static + AsRef<str> + Clone>(name: T, column_type: ColumnType) -> Column {
        Column {
            is_key: false,
            name: String::from(name.as_ref()),
            column_type: column_type,
        }
    }

    pub fn key<T: 'static + AsRef<str> + Clone>(name: T, column_type: ColumnType) -> Column {
        Column {
            is_key: true,
            name: String::from(name.as_ref()),
            column_type: column_type,
        }
    }

    pub fn get_type(&self) -> ColumnType {
        self.column_type.clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn equal() {
        let column1 = super::Column::new("Test1", super::ColumnType::Integer);
        let column2 = super::Column::new("Test1", super::ColumnType::Integer);

        assert_eq!(column1, column2);
    }

    #[test]
    fn is_key_nequal() {
        let column1 = super::Column::new("Test1", super::ColumnType::Integer);
        let column2 = super::Column::key("Test1", super::ColumnType::Integer);

        assert_ne!(column1, column2);
    }

    #[test]
    fn name_nequal() {
        let column1 = super::Column::new("Test1", super::ColumnType::Integer);
        let column2 = super::Column::new("Test2", super::ColumnType::Integer);

        assert_ne!(column1, column2);
    }

    #[test]
    fn type_nequal() {
        let column1 = super::Column::new("Test1", super::ColumnType::Integer);
        let column2 = super::Column::new("Test1", super::ColumnType::String);

        assert_ne!(column1, column2);
    }
}
