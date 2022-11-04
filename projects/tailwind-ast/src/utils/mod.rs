use std::mem::{swap, take};

use crate::AstStyle;

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

/// Catch any valid css class
pub fn eat_arbitrary(s: &str) -> Result<(&str, usize), &'static str> {
    let mut offset = 0;
    let mut depth = 1;
    for char in s.chars() {
        offset += char.len_utf8();
        match char {
            '[' => depth += 1,
            ']' => depth -= 1,
            _ => {}
        }
        if depth == 0 {
            // do not catch last `]`
            offset -= 1;
            return Ok((&s[0..offset], offset));
        }
    }
    Err("arbitrary unbalance")
}

/// Expand ast group to single elements
pub fn expand(groups: &mut Vec<AstStyle>, hint_size: usize) -> usize {
    let mut count = 0;
    let mut buffer = Vec::with_capacity(hint_size);
    swap(groups, &mut buffer);
    for mut parent in buffer {
        count += expand_hint(&parent);
        let children = take(&mut parent.children);
        if children.is_empty() {
            groups.push(parent)
        }
        else {
            for child in children {
                groups.push(child.expand_visit(&parent))
            }
        }
    }
    count
}

/// Expand next
pub fn expand_nested(mut groups: Vec<AstStyle>, depth: usize) -> Vec<AstStyle> {
    let mut hint = groups.iter().map(expand_hint).sum();
    for _ in 0..depth {
        hint = expand(&mut groups, hint);
        if expand_can_finish(&groups) {
            return groups;
        }
    }
    groups
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
