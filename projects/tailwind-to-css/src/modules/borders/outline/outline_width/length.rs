use super::*;

#[derive(Clone, Debug)]
pub(super) enum OutlineWidth {
    Unit(usize),
    Standard(String),
    Length(LengthUnit),
}

impl OutlineWidth {
    /// https://tailwindcss.com/docs/outline-offset
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after outline-offset");
        match pattern {
            [n] if Self::check_valid(n) => Ok(Self::Standard(n.to_string())),
            [n] => {
                let a = TailwindArbitrary::from(*n);
                Self::maybe_no_unit(&a).or_else(|_| Self::maybe_length(&a))
            },
            _ => return syntax_error!("Unknown font-smoothing instructions: {}", pattern.join("-")),
        }
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/outline-offset#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "medium", "revert", "thick", "thin", "unset"]);
        set.contains(mode)
    }
    pub fn maybe_no_unit(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Unit(arbitrary.as_integer()?))
    }
    pub fn maybe_length(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Length(arbitrary.as_length()?))
    }
}
