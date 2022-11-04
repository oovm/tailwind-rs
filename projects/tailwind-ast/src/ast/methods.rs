use super::*;

impl AstStyle {
    ///
    #[inline]
    pub fn is_self_reference(&self) -> bool {
        matches!(self.elements.as_slice(), ["&"])
    }
}

impl AstGroup {
    ///
    #[inline]
    pub fn expand(self, styles: &mut Vec<AstStyle>) {
        let head = &self.head;
        for item in self.children {
            item.expand_with_head(styles, head)
        }
    }
}

impl Add<AstGroup> for AstStyle {
    type Output = AstGroup;
    #[inline]
    fn add(self, rhs: AstGroup) -> Self::Output {
        let mut head = self;
        head.add_assign(&rhs.head);
        AstGroup { important: false, head, children: rhs.children }
    }
}

impl AstGroupItem {
    ///
    #[inline]
    pub fn expand(self, styles: &mut Vec<AstStyle>) {
        match self {
            Self::Grouped(g) => g.expand(styles),
            Self::Styled(rhs) => styles.push(rhs),
        }
    }

    ///
    #[inline]
    pub fn expand_with_head(self, styles: &mut Vec<AstStyle>, head: &AstStyle) {
        match self {
            Self::Grouped(g) => {
                let new = head.clone().add(g);
                new.expand(styles)
            }
            Self::Styled(rhs) => {
                let mut new = head.clone();
                new.add_assign(&rhs);
                styles.push(new)
            }
        }
    }
}

impl<'a> AddAssign<&AstStyle> for AstStyle {
    #[inline]
    fn add_assign(&mut self, rhs: &AstStyle) {
        self.negative = merge_negative(self.negative, rhs.negative);
        self.variants.extend(rhs.variants.iter().cloned());
        self.arbitrary = self.arbitrary.or(self.arbitrary);
        match rhs.is_self_reference() {
            true => {}
            false => self.elements.extend(rhs.elements.iter().cloned()),
        };
    }
}

#[inline]
fn merge_negative(lhs: bool, rhs: bool) -> bool {
    match (lhs, rhs) {
        (true, true) => true,
        (true, false) | (false, true) => true,
        (false, false) => false,
    }
}
