use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindDisplay {
    kind: DisplayLayout,
}

#[derive(Debug, Clone)]
enum DisplayLayout {
    Standard(String),
    Arbitrary(String),
}

impl<T> From<T> for TailwindDisplay
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: DisplayLayout::Standard(kind.into()) }
    }
}

impl Display for TailwindDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DisplayLayout::Standard(s) => write!(f, "display-{}", s),
            DisplayLayout::Arbitrary(s) => write!(f, "display-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindDisplay {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let align = match &self.kind {
            DisplayLayout::Standard(s) => s,
            DisplayLayout::Arbitrary(s) => s,
        };
        css_attributes! {
            "display" => align
        }
    }
}

impl TailwindDisplay {
    /// https://tailwindcss.com/docs/display
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            _ => {
                let s = pattern.join("-");
                debug_assert!(Self::check_valid(&s));
                Ok(Self { kind: DisplayLayout::Standard(s) })
            },
        }
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/display/two-value_syntax_of_display
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: DisplayLayout::Arbitrary(arbitrary.to_string()) })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/display#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            // https://developer.mozilla.org/en-US/docs/Web/CSS/display#outside
            "block",
            "inline",
            // https://developer.mozilla.org/en-US/docs/Web/CSS/display#inside
            "flow",
            "flow-root",
            "table",
            "flex",
            "grid",
            "ruby",
            // https://developer.mozilla.org/en-US/docs/Web/CSS/display#list_item
            "table",
            "table-row",
            "list-item",
            // https://developer.mozilla.org/en-US/docs/Web/CSS/display#internal
            "table-row-group",
            // https://developer.mozilla.org/en-US/docs/Web/CSS/display#box
            "contents",
            "none",
            // https://developer.mozilla.org/en-US/docs/Web/CSS/display#legacy
            "inline-block",
            "inline-table",
            "inline-flex",
            "inline-grid",
            // https://developer.mozilla.org/en-US/docs/Web/CSS/display#global
            "inherit",
            "initial",
            "unset",
        ]);
        set.contains(mode)
    }
}
