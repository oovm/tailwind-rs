use super::*;

impl Default for CssBundle {
    fn default() -> Self {
        Self { inline: None, items: Default::default() }
    }
}
