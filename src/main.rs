// #[macro_use]
extern crate json;
extern crate termion;
extern crate textwrap;

use boxup::boxer::adjoin;
use buttons::Button;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use std::vec::Vec;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod boxup;
mod buttons;
mod colors;
mod commands;
mod notes;
mod table;
mod utils;

fn main() {
    // -------------- INITIALIZATION --------------
    let source_file: PathBuf = PathBuf::from("data.json");
    let mut nr = notes::NotesReader::new(PathBuf::from("elements.json"));
    let mut temp_buffer: String = String::new();
    let mut buffer: String = String::new();

    let table_names: Vec<String> = utils::get_tables(&source_file);
    let tables: Vec<table::models::Table> = (table_names.clone())
        .iter()
        .map(|table_name| table::models::Table::new(source_file.clone(), table_name.clone()))
        .collect::<Vec<table::models::Table>>();
    let table_count = tables.len();

    let mut curr: usize = 0;

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut prev_button: Button = Button::new(
        String::from("<="),
        String::from(table_names.last().unwrap()),
        28,
    );

    let mut next_button: Button =
        Button::new(String::from("=>"), String::from(&table_names[1]), 28);

    write!(
        stdout,
        "{}{}{}{}{}\r\n",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        tables[curr].display(),
        adjoin(prev_button.display(), next_button.display()),
        buffer
    )
    .unwrap();

    stdout.flush().unwrap();

    // -------------- EVENT LOOP --------------
    for k in stdin.keys() {
        match k.as_ref().unwrap() {
            Key::Ctrl('c') => break,
            Key::Char('\n') => {
                buffer = temp_buffer.clone();
                temp_buffer = String::new();
                buffer = commands::parse_command(&mut nr, &buffer);
            }
            Key::Char(letter) => {
                buffer = String::new();
                temp_buffer.push(*letter);
            }
            Key::Backspace => {
                temp_buffer.pop();
            }
            Key::Left => curr = if curr == 0 { table_count - 1 } else { curr - 1 },
            Key::Right => curr = (curr + 1) % table_count,
            _ => {}
        }

        prev_button.update(table_names[if curr == 0 { table_count - 1 } else { curr - 1 }].clone());
        next_button.update(table_names[(curr + 1) % table_count].clone());

        write!(
            stdout,
            "{}{}{}{}{}{}\r\n",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            tables[curr].display(),
            adjoin(prev_button.display(), next_button.display()),
            temp_buffer,
            buffer
        )
        .unwrap();

        stdout.flush().unwrap();
    }
}
