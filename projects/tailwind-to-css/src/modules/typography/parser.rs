use super::*;

impl TailwindFontArbitrary {
    pub fn parse() {}
}

impl TailwindFontFamily {
    #[inline]
    pub fn new(input: &str) -> Self {
        Self { name: input.to_string() }
    }
}

impl TailwindFontSmoothing {
    #[inline]
    pub fn new(subpixel: bool) -> Self {
        match subpixel {
            true => Self::Subpixel,
            false => Self::Normal,
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
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary in tracking");
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

// normal-nums	font-variant-numeric: normal;
// ordinal	font-variant-numeric: ordinal;
// slashed-zero	font-variant-numeric: slashed-zero;
// lining-nums	font-variant-numeric: lining-nums;
// oldstyle-nums	font-variant-numeric: oldstyle-nums;
// proportional-nums	font-variant-numeric: proportional-nums;
impl TailwindFontVariantNumeric {
    ///
    pub const Normal: Self = Self { kind: FontVariantNumeric::Normal };
    ///
    pub const Ordinal: Self = Self { kind: FontVariantNumeric::Ordinal };
    ///
    pub const SlashedZero: Self = Self { kind: FontVariantNumeric::SlashedZero };
    ///
    pub const Lining: Self = Self { kind: FontVariantNumeric::Lining };
    ///
    pub const OldStyle: Self = Self { kind: FontVariantNumeric::OldStyle };
    ///
    pub const Proportional: Self = Self { kind: FontVariantNumeric::Proportional };
    /// `tabular-nums`
    pub const Tabular: Self = Self { kind: FontVariantNumeric::Tabular };
    /// `diagonal-fractions`
    pub const DiagonalFractions: Self = Self { kind: FontVariantNumeric::DiagonalFractions };
    /// `stacked-fractions`
    pub const StackedFractions: Self = Self { kind: FontVariantNumeric::StackedFractions };
}

// underline	text-decoration-line: underline;
// overline	text-decoration-line: overline;
// line-through	text-decoration-line: line-through;
// no-underline	text-decoration-line: none;
impl TailwindTextDecoration {
    ///
    pub const Normal: Self = Self { kind: TextDecoration::Underline };
    ///
    pub const Ordinal: Self = Self { kind: TextDecoration::Ordinal };
    ///
    pub const SlashedZero: Self = Self { kind: TextDecoration::SlashedZero };
    ///
    pub const Lining: Self = Self { kind: TextDecoration::Lining };
}

impl TailwindLeading {
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match input {
            ["none"] => Ok(Self::Scale(1.0)),
            ["tight"] => Ok(Self::Scale(1.25)),
            ["snug"] => Ok(Self::Scale(1.375)),
            // different from tailwind.js
            ["wide"] => Ok(Self::Scale(1.5)),
            ["wider" | "relaxed"] => Ok(Self::Scale(1.625)),
            ["widest" | "loose"] => Ok(Self::Scale(2.0)),
            // https://developer.mozilla.org/zh-CN/docs/Web/CSS/line-height#normal
            ["normal"] => Ok(Self::Normal),
            [] => Self::parse_arbitrary(arbitrary),
            [n] => Self::parse_arbitrary(todo!()),
            _ => syntax_error!("Unknown tracking instructions: {}", input.join("-")),
        }
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        let out = alt((Self::arbitrary_percent, Self::arbitrary_rem, Self::arbitrary_unit))(todo!())?;
        Ok(out.1)
    }
    #[inline]
    fn arbitrary_percent(input: &str) -> IResult<&str, Self> {
        let (rest, f) = parse_f_percent(input)?;
        Ok((rest, Self::Scale(f / 100.0)))
    }
    #[inline]
    fn arbitrary_unit(input: &str) -> IResult<&str, Self> {
        let (rest, u) = parse_integer(input)?;
        Ok((rest, Self::Unit(u)))
    }
    // #[inline]
    // fn arbitrary_px(input: &str) -> IResult<&str, Self> {
    //     let (rest, (f, _)) = tuple((parse_f32, tag("px")))(input)?;
    //     Ok((rest, Self::Px(f)))
    // }
    #[inline]
    fn arbitrary_rem(input: &str) -> IResult<&str, Self> {
        let (rest, (f, _)) = tuple((parse_f32, tag("rem")))(input)?;
        Ok((rest, Self::Rem(f)))
    }
}

impl TailwindListStyle {
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Custom(arbitrary.to_string()))
    }
}

impl TailwindFontSize {
    #[inline]
    pub fn new(size: f32, height: f32) -> Self {
        Self { size: TailwindTracking::em(size).unwrap(), height: TailwindLeading::Rem(height) }
    }
}

impl TailwindFontWeight {
    pub const THIN: Self = Self { weight: 100 };
    pub const EXTRA_LIGHT: Self = Self { weight: 200 };
    pub const LIGHT: Self = Self { weight: 300 };
    pub const NORMAL: Self = Self { weight: 400 };
    pub const MEDIUM: Self = Self { weight: 500 };
    pub const SEMI_BOLD: Self = Self { weight: 600 };
    pub const BOLD: Self = Self { weight: 700 };
    pub const EXTRA_BOLD: Self = Self { weight: 800 };
    pub const BLACK: Self = Self { weight: 900 };
    #[inline]
    pub fn new(weight: usize) -> Self {
        Self { weight }
    }
}

impl TailwindIndent {
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindAlign {
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindWhiteSpace {
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindUnderlineOffset {
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindContentElement {
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}
