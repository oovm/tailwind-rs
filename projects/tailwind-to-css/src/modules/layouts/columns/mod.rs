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
        let columns = match &self.kind {
            Columns::Columns(n) => format!("{}", n),
            Columns::Length(n) => n.get_properties(),
            Columns::Standard(g) => g.to_string(),
        };
        css_attributes! {
            "columns" => columns
        }
    }
}

impl TailwindColumns {
    /// https://tailwindcss.com/docs/columns
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: Columns::parse(input, arbitrary)? })
    }
}
