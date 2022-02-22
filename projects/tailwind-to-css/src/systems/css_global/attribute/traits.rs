use super::*;

impl Display for CssAttributes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.normal)?;
        if !self.transforms.is_empty() {
            write!(f, "transform:{}", self.transforms)?
        }
        if !self.filter.is_empty() {
            write!(f, "filter:{}", self.filter)?
        }
        if !self.backdrop_filter.is_empty() {
            write!(f, "backdrop-filter:{}", self.backdrop_filter)?
        }
        Ok(())
    }
}

impl Add<Self> for CssAttributes {
    type Output = CssAttributes;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = self;
        out += rhs;
        out
    }
}

impl AddAssign<Self> for CssAttributes {
    fn add_assign(&mut self, rhs: Self) {
        self.normal += rhs.normal;
        self.transforms += rhs.transforms;
        self.filter += rhs.filter;
        self.backdrop_filter += rhs.backdrop_filter;
    }
}
