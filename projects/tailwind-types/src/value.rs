use serde::{Deserialize, Serialize};

/// Structured CSS-domain values. Strings are allowed only as unresolved literals
/// after typed resolvers have run (or for arbitrary passthrough).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum ThemeValue {
    Keyword(String),
    Length(LengthValue),
    Number(String),
    Color(ColorValue),
    Ratio(String),
    Shadow(String),
    Image(String),
    Raw(String),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LengthValue {
    pub css: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ColorValue {
    pub css: String,
}

/// Declaration value after rule resolve.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum CssValue {
    Keyword(String),
    Length(LengthValue),
    Number(String),
    Color(ColorValue),
    Ratio(String),
    Shadow(String),
    Image(String),
    /// Typed-but-opaque CSS text (e.g. `var(--x)`, `calc(...)`).
    Raw(String),
}
