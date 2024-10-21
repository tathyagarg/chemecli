use super::len;
use super::wrap;
use std::cmp::max;
use std::cmp::Ordering;
use std::iter::zip;
use textwrap::Options;

use super::models::{Alignment::*, BoxupOptions, OverflowHandler::*};

pub fn boxup(title: String, content: String, options: BoxupOptions) -> String {
    let wrapped: Vec<String>;

    let mut elements = content.split("\n").collect::<Vec<&str>>();
    let mut longest = max(
        elements.clone().iter().map(|elem| len(elem)).max().unwrap(),
        title.len() + 1,
    );

    if longest > options.max_width {
        elements = match options.overflow_handler {
            Wrap => {
                wrapped = wrap(
                    &content,
                    &textwrap::Options::new(options.max_width).break_words(true),
                );
                wrapped
                    .iter()
                    .map(|elem| elem.as_str())
                    .collect::<Vec<&str>>()
            }
            Ellipses => {
                wrapped = (elements)
                    .iter()
                    .map(|elem| {
                        if elem.len() > options.max_width {
                            format!("{}...", &elem[..(options.max_width - 5)])
                        } else {
                            elem.to_string()
                        }
                    })
                    .collect::<Vec<String>>();

                wrapped
                    .iter()
                    .map(|elem| elem.as_str())
                    .collect::<Vec<&str>>()
            }
        }
    }

    longest = options.max_width - 2;

    let mut buffer = String::new();
    buffer.push('╭');
    buffer.push_str(title.as_str());
    buffer.push_str(
        (0..longest - title.len())
            .map(|_| "─")
            .collect::<String>()
            .as_str(),
    );
    buffer.push_str("╮\r\n");

    if options.line_after_title {
        buffer.push('├');
        buffer.push_str((0..longest).map(|_| "─").collect::<String>().as_str());
        buffer.push_str("┤\r\n");
    }

    println!("{:?}", elements);

    for elem in elements {
        if !elem.is_empty() {
            buffer.push_str(
                match options.alignment {
                    Left => format!("│{:<longest$}│\r\n", elem),
                    Center => format!("│{:^longest$}│\r\n", elem),
                    Right => format!("│{:>longest$}│\r\n", elem),
                }
                .as_str(),
            );
        } else if options.line_after_newline {
            buffer.push_str(format!("├{:─^longest$}┤\r\n", elem).as_str());
        }
    }

    buffer.push('╰');
    buffer.push_str((0..longest).map(|_| "─").collect::<String>().as_str());
    buffer.push_str("╯\r\n");

    buffer
}

pub fn adjoin(box1: String, box2: String) -> String {
    let mut buffer = String::new();
    for (l1, l2) in zip(box1.split("\r\n"), box2.split("\r\n")) {
        buffer.push_str(format!("{}{}\r\n", l1, l2).as_str());
    }

    buffer.pop();
    buffer.pop();
    buffer
}

pub fn weaver(
    data: &Vec<(String, String)>,
    longest_key: usize,
    longest_value: usize,
    key_opts: Options,
    value_opts: Options,
) -> (String, String) {
    let mut keys = String::new();
    let mut values = String::new();

    for (key, value) in data {
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

    // Get elements in 0..-2 to remove the trailing \n\n
    (
        keys[..(keys.len() - 2)].to_string(),
        values[..values.len() - 2].to_string(),
    )
}
