use crate::boxup::boxer::{adjoin, boxup};
use crate::boxup::models::{Alignment, BoxupOptions};
use crate::commands::utils::parse_strings;
use crate::notes::NotesReader;
use crate::table::constants::{self, BUILTINS};
use std::collections::VecDeque;

pub fn list(_: &mut VecDeque<&str>, _: &mut NotesReader) -> String {
    let mut keys = constants::BUILTINS
        .names
        .iter()
        .fold(String::new(), |acc, elem| format!("{}{}\n", acc, elem));

    let mut values = constants::BUILTINS
        .labels
        .iter()
        .fold(String::new(), |acc, elem| format!("{}{}\n", acc, elem));

    keys.pop();
    values.pop();

    adjoin(
        boxup(
            "Builtins".to_string(),
            keys,
            BoxupOptions::new()
                .max_width(28)
                .alignment(Alignment::Center),
        ),
        boxup(
            "Alias".to_string(),
            values,
            BoxupOptions::new()
                .max_width(28)
                .alignment(Alignment::Center),
        ),
    )
}

pub fn target_lookup(target: &str, key: &str) -> String {
    for (i, field) in BUILTINS.labels.iter().enumerate() {
        if *field == key {
            return String::from(BUILTINS.data[target][i]);
        }
    }

    String::new()
}

pub fn lookup(arg: &mut VecDeque<&str>, nr: &mut NotesReader) -> String {
    let target = arg.pop_front().unwrap();

    if target == "list" {
        list(arg, nr)
    } else {
        let mut props: Vec<&str> = Vec::new();
        while let Some(prop) = arg.pop_front() {
            props.push(prop);
        }

        let props = parse_strings(&props);
        if let [key] = &props[..] {
            return boxup(
                format!("Lookup {}:{}", target, key),
                target_lookup(target, key),
                BoxupOptions::new().line_after_title(true),
            );
        }

        String::from("new")
    }
}
