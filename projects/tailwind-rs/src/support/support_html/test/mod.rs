use super::*;

#[test]
fn go() {
    let input = include_str!("arbitrary-values.html");
    let out = HtmlConfig::find_all_class(input).unwrap();
    println!("{:?}", out)
}
