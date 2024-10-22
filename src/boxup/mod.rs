pub mod boxer;
pub mod models;

pub fn wrap(text: &str, opts: &textwrap::Options) -> Vec<String> {
    let mut res = Vec::new();

    for elem in textwrap::wrap(text, opts) {
        res.push(elem.to_string());
    }

    res
}

pub fn len(elem: &str) -> usize {
    println!("{}\r", elem);

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

    println!("{}", length);
    // panic!();

    length
}
