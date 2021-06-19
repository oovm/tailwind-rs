use super::*;
use crate::parse_fraction;

impl TailwindAspect {
    /// https://tailwindcss.com/docs/aspect-ratio
    pub fn parse(kind: &[&str], arbitrary: &str) -> Result<Self> {
        let out = match kind {
            [] => {
                let (a, b) = parse_fraction(arbitrary)?.1;
                Self::Arbitrary(a, b)
            }
            ["auto"] => Self::Auto,
            ["square"] => Self::Arbitrary(1, 1),
            ["video"] => Self::Arbitrary(16, 9),
            ["auto"] => Self::Auto,
            ["inherit"] => todo!(),
            ["w", _n] => todo!(),
            ["h", _n] => todo!(),
            _ => return syntax_error!("unknown aspect-ratio elements"),
        };
        Ok(out)
    }
}

impl TailwindColumns {
    /// https://tailwindcss.com/docs/columns
    pub fn parse(input: &[&str]) -> Result<Self> {
        let out = match input {
            ["auto"] => Self::Auto,
            ["3xs"] => Self::Rem(16),
            ["2xs"] => Self::Rem(18),
            ["xs"] => Self::Rem(20),
            ["sm"] => Self::Rem(24),
            ["md"] => Self::Rem(28),
            ["lg"] => Self::Rem(32),
            ["xl"] => Self::Rem(36),
            ["2xl"] => Self::Rem(42),
            ["3xl"] => Self::Rem(48),
            ["4xl"] => Self::Rem(56),
            ["5xl"] => Self::Rem(64),
            ["6xl"] => Self::Rem(72),
            ["7xl"] => Self::Rem(80),
            [name] => Self::Columns(parse_integer(name)?.1),
            _ => return syntax_error!("Unknown column instructions: {}", input.join("-")),
        };
        Ok(out)
    }
}

impl TailwindBreak {
    /// https://tailwindcss.com/docs/break-before
    pub fn parse_before(input: &[&str]) -> Result<Self> {
        let kind = TailwindBreakKind::Before;
        let info = input.join("-");
        match input {
            ["auto"] | ["avoid"] | ["all"] | ["avoid", "page"] | ["page"] | ["left"] | ["right"] | ["column"] => {
                Ok(Self { kind, info })
            }
            _ => syntax_error!("Unknown break-before instructions: {}", info),
        }
    }
    /// https://tailwindcss.com/docs/break-after
    pub fn parse_after(input: &[&str]) -> Result<Self> {
        let kind = TailwindBreakKind::After;
        let info = input.join("-");
        match input {
            ["auto"] | ["avoid"] | ["all"] | ["avoid", "page"] | ["page"] | ["left"] | ["right"] | ["column"] => {
                Ok(Self { kind, info })
            }
            _ => syntax_error!("Unknown break-after instructions: {}", info),
        }
    }
    /// https://tailwindcss.com/docs/break-inside
    pub fn parse_inside(input: &[&str]) -> Result<Self> {
        let kind = TailwindBreakKind::Inside;
        let info = input.join("-");
        match input {
            ["auto"] | ["avoid"] | ["avoid", "page"] | ["avoid", "column"] => Ok(Self { kind, info }),
            _ => syntax_error!("Unknown break-inside instructions: {}", info),
        }
    }
}

impl TailWindZIndex {
    pub fn parse(kind: &[&str], neg: bool) -> Box<dyn TailwindInstance> {
        match kind.len() {
            1 => {}
            r => panic!("break-inside expected 1 element but found {} elements", r),
        }
        let instance = match kind {
            ["auto"] => Self::Auto,
            [r] => Self::parse_number(r, neg).expect("not number"),
            _ => {
                panic!("Unknown aspect-ratio pattern")
            }
        };
        Box::new(instance)
    }
    #[inline]
    fn parse_number(input: &str, neg: bool) -> Result<Self> {
        let n = parse_integer(input)?.1;
        match neg {
            true => Ok(Self::Negative(n)),
            false => Ok(Self::Positive(n)),
        }
    }
}

impl TailwindClear {
    /// https://tailwindcss.com/docs/clear
    #[inline]
    pub fn parse(kind: &[&str]) -> Result<Self> {
        let out = match kind {
            ["left"] => Self::Left,
            ["right"] => Self::Right,
            ["both"] => Self::Both,
            ["none"] => Self::None,
            _ => return syntax_error!("unknown clear elements"),
        };
        Ok(out)
    }
}
