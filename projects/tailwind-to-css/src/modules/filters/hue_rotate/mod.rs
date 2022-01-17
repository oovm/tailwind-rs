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
        self.backdrop.get_filter(format!("hue-rotate({})", n))
    }
}

impl TailwindHueRotate {
    /// <https://tailwindcss.com/docs/hue-rotate>
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool, negative: Negative) -> Result<Self> {
        let degree = match rest {
            [] if arbitrary.is_none() => 180u32.into(),
            _ => NumericValue::negative_parser("hue-rotate", |_| false)(rest, arbitrary, negative)?,
        };
        Ok(Self { degree, backdrop: Backdrop::from(backdrop) })
    }
}
