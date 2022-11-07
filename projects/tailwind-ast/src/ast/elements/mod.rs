use super::*;

mod from;

impl Display for TailwindElements {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.negative {
            f.write_char('-')?
        }
        write!(f, "{}", self.items.join("-"))
    }
}

impl TailwindElements {
    /// Build new TailwindElements
    pub fn new<T, I>(negative: bool, iter: T) -> Self
    where
        T: IntoIterator<Item = I>,
        String: From<I>,
    {
        TailwindElements { negative, items: iter.into_iter().map(String::from).collect() }
    }
}
