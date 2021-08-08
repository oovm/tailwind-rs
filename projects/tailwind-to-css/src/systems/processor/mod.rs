mod methods;

use super::*;

///
#[derive(Clone, Debug)]
pub struct CssProcessor {
    pub minify: bool,
    pub unused_symbols: HashSet<String>,
}
