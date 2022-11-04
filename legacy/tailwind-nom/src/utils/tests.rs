use super::*;

#[test]
fn test_isize() {
    assert_eq!(parse_integer("0"), Ok(("", 0isize)));
    assert_eq!(parse_integer("42"), Ok(("", 42isize)));
}

#[test]
fn test_usize() {
    assert_eq!(parse_integer("0"), Ok(("", 0usize)));
    assert_eq!(parse_integer("42"), Ok(("", 42usize)));
}

#[test]
fn test_f32() {
    assert_eq!(parse_f32("0"), Ok(("", 0.0)));
    assert_eq!(parse_f32("42"), Ok(("", 42.0)));
    assert_eq!(parse_f32("99.99"), Ok(("", 99.99)));
}

#[test]
fn test_fraction() {
    assert_eq!(parse_fraction("1/12"), Ok(("", (1, 12))));
    assert_eq!(parse_fraction("12/2"), Ok(("", (12, 2))));
    assert_eq!(parse_fraction("12/24"), Ok(("", (12, 24))));
}
