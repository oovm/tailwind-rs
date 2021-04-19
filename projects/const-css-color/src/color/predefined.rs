use super::*;

impl Color {
    #[doc = include_str!("predefined.md")]
    #[track_caller]
    pub const fn predefined(name: &str) -> Color {
        match () {
            _ if const_eq!(name, "red") => Color { r: 255, g: 0, b: 0, a: 0.0 },
            _ => panic!("unknown color code"),
        }
    }
}
