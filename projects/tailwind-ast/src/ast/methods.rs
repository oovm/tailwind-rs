use super::*;

impl<'a> AstStyle<'a> {
    ///
    #[inline]
    pub fn is_self_reference(&self) -> bool {
        matches!(self.elements.as_slice(), ["&"])
    }
}

impl<'a> AstGroup<'a> {
    ///
    pub fn expand(self, styles: &mut Vec<AstStyle<'a>>) {
        let head = &self.head;
        for item in self.children {
            item.expand(styles, head)
        }
    }
}

impl<'a> Add<AstGroup<'a>> for AstStyle<'a> {
    type Output = AstGroup<'a>;

    fn add(self, rhs: AstGroup<'a>) -> Self::Output {
        let mut head = self;
        head.add_assign(&rhs.head);
        AstGroup { head, children: rhs.children }
    }
}

impl<'a> AstGroupItem<'a> {
    ///
    pub fn expand(self, styles: &mut Vec<AstStyle<'a>>, head: &AstStyle<'a>) {
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

impl<'a> AddAssign<&AstStyle<'a>> for AstStyle<'a> {
    fn add_assign(&mut self, rhs: &AstStyle<'a>) {
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
