use super::*;
use std::collections::btree_map::IntoIter;

impl Display for CssAttributes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (k, v) in &self.normal {
            write!(f, "{}:{};", k, v)?
        }
        Ok(())
    }
}

impl IntoIterator for CssAttributes {
    type Item = (String, String);
    type IntoIter = IntoIter<String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.normal.into_iter()
    }
}

impl<'a> IntoIterator for &'a CssAttributes {
    type Item = (&'a String, &'a String);
    type IntoIter = Iter<'a, String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.normal.iter()
    }
}
