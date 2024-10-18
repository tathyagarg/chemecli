use crate::colors;

pub fn display_group(curr_obj: &(String, String), result: &mut String, start: u8) {
    let (curr_group, curr_color) = curr_obj;

    (*result).push_str(curr_color);
    (*result).push('█');
    (*result).push_str(colors::RESET);
    (*result).push(' ');
    (*result).push_str(curr_group);

    for _ in 0..(27 - (curr_group.len() + 2) as u16) {
        (*result).push(' ');
    }
}
