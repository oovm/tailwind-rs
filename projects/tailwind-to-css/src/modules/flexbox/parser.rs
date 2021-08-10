use super::*;

impl TailwindBasis {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindFlexDirection {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindFlexWrap {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindFlex {
    pub fn parse(flex: &str) -> Result<Self> {
        let n = parse_integer(flex)?.1;
        Ok(Self::Percent { grow: n, shrink: n, basis: 0 })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailWindGrow {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary in flex-grow");
        match pattern {
            [] => Ok(Self { grow: 0 }),
            [n] => Ok(Self { grow: parse_integer(n)?.1 }),
            _ => syntax_error!("Unknown grow instructions: {}", pattern.join("-")),
        }
    }
}

impl TailWindShrink {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary in flex-shrink");
        match pattern {
            [] => Ok(Self { shrink: 0 }),
            [n] => Ok(Self { shrink: parse_integer(n)?.1 }),
            _ => syntax_error!("Unknown shrink instructions: {}", pattern.join("-")),
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
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary in order");
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

impl TailwindGridTemplate {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl SpanKind {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindColumn {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindRow {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl GapSize {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindGridFlow {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindGridAuto {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindGap {
    #[inline]
    pub fn parse_x(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { size: GapSize::parse(pattern, arbitrary)?, axis: Some(true) })
    }
    #[inline]
    pub fn parse_y(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { size: GapSize::parse(pattern, arbitrary)?, axis: Some(false) })
    }
    #[inline]
    pub fn parse_xy(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { size: GapSize::parse(pattern, arbitrary)?, axis: None })
    }
}

impl TailwindJustifyContent {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindJustifyItems {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindJustifySelf {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindContent {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindItems {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindSelf {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindPlaceContent {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindPlaceItems {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindPlaceSelf {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}
