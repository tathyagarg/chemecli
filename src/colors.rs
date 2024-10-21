use std::collections::HashMap;

use json::JsonValue;

pub const RESET: &str = "\x1b[0m";

pub fn json_to_rgb(json_object: &JsonValue) -> [u8; 3] {
    if let json::JsonValue::Array(items) = json_object {
        [
            items[0].as_u8().unwrap(),
            items[1].as_u8().unwrap(),
            items[2].as_u8().unwrap(),
        ]
    } else {
        generic_to_rgb(json_object.as_str().unwrap())
    }
}
pub fn generic_to_rgb(generic: &str) -> [u8; 3] {
    let color_map: HashMap<&str, [u8; 3]> = HashMap::from([
        ("RED", [255, 0, 0]),
        ("GREEN", [0, 255, 0]),
        ("BLUE", [0, 0, 255]),
        ("YELLOW", [255, 255, 0]),
        ("MAGENTA", [255, 0, 255]),
        ("CYAN", [0, 255, 255]),
    ]);

    color_map.get(&generic).copied().unwrap_or([255, 255, 255])
}

pub fn rgb_to_hex(rgb: &[u8; 3]) -> String {
    format!("\x1b[2;38;2;{};{};{}m", rgb[0], rgb[1], rgb[2])
}
