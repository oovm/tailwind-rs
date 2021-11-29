use std::collections::HashSet;

use crate::Result;
use parcel_css::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
mod methods;

///
#[derive(Clone, Debug)]
pub struct CssProcessor {
    pub minify: bool,
    pub unused_symbols: HashSet<String>,
}
