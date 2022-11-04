/// Catch any valid css class
/// https://stackoverflow.com/questions/448981/which-characters-are-valid-in-css-class-names-selectors
pub fn eat_css_class(s: &str) -> Result<(&str, usize), &'static str> {
    let mut offset = 0;
    for char in s.chars() {
        if char.is_ascii_alphanumeric() { offset += char.len_utf8() } else { break }
    }
    match offset {
        0 => Err("Except ascii alphanumeric"),
        _ => Ok((&s[0..offset], offset)),
    }
}
