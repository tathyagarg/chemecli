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
