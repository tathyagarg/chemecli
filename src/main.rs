#[macro_use]
extern crate json;
extern crate termion;

use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod table;

fn main() {
    let table = table::Table {
        source_file: PathBuf::from("data.json"),
        table_name: String::from("main"),
    };

    println!("{}", table.content());
    println!("\x1b[38;2;0;255;0mHello!\x1b[0m")
}

fn secondary() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut i: i8 = 0;
    let table_count = 3;

    let tables: [&str; 3] = ["Table Awesome", "Table Cool", "Table Bad"];

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    stdout.flush().unwrap();

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
    )
    .unwrap();

    write!(stdout, "{}{}", tables[i as usize], i).unwrap();

    for k in stdin.keys() {
        match k.as_ref().unwrap() {
            Key::Ctrl('c') => break,
            Key::Left => i = if i == 0 { table_count - 1 } else { i - 1 },
            Key::Right => i = (i + 1) % table_count,
            _ => {}
        }

        write!(
            stdout,
            "{}{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            tables[i as usize],
            i
        )
        .unwrap();

        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
