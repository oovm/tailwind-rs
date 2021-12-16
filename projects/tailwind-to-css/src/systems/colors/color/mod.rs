use super::*;


///
#[derive(Clone, Debug)]
pub enum TailwindColor {
    Transparent,
    Current,
    RGB(Srgb),
    Themed(String, usize),
    Arbitrary(String),
    Global(CssBehavior),
}

struct ColorWrapper(Srgb);

impl Display for ColorWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#{:02X?}",
            &[
                (255.0 * self.0.red) as u8,
                (255.0 * self.0.green) as u8,
                (255.0 * self.0.blue) as u8,
                (255.0 * self.0.alpha) as u8
            ]
        )
    }
}

impl Display for TailwindColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transparent => write!(f, "none"),
            Self::Current => write!(f, "current"),
            Self::RGB(c) => write!(f, "[{}]", ColorWrapper(*c)),
            Self::Themed(name, weight) => write!(f, "{}-{}", name, weight),
            Self::Arbitrary(a) => write!(f, "[{}]", a),
            Self::Global(g) => write!(f, "{}", g),
        }
    }
}

impl ColorWrapper {}

#[allow(non_upper_case_globals)]
impl TailwindColor {
    /// `black`
    pub const Black: Self = Self::RGB(Srgb { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 });
    /// `white`
    pub const White: Self = Self::RGB(Srgb { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 });
    /// `inherit`
    pub const Inherit: Self = Self::Global(CssBehavior::Inherit);
    /// `initial`
    pub const Initial: Self = Self::Global(CssBehavior::Initial);
    /// `unset`
    pub const Unset: Self = Self::Global(CssBehavior::Unset);
    /// https://developer.mozilla.org/zh-CN/docs/Web/CSS/color_value
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let out = match pattern {
            ["none"] | ["transparent"] => Self::Transparent,
            ["black"] => Self::Black,
            ["white"] => Self::White,
            ["current"] => Self::Current,
            ["inherit"] => Self::Inherit,
            ["initial"] => Self::Initial,
            ["unset"] => Self::Unset,
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
        let weight = TailwindArbitrary::from(weight).as_integer()?;
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
            Self::Transparent => "transparent".to_string(),
            Self::Current => "currentColor".to_string(),
            Self::RGB(c) => format!("#{:02X?}", &[c.red, c.green, c.blue, c.alpha]),
            Self::Global(g) => format!("{}", g),
            Self::Arbitrary(a) => a.to_string(),
            Self::Themed(name, weight) => match ctx.palettes.get_color(name, *weight) {
                Ok(c) => ColorWrapper(c).to_string(),
                Err(_) => "currentColor".to_string(),
            },
        }
    }
}
