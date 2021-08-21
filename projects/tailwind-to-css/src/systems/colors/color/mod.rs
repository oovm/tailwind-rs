use super::*;

#[derive(Clone, Debug)]
pub enum TailwindColor {
    Transparent,
    /// https://developer.mozilla.org/zh-CN/docs/Web/CSS/color_value
    Current,
    RGB(Srgb),
    Themed(String, usize),
    Global(CssBehavior),
}

impl Display for TailwindColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transparent => write!(f, "none"),
            Self::Current => write!(f, "current"),
            Self::RGB(c) => write!(f, "[#{:02X?}]", &[c.red, c.green, c.blue, c.alpha]),
            Self::Themed(name, weight) => write!(f, "{}-{}", name, weight),
            Self::Global(g) => write!(f, "{}", g),
        }
    }
}

impl TailwindColor {
    /// https://developer.mozilla.org/zh-CN/docs/Web/CSS/color_value
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let out = match pattern {
            ["none"] | ["transparent"] => Self::Transparent,
            ["black"] => Self::RGB(TailwindColor { r: 0, g: 0, b: 0, a: 1.0 }),
            ["white"] => Self::RGB(TailwindColor { r: 255, g: 255, b: 255, a: 1.0 }),
            ["current"] => Self::Current,
            ["inherit"] => Self::Global(CssBehavior::Inherit),
            ["initial"] => Self::Global(CssBehavior::Initial),
            ["unset"] => Self::Global(CssBehavior::Unset),
            [] => Self::parse_arbitrary(arbitrary),
            [name, weight] => Self::parse_themed(name, weight),
            _ => syntax_error!("Unknown color pattern: {}", pattern.join("-")),
        };
        Ok(out)
    }
    #[inline]
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<TailwindColor> {
        Ok(Self::RGB(arbitrary.as_color()?))
    }
    #[inline]
    pub fn parse_themed(name: &str, weight: &str) -> Result<TailwindColor> {
        let name = name.to_string();
        let weight = TailwindArbitrary::from(weight).as_integer()?;
        Ok(Self::Themed { name, weight })
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
            Self::Transparent => format!( "transparent"),
            Self::Current => format!( "currentColor"),
            Self::RGB(c) => format!( "#{:02X?}", &[c.red, c.green, c.blue, c.alpha]),
            Self::Global(g) => format!( "{}", g),
            Self::Themed(name, weight) => {
                ctx.palettes.get(name)


                format!("{}-{}", name, weight)
            },
        }
    }
}
