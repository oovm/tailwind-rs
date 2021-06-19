use super::*;

impl LengthUnit {
    pub fn get_attribute(&self, is_width: bool) -> String {
        let is_width = match is_width {
            true => "vw",
            false => "vh",
        };
        match self {
            Self::Min => format!("min-content"),
            Self::Max => format!("max-content"),
            Self::Fit => format!("fit-content"),
            Self::Full => format!("100%"),
            Self::Auto => format!("auto"),
            Self::Screen => format!("100{}", is_width),
            Self::Px(n) => format!("{}px", n),
            Self::Rem(n) => format!("{}rem", n),
            Self::Unit(n) => format!("{}rem", *n as f32 / 4.0),
            Self::Fraction(numerator, denominator) => format!("{}%", Self::Percent(*numerator as f32 / *denominator as f32)),
            Self::Percent(percent) => format!("{}%", 100.0 * percent),
        }
    }
}

impl Display for LengthUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Min => f.write_str("min"),
            Self::Max => f.write_str("max"),
            Self::Fit => f.write_str("fit"),
            Self::Screen => f.write_str("screen"),
            Self::Full => f.write_str("full"),
            Self::Auto => f.write_str("auto"),
            Self::Px(px) => match px {
                n if n.eq(&0.0) => f.write_char('0'),
                n if n.eq(&1.0) => f.write_str("px"),
                _ => f.write_str("{}px"),
            },
            Self::Unit(n) => write!(f, "{}", n),
            Self::Fraction(a, b) => write!(f, "{}/{}", a, b),
            Self::Rem(rem) => write!(f, "{}rem", rem),
            Self::Percent(p) => write!(f, "{}%", (p * 100.0).round()),
        }
    }
}

impl Display for TailwindSizingKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Width => f.write_str("w"),
            Self::MinWidth => f.write_str("min-w"),
            Self::MaxWidth => f.write_str("max-w"),
            Self::Height => f.write_str("h"),
            Self::MinHeight => f.write_str("min-h"),
            Self::MaxHeight => f.write_str("max-h"),
        }
    }
}

impl Display for TailwindSizing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.kind, self.size)
    }
}

impl TailwindInstance for TailwindSizing {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let class = self.kind.to_string();
        let width = self.size.get_attribute(true);
        css_attributes! {
            class => width
        }
    }
}
