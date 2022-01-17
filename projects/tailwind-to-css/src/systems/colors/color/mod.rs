use super::*;

mod traits;

///
#[derive(Clone, Debug)]
pub enum TailwindColor {
    RGB(Srgb),
    Themed(String, u32),
    Keyword(String),
    Arbitrary(TailwindArbitrary),
}

impl Display for TailwindColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RGB(c) => write!(
                f,
                "[#{:02X?}{:02X?}{:02X?}{:02X?}]",
                (255.0 * c.red) as u8,
                (255.0 * c.green) as u8,
                (255.0 * c.blue) as u8,
                (255.0 * c.alpha) as u8
            ),
            Self::Themed(name, weight) => write!(f, "{}-{}", name, weight),
            Self::Arbitrary(a) => a.write(f),
            Self::Keyword(s) => match s.as_str() {
                "transparent" => write!(f, "transparent"),
                "current" => write!(f, "current"),
                _ => write!(f, "{}", s),
            },
        }
    }
}

#[allow(non_upper_case_globals)]
impl TailwindColor {
    /// `black`
    pub const Black: Self = Self::RGB(Srgb { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 });
    /// `white`
    pub const White: Self = Self::RGB(Srgb { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 });
    /// https://developer.mozilla.org/zh-CN/docs/Web/CSS/color_value
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let out = match pattern {
            ["none"] | ["transparent"] => Self::from("transparent"),
            ["black"] => Self::Black,
            ["white"] => Self::White,
            [s @ ("current" | "inherit" | "initial" | "unset")] => Self::from(*s),
            [] => Self::parse_arbitrary(arbitrary)?,
            [name, weight] => Self::parse_themed(name, weight)?,
            _ => return syntax_error!("Unknown color pattern: {}", pattern.join("-")),
        };
        Ok(out)
    }
    #[inline]
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<TailwindColor> {
        Ok(Self::RGB(arbitrary.as_color()?))
    }
    ///
    #[inline]
    pub fn parse_themed(name: &str, weight: &str) -> Result<TailwindColor> {
        let name = name.to_string();
        let weight = TailwindArbitrary::from(weight).as_integer()? as u32;
        Ok(Self::Themed(name, weight))
    }
    /// get class of `<color>`
    ///
    /// - https://developer.mozilla.org/zh-CN/docs/Web/CSS/color_value
    #[inline]
    pub fn get_class(&self) -> String {
        self.to_string()
    }
    /// get properties of `<color>`
    ///
    /// - https://developer.mozilla.org/zh-CN/docs/Web/CSS/color_value
    #[inline]
    pub fn get_properties(&self, ctx: &TailwindBuilder) -> String {
        match self {
            Self::RGB(c) => format!("rgba({}, {}, {}, {})", 255.0 * c.red, 255.0 * c.green, 255.0 * c.blue, c.alpha),
            Self::Arbitrary(a) => a.get_properties(),
            Self::Keyword(s) => match s.as_str() {
                "transparent" => "transparent".to_string(),
                "current" => "currentColor".to_string(),
                _ => s.to_string(),
            },
            Self::Themed(name, weight) => match ctx.palettes.try_get_color(name, *weight) {
                Ok(c) => format!("rgba({}, {}, {}, {})", 255.0 * c.red, 255.0 * c.green, 255.0 * c.blue, c.alpha),
                Err(_) => "currentColor".to_string(),
            },
        }
    }
}
