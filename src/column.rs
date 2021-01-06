use crate::types::ColumnType;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Column {
    pub(crate) is_key: bool,
    pub(crate) name: String,
    pub(crate) column_type: ColumnType,
}

impl Column {
    pub fn new<T: 'static + AsRef<str> + Clone>(name: T, column_type: ColumnType) -> Column {
        Column {
            is_key: false,
            name: String::from(name.as_ref()),
            column_type,
        }
    }

    pub fn key<T: 'static + AsRef<str> + Clone>(name: T, column_type: ColumnType) -> Column {
        Column {
            is_key: true,
            name: String::from(name.as_ref()),
            column_type,
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
