use crate::colors;
use crate::table::constants::TABLE;
use crate::table::utils::display_group;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

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
        let mut result;

        let content = self.content();

        let mut element_color_map: HashMap<String, String> = HashMap::new();
        let mut group_color_map: Vec<(String, String)> = Vec::new();

        result = format!("╭{}", self.table_name);
        result.push_str(
            (0..(54 - self.table_name.len()))
                .map(|_| "─")
                .collect::<String>()
                .as_str(),
        );
        result.push_str("╮\r\n");

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
            for (i, curr) in group.iter().enumerate() {
                let push = if curr.len() == 1 { " " } else { "" };

                if i == 0 {
                    result.push('│');
                }

                result.push_str(element_color_map.get(*curr).unwrap_or(&String::from("")));
                result.push_str(curr);
                result.push_str(push);
                result.push(' ');
                result.push_str("\x1b[0m");

                if i == 17 {
                    result.push('│');
                }
            }
            result.push_str("\r\n");
        }

        result.push('│');
        result.push_str((0..54).map(|_| " ").collect::<String>().as_str());
        result.push_str("│\r\n");

        let mut curr_obj: &(String, String);
        let group_count = group_color_map.len() / 2;

        for i in 0..group_count {
            curr_obj = &group_color_map[i];
            display_group(curr_obj, &mut result, 0);
            curr_obj = &group_color_map[i + group_count];
            display_group(curr_obj, &mut result, 1);
            result.push_str("\r\n");
        }

        if group_count * 2 != group_color_map.len() {
            curr_obj = group_color_map.last().unwrap();
            display_group(curr_obj, &mut result, 0);
            for _ in 0..27 {
                result.push(' ');
            }
            result.push_str("│\r\n");
        }
        result.push('╰');
        result.push_str((0..54).map(|_| "─").collect::<String>().as_str());
        result.push_str("╯\r\n");
        result
    }
}