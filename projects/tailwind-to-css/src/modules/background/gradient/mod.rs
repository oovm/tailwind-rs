use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindFrom {
    color: TailwindColor,
}

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindVia {
    color: TailwindColor,
}

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindTo {
    color: TailwindColor,
}

impl From<TailwindColor> for TailwindFrom {
    fn from(color: TailwindColor) -> Self {
        Self { color }
    }
}
impl From<TailwindColor> for TailwindVia {
    fn from(color: TailwindColor) -> Self {
        Self { color }
    }
}
impl From<TailwindColor> for TailwindTo {
    fn from(color: TailwindColor) -> Self {
        Self { color }
    }
}
impl Display for TailwindFrom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "from-{}", self.color)
    }
}
impl Display for TailwindVia {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "via-{}", self.color)
    }
}
impl Display for TailwindTo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "to-{}", self.color)
    }
}
impl TailwindInstance for TailwindFrom {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let color = self.color.get_properties(ctx);
        css_attributes! {
            "--tw-gradient-from" => color,
            "--tw-gradient-stops" => format!("var(--tw-gradient-from),var(--tw-gradient-to,{color})", color=color)
        }
    }
}
impl TailwindInstance for TailwindVia {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let color = self.color.get_properties(ctx);
        css_attributes! {
            "--tw-gradient-stops" => format!("var(--tw-gradient-from),{color},var(--tw-gradient-to,{color})", color=color)
        }
    }
}
impl TailwindInstance for TailwindTo {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let color = self.color.get_properties(ctx);
        css_attributes! {
            "--tw-gradient-to" => color
        }
    }
}

impl TailwindFrom {
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { color: TailwindColor::parse(input, arbitrary)? })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { color: TailwindColor::parse_arbitrary(arbitrary)? })
    }
}
impl TailwindVia {
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { color: TailwindColor::parse(input, arbitrary)? })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { color: TailwindColor::parse_arbitrary(arbitrary)? })
    }
}
impl TailwindTo {
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { color: TailwindColor::parse(input, arbitrary)? })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { color: TailwindColor::parse_arbitrary(arbitrary)? })
    }
}
