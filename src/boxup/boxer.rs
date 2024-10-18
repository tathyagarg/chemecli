use super::utils::len;
use crate::utils::wrap;
use std::cmp::max;
use std::iter::zip;

use super::models::{Alignment::*, BoxupOptions, OverflowHandler::*};

pub fn boxup(title: String, content: String, options: BoxupOptions) -> String {
    let wrapped: Vec<String>;

    let mut elements = content
        .split("\n")
        .filter(|elem| len(elem) > 0)
        .collect::<Vec<&str>>();
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
                            format!("{}...", elem[..(options.max_width - 5)].to_string())
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

    for elem in elements {
        buffer.push_str(
            match options.alignment {
                Left => format!("│{:<longest$}│\r\n", elem),
                Center => format!("│{:^longest$}│\r\n", elem),
                Right => format!("│{:>longest$}│\r\n", elem),
            }
            .as_str(),
        );

        //        print!("{:?}\r\n", buffer);
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
