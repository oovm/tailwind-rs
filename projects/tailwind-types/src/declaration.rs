use serde::{Deserialize, Serialize};

use super::CssValue;

/// One structured CSS declaration. Not a serialized rule string.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Declaration {
    pub property: String,
    pub value: CssValue,
    #[serde(default)]
    pub important: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeclarationSet {
    pub declarations: Vec<Declaration>,
}
