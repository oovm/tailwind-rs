use super::*;

impl Debug for CssAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{};", self.key, self.value)
    }
}

impl Display for CssAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Eq for CssAttribute {}

impl PartialEq<Self> for CssAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }
}

impl PartialOrd<Self> for CssAttribute {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl Ord for CssAttribute {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl Hash for CssAttribute {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        self.value.hash(state);
    }
}
