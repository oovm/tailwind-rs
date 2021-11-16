use super::*;

/// https://tailwindcss.com/docs/box-shadow
#[derive(Clone, Debug)]
pub struct TailwindShadow {
    kind: Shadow,
    is_drop: bool,
}

#[derive(Clone, Debug)]
enum Shadow {
    None,
    Small,
    Standard,
    Medium,
    Large,
    ExtraLarge,
    ExtraLarge2,
    Arbitrary(String),
}

impl Display for TailwindShadow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.is_drop {
            true => write!(f, "drop-shadow")?,
            false => write!(f, "shadow")?,
        }
        match &self.kind {
            Shadow::None => write!(f, "-none",),
            Shadow::Small => write!(f, "-sm",),
            Shadow::Standard => write!(f, "",),
            Shadow::Medium => write!(f, "-md",),
            Shadow::Large => write!(f, "-lg",),
            Shadow::ExtraLarge => write!(f, "-xl",),
            Shadow::ExtraLarge2 => write!(f, "-2xl",),
            Shadow::Arbitrary(s) => write!(f, "-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindShadow {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        match self.is_drop {
            true => {
                let class = "filter";
                let shadow = match &self.kind {
                    Shadow::None => "drop-shadow(0 0 #0000)",
                    Shadow::Small => "drop-shadow(0 1px 1px rgb(0 0 0 / 0.05))",
                    Shadow::Standard => "drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06))",
                    Shadow::Medium => "drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06))",
                    Shadow::Large => "drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1))",
                    Shadow::ExtraLarge => "drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08))",
                    Shadow::ExtraLarge2 => "drop-shadow(0 25px 25px rgb(0 0 0 / 0.15))",
                    Shadow::Arbitrary(c) => c.as_str(),
                };
                css_attributes! {
                    class => shadow
                }
            },
            false => {
                let class = "box-shadow";
                let shadow = match &self.kind {
                    Shadow::None => "0 0 #0000",
                    Shadow::Small => "0 1px 2px 0 rgb(0 0 0 / 0.05)",
                    Shadow::Standard => "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)",
                    Shadow::Medium => "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)",
                    Shadow::Large => "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)",
                    Shadow::ExtraLarge => "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)",
                    Shadow::ExtraLarge2 => "0 25px 50px -12px rgb(0 0 0 / 0.25)",
                    // shadow-inner	box-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05);
                    Shadow::Arbitrary(c) => c.as_str(),
                };
                css_attributes! {
                    class => shadow
                }
            },
        }
    }
}

impl Shadow {
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match input {
            [] if arbitrary.is_some() => Shadow::Standard,
            [] => Self::parse_arbitrary(arbitrary)?,
            ["inner"] => Shadow::Small,
            ["none"] => Shadow::None,
            ["s" | "sm"] => Shadow::Small,
            ["m" | "md"] => Shadow::Medium,
            ["l" | "lg"] => Shadow::Large,
            ["x" | "xl"] => Shadow::ExtraLarge,
            ["u" | "2xl" | "xxl" | "ul"] => Shadow::ExtraLarge2,
            _ => return syntax_error!("Unknown shadow instructions: {}", input.join("-")),
        };
        Ok(kind)
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary(arbitrary.to_string()))
    }
}

impl TailwindShadow {
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary, drop: bool) -> Result<Self> {
        Ok(Self { kind: Shadow::parse(input, arbitrary)?, is_drop: drop })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, drop: bool) -> Result<Self> {
        Ok(Self { kind: Shadow::parse_arbitrary(arbitrary)?, is_drop: drop })
    }
}
