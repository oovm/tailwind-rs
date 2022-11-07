use serde::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer};
use std::fs::read_to_string;

use tailwind_ast::{expand_nested, parse, AstStyle};
use tailwind_error::{TailwindError, TextStorage};

#[test]
fn ready() {
    println!("it works!")
}

mod simple;

pub fn as_json_string(value: impl Serialize) -> String {
    let buf = Vec::new();
    let formatter = PrettyFormatter::with_indent(b"    ");
    let mut ser = Serializer::with_formatter(buf, formatter);
    value.serialize(&mut ser).unwrap();
    String::from_utf8(ser.into_inner()).unwrap()
}

// #[track_caller]
pub fn test_expand(store: &mut TextStorage, path: &str) -> Result<(), TailwindError> {
    let input = format!("tests/{path}.in.txt");
    let output = format!("tests/{path}.out.txt");
    let output = read_to_string(output)?;

    let file = store.file(input)?;
    let text = store.get_text(&file)?;
    match parse(text) {
        Ok(o) => assert_expand(o, 8, &output),
        Err(e) => {
            TailwindError::emit(&store, &[e.with_file(&file).as_report()])?;
        }
    };
    Ok(())
}

fn assert_expand(ast: Vec<AstStyle>, depth: usize, output: &str) {
    let styles: Vec<_> = expand_nested(ast, depth).into_iter().map(|v| v.to_string()).collect();
    assert_eq!(styles.join("\n"), output.trim());
}
