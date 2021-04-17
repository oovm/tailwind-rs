use super::*;

impl Color {
    /// Parse predefined colors at compile time
    /// https://www.w3.org/wiki/CSS/Properties/color/keywords
    #[track_caller]
    pub const fn predefined(name: &str) -> Color {
        match () {
            _ if const_eq!(name, "red") => Color { r: 255, g: 0, b: 0, a: 0.0 },
            _ => panic!("unknown color code"),
        }
    }
}
