use crate::commands::parse_strings;
use crate::notes::NotesReader;
use std::collections::VecDeque;

pub fn delete(arg: &mut VecDeque<&str>, nr: &mut NotesReader) -> String {
    let target = String::from(arg.pop_front().unwrap());
    let mut props: Vec<&str> = Vec::new();

    while let Some(prop) = arg.pop_front() {
        props.push(prop);
    }
    println!("{:?}", props);

    if !props.is_empty() {
        let props = parse_strings(&props);
        for key in props {
            nr.delete_notes(&target, &key);
        }
    } else {
        nr.destroy_notes(&target);
    }

    String::new()
}
