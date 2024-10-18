use crate::boxup::boxer::{adjoin, boxup};
use crate::boxup::models::{Alignment, BoxupOptions};
use crate::commands::utils::parse_strings;
use crate::notes::NotesReader;
use crate::table::constants;
use std::collections::VecDeque;

pub fn list(_: &mut VecDeque<&str>, _: &mut NotesReader) -> String {
    adjoin(
        boxup(
            "Builtins".to_string(),
            constants::BUILTINS
                .names
                .iter()
                .fold(String::new(), |acc, elem| format!("{}{}\n", acc, elem)),
            BoxupOptions::new()
                .max_width(28)
                .alignment(Alignment::Center),
        ),
        boxup(
            "Alias".to_string(),
            constants::BUILTINS
                .labels
                .iter()
                .fold(String::new(), |acc, elem| format!("{}{}\n", acc, elem)),
            BoxupOptions::new()
                .max_width(28)
                .alignment(Alignment::Center),
        ),
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
