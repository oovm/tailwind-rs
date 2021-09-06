mod methods;

use parcel_css::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};

///
#[derive(Clone, Debug)]
pub struct CssProcessor {
    pub minify: bool,
    pub unused_symbols: HashSet<String>,
}
