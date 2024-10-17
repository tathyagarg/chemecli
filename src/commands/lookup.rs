use crate::boxup::boxer::boxup;
use crate::boxup::models::{Alignment, BoxupOptions};
use crate::commands::utils::parse_strings;
use crate::notes::NotesReader;
use crate::table::constants;
use std::collections::VecDeque;

pub fn list(_: &mut VecDeque<&str>, _: &mut NotesReader) -> String {
    boxup(
        "Builtins".to_string(),
        constants::BUILTINS
            .iter()
            .fold(String::new(), |acc, elem| format!("{}{}\n", acc, elem)),
        BoxupOptions::new().alignment(Alignment::Center),
    )
}

pub fn lookup(arg: &mut VecDeque<&str>, nr: &mut NotesReader) -> String {
    let target = String::from(arg.pop_front().unwrap());

    if target == "list" {
        list(arg, nr)
    } else {
        String::from("new")
    }
}
