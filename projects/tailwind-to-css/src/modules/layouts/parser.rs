use super::*;
use crate::TailwindArbitrary;

impl TailwindBreakLayout {
    /// https://tailwindcss.com/docs/break-before
    pub fn parse_before(input: &[&str]) -> Result<Self> {
        let kind = BreakKind::Before;
        let info = input.join("-");
        match input {
            ["auto"] | ["avoid"] | ["all"] | ["avoid", "page"] | ["page"] | ["left"] | ["right"] | ["column"] =>
                Ok(Self { kind, info }),
            _ => syntax_error!("Unknown break-before instructions: {}", info),
        }
    }
    /// https://tailwindcss.com/docs/break-after
    pub fn parse_after(input: &[&str]) -> Result<Self> {
        let kind = BreakKind::After;
        let info = input.join("-");
        match input {
            ["auto"] | ["avoid"] | ["all"] | ["avoid", "page"] | ["page"] | ["left"] | ["right"] | ["column"] =>
                Ok(Self { kind, info }),
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

impl TailwindBoxDecoration {
    /// ``
    pub const Clone: Self = Self { kind: BoxDecoration::Clone };
    ///
    pub const Slice: Self = Self { kind: BoxDecoration::Slice };
}

impl TailwindBoxSizing {
    ///
    pub const Border: Self = Self { kind: BoxSizing::Border };
    ///
    pub const Content: Self = Self { kind: BoxSizing::Content };
}
