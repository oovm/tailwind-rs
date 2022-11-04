#[test]
fn ready() {
    println!("it works!")
}

use serde::Serialize;
use serde_json::json;

#[test]
fn main() {
    let obj = json!({"foo":1,"bar":2});

    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    obj.serialize(&mut ser).unwrap();
    println!("{}", String::from_utf8(ser.into_inner()).unwrap());
}
