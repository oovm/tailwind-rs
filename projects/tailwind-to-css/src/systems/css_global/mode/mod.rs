use super::*;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CssInlineMode {
    /// ```html
    /// <img class="tailwind"/>
    /// ```
    None,
    /// ```html
    /// <img style="key:value"/>
    /// ```
    Inline,
    /// ```html
    /// <img class="_b2JmdXNjYXRl"/>
    /// ```
    Scoped,
    /// ```html
    /// <img data-tw-b2JmdXNjYXRl/>
    /// ```
    DataKey,
    /// ```html
    /// <img data-tw="b2JmdXNjYXRl"/>
    /// ```
    DataValue,
}

impl Default for CssInlineMode {
    fn default() -> Self {
        Self::None
    }
}
