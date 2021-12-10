use super::*;

#[derive(Copy, Clone, Debug)]
pub enum InlineMode {
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
