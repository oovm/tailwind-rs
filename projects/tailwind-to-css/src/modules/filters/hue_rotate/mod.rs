use super::*;
use crate::Negative;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindHueRotate {
    degree: NumericValue,
    backdrop: Backdrop,
}

impl Display for TailwindHueRotate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.backdrop.write(f)?;
        write!(f, "hue-rotate-{}", self.degree)
    }
}

impl TailwindInstance for TailwindHueRotate {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let n = self.degree.get_properties(|f| format!("{}deg", f));
        let value = format!("hue-rotate({})", n);
        match self.backdrop.0 {
            true => css_attributes! {
                "backdrop-filter" => value
            },
            false => css_attributes! {
                "filter" => value
            },
        }
    }
}

impl TailwindHueRotate {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool, negative: Negative) -> Result<Self> {
        let percent = NumericValue::negative_parser("hue-rotate", |_| false)(rest, arbitrary, negative)?;
        Ok(Self { degree: percent, backdrop: Backdrop::from(backdrop) })
    }
}
