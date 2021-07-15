use super::*;

impl SizeKind {
    pub fn parse(kind: &[&str], arbitrary: &str) -> Result<Self> {
        match kind {
            [] => Self::parse_arbitrary(arbitrary),
            ["auto"] => Ok(Self::Auto),
            ["reverse"] => Ok(Self::Reverse),
            _ => Self::parse_arbitrary(arbitrary),
        }
    }
    pub fn parse_arbitrary(_arbitrary: &str) -> Result<Self> {
        todo!()
    }
}

impl TailwindSpacing {
    /// `(p)(t|r|b|l|x|y)?-(px|n|auto)`
    #[inline]
    pub fn parse_padding(input: &[&str], p: &str, arbitrary: &str) -> Result<Self> {
        Self::parse_pm(input, p, arbitrary)
    }
    /// `(m)(t|r|b|l|x|y)?-(px|n|auto)`
    #[inline]
    pub fn parse_margin(input: &[&str], m: &str, arbitrary: &str) -> Result<Self> {
        Self::parse_pm(input, m, arbitrary)
    }
    fn parse_pm(input: &[&str], pm: &str, arbitrary: &str) -> Result<Self> {
        let kind = TailwindSpacingKind::parse_pm(pm)?;
        let size = SizeKind::parse(input, arbitrary)?;
        Ok(Self { kind, size })
    }
    /// `(space)(x|y)-(px|n|reverse)`
    pub fn parse_space(input: &[&str], kind: char, arbitrary: &str) -> Result<Self> {
        let kind = TailwindSpacingKind::parse_space(kind)?;
        let size = SizeKind::parse(input, arbitrary)?;
        Ok(Self { kind, size })
    }
}

impl TailwindSpacingKind {
    /// `(p|m)(t|r|b|l|x|y)?`
    fn parse_pm(pm: &str) -> Result<Self> {
        let cs: Vec<char> = pm.chars().collect();
        let kind = match cs.as_slice() {
            ['p'] => Self::Padding,
            ['p', 't'] => Self::PaddingT,
            ['p', 'r'] => Self::PaddingR,
            ['p', 'b'] => Self::PaddingB,
            ['p', 'l'] => Self::PaddingL,
            ['p', 'x'] => Self::PaddingX,
            ['p', 'y'] => Self::PaddingY,
            ['m'] => Self::Margin,
            ['m', 't'] => Self::MarginT,
            ['m', 'r'] => Self::MarginR,
            ['m', 'b'] => Self::MarginB,
            ['m', 'l'] => Self::MarginL,
            ['m', 'x'] => Self::MarginX,
            ['m', 'y'] => Self::MarginY,
            _ => return syntax_error!("space"),
        };
        return Ok(kind);
    }
    /// `(space)-(x|y)`
    fn parse_space(c: char) -> Result<Self> {
        match c {
            'x' => Ok(Self::SpaceBetweenX),
            'y' => Ok(Self::SpaceBetweenY),
            _ => syntax_error!("space"),
        }
    }
}
