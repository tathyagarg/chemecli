use textwrap::Options;

use super::lookup::target_lookup;
use crate::{
    boxup::{
        boxer::{adjoin, boxup, weaver},
        models::{Alignment, BoxupOptions},
    },
    commands::parse,
    notes::NotesReader,
};
use std::collections::{HashMap, VecDeque};

fn calculate_molar_mass(target: HashMap<String, u32>) -> f32 {
    let mut res: f32 = 0.;

    for (k, v) in target {
        res += (v as f32) * target_lookup(k.as_str(), "mass").parse::<f32>().unwrap();
    }

    res
}

pub fn molar_mass(props: &mut VecDeque<&str>, _: &mut NotesReader) -> String {
    let mut data: Vec<(String, String)> = Vec::new();
    for target in props {
        data.push((
            target.to_string(),
            calculate_molar_mass(parse(target)).to_string(),
        ));
    }

    let (longest_key, longest_value) = (26, 26);
    let (keys, values) = weaver(
        &data,
        longest_key,
        longest_value,
        Options::new(longest_key),
        Options::new(longest_value),
    );

    adjoin(
        boxup(
            "Formula".to_string(),
            keys,
            BoxupOptions::new()
                .max_width(longest_key + 2)
                .alignment(Alignment::Center)
                .line_after_title(true),
        ),
        boxup(
            "Molar Mass".to_string(),
            values,
            BoxupOptions::new()
                .max_width(longest_value + 2)
                .alignment(Alignment::Center)
                .line_after_title(true),
        ),
    )
}
