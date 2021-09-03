use super::*;
use crate::TailwindArbitrary;

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
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after columns");
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
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!("{}", arbitrary)
    }
}

impl Overflow {
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
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary, axis: Option<bool>) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after overflow");
        let kind = Overflow::parse(kind)?;
        Ok(Self { kind, axis })
    }
}

impl Overscroll {
    #[inline]
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
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary, axis: Option<bool>) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after overflow");
        let kind = Overscroll::parse(kind)?;
        Ok(Self { kind, axis })
    }
}

impl TailwindFloat {
    /// https://tailwindcss.com/docs/float
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after float");
        let out = match kind {
            ["left"] => Self { kind: FloatKind::Left },
            ["right"] => Self { kind: FloatKind::Right },
            ["none"] => Self { kind: FloatKind::None },
            _ => return syntax_error!("Unknown float elements: {}", kind.join("-")),
        };
        Ok(out)
    }
}

impl TailwindClear {
    /// https://tailwindcss.com/docs/clear
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after brightness");
        let out = match kind {
            ["left"] => Self { kind: ClearKind::Left },
            ["right"] => Self { kind: ClearKind::Right },
            ["both"] => Self { kind: ClearKind::Both },
            ["none"] => Self { kind: ClearKind::None },
            _ => return syntax_error!("Unknown clear elements: {}", kind.join("-")),
        };
        Ok(out)
    }
}

impl TailwindObjectFit {
    /// `object-contain`
    pub const Contain: Self = Self { kind: ObjectFit::Contain };
    /// `object-cover`
    pub const Cover: Self = Self { kind: ObjectFit::Cover };
    /// `object-fill`
    pub const Fill: Self = Self { kind: ObjectFit::Fill };
    /// `object-none`
    pub const None: Self = Self { kind: ObjectFit::None };
    /// `object-scale-down`
    pub const ScaleDown: Self = Self { kind: ObjectFit::ScaleDown };
}

impl TailwindIsolation {
    /// `isolate`
    pub const Isolate: Self = Self { kind: Isolation::Isolate };
    /// `isolation-auto`
    pub const Auto: Self = Self { kind: Isolation::Auto };
}

impl TailwindPosition {
    /// `static`
    pub const Static: Self = Self { kind: PositionKind::Static };
    /// `fixed`
    pub const Fixed: Self = Self { kind: PositionKind::Fixed };
    /// `absolute`
    pub const Absolute: Self = Self { kind: PositionKind::Absolute };
    /// `relative`
    pub const Relative: Self = Self { kind: PositionKind::Relative };
    /// `sticky`
    pub const Sticky: Self = Self { kind: PositionKind::Sticky };
}

impl TailwindBoxDecoration {
    /// ``
    pub const Clone: Self = Self { kind: BoxDecoration::Clone };
    ///
    pub const Slice: Self = Self { kind: BoxDecoration::Slice };
}

impl TailwindVisibility {
    /// `visible`
    pub const Visible: Self = Self { kind: Visibility::Visible };
    /// `invisible`
    pub const Invisible: Self = Self { kind: Visibility::Invisible };
}

impl TailwindBoxSizing {
    ///
    pub const Border: Self = Self { kind: BoxSizing::Border };
    ///
    pub const Content: Self = Self { kind: BoxSizing::Content };
}

impl TailWindZIndex {
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary, neg: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after z-index");
        match kind {
            ["auto"] => Ok(Self { kind: ZIndex::Auto, neg }),
            [r] => Self::parse_number(r, neg),
            _ => syntax_error!("Unknown contrast instructions"),
        }
    }
    #[inline]
    fn parse_number(input: &str, neg: bool) -> Result<Self> {
        let n = parse_integer(input)?.1;
        Ok(Self { kind: ZIndex::Unit(n), neg })
    }
}
