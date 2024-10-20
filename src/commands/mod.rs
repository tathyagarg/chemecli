pub mod utils;

pub mod delete;
pub mod lookup;
pub mod molar_mass;
pub mod read;
pub mod write;

use delete::delete;
use lookup::{list, lookup};
use molar_mass::molar_mass;
use read::read;
use write::{add, update};

use crate::{
    boxup::{boxer::boxup, models::BoxupOptions},
    notes::NotesReader,
};
use std::collections::VecDeque;

fn appendage(
    count_buffer: &mut u32,
    elems: &mut Vec<String>,
    stack: &mut Vec<String>,
    buffer: &mut String,
    condition: bool,
) {
    if *count_buffer > 0 {
        if (*elems).len() > 0 {
            for _ in 0..*count_buffer {
                (*stack).extend((*elems).clone());
            }
            *elems = Vec::new();
        } else {
            for _ in 0..*count_buffer {
                (*stack).push(buffer.clone());
            }
            *buffer = String::new();
        }
        *count_buffer = 0;
    } else if (*buffer).len() > 0 && condition {
        (*stack).push(buffer.clone());
        *buffer = String::new();
    }
}

fn parse(target: &str) -> Vec<String> {
    let mut stack: Vec<String> = Vec::new();
    let mut elems: Vec<String> = Vec::new();
    let mut buffer: String = String::new();
    let mut count_buffer: u32 = 0;
    let mut skip: usize = 0;

    for (i, curr) in target.chars().enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }

        if curr == '(' {
            for (j, next) in target[i..].chars().enumerate() {
                if next == ')' {
                    skip = j - i;
                    break;
                }
            }
            elems = parse(&(target[i + 1..]));
        }

        if curr.is_ascii_alphabetic() {
            appendage(
                &mut count_buffer,
                &mut elems,
                &mut stack,
                &mut buffer,
                curr.is_ascii_uppercase(),
            );
            buffer.push(curr);
        }

        if curr.is_ascii_digit() {
            count_buffer *= 10;
            count_buffer += (curr as u32) - 48;
        }

        if curr == ')' {
            appendage(&mut count_buffer, &mut elems, &mut stack, &mut buffer, true);
            return stack;
        }
    }
    appendage(&mut count_buffer, &mut elems, &mut stack, &mut buffer, true);
    stack
}

pub fn command_not_found(_: &mut VecDeque<&str>, _: &mut NotesReader) -> String {
    boxup(
        "Error".to_string(),
        "Command not found".to_string(),
        BoxupOptions::new().line_after_title(true),
    )
}

pub fn parse_command(nr: &mut NotesReader, command: &String) -> String {
    let mut parts = command.as_str().split(" ").collect::<VecDeque<&str>>();
    let command = parts.pop_front();

    let action: fn(&mut VecDeque<&str>, &mut NotesReader) -> String = match command {
        Some("read") | Some("r") => read,
        Some("add") | Some("a") => add,
        Some("update") | Some("u") => update,
        Some("delete") | Some("d") => delete,
        Some("lookup") | Some("l") => lookup,
        Some("ll") => list,
        Some("molar") | Some("mm") => molar_mass,
        _ => command_not_found,
    };

    action(&mut parts, nr)
}
