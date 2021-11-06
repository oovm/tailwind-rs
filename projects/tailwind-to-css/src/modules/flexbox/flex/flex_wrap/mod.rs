use crate::modules::flexbox::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindFlexWrap {
    Wrap,
    WrapReverse,
    NoWrap,
}

impl Display for TailwindFlexWrap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("flex-")?;
        match self {
            Self::Wrap => f.write_str("wrap"),
            Self::WrapReverse => f.write_str("wrap-reverse"),
            Self::NoWrap => f.write_str("nowrap"),
        }
    }
}

impl TailwindInstance for TailwindFlexWrap {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let wrap = match self {
            Self::Wrap => "wrap",
            Self::WrapReverse => "wrap-reverse",
            Self::NoWrap => "nowrap",
        };
        css_attributes! {
            "flex-direction" => wrap
        }
    }
}

impl TailwindFlexWrap {
    pub fn parse(_pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}
