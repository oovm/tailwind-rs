use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindGridRows {
    axis: bool,
    kind: GridTemplate,
}




impl Display for TailwindGridRows {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let kind = match self.axis {
            true => "rows",
            false => "cols",
        };
        write!(f, "grid-{}-{}", kind, self.kind)
    }
}

impl TailwindInstance for TailwindGridRows {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let class = match self.axis {
            true => "grid-template-columns",
            false => "grid-template-columns",
        };
        let grid = match self.kind {
            0 => "none".to_string(),
            n => format!("repeat({},minmax(0,1fr))", n),
        };
        css_attributes! {
            class => grid
        }
    }
}

impl TailwindGridRows {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { axis: false, kind: GridTemplate::parse(pattern, arbitrary)? })
    }
}
