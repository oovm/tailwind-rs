const fn parse_bool(s: &str) -> bool {
    match s {
        "true" => true,
        "false" => false,
        other => panic!("`{}` is not a valid bool", other),
    }
}
#[test]
fn test () {
    parse_bool("true");
    parse_bool("false");
    parse_bool("foo");
}


/// Color in rgba format,
/// where {red,green,blue} in 0..255,
/// alpha in 0.0..1.0
/// https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/rgba()
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    /// red channel, ranges from 0 to 255
    pub r: u8,
    /// green channel, ranges from 0 to 255
    pub g: u8,
    /// blue channel, ranges from 0 to 255
    pub b: u8,
    /// alpha channel, ranges from 0.0 to 1.0
    pub a: f32,
}

impl Display for Color {

}