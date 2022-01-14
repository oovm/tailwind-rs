use super::*;

mod traits;

#[derive(Copy, Clone, Debug)]
pub struct Negative(pub(crate) bool);

impl Negative {
    pub fn write(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            true => write!(f, "-"),
            false => write!(f, ""),
        }
    }
    pub fn get_properties(&self, value: &str) -> String {
        match self.0 {
            true => format!("-{}", value),
            false => value.to_string(),
        }
    }
}
