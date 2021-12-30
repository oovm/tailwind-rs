use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindRingWidth {
    width: LengthUnit,
}

impl Display for TailwindRingWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ring-offset-{}", self.width)
    }
}

impl TailwindInstance for TailwindRingWidth {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "--tw-ring-offset-width" => self.width,
            "box-shadow" => "0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow)"
        }
    }
}
impl TailwindRingWidth {
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let out = match input {
            [] => Self { width: LengthUnit::px(3.0) },
            [n] => {
                let a = TailwindArbitrary::from(*n);
                Self { width: a.as_length()? }
            },
            _ => return syntax_error!("Unknown ring-width instructions"),
        };
        Ok(out)
    }
}
