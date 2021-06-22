use super::*;

impl TailwindFlexBasis {
    #[inline]
    fn parse(_kind: &'static str, _ratio: &'static str) -> Box<dyn TailwindInstance> {
        todo!()
    }
}

impl TailwindFlex {
    #[inline]
    pub fn parse(flex: &str) -> Result<Self> {
        let n = parse_integer(flex)?.1;
        Ok(Self::Percent { grow: n, shrink: n, basis: 0 })
    }
    #[inline]
    pub fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        todo!()
    }
}

impl TailWindGrow {
    #[inline]
    pub fn parse(pattern: &[&str], arbitrary: &str) -> Result<Self> {
        match pattern {
            [] if arbitrary.is_empty() => Ok(Self { grow: 0 }),
            [] => Self::parse_arbitrary(arbitrary),
            [n] => Ok(Self { grow: parse_integer(n)?.1 }),
            _ => syntax_error!("Unknown flex-grow instructions: {}", pattern.join("-")),
        }
    }
    #[inline]
    pub fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        let grow = parse_integer(arbitrary)?.1;
        Ok(Self { grow })
    }
}

impl TailWindShrink {
    #[inline]
    pub fn parse(pattern: &[&str], arbitrary: &str) -> Result<Self> {
        match pattern {
            [] if arbitrary.is_empty() => Ok(Self { shrink: 0 }),
            [] => Self::parse_arbitrary(arbitrary),
            [n] => Ok(Self { shrink: parse_integer(n)?.1 }),
            _ => syntax_error!("Unknown flex-grow instructions: {}", pattern.join("-")),
        }
    }
    #[inline]
    pub fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        let shrink = parse_integer(arbitrary)?.1;
        Ok(Self { shrink })
    }
}

impl TailWindOrder {
    pub const NONE: Self = Self { order: 0 };
    pub const FIRST: Self = Self { order: 9999 };
    pub const LAST: Self = Self { order: -9999 };
    #[inline]
    pub fn parse(pattern: &[&str], arbitrary: &str, negative: bool) -> Result<Self> {
        let out = match pattern {
            [] if arbitrary.is_empty() => Self { order: 0 },
            [] => Self::parse_arbitrary(arbitrary, negative)?,
            ["none"] => Self::NONE,
            ["first"] => Self::FIRST,
            ["last"] => Self::LAST,
            [n] => Self::parse_arbitrary(n, negative)?,
            _ => return syntax_error!("Unknown flex-order instructions: {}", pattern.join("-")),
        };
        Ok(out)
    }
    #[inline]
    pub fn parse_arbitrary(arbitrary: &str, negative: bool) -> Result<Self> {
        let mut order: isize = parse_integer(arbitrary)?.1;
        if negative {
            order = -order
        }
        Ok(Self { order })
    }
}
