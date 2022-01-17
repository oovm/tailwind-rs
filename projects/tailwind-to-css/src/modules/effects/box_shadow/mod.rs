use super::*;

/// https://tailwindcss.com/docs/box-shadow
#[derive(Clone, Debug)]
pub struct TailwindShadow {
    kind: StandardValue,
    drop: Backdrop,
}

impl Display for TailwindShadow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.drop.0 {
            write!(f, "drop-")?;
        }
        match &self.kind {
            StandardValue::Keyword(s) if s.is_empty() => write!(f, "shadow"),
            StandardValue::Keyword(s) => write!(f, "shadow-{}", s),
            StandardValue::Arbitrary(s) => s.write_class(f, "shadow-"),
        }
    }
}

impl TailwindInstance for TailwindShadow {
    fn attributes(&self, ctx: &TailwindBuilder) -> CssAttributes {
        let shadow = match &self.kind {
            StandardValue::Keyword(s) if self.drop.0 => ctx.effects.get_drop_shadow(s),
            StandardValue::Keyword(s) => ctx.effects.get_box_shadow(s),
            StandardValue::Arbitrary(s) => s.get_properties(),
        };
        self.drop.get_shadow(shadow)
    }
}

impl TailwindShadow {
    /// <https://tailwindcss.com/docs/box-shadow>
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary, drop: bool) -> Result<Self> {
        let kind = match input {
            [] if arbitrary.is_some() => StandardValue::parse_arbitrary(arbitrary)?,
            _ => StandardValue::Keyword(input.join("-")),
        };
        Ok(Self { kind, drop: Backdrop(drop) })
    }
    /// <https://tailwindcss.com/docs/box-shadow#arbitrary-values>
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, drop: bool) -> Result<Self> {
        Ok(Self { kind: StandardValue::parse_arbitrary(arbitrary)?, drop: Backdrop(drop) })
    }
}
