# file-minidb
A very minimal database stored in a file. Written in Rust

# What this is
This is a small library to store and access data in a database-like format without the need to install and setup a big and complicated database.

# What this is not
This is not and never will be a fully fleshed out and high speed database and should therefore not be used in giant projects using thousands of datapoints.

# What it can do
- Create tables with the column-types String and Integer, can have key pairs
- Insert data into a table
- Remove data from a table

# What it can not do yet
- Do basic database operations (join, selection, projection)
- Write and read from a file

Because these are not implemented yet (mainly the read and write from file), this program can not be used in a productive way.
