pub mod delete;
pub mod lookup;
pub mod read;
pub mod utils;
pub mod write;

use delete::delete;
use lookup::{list, lookup};
use read::read;
use write::{add, update};

use crate::notes::NotesReader;
use std::collections::VecDeque;

pub fn parse_command(nr: &mut NotesReader, command: &String) -> String {
    let mut parts = command.as_str().split(" ").collect::<VecDeque<&str>>();
    let command = parts.pop_front();

    match command {
        Some("read") | Some("r") => read(&mut parts, nr),
        Some("add") | Some("a") => add(&mut parts, nr),
        Some("update") | Some("u") => update(&mut parts, nr),
        Some("delete") | Some("d") => delete(&mut parts, nr),
        Some("lookup") | Some("l") => lookup(&mut parts, nr),
        Some("ll") => list(&mut parts, nr),
        _ => String::from("none"),
    }
}
