use crate::StandardValue;

use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindCursor {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindCursor => "cursor");

impl Display for TailwindCursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "cursor-{}", self.kind)
    }
}

impl TailwindCursor {
    /// <https://tailwindcss.com/docs/cursor>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("cursor", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "alias",
            "all-scroll",
            "auto",
            "cell",
            "col-resize",
            "context-menu",
            "copy",
            "crosshair",
            "default",
            "e-resize",
            "ew-resize",
            "grab",
            "grabbing",
            "help",
            "inherit",
            "initial",
            "move",
            "ne-resize",
            "nesw-resize",
            "no-drop",
            "none",
            "not-allowed",
            "n-resize",
            "ns-resize",
            "nw-resize",
            "nwse-resize",
            "pointer",
            "progress",
            "revert",
            "row-resize",
            "se-resize",
            "s-resize",
            "sw-resize",
            "text",
            "unset",
            "vertical-text",
            "wait",
            "w-resize",
            "zoom-in",
            "zoom-out",
        ]);
        set.contains(mode)
    }
}
