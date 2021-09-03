use super::*;

#[derive(Debug, Copy, Clone)]
enum BasisSize {
    Auto,
    Fill,
    Max,
    Min,
    Fit,
    Full,
    Unit(usize),
    Length(LengthUnit),
    Global(CssBehavior),
}

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindBasis {
    kind: BasisSize,
}

impl Display for BasisSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Full => write!(f, "full"),
            Self::Fill => write!(f, "fill"),
            Self::Max => write!(f, "max"),
            Self::Min => write!(f, "min"),
            Self::Fit => write!(f, "fit"),
            Self::Unit(n) => write!(f, "{}", n),
            Self::Length(n) if n.is_fraction() => write!(f, "{}", n.get_class()),
            Self::Length(n) => write!(f, "{}", n.get_class_arbitrary()),
            Self::Global(g) => write!(f, "{}", g),
        }
    }
}

impl Display for TailwindBasis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "basis-{}", self.kind)
    }
}
/// * 指定<'width'> */
// flex-basis: 10em;
// flex-basis: 3px;
// flex-basis: auto;
//
// /* 固有的尺寸关键词 */
// flex-basis: fill;
// flex-basis: max-content;
// flex-basis: min-content;
// flex-basis: fit-content;
//
// /* 在flex item内容上的自动尺寸 */
// flex-basis: content;
//
// /* 全局数值 */
// flex-basis: inherit;
// flex-basis: initial;
// flex-basis: unset;
impl BasisSize {
    pub fn get_properties(&self) -> String {
        match self {
            BasisSize::Auto => "auto".to_string(),
            BasisSize::Fill => "fill".to_string(),
            BasisSize::Max => "max-content".to_string(),
            BasisSize::Min => "min-content".to_string(),
            BasisSize::Fit => "fit-content".to_string(),
            BasisSize::Full => "100%".to_string(),
            BasisSize::Unit(n) => format!("{}rem", *n as f32 / 4.0),
            BasisSize::Length(n) => n.get_properties(),
            BasisSize::Global(g) => g.to_string(),
        }
    }
}

impl TailwindInstance for TailwindBasis {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        todo!()
    }
}

impl TailwindBasis {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            ["auto"] => Ok(Self { kind: BasisSize::Auto }),
            ["full"] => Ok(Self { kind: BasisSize::Full }),
            [n] => {
                let a = TailwindArbitrary::from(*n);

                maybe_unit().or_else(|_: TailwindError| maybe_frac())
            }
            [] => {
                todo!()
            }
            _ => syntax_error!("Unknown basis instructions"),
        }
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) {}
    fn maybe_unit(arbitrary: &TailwindArbitrary) -> Result<TailwindBasis> {
        Ok(Self { kind: BasisSize::Unit(arbitrary.as_integer()?) })
    }
    fn maybe_frac(arbitrary: &TailwindArbitrary) -> Result<TailwindBasis> {
        let (a, b) = arbitrary.as_fraction()?;
        Ok(Self { kind: BasisSize::Length(LengthUnit::Fraction(a, b)) })
    }
}
