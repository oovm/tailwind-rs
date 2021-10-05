use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindCursor {
    kind: Cursor,
}

#[derive(Debug, Clone)]
enum Cursor {
    Standard(String),
    Arbitrary(String),
}

impl<T> From<T> for TailwindCursor
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: Cursor::Standard(kind.into()) }
    }
}

impl Display for TailwindCursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            Cursor::Standard(s) => write!(f, "cursor-{}", s),
            Cursor::Arbitrary(s) => write!(f, "cursor-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindCursor {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let cursor = match &self.kind {
            Cursor::Standard(s) => s,
            Cursor::Arbitrary(s) => s,
        };
        css_attributes! {
            "cursor" => cursor
        }
    }
}

impl TailwindCursor {
    /// https://tailwindcss.com/docs/cursor
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [] => {
                debug_assert!(arbitrary.is_some());
                Self::parse_arbitrary(arbitrary)
            },
            _ => {
                let s = pattern.join("-");
                debug_assert!(Self::check_valid(&s));
                Ok(Self { kind: Cursor::Standard(s) })
            },
        }
    }
    /// https://tailwindcss.com/docs/cursor#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: Cursor::Arbitrary(arbitrary.to_string()) })
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
