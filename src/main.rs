extern crate json;
extern crate termion;
extern crate textwrap;

use std::{
    io::{stdin, stdout, Write},
    path::PathBuf,
    vec::Vec,
};

use clap::Parser;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

mod boxup;
mod buttons;
mod colors;
mod commands;
mod notes;
mod table;
use boxup::boxer::adjoin;
use buttons::Button;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("tables.json"))]
    tables: String,

    #[arg(short, long, default_value_t = String::from("elements.json"))]
    elements: String,
}

fn main() {
    let args = Args::parse();

    // -------------- INITIALIZATION --------------
    let source_file: PathBuf = PathBuf::from(&args.tables);
    let mut nr = notes::NotesReader::new(PathBuf::from(&args.elements));
    let mut temp_buffer: String = String::new();
    let mut buffer: String = String::new();
    let mut history: Vec<String> = Vec::new();

    // -------------- TABLE DATA INITIALIZATION --------------
    let table_names: Vec<String> = table::get_tables(&source_file);
    let tables: Vec<table::models::Table> = (table_names.clone())
        .iter()
        .map(|table_name| table::models::Table::new(source_file.clone(), table_name.clone()))
        .collect::<Vec<table::models::Table>>();
    let table_count = tables.len();

    let mut curr_table: usize = 0;

    // -------------- I/O INITIALIZATION --------------
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut prev_button: Button = Button::new(
        String::from("<="),
        String::from(table_names.last().unwrap()),
        28,
    );

    let mut next_button: Button =
        Button::new(String::from("=>"), String::from(&table_names[1]), 28);

    // Write terminal
    write!(
        stdout,
        "{}{}{}{}{}\r\n",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        tables[curr_table].display(),
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
                history.push(temp_buffer.clone());
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
            Key::Left => {
                curr_table = if curr_table == 0 {
                    table_count - 1
                } else {
                    curr_table - 1
                }
            }
            Key::Right => curr_table = (curr_table + 1) % table_count,
            Key::Up => {
                buffer = String::new();
                temp_buffer = history.pop().unwrap();
            }
            _ => {}
        }

        // Update buttons to include names of next and previous tables in the respective buttons'
        // labels.
        prev_button.update(
            table_names[if curr_table == 0 {
                table_count - 1
            } else {
                curr_table - 1
            }]
            .clone(),
        );
        next_button.update(table_names[(curr_table + 1) % table_count].clone());

        // Rewrite terminal
        write!(
            stdout,
            "{}{}{}{}{}{}\r\n",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            tables[curr_table].display(),
            adjoin(prev_button.display(), next_button.display()),
            temp_buffer,
            buffer
        )
        .unwrap();

        stdout.flush().unwrap();
    }
}
