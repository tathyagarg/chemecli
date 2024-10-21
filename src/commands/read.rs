use std::collections::VecDeque;

use textwrap::Options;

use super::parse_strings;
use crate::{
    boxup::{
        boxer::{adjoin, boxup, weaver},
        models::{BoxupOptions, OverflowHandler},
    },
    notes::NotesReader,
};

pub fn read(arg: &mut VecDeque<&str>, nr: &mut NotesReader) -> String {
    let target = String::from(arg.pop_front().unwrap());
    let mut data: Vec<(String, String)> = nr.get_notes(&target);
    let mut props: Vec<&str> = Vec::new();

    while let Some(prop) = arg.pop_front() {
        props.push(prop);
    }

    if !props.is_empty() {
        let props = parse_strings(&props);
        let mut buffer: Vec<(String, String)> = Vec::new();
        for (key, value) in &data {
            if props.contains(key) {
                buffer.push((key.clone(), value.clone()));
            }
        }

        data = buffer;
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
