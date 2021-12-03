use super::*;
mod columns;
pub use self::columns::Columns;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindColumns {
    kind: Columns,
}

impl Display for TailwindColumns {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "columns-{}", self.kind)
    }
}

impl TailwindInstance for TailwindColumns {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "columns" => self.kind.get_properties()
        }
    }
}

impl TailwindColumns {
    /// https://tailwindcss.com/docs/columns
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: Columns::parse(input, arbitrary)? })
    }
    /// dispatch to [columns](https://developer.mozilla.org/en-US/docs/Web/CSS/columns)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: Columns::parse_arbitrary(arbitrary)? })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/columns#syntax>
    pub fn check_valid(mode: &str) -> bool {
        Columns::check_valid(mode)
    }
}
