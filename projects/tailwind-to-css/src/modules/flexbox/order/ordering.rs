use super::*;

#[derive(Debug, Clone)]
pub enum Order {
    Number(i32, Negative),
    Standard(String),
    Arbitrary(TailwindArbitrary),
}

impl Display for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Order::Number(number, _) => write!(f, "{}", number.abs()),
            Order::Standard(value) => write!(f, "{}", value),
            Order::Arbitrary(value) => value.write(f),
        }
    }
}

impl Order {
    #[inline]
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let out = match pattern {
            ["none"] => Self::Number(0, Negative::from(false)),
            ["first"] => Self::Number(9999, Negative::from(false)),
            ["last"] => Self::Number(-9999, Negative::from(true)),
            [s] if Self::check_valid(s) => Self::Standard(s.to_string()),
            [n] => {
                let mut order: i32 = parse_integer(n)?.1;
                if negative.unwrap() {
                    order = -order;
                }
                Self::Number(order, negative)
            },
            [] => Self::parse_arbitrary(arbitrary)?,
            _ => return syntax_error!("Unknown order instructions: {}", pattern.join("-")),
        };
        Ok(out)
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary(TailwindArbitrary::new(arbitrary)?))
    }
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
    pub fn write_negative(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n, neg) if n.ne(&0) && neg.eq(&true) => write!(f, "-"),
            _ => write!(f, ""),
        }
    }
}
