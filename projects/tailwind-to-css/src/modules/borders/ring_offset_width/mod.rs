use super::*;
use crate::LengthUnit;

///
#[derive(Copy, Clone, Debug)]
pub struct TailwindRingOffsetWidth {
    width: LengthUnit,
}

impl Display for TailwindRingOffsetWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ring-offset-{}", self.width)
    }
}

impl TailwindInstance for TailwindRingOffsetWidth {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "--tw-ring-offset-width" => self.width,
            "box-shadow" => "0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow)"
        }
    }
}
impl TailwindRingOffsetWidth {
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after ring-width");
        let out = match input {
            [] => Self { width: LengthUnit::Px(3.0) },
            [n] => {
                let a = TailwindArbitrary::from(*n);
                Self { width: a.as_length()? }
            },
            _ => return syntax_error!("Unknown ring-width instructions"),
        };
        Ok(out)
    }
}
