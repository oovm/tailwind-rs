use super::*;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindIndent {
    kind: Indent,
}

#[derive(Debug, Clone)]
enum Indent {
    Unit(f32),
    Length(LengthUnit),
}

impl Display for TailwindIndent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            Indent::Unit(n) => write!(f, "indent-{}", n),
            Indent::Length(n) => write!(f, "indent-{}", n.get_class()),
        }
    }
}

impl TailwindInstance for TailwindIndent {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let indent = match &self.kind {
            Indent::Unit(n) => format!("{}rem", n / 4.0),
            Indent::Length(n) => n.get_properties(),
        };
        css_attributes! {
            "text-indent" => indent
        }
    }
}

impl TailwindIndent {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: Indent::parse(pattern, arbitrary)? })
    }
}

impl Indent {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after indent");
        match pattern {
            ["px"] => Ok(Self::Length(LengthUnit::px(1.0))),
            [n] => {
                let a = TailwindArbitrary::from(*n);
                Self::maybe_no_unit(&a).or_else(|_| Self::maybe_length(&a))
            },
            _ => syntax_error!("Unknown indent instructions: {}", pattern.join("-")),
        }
    }
    fn maybe_no_unit(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Unit(arbitrary.as_float()?))
    }
    fn maybe_length(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Length(arbitrary.as_length()?))
    }
}
