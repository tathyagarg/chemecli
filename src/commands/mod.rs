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
use std::collections::{HashMap, VecDeque};

fn appendage(
    count_buffer: &mut u32,
    elems: &mut HashMap<String, u32>,
    stack: &mut HashMap<String, u32>,
    buffer: &mut String,
    condition: bool,
) {
    if *count_buffer > 0 {
        if !(*elems).is_empty() {
            for (k, v) in elems.clone() {
                (*stack).entry(k).and_modify(|x| *x += v).or_insert(v);
            }
            *elems = HashMap::new();
        } else {
            (*stack)
                .entry(buffer.clone())
                .and_modify(|x| *x += *count_buffer)
                .or_insert(*count_buffer);
            *buffer = String::new();
        }
        *count_buffer = 0;
    } else if !(*buffer).is_empty() && condition {
        (*stack)
            .entry(buffer.clone())
            .and_modify(|x| *x += 1)
            .or_insert(1);
        *buffer = String::new();
    }
}

fn parse(target: &str) -> HashMap<String, u32> {
    let mut stack: HashMap<String, u32> = HashMap::new();
    let mut elems: HashMap<String, u32> = HashMap::new();
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

pub fn parse_strings(items: &Vec<&str>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut buffer = String::new();

    for item in items {
        if item.ends_with('"') {
            let mut temp_buffer = item.chars();
            temp_buffer.next_back();
            if item.starts_with('"') {
                temp_buffer.next();
            }
            buffer.push_str(temp_buffer.collect::<String>().as_str());

            res.push(buffer);

            buffer = String::new();
        } else if !buffer.is_empty() {
            buffer.push_str(item);
            buffer.push(' ');
        } else if item.starts_with('"') {
            let mut temp_buffer = item.chars();
            temp_buffer.next();
            buffer.push_str(temp_buffer.collect::<String>().as_str());
            buffer.push(' ');
        } else {
            println!("Hello!");
            res.push(item.to_string());
        }
    }
    res
}

pub fn command_not_found(_: &mut VecDeque<&str>, _: &mut NotesReader) -> String {
    boxup(
        "Error".to_string(),
        "Command not found".to_string(),
        BoxupOptions::new().line_after_title(true),
    )
}

pub fn parse_command(nr: &mut NotesReader, command: &str) -> String {
    let mut parts = command.split(" ").collect::<VecDeque<&str>>();
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
