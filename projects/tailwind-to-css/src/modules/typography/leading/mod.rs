use super::*;

#[derive(Copy, Debug, Clone)]
enum Leading {
    Normal,
    Length(LengthUnit),
    Global(CssBehavior),
}

#[doc = include_str!("readme.md")]
#[derive(Copy, Debug, Clone)]
pub struct TailwindLeading {
    kind: Leading,
}

impl Display for Leading {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Length(n) => write!(f, "{}", n.get_class_arbitrary()),
            Self::Global(g) => write!(f, "{}", g),
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
        let leading = match self.kind {
            Leading::Normal => "normal".to_string(),
            Leading::Length(n) => n.get_properties(),
            Leading::Global(g) => g.to_string(),
        };
        css_attributes! {
            "line-height" => leading
        }
    }
}

impl TailwindLeading {
    /// `leading-normal`
    pub const Normal: Self = Self { kind: Leading::Normal };
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
        rem(arbitrary.as_float()? / 4.0)
    }
    #[inline]
    fn maybe_length(arbitrary: &TailwindArbitrary) -> Result<Self> {
        rem(arbitrary.as_float()? / 4.0)
    }
}

#[inline(always)]
fn scale(x: f32) -> Result<TailwindLeading> {
    Ok(TailwindLeading { kind: Leading::Length(LengthUnit::percent(x * 100.0)) })
}
#[inline(always)]
fn rem(x: f32) -> Result<TailwindLeading> {
    Ok(TailwindLeading { kind: Leading::Length(LengthUnit::rem(x)) })
}
