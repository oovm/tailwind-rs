mod methods;
mod parser;
mod utils;

pub use self::utils::*;
use super::*;

/// https://github.com/tw-in-js/twind/blob/main/src/twind/variants.ts
#[derive(Debug)]
pub enum AstVariantKind {
    Dark,
    Sticky,
    MotionReduce,
    MotionSafe,
    First,
    Last,
    Even,
    Odd,
    Children,
    Siblings,
    Sibling,
    Override,
}

impl Display for TailwindInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl TailwindInstance for TailwindInstruction {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let mut out = BTreeSet::default();
        match self.get_instance() {
            Ok(o) => out.extend(o.attributes(ctx)),
            Err(e) => {
                #[cfg(debug_assertions)]
                error!("{:?}", e)
            }
        }
        out
    }
}
