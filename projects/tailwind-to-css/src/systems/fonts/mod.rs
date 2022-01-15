use super::*;

pub use self::font_size::FontSize;

mod builtin;
mod font_size;

#[derive(Clone, Debug, Default)]
pub struct FontSystem {
    size: HashMap<String, FontSize>,
    family: HashMap<String, Vec<String>>,
    tracking: HashMap<String, f32>,
}

impl FontSystem {
    #[inline]
    pub fn get_size(&self, name: &str) -> FontSize {
        match self.size.get(name).cloned() {
            None => FontSize::new(1.0, -1.0),
            Some(s) => s,
        }
    }
    /// Insert a new font size
    #[inline]
    pub fn insert_size(&mut self, name: impl Into<String>, size: FontSize) -> Option<FontSize> {
        self.size.insert(name.into(), size)
    }
    /// Get the named font family,
    ///
    /// never fail, fallback to the `serif, sans-serif, monospace`
    #[inline]
    pub fn get_family(&self, name: &str) -> String {
        match self.family.get(name) {
            None => String::new(),
            Some(s) => s.join(", "),
        }
    }
    /// Insert a new font family
    #[inline]
    pub fn insert_family(&mut self, name: impl Into<String>, family: &str) -> Option<Vec<String>> {
        let family = Self::normalize_family(family)?;
        self.family.insert(name.into(), family)
    }
    #[inline]
    fn normalize_family(input: &str) -> Option<Vec<String>> {
        Some(vec![input.to_string()])
    }
    #[inline]
    pub fn get_tracking(&self, name: &str) -> f32 {
        self.tracking.get(name).cloned().unwrap_or(0.0)
    }
    /// Insert a new font size
    #[inline]
    pub fn insert_tracking(&mut self, name: impl Into<String>, size: f32) -> Option<f32> {
        self.tracking.insert(name.into(), size)
    }
}
