use super::*;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindLeading {
    kind: LineHeight,
}

#[derive(Debug, Clone)]
enum LineHeight {
    Length(LengthUnit),
    Standard(String),
}

impl Display for LineHeight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Length(n) => write!(f, "{}", n.get_class_arbitrary()),
            Self::Standard(g) => write!(f, "{}", g),
        }
    }
}

impl Display for TailwindLeading {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "leading-{}", self.kind)
    }
}

impl TailwindInstance for TailwindLeading {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let leading = match &self.kind {
            LineHeight::Length(n) => n.get_properties(),
            LineHeight::Standard(g) => g.to_string(),
        };
        css_attributes! {
            "line-height" => leading
        }
    }
}

impl TailwindLeading {
    /// https://tailwindcss.com/docs/line-height
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            ["none"] => scale(1.0),
            ["tight"] => scale(1.25),
            ["snug"] => scale(1.375),
            // different from tailwind.js
            ["wide"] => scale(1.5),
            ["wider" | "relaxed"] => scale(1.625),
            ["widest" | "loose"] => scale(2.0),
            // https://developer.mozilla.org/zh-CN/docs/Web/CSS/line-height#normal
            ["normal"] => Ok(Self { kind: LineHeight::Standard("normal".to_string()) }),
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
        rem(arbitrary.as_float()? / 4.0)
    }
    #[inline]
    fn maybe_length(arbitrary: &TailwindArbitrary) -> Result<Self> {
        rem(arbitrary.as_float()? / 4.0)
    }
}

#[inline(always)]
fn scale(x: f32) -> Result<TailwindLeading> {
    Ok(TailwindLeading { kind: LineHeight::Length(LengthUnit::percent(x * 100.0)) })
}
#[inline(always)]
fn rem(x: f32) -> Result<TailwindLeading> {
    Ok(TailwindLeading { kind: LineHeight::Length(LengthUnit::rem(x)) })
}
