use std::collections::VecDeque;

use super::parse_strings;
use crate::notes::NotesReader;

pub fn add(arg: &mut VecDeque<&str>, nr: &mut NotesReader) -> String {
    let target = String::from(arg.pop_front().unwrap());
    let mut props: Vec<&str> = Vec::new();

    while let Some(prop) = arg.pop_front() {
        props.push(prop);
    }

    if !props.is_empty() {
        let props = parse_strings(&props);
        if let [key, value] = &props[..] {
            nr.add_notes(&target, key, value);
        }
    } else {
        nr.create_notes(&target);
    }

    String::new()
}

pub fn update(arg: &mut VecDeque<&str>, nr: &mut NotesReader) -> String {
    let target = String::from(arg.pop_front().unwrap());
    let mut props: Vec<&str> = Vec::new();

    while let Some(prop) = arg.pop_front() {
        props.push(prop);
    }

    if !props.is_empty() {
        let props = parse_strings(&props);
        if let [key, value] = &props[..] {
            nr.update_notes(&target, key, value);
        }
    }

    String::new()
}
