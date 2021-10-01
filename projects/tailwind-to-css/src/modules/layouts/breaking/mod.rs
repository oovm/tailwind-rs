mod after;
mod before;
mod inside;

use super::*;

#[derive(Copy, Clone, Debug)]
enum BreakKind {
    Before,
    After,
    Inside,
}

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBreakLayout {
    kind: BreakKind,
    info: String,
}

impl Display for BreakKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Before => f.write_str("break-before"),
            Self::After => f.write_str("break-after"),
            Self::Inside => f.write_str("break-inside"),
        }
    }
}

impl Display for TailwindBreakLayout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.kind, self.info)
    }
}

impl TailwindInstance for TailwindBreakLayout {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            self.kind => self.info
        }
    }
}

impl Display for BoxDecoration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clone => f.write_str("clone"),
            Self::Slice => f.write_str("slice"),
        }
    }
}

impl Display for TailwindBoxDecoration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "box-decoration-{}", self.kind)
    }
}

impl TailwindInstance for TailwindBoxDecoration {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let decoration = self.kind.to_string();
        css_attributes! {
            "box-decoration-break" => decoration
        }
    }
}

impl Display for BoxSizing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Border => f.write_str("border"),
            Self::Content => f.write_str("content"),
        }
    }
}

impl Display for TailwindBoxSizing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "box-{}", self.kind)
    }
}

impl TailwindInstance for TailwindBoxSizing {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let size = format!("{}-box", self.kind);
        css_attributes! {
            "box-sizing" => size
        }
    }
}

impl TailwindBreakLayout {
    /// https://tailwindcss.com/docs/break-before
    pub fn parse_before(input: &[&str]) -> Result<Self> {
        let kind = BreakKind::Before;
        let info = input.join("-");
        match input {
            ["auto"] | ["avoid"] | ["all"] | ["avoid", "page"] | ["page"] | ["left"] | ["right"] | ["column"] =>
                Ok(Self { kind, info }),
            _ => syntax_error!("Unknown break-before instructions: {}", info),
        }
    }
    /// https://tailwindcss.com/docs/break-after
    pub fn parse_after(input: &[&str]) -> Result<Self> {
        let kind = BreakKind::After;
        let info = input.join("-");
        match input {
            ["auto"] | ["avoid"] | ["all"] | ["avoid", "page"] | ["page"] | ["left"] | ["right"] | ["column"] =>
                Ok(Self { kind, info }),
            _ => syntax_error!("Unknown break-after instructions: {}", info),
        }
    }
    /// https://tailwindcss.com/docs/break-inside
    pub fn parse_inside(input: &[&str]) -> Result<Self> {
        let kind = BreakKind::Inside;
        let info = input.join("-");
        match input {
            ["auto"] | ["avoid"] | ["avoid", "page"] | ["avoid", "column"] => Ok(Self { kind, info }),
            _ => syntax_error!("Unknown break-inside instructions: {}", info),
        }
    }
}
