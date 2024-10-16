use crate::notes::NotesReader;
use std::str::Split;

pub fn parse_command(nr: &NotesReader, command: &String) -> String {
    let mut parts = command.as_str().split(" ");
    let command = parts.next().unwrap();

    match command {
        "info" => info(&mut parts, nr),
        _ => String::from("none"),
    }
}

fn info(arg: &mut Split<&str>, nr: &NotesReader) -> String {
    let data = nr.get_notes(String::from(arg.next().unwrap()));
    let mut res = String::new();
    for (key, value) in &data {
        res.push_str(format!("{} {}", key, value).as_str());
    }

    res
}
