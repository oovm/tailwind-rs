use super::*;
use crate::parse_fraction;

impl AspectKind {
    #[inline]
    pub fn parse(kind: &[&str]) -> Result<Self> {
        let out = match kind {
            ["auto"] => Self::Auto,
            ["square"] => Self::Radio(1, 1),
            ["video"] => Self::Radio(16, 9),
            ["inherit"] => Self::Global(CssBehavior::Inherit),
            ["initial"] => Self::Global(CssBehavior::Initial),
            ["unset"] => Self::Global(CssBehavior::Unset),
            [n] => {
                let (a, b) = parse_fraction(n)?.1;
                Self::Radio(a, b)
            }
            _ => return syntax_error!("unknown aspect-ratio elements"),
        };
        Ok(out)
    }
}

impl TailwindAspect {
    /// https://tailwindcss.com/docs/aspect-ratio
    pub fn parse(kind: &[&str], arbitrary: &str) -> Result<Self> {
        debug_assert!(arbitrary.is_empty(), "forbidden arbitrary in aspect");
        Ok(Self { kind: AspectKind::parse(kind)? })
    }
}

impl ColumnKind {
    #[inline]
    pub fn parse(input: &[&str]) -> Result<Self> {
        let out = match input {
            ["auto"] => Self::Auto,
            ["3xs"] => Self::rem(16),
            ["2xs"] => Self::rem(18),
            ["xs"] => Self::rem(20),
            ["sm"] => Self::rem(24),
            ["md"] => Self::rem(28),
            ["lg"] => Self::rem(32),
            ["xl"] => Self::rem(36),
            ["2xl"] => Self::rem(42),
            ["3xl"] => Self::rem(48),
            ["4xl"] => Self::rem(56),
            ["5xl"] => Self::rem(64),
            ["6xl"] => Self::rem(72),
            ["7xl"] => Self::rem(80),
            [name] => {
                debug_assert!(!name.contains('%'), "forbidden use percent");
                alt((Self::parse_length, Self::parse_columns))(name)?.1
            }
            _ => return syntax_error!("Unknown column instructions: {}", input.join("-")),
        };
        Ok(out)
    }
    #[inline]
    fn parse_columns(input: &str) -> IResult<&str, Self> {
        let (rest, i) = parse_integer(input)?;
        Ok((rest, Self::Columns(i)))
    }
    #[inline]
    fn parse_length(input: &str) -> IResult<&str, Self> {
        let (rest, l) = LengthUnit::parse(input)?;
        Ok((rest, Self::Length(l)))
    }

    #[inline(always)]
    fn rem(n: usize) -> ColumnKind {
        Self::Length(LengthUnit::Rem(n as f32))
    }
}

impl TailwindColumns {
    /// https://tailwindcss.com/docs/columns
    pub fn parse(input: &[&str], arbitrary: &str) -> Result<Self> {
        debug_assert!(arbitrary.is_empty(), "forbidden arbitrary in aspect");
        Ok(Self { kind: ColumnKind::parse(input)? })
    }
}

impl TailwindBreakLayout {
    /// https://tailwindcss.com/docs/break-before
    pub fn parse_before(input: &[&str]) -> Result<Self> {
        let kind = BreakKind::Before;
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
        let kind = BreakKind::After;
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
        let kind = BreakKind::Inside;
        let info = input.join("-");
        match input {
            ["auto"] | ["avoid"] | ["avoid", "page"] | ["avoid", "column"] => Ok(Self { kind, info }),
            _ => syntax_error!("Unknown break-inside instructions: {}", info),
        }
    }
}

impl TailwindObjectPosition {
    pub fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        todo!("{}", arbitrary)
    }
}

impl OverflowKind {
    pub fn parse(input: &[&str]) -> Result<Self> {
        match input {
            ["auto"] => Ok(Self::Auto),
            ["hidden"] => Ok(Self::Hidden),
            ["clip"] => Ok(Self::Clip),
            ["visible"] => Ok(Self::Visible),
            ["scroll"] => Ok(Self::Scroll),
            _ => syntax_error!("Unknown overflow instructions: {}", input.join("-")),
        }
    }
}

impl TailwindOverflow {
    #[inline]
    pub fn parse_x(kind: &[&str]) -> Result<Self> {
        let kind = OverflowKind::parse(kind)?;
        Ok(Self { kind, axis: Some(true) })
    }
    #[inline]
    pub fn parse_y(kind: &[&str]) -> Result<Self> {
        let kind = OverflowKind::parse(kind)?;
        Ok(Self { kind, axis: Some(false) })
    }
    #[inline]
    pub fn parse_xy(kind: &[&str]) -> Result<Self> {
        let kind = OverflowKind::parse(kind)?;
        Ok(Self { kind, axis: None })
    }
}

impl OverscrollKind {
    pub fn parse(input: &[&str]) -> Result<Self> {
        match input {
            ["auto"] => Ok(Self::Auto),
            ["contain"] => Ok(Self::Contain),
            ["none"] => Ok(Self::None),
            _ => syntax_error!("Unknown overflow instructions: {}", input.join("-")),
        }
    }
}

impl TailwindOverscroll {
    #[inline]
    pub fn parse_x(kind: &[&str]) -> Result<Self> {
        let kind = OverscrollKind::parse(kind)?;
        Ok(Self { kind, axis: Some(true) })
    }
    #[inline]
    pub fn parse_y(kind: &[&str]) -> Result<Self> {
        let kind = OverscrollKind::parse(kind)?;
        Ok(Self { kind, axis: Some(false) })
    }
    #[inline]
    pub fn parse_xy(kind: &[&str]) -> Result<Self> {
        let kind = OverscrollKind::parse(kind)?;
        Ok(Self { kind, axis: None })
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
