use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

extern crate json;

pub struct Table {
    pub source_file: PathBuf,
    pub table_name: String,
}

impl Table {
    pub fn content(&self) -> json::JsonValue {
        let mut file = File::open(&self.source_file).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let json_data = json::parse(&contents).unwrap();

        json_data[&self.table_name].clone()
    }

    pub fn display(&self) {}
}
