use super::*;
use crate::NumericValue;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailWindOrder {
    kind: NumericValue,
}

impl Display for TailWindOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_negative(f)?;
        write!(f, "order-{}", self.kind)
    }
}

impl TailwindInstance for TailWindOrder {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "order" => self.kind
        }
    }
}

impl TailWindOrder {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let kind = match pattern {
            ["none"] => NumericValue::Number(0.0, Negative::from(false)),
            ["first"] => NumericValue::Number(9999.0, Negative::from(false)),
            ["last"] => NumericValue::Number(-9999.0, Negative::from(true)),
            [s] if Self::check_valid(s) => NumericValue::Standard(s.to_string()),
            _ => NumericValue::negative_checker_parser("order", &Self::check_valid)(pattern, arbitrary, negative)?,
        };
        Ok(Self { kind })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: NumericValue::parse_arbitrary(arbitrary)? })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
