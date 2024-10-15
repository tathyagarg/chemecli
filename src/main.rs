// #[macro_use]
extern crate json;
extern crate termion;

use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::iter::zip;
use std::path::PathBuf;
use std::vec::Vec;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod colors;
mod table;

fn get_tables(source_file: &PathBuf) -> Vec<String> {
    let mut file = File::open(source_file).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    let json_data = json::parse(&contents).unwrap();

    let mut curr;

    let mut tables = Vec::new();

    for entry in json_data.entries() {
        (curr, _) = entry;
        tables.push(String::from(curr));
    }

    tables
}

fn get_button(table_name: &String, direction: String) -> String {
    let mut buffer = String::new();

    buffer.push_str("╭──────────────────────────╮\n");
    buffer.push_str(format!("│{: ^26}│\n", direction).as_str());
    let name = if table_name.len() > 26 {
        &format!("{}...", &table_name.to_string()[0..21])
    } else {
        table_name
    };

    buffer.push_str(format!("│ {: ^24} │\n", name).as_str());
    buffer.push_str("╰──────────────────────────╯\n");

    buffer
}

fn get_button_row(prev_button: String, next_button: String) -> String {
    let mut buffer = String::new();
    for (l1, l2) in zip(prev_button.split('\n'), next_button.split('\n')) {
        buffer.push_str(format!("{}{}\r\n", l1, l2).as_str());
    }

    buffer
}

fn main() {
    let source_file: PathBuf = PathBuf::from("data.json");

    let table_names: Vec<String> = get_tables(&source_file);
    let mut tables: Vec<table::Table> = Vec::new();
    let mut table_count = 0;

    for table_name in table_names.clone() {
        tables.push(table::Table::new(source_file.clone(), table_name));
        table_count += 1;
    }

    let mut curr: usize = 0;

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    stdout.flush().unwrap();

    let mut prev_button: String = get_button(
        &String::from(table_names.last().unwrap()),
        String::from("<-"),
    );
    let mut next_button: String = get_button(&table_names[1], String::from("->"));

    write!(
        stdout,
        "{}{}",
        tables[curr].display(),
        get_button_row(prev_button, next_button)
    )
    .unwrap();

    for k in stdin.keys() {
        match k.as_ref().unwrap() {
            Key::Ctrl('c') => break,
            Key::Left => curr = if curr == 0 { table_count - 1 } else { curr - 1 },
            Key::Right => curr = (curr + 1) % table_count,
            _ => {}
        }

        prev_button = get_button(
            &table_names[if curr == 0 { table_count - 1 } else { curr - 1 }],
            String::from("<-"),
        );

        next_button = get_button(&table_names[(curr + 1) % table_count], String::from("->"));

        write!(
            stdout,
            "{}{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            tables[curr].display(),
            get_button_row(prev_button, next_button)
        )
        .unwrap();

        stdout.flush().unwrap();
    }
}
