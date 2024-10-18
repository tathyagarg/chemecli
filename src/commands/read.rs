use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use textwrap::Options;

use crate::boxup::boxer::{adjoin, boxup};
use crate::boxup::models::{BoxupOptions, OverflowHandler};
use crate::commands::utils::parse_strings;
use crate::notes::NotesReader;
use crate::utils::wrap;
use std::cmp::Ordering;
use std::collections::VecDeque;

pub fn read(arg: &mut VecDeque<&str>, nr: &NotesReader) -> String {
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

    let mut keys = String::new();
    let mut values = String::new();

    let key_opts = Options::new(longest_key).break_words(true);
    let value_opts = Options::new(longest_value).break_words(true);

    for (key, value) in &data {
        let mut kbuf: Vec<String> = if key.len() > longest_key {
            wrap(key, &key_opts)
        } else {
            Vec::from([key.to_string()])
        };

        let mut vbuf = if value.len() > longest_value {
            wrap(value, &value_opts)
        } else {
            Vec::from([value.to_string()])
        };

        match vbuf.len().cmp(&kbuf.len()) {
            Ordering::Less => {
                for _ in 0..(kbuf.len() - vbuf.len()) {
                    vbuf.push(" ".to_string());
                }
            }
            Ordering::Greater => {
                for _ in 0..(vbuf.len() - kbuf.len()) {
                    kbuf.push(" ".to_string())
                }
            }
            Ordering::Equal => {}
        };

        keys.push_str(
            format!(
                "{}\n",
                kbuf.iter()
                    .fold(String::new(), |acc, elem| format!("{}{}\n", acc, elem))
            )
            .as_str(),
        );

        values.push_str(
            format!(
                "{}\n",
                vbuf.iter()
                    .fold(String::new(), |acc, elem| format!("{}{}\n", acc, elem))
            )
            .as_str(),
        );
    }

    adjoin(
        boxup(
            "Keys".to_string(),
            keys[..(keys.len() - 2)].to_string(),
            BoxupOptions::new()
                .max_width(longest_key + 2)
                .overflow_handler(OverflowHandler::Ellipses),
        ),
        boxup(
            "Values".to_string(),
            values[..(values.len() - 2)].to_string(),
            BoxupOptions::new()
                .max_width(longest_value + 2)
                .overflow_handler(OverflowHandler::Ellipses),
        ),
    )
}
