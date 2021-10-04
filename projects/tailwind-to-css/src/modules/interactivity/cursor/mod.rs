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
            Cursor::Standard(s) => write!(f, "display-{}", s),
            Cursor::Arbitrary(s) => write!(f, "display-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindCursor {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let align = match &self.kind {
            Cursor::Standard(s) => s,
            Cursor::Arbitrary(s) => s,
        };
        css_attributes! {
            "display" => align
        }
    }
}

impl TailwindCursor {
    /// https://tailwindcss.com/docs/display
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            _ => {
                let s = pattern.join("-");
                debug_assert!(Self::check_valid(&s));
                Ok(Self { kind: Cursor::Standard(s) })
            },
        }
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/display/two-value_syntax_of_display
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: Cursor::Arbitrary(arbitrary.to_string()) })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/display#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "auto",
            "default",
            "none",
            "context-menu",
            "help",
            "pointer",
            "progress",
            "wait",
            "cell",
            "crosshair",
            "text",
            "vertical-text",
            "alias",
            "copy",
            "move",
            "no-drop",
            "not-allowed",
            "e-resize",
            "n-resize",
            "ne-resize",
            "nw-resize",
            "s-resize",
            "se-resize",
            "sw-resize",
            "w-resize",
            "ew-resize",
            "ns-resize",
            "nesw-resize",
            "nwse-resize",
            "col-resize",
            "row-resize",
            "all-scroll",
            "zoom-in",
            "zoom-out",
            "grab",
            "grabbing",
            // Global
            "inherit",
            "initial",
            "revert",
            "unset",
        ]);
        set.contains(mode)
    }
}
