use super::*;

impl AstStyle {
    /// Expand all group to simple styles
    ///
    /// ```tw
    /// text(red blue)
    ///   - text-red
    ///   - text-blue
    /// ```
    pub fn expand(&self) -> Vec<AstStyle> {
        let mut out = vec![];
        if self.children.is_empty() {
            out.push(self.clone());
            return out;
        }
        for x in &self.children {
            out.push(AstStyle {
                important: merge_important(x.important, self.important),
                negative: false,
                variants: vec![],
                elements: vec![],
                arbitrary: AstArbitrary { arbitrary: "".to_string() },
                children: vec![],
            })
        }
        out
    }
}

impl AstArbitrary {
    pub fn as_str(&self) -> &str {
        self.arbitrary.as_str()
    }
    pub fn is_empty(&self) -> bool {
        self.arbitrary.is_empty()
    }
}

#[inline]
fn merge_important(lhs: bool, rhs: bool) -> bool {
    lhs == true || rhs == true
}

#[inline]
fn merge_negative(lhs: bool, rhs: bool) -> bool {
    match (lhs, rhs) {
        (true, true) => true,
        (true, false) | (false, true) => true,
        (false, false) => false,
    }
}
