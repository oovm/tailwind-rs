use super::*;

impl Hash for CssBundle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.attribute.hash(state);
        self.addition.hash(state);
    }
}
