use self::ordering::Order;
use super::*;
mod ordering;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailWindOrder {
    kind: Order,
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
        Ok(Self { kind: Order::parse(pattern, arbitrary, negative)? })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: Order::parse_arbitrary(arbitrary)? })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit#syntax
    pub fn check_valid(mode: &str) -> bool {
        Order::check_valid(mode)
    }
}
