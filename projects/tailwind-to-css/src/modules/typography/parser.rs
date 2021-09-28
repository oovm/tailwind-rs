use super::*;

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
            ["tighter"] => Self::em(-0.05),
            ["tight"] => Self::em(-0.025),
            // different from tailwind.js
            ["none"] => Self::em(0.0),
            ["wide"] => Self::em(0.025),
            ["wider" | "relaxed"] => Self::em(0.05),
            ["widest" | "loose"] => Self::em(0.1),
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
    #[inline(always)]
    pub(crate) fn em(n: f32) -> Result<Self> {
        Ok(Self { kind: Tracking::Length(LengthUnit::Em(n)) })
    }
}

impl TailwindListStylePosition {
    /// `list-inside`
    pub const Inside: Self = Self { kind: ListStylePosition::Inside };
    /// `list-outside`
    pub const Outside: Self = Self { kind: ListStylePosition::Outside };
}

impl TailwindLeading {
    /// `leading-normal`
    pub const Normal: Self = Self { kind: Leading::Normal };
    /// https://tailwindcss.com/docs/line-height
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            ["none"] => Self::scale(1.0),
            ["tight"] => Self::scale(1.25),
            ["snug"] => Self::scale(1.375),
            // different from tailwind.js
            ["wide"] => Self::scale(1.5),
            ["wider" | "relaxed"] => Self::scale(1.625),
            ["widest" | "loose"] => Self::scale(2.0),
            // https://developer.mozilla.org/zh-CN/docs/Web/CSS/line-height#normal
            ["normal"] => Ok(Self::Normal),
            [] => Self::parse_arbitrary(arbitrary),
            [n] => Self::parse_arbitrary(&TailwindArbitrary::from(*n)),
            _ => syntax_error!("Unknown leading instructions: {}", pattern.join("-")),
        }
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Self::maybe_no_unit(arbitrary).or_else(|_| Self::maybe_length(arbitrary))
    }
    #[inline]
    fn maybe_no_unit(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Self::rem(arbitrary.as_float()? / 4.0)
    }
    #[inline]
    fn maybe_length(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Self::rem(arbitrary.as_float()? / 4.0)
    }
    #[inline(always)]
    fn scale(x: f32) -> Result<Self> {
        Ok(Self { kind: Leading::Length(LengthUnit::Percent(x * 100.0)) })
    }
    #[inline(always)]
    fn rem(x: f32) -> Result<Self> {
        Ok(Self { kind: Leading::Length(LengthUnit::Rem(x)) })
    }
}

impl TailwindListStyle {
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Custom(arbitrary.to_string()))
    }
}

impl TailwindIndent {
    pub fn parse(_input: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindAlign {
    pub fn parse(_input: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindWhiteSpace {
    pub fn parse(_input: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindContentElement {
    pub fn parse_arbitrary(_arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}
