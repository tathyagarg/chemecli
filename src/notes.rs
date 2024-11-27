use std::{collections::HashMap, fs::File, io::prelude::*, path::PathBuf, vec::Vec};

use json::{stringify, JsonValue};

pub struct NotesReader {
    pub source_file: PathBuf,

    /* Contents is a private attribute to prevent user from mutating it directly */
    contents: HashMap<String, Vec<(String, String)>>,
}

impl NotesReader {
    pub fn new(source_file: PathBuf) -> NotesReader {
        let contents: HashMap<String, Vec<(String, String)>> = HashMap::new();

        // Initialize a NotesReader with no contents
        let mut er = NotesReader {
            source_file,
            contents,
        };

        er.set_contents();
        er
    }

    pub fn set_contents(&mut self) {
        /* Opens the source file and resets the contents variable */

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
        /* Takes the current contents and serializes them into JSON objects for further use. */

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
        /* Contents getter */

        self.contents.clone()
    }

    pub fn get_notes(&self, target: &String) -> Vec<(String, String)> {
        /* Get notes of a specific element */

        self.contents[target].clone()
    }

    fn write_to_file(&mut self) {
        /* Private function to rewrite contents into the source file */

        let stringified = stringify(self.serialize_contents());

        let mut file = File::create(&self.source_file).unwrap();
        file.write_all(stringified.as_bytes()).unwrap();
        self.contents = self.get_contents();
    }

    pub fn add_notes(&mut self, target: &String, key: &str, value: &str) {
        /* Add note about given target */

        let mut buffer = self.get_contents();
        let mut subbuffer = self.get_notes(target);

        subbuffer.push((String::from(key), String::from(value)));
        println!("{:?}", subbuffer);
        *buffer.get_mut(target).unwrap() = subbuffer;

        self.contents = buffer.clone();
        self.write_to_file();
    }

    pub fn update_notes(&mut self, target: &String, key: &String, value: &str) {
        /* Update notes of the given target */

        let mut buffer = self.get_contents();
        let mut subbuffer = buffer[target].clone();

        for (i, (k, _)) in subbuffer.iter().enumerate() {
            if k == key {
                subbuffer[i] = (key.clone(), String::from(value));

                break;
            }
        }

        let _ = buffer
            .get_mut(target)
            .unwrap()
            .iter_mut()
            .enumerate()
            .map(|(i, v)| *v = subbuffer[i].clone());

        self.contents = buffer;
        self.write_to_file();
    }

    pub fn create_notes(&mut self, target: &str) {
        /* Create an empty note for the given target */

        let mut buffer = self.get_contents();
        buffer.insert(String::from(target), Vec::new());

        self.contents = buffer;
        self.write_to_file();
    }

    pub fn delete_notes(&mut self, target: &String, key: &String) {
        /* Delete a specific part of notes from the target notes data. */

        let mut buffer = self.get_contents();
        let mut subbuffer = buffer[target].clone();

        for (i, (k, _)) in subbuffer.iter().enumerate() {
            if k == key {
                subbuffer.remove(i);
                break;
            }
        }

        let _ = buffer
            .get_mut(target)
            .unwrap()
            .iter_mut()
            .enumerate()
            .map(|(i, v)| *v = subbuffer[i].clone());

        self.contents = buffer;
        self.write_to_file();
    }

    pub fn destroy_notes(&mut self, target: &String) {
        /* Destroys the entire notes of a given target.
         * All sub-notes are deleted, along with the key by the value of target */

        let mut buffer = self.get_contents();
        buffer.remove(target);

        self.contents = buffer;
        self.write_to_file();
    }
}
