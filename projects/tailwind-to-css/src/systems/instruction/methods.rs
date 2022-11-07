use super::*;
use tailwind_ast::TailwindElements;

impl From<AstStyle> for TailwindInstruction {
    fn from(node: AstStyle) -> Self {
        Self {
            negative: Negative::from(node.negative),
            variants: node.variants.into_iter().map(|s| s.into()).collect(),
            elements: TailwindElements { items: node.elements.into_iter().map(|s| s.to_string()).collect() },
            arbitrary: TailwindArbitrary::from(node.arbitrary.unwrap_or_default()),
        }
    }
}

impl TailwindInstruction {
    #[inline]
    pub fn view_elements(&self) -> Vec<&str> {
        self.elements.items.iter().map(|s| s.as_str()).collect()
    }
    #[inline]
    pub fn view_arbitrary(&self) -> &TailwindArbitrary {
        &self.arbitrary
    }
    // TODO
    pub fn normalization(self) -> Self {
        self
    }
}
