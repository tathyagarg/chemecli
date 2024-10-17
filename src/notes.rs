use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::vec::Vec;

use json::{stringify, JsonValue};

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

    fn serialize_contents(&self) -> JsonValue {
        let mut parent = JsonValue::new_object();
        for (target, elem) in self.get_contents() {
            let mut value_buff = JsonValue::new_object();
            for (key, value) in elem {
                value_buff[key] = value.into();
            }

            parent[target] = value_buff;
        }

        parent
    }

    pub fn get_contents(&self) -> HashMap<String, Vec<(String, String)>> {
        self.contents.clone()
    }

    pub fn get_notes(&self, target: &String) -> Vec<(String, String)> {
        self.contents[target].clone()
    }

    pub fn add_notes(&mut self, target: &String, key: &String, value: &String) {
        let mut buffer = self.get_contents();
        let mut subbuffer = buffer[target].clone();

        subbuffer.push((key.clone(), value.clone()));
        buffer.get_mut(target).map(|v| *v = subbuffer);

        self.contents = buffer;

        let stringified = stringify(self.serialize_contents());

        let mut file = File::create(&self.source_file).unwrap();
        file.write_all(stringified.as_bytes()).unwrap();
        self.contents = self.get_contents();
    }

    pub fn update_notes(&mut self, target: &String, key: &String, value: &String) {
        let mut buffer = self.get_contents();
        let mut subbuffer = buffer[target].clone();

        for (i, (k, _)) in subbuffer.iter().enumerate() {
            if k == key {
                subbuffer[i] = (key.clone(), value.clone());
                break;
            }
        }
        buffer.get_mut(target).map(|v| *v = subbuffer);

        self.contents = buffer;

        let stringified = stringify(self.serialize_contents());

        let mut file = File::create(&self.source_file).unwrap();
        file.write_all(stringified.as_bytes()).unwrap();
        self.contents = self.get_contents();
    }
}
