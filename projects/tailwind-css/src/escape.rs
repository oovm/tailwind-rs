//! CSS identifier escaping for class selectors derived from `candidate_key`.

/// Escape a candidate key so it is safe as a CSS class name (without the leading `.`).
///
/// Mirrors the practical subset of `CSS.escape` used by Tailwind class selectors:
/// non `[a-zA-Z0-9_-]` code points (and digits at the start) are backslash-escaped.
pub fn escape_class_name(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for (i, ch) in raw.chars().enumerate() {
        let safe = matches!(ch, 'a'..='z' | 'A'..='Z' | '_' | '-')
            || (ch.is_ascii_digit() && i > 0);
        if safe {
            out.push(ch);
        } else {
            out.push('\\');
            out.push(ch);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_variant_and_arbitrary() {
        assert_eq!(escape_class_name("hover:bg-red-500"), r"hover\:bg-red-500");
        assert_eq!(escape_class_name("!p-4"), r"\!p-4");
        assert_eq!(escape_class_name("p-[1.5rem]"), r"p-\[1\.5rem\]");
        assert_eq!(escape_class_name("w-1/2"), r"w-1\/2");
    }
}
