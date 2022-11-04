use crate::AstStyle;
use std::mem::take;

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

/// Expand ast group to single elements
pub fn expand(groups: Vec<AstStyle>, hint_size: usize) -> (Vec<AstStyle>, usize) {
    let mut out = Vec::with_capacity(hint_size);
    let mut count = 0;
    for mut parent in groups {
        count += expand_hint(&parent);
        let children = take(&mut parent.children);
        for child in children {
            out.push(child.expand_visit(&parent))
        }
    }
    (out, count)
}

/// Count output elements after expand
pub fn expand_hint(ast: &AstStyle) -> usize {
    match ast.children.len() {
        0 => 1,
        _ => ast.children.len(),
    }
}

/// Should do next expand
pub fn expand_can_finish(v: &[AstStyle]) -> bool {
    for item in v.iter() {
        if !item.children.is_empty() {
            return false;
        }
    }
    true
}
