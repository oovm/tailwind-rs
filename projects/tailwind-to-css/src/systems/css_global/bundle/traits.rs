use super::*;

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for CssBundle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.attribute.hash(state);
        self.addition.hash(state);
    }
}
