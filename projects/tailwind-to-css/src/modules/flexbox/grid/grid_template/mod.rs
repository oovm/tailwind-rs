use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindGridTemplate {
    row: bool,
    unit: usize,
}

impl Display for TailwindGridTemplate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let kind = match self.row {
            true => "rows",
            false => "cols",
        };
        write!(f, "grid-{}-{}", kind, self.unit)
    }
}

impl TailwindInstance for TailwindGridTemplate {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let class = match self.row {
            true => "grid-template-columns",
            false => "grid-template-columns",
        };
        let grid = match self.unit {
            0 => format!("none"),
            n => format!("repeat({},minmax(0,1fr))", n),
        };
        css_attributes! {
            class => grid
        }
    }
}

impl TailwindGridTemplate {
    pub fn parse(_pattern: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}
