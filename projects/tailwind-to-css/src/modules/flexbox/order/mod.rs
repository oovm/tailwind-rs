use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailWindOrder {
    order: i32,
}

impl Display for TailWindOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.order < 0 {
            f.write_char('-')?
        }
        write!(f, "order-{}", self.order)
    }
}

impl TailwindInstance for TailWindOrder {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "order" => self.order
        }
    }
}

impl TailWindOrder {
    /// `order-none`
    pub const NONE: Self = Self { order: 0 };
    /// `order-first`
    pub const FIRST: Self = Self { order: 9999 };
    /// `order-last`
    pub const LAST: Self = Self { order: -9999 };
    #[inline]
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after order");
        let out = match pattern {
            ["none"] => Self::NONE,
            ["first"] => Self::FIRST,
            ["last"] => Self::LAST,
            [n] => {
                let mut order: i32 = parse_integer(n)?.1;
                if negative.unwrap() {
                    order = -order;
                }
                Self { order }
            },
            _ => return syntax_error!("Unknown order instructions: {}", pattern.join("-")),
        };
        Ok(out)
    }
}
