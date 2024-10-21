pub mod boxer;
pub mod models;

pub fn wrap(text: &String, opts: &textwrap::Options) -> Vec<String> {
    let mut res = Vec::new();

    for elem in textwrap::wrap(text.as_str(), opts) {
        res.push(elem.to_string());
    }

    res
}

pub fn len(elem: &str) -> usize {
    let mut length: usize = 0;
    let mut escape: bool = false;
    for char in elem.chars() {
        if char == '\x1b' {
            escape = true;
        }

        if char == 'm' && escape {
            escape = false;
        } else if escape {
            continue;
        } else {
            length += 1;
        }
    }

    length
}
