use super::*;

impl Display for CssAttributes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (k, v) in &self.normal {
            write!(f, "{}:{};", k, v)?
        }
        if !self.transforms.is_empty() {
            write!(f, "transform:{};", self.transforms.iter().join(" "))?
        }
        if !self.filter.is_empty() {
            write!(f, "filter:{};", self.filter.iter().join(" "))?
        }
        if !self.backdrop_filter.is_empty() {
            write!(f, "backdrop-filter:{};", self.backdrop_filter.iter().join(" "))?
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
        self.normal.extend(rhs.normal.into_iter());
        self.transforms.extend(rhs.transforms.into_iter());
        self.filter.extend(rhs.filter.into_iter());
        self.backdrop_filter.extend(rhs.backdrop_filter.into_iter());
    }
}
