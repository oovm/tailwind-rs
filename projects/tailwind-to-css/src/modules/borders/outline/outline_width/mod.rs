use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindOutlineWidth {
    kind: UnitValue,
}

impl<T> From<T> for TailwindOutlineWidth
where
    T: Into<UnitValue>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindOutlineWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            UnitValue::Number(s, _) => write!(f, "outline-{}", s),
            UnitValue::Length(s) => write!(f, "outline-{}", s),
            UnitValue::Keyword(s) => write!(f, "outline-width-{}", s),
            UnitValue::Arbitrary(s) => write!(f, "outline-width-{}", s),
        }
    }
}

impl TailwindInstance for TailwindOutlineWidth {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        if cfg!(compile_time) {
            // TODO: not percent
        }
        let width = self.kind.get_properties(|f| format!("{}px", f));
        css_attributes! {
            "outline-width" => width
        }
    }
}

impl TailwindOutlineWidth {
    /// <https://tailwindcss.com/docs/outline-width>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = UnitValue::positive_parser("outline-width", Self::check_valid, true, false, false)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "medium", "revert", "thick", "thin", "unset"]);
        set.contains(mode)
    }
}
