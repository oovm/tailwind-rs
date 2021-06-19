use super::*;

impl SizeUnit {
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

impl Display for SizeUnit {
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

impl Display for TailwindSizing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Min(s) => write!(f, "min-w-{}", s),
            Self::Max(s) => write!(f, "max-w-{}", s),
            Self::Normal(s) => write!(f, "w-{}", s),
        }
    }
}

impl TailwindInstance for TailwindSizing {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let class = match self {
            Self::Min(_) => "min-width",
            Self::Max(_) => "max-width",
            Self::Normal(_) => "width",
        };
        let width = match self {
            Self::Min(s) | Self::Max(s) | Self::Normal(s) => s.get_attribute(true),
        };
        css_attributes! {
            class => width
        }
    }
}

impl Display for TailwindHeight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Min(s) => write!(f, "min-h-{}", s),
            Self::Max(s) => write!(f, "max-h-{}", s),
            Self::Normal(s) => write!(f, "h-{}", s),
        }
    }
}

impl TailwindInstance for TailwindHeight {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let class = match self {
            Self::Min(_) => "min-height",
            Self::Max(_) => "max-height",
            Self::Normal(_) => "height",
        };
        let height = match self {
            Self::Min(s) | Self::Max(s) | Self::Normal(s) => s.get_attribute(false),
        };
        css_attributes! {
            class => height
        }
    }
}
