use super::*;

impl TailwindSpacing {
    pub fn auto(kind: TailwindSpacingKind) -> Self {
        Self { kind, size: TailwindSpacingSize::Auto }
    }
    pub fn reversed(kind: TailwindSpacingKind) -> Self {
        Self { kind, size: TailwindSpacingSize::Reversed }
    }
    pub fn px(n: usize, kind: TailwindSpacingKind) -> Self {
        Self { kind, size: TailwindSpacingSize::Px(n) }
    }
    pub fn unit(n: usize, kind: TailwindSpacingKind) -> Self {
        Self { kind, size: TailwindSpacingSize::Number(n) }
    }
}

impl TailwindSpacing {
    /// `(p|m)(t|r|b|l|x|y)?-(px|n|auto)`
    ///
    /// `(space)(x|y)-(px|n|reverse)`

    pub fn parse_padding(input: &[&str], p: &str) -> Box<dyn TailwindInstance> {
        Self::parse_pm(input, p).unwrap()
    }
    pub fn parse_margin(input: &[&str], m: &str) -> Box<dyn TailwindInstance> {
        Self::parse_pm(input, m).unwrap()
    }
    fn parse_pm(input: &[&str], pm: &str) -> Result<Box<dyn TailwindInstance>> {
        let kind = TailwindSpacingKind::parse_p(pm.chars().collect::<Vec<_>>().as_slice()).expect("not vaild");
        let size = match input {
            [s] => TailwindSpacingSize::parse(s).expect("Todo"),
            _ => panic!("Todo"),
        };
        Ok(Box::new(Self { kind, size }))
    }

    pub fn parse_space(input: &[&str], kind: char) -> Box<dyn TailwindInstance> {
        let kind = TailwindSpacingKind::parse_space(kind).expect("Spacing: No such direction");
        let size = match input {
            [s] => TailwindSpacingSize::parse(s).expect("Todo"),
            _ => panic!("Todo"),
        };
        Box::new(Self { kind, size })
    }
}

impl TailwindSpacingKind {
    /// `(p|m)(t|r|b|l|x|y)?`
    fn parse_p(cs: &[char]) -> Option<TailwindSpacingKind> {
        let kind = match cs {
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
            _ => return None,
        };
        return Some(kind);
    }
    /// `(space)-(x|y)`
    fn parse_space(c: char) -> Option<TailwindSpacingKind> {
        match c {
            'x' => Some(Self::SpaceBetweenX),
            'y' => Some(Self::SpaceBetweenY),
            _ => None,
        }
    }
}

impl TailwindSpacingSize {
    fn parse(input: &str) -> Result<Self> {
        // match input {
        //     "px" => {}
        //     "reverse" => {}
        //     "auto" => {}
        // }
        todo!()
    }
    fn parse_arb() {}
}
