use super::*;

#[derive(Debug, Clone)]
enum GridAutoKind {
    Auto,
    Min,
    Max,
    Fr,
    Arbitrary(String),
}

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindGridAuto {
    kind: GridAutoKind,
    // - ture: rows
    // - false: cols
    axis: bool,
}

impl Display for GridAutoKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Min => write!(f, "min"),
            Self::Max => write!(f, "max"),
            Self::Fr => write!(f, "fr"),
            Self::Arbitrary(s) => write!(f, "[{}]", s),
        }
    }
}

impl Display for TailwindGridAuto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.axis {
            true => write!(f, "auto-rows-{}", self.kind),
            false => write!(f, "auto-cols-{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindGridAuto {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let class = match self.axis {
            true => "grid-auto-rows",
            false => "grid-auto-columns",
        };
        let auto = match &self.kind {
            GridAutoKind::Auto => "auto".to_string(),
            GridAutoKind::Min => "min-content".to_string(),
            GridAutoKind::Max => "max-content".to_string(),
            GridAutoKind::Fr => "minmax(0,1fr)".to_string(),
            GridAutoKind::Arbitrary(a) => a.to_string(),
        };

        css_attributes! {
            class => auto
        }
    }
}

impl GridAutoKind {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            [] => {
                debug_assert!(arbitrary.is_some());
                Self::parse_arbitrary(arbitrary)?
            },
            ["auto"] => Self::Auto,
            ["min"] => Self::Min,
            ["max"] => Self::Max,
            ["fr"] => Self::Fr,
            _ => return syntax_error!("Unknown shadow instructions: {}", pattern.join("-")),
        };
        Ok(kind)
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary(arbitrary.to_string()))
    }
}

impl TailwindGridAuto {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            ["rows", rest @ ..] => Self::parse_axis(rest, arbitrary, true),
            ["cols", rest @ ..] => Self::parse_axis(rest, arbitrary, false),
            _ => syntax_error!("Unknown auto instructions: {}", pattern.join("-")),
        }
    }
    fn parse_axis(pattern: &[&str], arbitrary: &TailwindArbitrary, axis: bool) -> Result<Self> {
        Ok(Self { kind: GridAutoKind::parse(pattern, arbitrary)?, axis })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, axis: bool) -> Result<Self> {
        Ok(Self { kind: GridAutoKind::parse_arbitrary(arbitrary)?, axis })
    }
}
