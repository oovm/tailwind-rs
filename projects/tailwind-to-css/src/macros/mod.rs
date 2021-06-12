#[macro_export]
macro_rules! css_attributes {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$(CssAttribute::new($k.to_string(), $v.to_string()),)*]))
    }};
}
