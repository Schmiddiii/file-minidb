extern crate file_minidb;

use file_minidb::column::Column;
use file_minidb::table::Table;
use file_minidb::types::ColumnType;

use std::io::{self, BufRead};

pub fn main() {
    let name_column = Column::new("Name", ColumnType::String);
    let age_column = Column::new("Age", ColumnType::Integer);

    let mut table = Table::new(vec![name_column, age_column]).unwrap();

    println!("Please input name and age.");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input = line.unwrap();
        let parts: Vec<&str> = input.split(' ').collect();

        if parts.len() != 2 {
            println!("Exactly two inputs have to be given. Try again.");
        } else {
            let name = parts.get(0).unwrap();
            let age_str = parts.get(1).unwrap();

            let age_opt = age_str.parse::<i32>();

            if let Ok(age) = age_opt {
                let success = table.insert(vec![(*name).into(), age.into()]);

                if success.is_err() {
                    println!("The name you inserted already exists");
                } else {
                    let entries = table.get_entries();

                    println!("");
                    for entry in entries {
                        let values = entry.get_values();

                        println!(
                            "{} is {} years old.",
                            values.get(0).unwrap(),
                            values.get(1).unwrap()
                        );
                    }
                }
            } else {
                println!("Cannot parse a number from the second argument");
            }
        }
        println!("");
        println!("");
        println!("Please input name and age.");
    }
}
