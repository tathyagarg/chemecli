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
