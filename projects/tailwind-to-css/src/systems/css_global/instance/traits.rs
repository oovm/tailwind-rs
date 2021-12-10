use super::*;

impl Hash for CssInstance {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl Eq for CssInstance {}

impl PartialEq<Self> for CssInstance {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialOrd<Self> for CssInstance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl Ord for CssInstance {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
}
