use super::*;

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindDivideWidth {
    axis: bool,
    width: LengthUnit,
}

impl Display for TailwindDivideWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.axis {
            true => write!(f, "divide-x-{}", self.width),
            false => write!(f, "divide-y-{}", self.width),
        }
    }
}

impl TailwindInstance for TailwindDivideWidth {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "--tw-ring-offset-width" => self.width,
            "box-shadow" => "0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow)"
        }
    }
}
impl TailwindDivideWidth {
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary, axis: bool) -> Result<Self> {
        let out = match input {
            [] => Self { axis, width: LengthUnit::px(3.0) },
            [n] => {
                let a = TailwindArbitrary::from(*n);
                Self { axis, width: a.as_length()? }
            },
            _ => return syntax_error!("Unknown ring-width instructions"),
        };
        Ok(out)
    }
}
