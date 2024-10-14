use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

extern crate json;

use json::JsonValue;

use crate::colors;

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
        "", "  ", "La", "Ce", "Pr", "Nd", "Pm", "Sm", "Eu", "Gd", "Tb", "Dy", "Ho", "Er", "Tm",
        "Yb", "  ", "  ",
    ],
    [
        "  ", "  ", "Ac", "Th", "Pa", "U", "Np", "Pu", "Am", "Cm", "Bk", "Cf", "Es", "Fm", "Md",
        "No", "  ", "  ",
    ],
];

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

        let mut color_map: HashMap<String, String> = HashMap::new();

        for i in TABLE {
            for j in i {
                color_map.insert(j.to_string(), String::new());
            }
        }

        for group in content["groups"].members() {
            let json_color = &group["color"];
            let color: [u8; 3] = colors::json_to_rgb(json_color);

            for element in group["elements"].members() {
                *color_map
                    .get_mut(&String::from(element.as_str().unwrap()))
                    .unwrap() = colors::rgb_to_hex(&color);
            }
        }

        for i in TABLE {
            for j in i {
                let push = if j.len() == 1 { " " } else { "" };

                print!("{}{}{}\x1b[0m ", color_map[j], j, push);
            }
            println!();
        }
    }
}
