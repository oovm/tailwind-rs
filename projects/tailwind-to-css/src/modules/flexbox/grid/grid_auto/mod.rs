use crate::{modules::flexbox::*, AxisXY};

#[derive(Debug, Clone)]
enum GridAutoKind {
    Auto,
    Min,
    Max,
    Fr,
    Arbitrary(TailwindArbitrary),
}

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindGridAuto {
    kind: GridAutoKind,
    // - ture: rows
    // - false: cols
    axis: AxisXY,
}

impl Display for GridAutoKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Min => write!(f, "min"),
            Self::Max => write!(f, "max"),
            Self::Fr => write!(f, "fr"),
            Self::Arbitrary(s) => s.write(f),
        }
    }
}

impl Display for TailwindGridAuto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.axis {
            AxisXY::X => write!(f, "auto-rows-{}", self.kind),
            AxisXY::Y => write!(f, "auto-cols-{}", self.kind),
            _ => unreachable!(),
        }
    }
}

impl TailwindInstance for TailwindGridAuto {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let class = match self.axis {
            AxisXY::X => "grid-auto-rows",
            AxisXY::Y => "grid-auto-columns",
            _ => unreachable!(),
        };
        let auto = match &self.kind {
            GridAutoKind::Auto => "auto".to_string(),
            GridAutoKind::Min => "min-content".to_string(),
            GridAutoKind::Max => "max-content".to_string(),
            GridAutoKind::Fr => "minmax(0,1fr)".to_string(),
            GridAutoKind::Arbitrary(a) => a.get_properties(),
        };
        css_attributes! {
            class => auto
        }
    }
}

impl GridAutoKind {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            [] => Self::parse_arbitrary(arbitrary)?,
            ["auto"] => Self::Auto,
            ["min"] => Self::Min,
            ["max"] => Self::Max,
            ["fr"] => Self::Fr,
            _ => return syntax_error!("Unknown shadow instructions: {}", pattern.join("-")),
        };
        Ok(kind)
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary(TailwindArbitrary::new(arbitrary)?))
    }
}

impl TailwindGridAuto {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let (axis, rest) = match pattern {
            ["rows", rest @ ..] => (AxisXY::X, rest),
            ["cols", rest @ ..] => (AxisXY::Y, rest),
            _ => return syntax_error!("Unknown auto instructions: {}", pattern.join("-")),
        };
        let kind = GridAutoKind::parse(rest, arbitrary)?;
        Ok(Self { kind, axis })
    }
}
