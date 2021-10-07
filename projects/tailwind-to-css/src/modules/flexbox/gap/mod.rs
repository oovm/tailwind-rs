use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindGap {
    size: LengthUnit,
    axis: Option<bool>,
}

impl Display for TailwindGap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.axis {
            None => write!(f, "gap-[{}]", self.size.get_class_arbitrary()),
            Some(true) => write!(f, "gap-x-[{}]", self.size.get_class_arbitrary()),
            Some(false) => write!(f, "gap-y-[{}]", self.size.get_class_arbitrary()),
        }
    }
}

impl TailwindInstance for TailwindGap {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let class = match self.axis {
            None => "gap",
            Some(true) => "column-gap",
            Some(false) => "row-gap",
        };
        css_attributes! {
            class => self.size.get_properties()
        }
    }
}

impl TailwindGap {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            ["x", rest @ ..] => Ok(Self { size: parse_size(rest, arbitrary)?, axis: Some(true) }),
            ["y", rest @ ..] => Ok(Self { size: parse_size(rest, arbitrary)?, axis: Some(false) }),
            _ => Ok(Self { size: parse_size(pattern, arbitrary)?, axis: None }),
        }
    }
}

fn parse_size(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<LengthUnit> {
    let size = match pattern {
        [] => arbitrary.as_length()?,
        ["px"] => LengthUnit::Px(1.0),
        [n] => {
            let a = TailwindArbitrary::from(*n);
            LengthUnit::Rem(a.as_float()? / 4.0)
        },
        _ => return syntax_error!("Unknown gap instructions"),
    };
    Ok(size)
}
