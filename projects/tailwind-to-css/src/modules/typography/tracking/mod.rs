use super::*;

#[doc=include_str!("readme.md")]
#[derive(Copy, Debug, Clone)]
pub struct TailwindTracking {
    kind: Tracking,
}

#[derive(Copy, Debug, Clone)]
enum Tracking {
    Normal,
    Length(LengthUnit),
    Global(CssBehavior),
}

impl Display for Tracking {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Global(g) => write!(f, "{}", g),
            Self::Length(l) => write!(f, "[{}]", l),
        }
    }
}

impl Display for TailwindTracking {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "tracking-{}", self.kind)
    }
}

impl TailwindInstance for TailwindTracking {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let spacing = match self.kind {
            Tracking::Length(n) => n.get_properties(),
            _ => self.kind.to_string(),
        };
        css_attributes! {
            "letter-spacing" => spacing
        }
    }
}

impl TailwindTracking {
    /// `tracking-normal`
    pub const Normal: Self = Self { kind: Tracking::Normal };
    /// `tracking-inherit`
    pub const Inherit: Self = Self { kind: Tracking::Global(CssBehavior::Inherit) };
    /// `tracking-initial`
    pub const Initial: Self = Self { kind: Tracking::Global(CssBehavior::Initial) };
    /// `tracking-unset`
    pub const Unset: Self = Self { kind: Tracking::Global(CssBehavior::Unset) };
    /// https://tailwindcss.com/docs/letter-spacing
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after tracking");
        match input {
            ["tighter"] => em(-0.05),
            ["tight"] => em(-0.025),
            // different from tailwind.js
            ["none"] => em(0.0),
            ["wide"] => em(0.025),
            ["wider" | "relaxed"] => em(0.05),
            ["widest" | "loose"] => em(0.1),
            // https://developer.mozilla.org/zh-CN/docs/Web/CSS/letter-spacing#%E5%80%BC
            ["normal"] => Ok(Self::Normal),
            ["inherit"] => Ok(Self::Inherit),
            ["initial"] => Ok(Self::Initial),
            ["unset"] => Ok(Self::Unset),
            [] => Self::parse_arbitrary(arbitrary),
            [n] => Self::parse_arbitrary(&TailwindArbitrary::from(*n)),
            _ => syntax_error!("Unknown tracking instructions: {}", input.join("-")),
        }
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: Tracking::Length(arbitrary.as_length()?) })
    }
}

#[inline(always)]
fn em(n: f32) -> Result<TailwindTracking> {
    Ok(TailwindTracking { kind: Tracking::Length(LengthUnit::em(n)) })
}
