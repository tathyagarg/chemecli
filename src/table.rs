extern crate json;

use crate::colors;
use std::collections::HashMap;
use std::vec::Vec;
use termion::terminal_size;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub struct Table {
    pub source_file: PathBuf,
    pub table_name: String,
}

const TABLE: [[&str; 18]; 10] = [
    [
        "H", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ",
        "  ", "  ", "He",
    ],
    [
        "Li", "Be", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "B", "C", "N", "O",
        "F", "Ne",
    ],
    [
        "Na", "Mg", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "Al", "Si", "P",
        "S", "Cl", "Ar",
    ],
    [
        "K", "Ca", "Sc", "Ti", "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ga", "Ge", "As",
        "Se", "Br", "Kr",
    ],
    [
        "Rb", "Sr", "Y", "Zr", "Nb", "Mo", "Tc", "Ru", "Rh", "Pd", "Ag", "Cd", "In", "Sn", "Sb",
        "Te", "I", "Xe",
    ],
    [
        "Cs", "Ba", "Lu", "Hf", "Ta", "W", "Re", "Os", "Ir", "Pt", "Au", "Hg", "Tl", "Pb", "Bi",
        "Po", "At", "Rn",
    ],
    [
        "Fr", "Ra", "Lr", "Rf", "Db", "Sg", "Bh", "Hs", "Mt", "Ds", "Rg", "Cn", "Nh", "Fl", "Mc",
        "Lv", "Ts", "Og",
    ],
    [
        "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ",
        "  ", "  ", "  ",
    ],
    [
        "  ", "  ", "La", "Ce", "Pr", "Nd", "Pm", "Sm", "Eu", "Gd", "Tb", "Dy", "Ho", "Er", "Tm",
        "Yb", "  ", "  ",
    ],
    [
        "  ", "  ", "Ac", "Th", "Pa", "U", "Np", "Pu", "Am", "Cm", "Bk", "Cf", "Es", "Fm", "Md",
        "No", "  ", "  ",
    ],
];

fn get_terminal_width() -> u16 {
    let (x, _) = terminal_size().unwrap();
    x
}

fn seperator() {
    for _ in 0..get_terminal_width() {
        print!(".");
    }
    println!();
}

fn display_group(curr_obj: &(String, String), width: &u16) {
    let (curr_group, curr_color) = curr_obj;
    print!("{}█\x1b[0m {}", curr_color, curr_group);
    for _ in 0..((width / 2) - (curr_group.len() + 2) as u16) {
        print!(" ");
    }
}

impl Table {
    pub fn new(source_file: PathBuf, table_name: String) -> Table {
        Table {
            source_file,
            table_name,
        }
    }

    pub fn content(&self) -> json::JsonValue {
        let mut file = File::open(&self.source_file).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let json_data = json::parse(&contents).unwrap();

        json_data[&self.table_name].clone()
    }

    pub fn display(&self) {
        let content = self.content();

        let mut element_color_map: HashMap<String, String> = HashMap::new();
        let mut group_color_map: Vec<(String, String)> = Vec::new();

        for group in content["groups"].members() {
            let json_color = &group["color"];
            let color: [u8; 3] = colors::json_to_rgb(json_color);

            let hex = colors::rgb_to_hex(&color);

            for element in group["elements"].members() {
                element_color_map.insert(String::from(element.as_str().unwrap()), hex.clone());
            }

            group_color_map.push((String::from(group["name"].as_str().unwrap()), hex.clone()));
        }

        for group in TABLE {
            for curr in group {
                let push = if curr.len() == 1 { " " } else { "" };

                print!(
                    "{}{}{}\x1b[0m ",
                    element_color_map.get(curr).unwrap_or(&String::from("")),
                    curr,
                    push
                );
            }
            println!();
        }

        seperator();

        let width = get_terminal_width();

        let mut curr_obj: &(String, String);

        let group_count = group_color_map.len() / 2;

        for i in 0..group_count {
            curr_obj = &group_color_map[i];
            display_group(curr_obj, &width);
            curr_obj = &group_color_map[i + group_count];
            display_group(curr_obj, &width);
        }
    }
}
