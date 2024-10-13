use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

extern crate json;

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
        "Ca", "Ba", "Lu", "Hf", "Ta", "W", "Re", "Os", "Ir", "Pt", "Au", "Hg", "Tl", "Pb", "Bi",
        "Po", "At", "Rn",
    ],
    [
        "Fr", "Ra", "Lr", "Rf", "Db", "Sg", "Bh", "Hs", "Mt", "Ds", "Rg", "Cn", "Nh", "Fl", "Mc",
        "Lv", "Ts", "Og",
    ],
    [
        "", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ", "  ",
        "  ", "  ", "  ",
    ],
    [
        "", "  ", "La", "Ce", "Pr", "Nd", "Pm", "Sm", "Eu", "Gd", "Tb", "Dy", "Ho", "Er", "Tm",
        "Yb", "  ", "  ",
    ],
    [
        "", "  ", "Ac", "Th", "Pa", "U", "Np", "Pu", "Am", "Cm", "Bk", "Cf", "Es", "Fm", "Md",
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
        for group in TABLE.iter() {
            for element in group.iter() {
                if element.len() == 1 {
                    print!("{} ", element);
                } else {
                    print!("{}", element);
                }
                print!(" ");
            }
            println!();
        }
    }
}
