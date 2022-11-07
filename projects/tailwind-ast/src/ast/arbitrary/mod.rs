use std::fmt::Write;

use tailwind_error::{error_3rd::Srgb, Result};

use super::*;

mod from;
mod methods;

impl Default for TailwindArbitrary {
    fn default() -> Self {
        Self { inner: Box::from("") }
    }
}

impl Display for TailwindArbitrary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;
        for c in self.inner.chars() {
            match c {
                ' ' => f.write_char('_')?,
                _ => f.write_char(c)?,
            }
        }
        f.write_char(']')
    }
}

impl TailwindArbitrary {
    /// Get class name of arbitrary
    pub fn get_class(&self) -> String {
        let mut class = String::with_capacity(self.inner.len() + 2);
        class.push('[');
        for c in self.inner.chars() {
            match c {
                ' ' => class.push('_'),
                _ => class.push(c),
            }
        }
        class.push(']');
        class
    }
    /// Write arbitrary into formatter
    pub fn write(&self, f: &mut Formatter) -> std::fmt::Result {
        self.write_class(f, "")
    }
    /// Write class name into formatter
    pub fn write_class(&self, f: &mut Formatter, before: &str) -> std::fmt::Result {
        write!(f, "{}{}", before, self.get_class())
    }
    /// Get properties of arbitrary
    pub fn get_properties(&self) -> String {
        self.inner.to_string()
    }
}
