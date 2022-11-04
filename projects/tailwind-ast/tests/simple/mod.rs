use crate::test_expand;

// not-hover:sm:text-red-[200/50]
pub const INPUT1: &str = r#"
text

text!
text(red bold)!
text-(red-bold!)!
text(blue(bold!)!)!

hover:text
not-hover:text
sm:not-hover:text
"#;

pub const OUTPUT1: &str = r#"
text
text!
text-red!
text-bold!
text-red-bold!
text-blue-bold!
hover:text
not-hover:text
"#;

#[test]
fn test_group() {
    test_expand(INPUT1, OUTPUT1, 8);
}
