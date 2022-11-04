use serde::Serialize;
use serde_json::json;

use tailwind_ast::{expand_nested, parse};

#[test]
fn ready() {
    println!("it works!")
}

mod simple;

#[test]
fn main() {
    let obj = json!({"foo":1,"bar":2});

    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    obj.serialize(&mut ser).unwrap();
    println!("{}", String::from_utf8(ser.into_inner()).unwrap());
}

#[track_caller]
pub fn test_expand(input: &str, output: &str, depth: usize) {
    let ast = parse(input).unwrap();
    let styles: Vec<_> = expand_nested(ast, depth).into_iter().map(|v| v.to_string()).collect();
    assert_eq!(styles.join("\n"), output.trim())
}
