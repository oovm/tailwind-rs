use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindInset {
    negative: bool,
    axis: Option<bool>,
    kind: PlacementSize,
}

impl Display for TailwindInset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.negative {
            f.write_char('-')?
        }
        match self.axis {
            None => write!(f, "inset-{}", self.kind),
            Some(true) => write!(f, "inset-x-{}", self.kind),
            Some(false) => write!(f, "inset-y-{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindInset {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        match self.axis {
            None => css_attributes! {
                "top" => self.kind.get_properties(),
                "right" => self.kind.get_properties(),
                "bottom" => self.kind.get_properties(),
                "left" => self.kind.get_properties(),
            },
            Some(true) => css_attributes! {
                "right" => self.kind.get_properties(),
                "left" => self.kind.get_properties(),
            },
            Some(false) => css_attributes! {
                "top" => self.kind.get_properties(),
                "bottom" => self.kind.get_properties(),
            },
        }
    }
}

impl TailwindInset {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: bool) -> Result<Self> {
        let (axis, rest) = match pattern {
            ["x", rest @ ..] => (Some(true), rest),
            ["y", rest @ ..] => (Some(false), rest),
            _ => (None, pattern),
        };
        match arbitrary.is_some() {
            true => Self::parse_arbitrary(arbitrary, axis, negative),
            false => Ok(Self { negative, axis, kind: PlacementSize::parse(rest, arbitrary)? }),
        }
    }

    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, axis: Option<bool>, negative: bool) -> Result<Self> {
        Ok(Self { negative, axis, kind: PlacementSize::parse_arbitrary(arbitrary)? })
    }
}
