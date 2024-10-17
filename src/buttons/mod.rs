pub mod models;
use std::iter::zip;

pub fn make_button_row(prev_button: &models::Button, next_button: &models::Button) -> String {
    let mut buffer = String::new();
    for (l1, l2) in zip(
        prev_button.display().split("\r\n"),
        next_button.display().split("\r\n"),
    ) {
        buffer.push_str(format!("{}{}\r\n", l1, l2).as_str());
    }

    buffer.pop();
    buffer.pop();
    buffer
}
