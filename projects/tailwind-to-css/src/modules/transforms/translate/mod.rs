use super::*;

#[derive(Copy, Clone, Debug)]
enum TranslateSize {
    Unit(usize),
    Fraction(usize, usize),
    Length(LengthUnit),
    Global(CssBehavior),
}

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindTranslate {
    negative: bool,
    axis: bool,
    kind: TranslateSize,
}

impl Display for TailwindTranslate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.negative {
            f.write_char('-')?
        }
        match self.kind {
            true => write!(f, "translate-x-{}", self.kind),
            false => write!(f, "translate-y-{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindTranslate {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let skew = match self.axis {
            true => format!("skewX({}deg)", self.deg),
            false => format!("skewY({}deg)", self.deg),
        };
        css_attributes! {
            "transform" => skew
        }
    }
}

impl TailwindTranslate {
    // https://tailwindcss.com/docs/skew
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, neg: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after skew");
        match pattern {
            ["x", n] => Ok(Self { neg, deg: TailwindArbitrary::from(*n).as_integer()?, axis: true }),
            ["y", n] => Ok(Self { neg, deg: TailwindArbitrary::from(*n).as_integer()?, axis: false }),
            _ => syntax_error!("Unknown skew instructions: {}", pattern.join("-")),
        }
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Self::maybe_length(arbitrary).or_else(|_| Self::maybe_frac(arbitrary)).or_else(|_| Self::maybe_unit(arbitrary))
    }
    fn maybe_unit(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: BasisSize::Unit(arbitrary.as_integer()?) })
    }
    fn maybe_length(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: BasisSize::Length(arbitrary.as_length()?) })
    }
    fn maybe_frac(arbitrary: &TailwindArbitrary) -> Result<Self> {
        let (a, b) = arbitrary.as_fraction()?;
        Ok(Self { kind: BasisSize::Length(LengthUnit::Fraction(a, b)) })
    }
}
