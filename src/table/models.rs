use std::{collections::HashMap, fs::File, io::prelude::*, path::PathBuf};

use super::constants::TABLE;
use crate::{
    boxup::{boxer::boxup, len, models::BoxupOptions},
    colors,
};

pub fn display_group(curr_obj: &(String, String), result: &mut String) {
    let (curr_group, curr_color) = curr_obj;

    (*result).push_str(curr_color);
    (*result).push('█');
    (*result).push_str(colors::RESET);
    (*result).push(' ');
    (*result).push_str(curr_group);

    // Adding 2 because of the full character + ' '
    // Adding 1 to compensate for the box border
    for _ in 0..(28 - (len(curr_group) + 2 + 1) as u16) {
        (*result).push(' ');
    }
}

pub struct Table {
    pub source_file: PathBuf,
    pub table_name: String,
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

    pub fn display(&self) -> String {
        let mut result = String::new();

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
            for curr in &group {
                // If current element symbol is 1 character long, append a " "
                let push = if curr.len() == 1 { " " } else { "" };

                result.push_str(element_color_map.get(*curr).unwrap_or(&String::from("")));
                result.push_str(curr);
                result.push_str(push);
                result.push(' ');
                result.push_str("\x1b[0m");
            }
            result.push('\n');
        }

        let mut curr_obj: &(String, String);
        let group_count = group_color_map.len() / 2;

        for i in 0..group_count {
            curr_obj = &group_color_map[i];
            display_group(curr_obj, &mut result);
            curr_obj = &group_color_map[i + group_count];
            display_group(curr_obj, &mut result);
            result.push('\n');
        }

        if group_count * 2 != group_color_map.len() {
            curr_obj = group_color_map.last().unwrap();
            display_group(curr_obj, &mut result);
            // Subtracting 1 to compensate for border
            for _ in 0..(28 - 1) {
                result.push(' ');
            }
        } else {
            result.pop();
        }

        boxup(
            self.table_name.clone(),
            result,
            BoxupOptions::new().max_width(56).line_after_title(true),
        )
    }
}
