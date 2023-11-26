use super::*;

impl Display for SizingUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Min => write!(f, "min"),
            Self::Max => write!(f, "max"),
            Self::Fit => write!(f, "fit"),
            Self::Auto => write!(f, "auto"),
            Self::Full => write!(f, "full"),
            Self::Screen => write!(f, "screen"),
            Self::Fraction(numerator, denominator) => write!(f, "{}/{}", numerator, denominator),
            Self::Length(x) => write!(f, "[{}]", x),
        }
    }
}

impl SizingUnit {
    fn get_attribute(&self, is_width: bool) -> String {
        let is_width = match is_width {
            true => "vw",
            false => "vh",
        };
        match self {
            Self::Min => "min-content".to_string(),
            Self::Max => "max-content".to_string(),
            Self::Fit => "fit-content".to_string(),
            Self::Auto => "auto".to_string(),
            Self::Full => "100%".to_string(),
            Self::Screen => format!("100{}", is_width),
            Self::Fraction(numerator, denominator) => format!("{}%", *numerator as f32 / *denominator as f32),
            Self::Length(x) => format!("{}", x),
        }
    }
}

impl TailwindSizingKind {
    fn is_width(self) -> bool {
        match self {
            TailwindSizingKind::Width => true,
            TailwindSizingKind::MinWidth => true,
            TailwindSizingKind::MaxWidth => true,
            TailwindSizingKind::Height => false,
            TailwindSizingKind::MinHeight => false,
            TailwindSizingKind::MaxHeight => false,
        }
    }
}

impl Display for TailwindSizingKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Width => f.write_str("width"),
            Self::MinWidth => f.write_str("min-width"),
            Self::MaxWidth => f.write_str("max-width"),
            Self::Height => f.write_str("height"),
            Self::MinHeight => f.write_str("min-height"),
            Self::MaxHeight => f.write_str("max-height"),
        }
    }
}

impl Display for TailwindSizing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.kind, self.size)
    }
}

impl TailwindInstance for TailwindSizing {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let class = self.kind.to_string();
        let width = self.size.get_attribute(self.kind.is_width());
        css_attributes! {
            class => width
        }
    }
}
