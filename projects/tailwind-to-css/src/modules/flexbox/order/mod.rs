use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailWindOrder {
    order: usize,
    negative: bool,
}

impl Display for TailWindOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.negative {
            f.write_char('-')?
        }
        write!(f, "order-{}", self.order)
    }
}

impl TailwindInstance for TailWindOrder {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let order = match self.negative {
            true => format!("-{}", self.order),
            false => format!("{}", self.order),
        };
        css_attributes! {
            "order" => order
        }
    }
}

impl TailWindOrder {
    /// `order-none`
    pub const NONE: Self = Self { order: 0, negative: false };
    /// `order-first`
    pub const FIRST: Self = Self { order: 9999, negative: false };
    /// `order-last`
    pub const LAST: Self = Self { order: 9999, negative: true };
    #[inline]
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after order");
        let out = match pattern {
            ["none"] => Self::NONE,
            ["first"] => Self::FIRST,
            ["last"] => Self::LAST,
            [n] => Self { order: parse_integer(n)?.1, negative },
            _ => return syntax_error!("Unknown order instructions: {}", pattern.join("-")),
        };
        Ok(out)
    }
}
