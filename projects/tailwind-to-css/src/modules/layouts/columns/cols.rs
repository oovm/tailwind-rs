use super::*;

#[derive(Clone, Debug)]
pub enum Columns {
    Columns(i32),
    Length(LengthUnit),
    Standard(String),
    Arbitrary(TailwindArbitrary),
}

impl Display for Columns {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Columns(n) => write!(f, "{}", n),
            Self::Length(n) => write!(f, "{}", n.get_class_arbitrary()),
            Self::Standard(s) => write!(f, "{}", s),
            Self::Arbitrary(s) => s.write(f),
        }
    }
}

impl Columns {
    #[inline]
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let rem = |n: usize| Self::Length(LengthUnit::rem(n as f32));
        let out = match input {
            [s] if Self::check_valid(s) => Self::Standard(s.to_string()),
            ["3xs"] => rem(16),
            ["2xs"] => rem(18),
            ["xs"] => rem(20),
            ["sm"] => rem(24),
            ["md"] => rem(28),
            ["lg"] => rem(32),
            ["xl"] => rem(36),
            ["2xl"] => rem(42),
            ["3xl"] => rem(48),
            ["4xl"] => rem(56),
            ["5xl"] => rem(64),
            ["6xl"] => rem(72),
            ["7xl"] => rem(80),
            [name] => {
                let a = TailwindArbitrary::from(*name);
                Self::Columns(a.as_integer()?)
            },
            [] => Self::parse_arbitrary(arbitrary)?,
            _ => return syntax_error!("Unknown columns instructions: {}", input.join("-")),
        };
        Ok(out)
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary(TailwindArbitrary::from(arbitrary)))
    }
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["auto", "inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
    pub fn get_properties(&self) -> String {
        match self {
            Columns::Columns(s) => s.to_string(),
            Columns::Length(s) => s.get_properties(),
            Columns::Standard(s) => s.to_string(),
            Columns::Arbitrary(s) => s.get_properties(),
        }
    }
}
