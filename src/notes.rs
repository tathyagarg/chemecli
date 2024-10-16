use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::vec::Vec;

pub struct NotesReader {
    pub source_file: PathBuf,
    contents: HashMap<String, Vec<(String, String)>>,
}

impl NotesReader {
    pub fn new(source_file: PathBuf) -> NotesReader {
        let contents: HashMap<String, Vec<(String, String)>> = HashMap::new();
        let mut er = NotesReader {
            source_file,
            contents,
        };

        er.set_contents();
        er
    }

    pub fn set_contents(&mut self) {
        let mut file = File::open(&self.source_file).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let json_data = json::parse(&contents).unwrap();

        let mut contents: HashMap<String, Vec<(String, String)>> = HashMap::new();
        let mut sub_contents: Vec<(String, String)>;

        for (k, v) in json_data.entries() {
            sub_contents = Vec::new();
            for (key, value) in v.entries() {
                sub_contents.push((key.to_string(), value.to_string()));
            }
            contents.insert(k.to_string(), sub_contents);
        }

        self.contents = contents;
    }

    pub fn get_notes(&self, target: &String) -> Vec<(String, String)> {
        self.contents[target].clone()
    }
}
