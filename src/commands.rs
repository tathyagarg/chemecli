extern crate textwrap;

use itertools::{EitherOrBoth::*, Itertools};

use crate::notes::NotesReader;
use std::str::Split;

pub fn parse_command(nr: &NotesReader, command: &String) -> String {
    let mut parts = command.as_str().split(" ");
    let command = parts.next().unwrap();

    match command {
        "read" | "r" => read(&mut parts, nr),
        _ => String::from("none"),
    }
}

fn wrap(text: &String, opts: &textwrap::Options) -> Vec<String> {
    let mut res = Vec::new();

    for elem in textwrap::wrap(text.as_str(), opts) {
        res.push(elem.to_string());
    }

    res
}

fn read(arg: &mut Split<&str>, nr: &NotesReader) -> String {
    let target = String::from(arg.next().unwrap());
    let data = nr.get_notes(&target);
    let mut res = String::new();
    let mut longest_key = data.iter().map(|(k, _)| String::len(k)).max().unwrap() + 1;
    let mut longest_value = data.iter().map(|(_, v)| String::len(v)).max().unwrap() + 1;

    if longest_key + longest_value > 53 {
        longest_key = 15;
        longest_value = 38;
    }

    let mut title = format!("╭Read {}", target);
    title.push_str(
        (0..(longest_key + longest_value - (4 + target.len())))
            .map(|_| "─")
            .collect::<String>()
            .as_str(),
    );
    title.push_str("╮\r\n");

    res.push_str(title.as_str());
    res.push('├');
    res.push_str((0..longest_key).map(|_| "─").collect::<String>().as_str());
    res.push('┬');
    res.push_str((0..longest_value).map(|_| "─").collect::<String>().as_str());
    res.push_str("┤\r\n");

    let key_opts = textwrap::Options::new(15).break_words(true);
    let value_opts = textwrap::Options::new(38).break_words(true);

    let mut i = 0;

    for (key, value) in &data {
        if key.len() > longest_key {
            let wrapped_key = wrap(key, &key_opts);
            let wrapped_value = wrap(value, &value_opts);

            for pair in wrapped_key.iter().zip_longest(wrapped_value.iter()) {
                let (k, v) = match pair {
                    Both(k1, v1) => (k1, v1),
                    Left(k1) => (k1, &(0..longest_value).map(|_| " ").collect::<String>()),
                    Right(v1) => (&(0..longest_key).map(|_| " ").collect::<String>(), v1),
                };

                let mut resk = String::from(k);
                let mut resv = String::from(v);

                if k.len() < longest_key {
                    resk.push_str(
                        (0..(longest_key - k.len()))
                            .map(|_| " ")
                            .collect::<String>()
                            .as_str(),
                    );
                }

                if v.len() < longest_value {
                    resv.push_str(
                        (0..(longest_value - v.len()))
                            .map(|_| " ")
                            .collect::<String>()
                            .as_str(),
                    );
                }

                res.push_str(format!("│{}│{}│\r\n", resk, resv).as_str());
            }
        } else {
            res.push_str(
                format!(
                    "│{}{}│{}{}│\r\n",
                    textwrap::fill(key, &key_opts),
                    if key.len() <= longest_key {
                        (0..(longest_key - key.len()))
                            .map(|_| " ")
                            .collect::<String>()
                    } else {
                        String::new()
                    },
                    textwrap::fill(value, &value_opts),
                    if value.len() <= longest_value {
                        (0..(longest_value - value.len()))
                            .map(|_| " ")
                            .collect::<String>()
                    } else {
                        String::new()
                    },
                )
                .as_str(),
            );
        }

        i += 1;

        if i < data.len() {
            res.push_str(
                format!(
                    "├{}┼{}┤\r\n",
                    (0..longest_key).map(|_| "─").collect::<String>(),
                    (0..longest_value).map(|_| "─").collect::<String>()
                )
                .as_str(),
            );
        }
    }

    res.push('╰');
    res.push_str((0..longest_key).map(|_| "─").collect::<String>().as_str());
    res.push('┴');
    res.push_str((0..longest_value).map(|_| "─").collect::<String>().as_str());
    res.push_str("╯\r\n");

    res
}
