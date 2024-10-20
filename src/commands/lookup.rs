use textwrap::Options;

use crate::boxup::boxer::{adjoin, boxup, weaver};
use crate::boxup::models::{Alignment, BoxupOptions, OverflowHandler};
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
            boxup(
                format!("Lookup {}:{}", target, key),
                target_lookup(target, key),
                BoxupOptions::new().line_after_title(true),
            )
        } else {
            let mut data: Vec<(String, String)> = Vec::new();
            for prop in props {
                data.push((prop.clone(), target_lookup(target, &prop)))
            }

            let (longest_key, longest_value) = (12, 40);
            let (key, value) = weaver(
                &data,
                longest_key,
                longest_value,
                Options::new(longest_key).break_words(true),
                Options::new(longest_value).break_words(true),
            );

            adjoin(
                boxup(
                    "Keys".to_string(),
                    key,
                    BoxupOptions::new()
                        .max_width(longest_key + 2)
                        .overflow_handler(OverflowHandler::Ellipses)
                        .line_after_title(true)
                        .line_after_newline(true),
                ),
                boxup(
                    "Values".to_string(),
                    value,
                    BoxupOptions::new()
                        .max_width(longest_value + 2)
                        .overflow_handler(OverflowHandler::Ellipses)
                        .line_after_title(true)
                        .line_after_newline(true),
                ),
            )
        }
    }
}
