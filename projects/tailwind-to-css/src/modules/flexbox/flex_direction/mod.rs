use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindFlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl Display for TailwindFlexDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("flex-")?;
        match self {
            Self::Row => f.write_str("row"),
            Self::RowReverse => f.write_str("row-reverse"),
            Self::Column => f.write_str("col"),
            Self::ColumnReverse => f.write_str("col-reverse"),
        }
    }
}

impl TailwindInstance for TailwindFlexDirection {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let direction = match self {
            TailwindFlexDirection::Row => "row",
            TailwindFlexDirection::RowReverse => "row-reverse",
            TailwindFlexDirection::Column => "column-reverse",
            TailwindFlexDirection::ColumnReverse => "column-reverse",
        };
        css_attributes! {
            "flex-direction" => direction
        }
    }
}
impl TailwindFlexDirection {
    pub fn parse(_pattern: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}
