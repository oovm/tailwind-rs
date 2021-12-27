use super::*;

#[derive(Clone, Debug)]
enum ZIndex {
    Unit(i32),
    Standard(String),
    Arbitrary(TailwindArbitrary),
}

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindZIndex {
    kind: ZIndex,
}

impl Display for TailwindZIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ZIndex::Unit(s) => {
                let n = s.abs();
                match *s > 0i32 {
                    true => write!(f, "z-{}", n),
                    false => write!(f, "-z-{}", n),
                }
            },
            ZIndex::Standard(s) => write!(f, "z-{}", s),
            ZIndex::Arbitrary(s) => s.write_class(f, "z-"),
        }
    }
}

impl TailwindInstance for TailwindZIndex {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let index = match &self.kind {
            ZIndex::Unit(n) => n.to_string(),
            ZIndex::Standard(s) => s.to_string(),
            ZIndex::Arbitrary(s) => s.get_properties(),
        };
        css_attributes! {
            "z-index" => index
        }
    }
}
impl TailwindZIndex {
    /// <https://tailwindcss.com/docs/z-index>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            [s] if Self::check_valid(s) => Ok(Self { kind: ZIndex::Standard(s.to_string()) }),
            [n] => {
                let mut i: i32 = TailwindArbitrary::from(*n).as_integer()?;
                if negative.unwrap() {
                    i = -i;
                }
                Ok(Self { kind: ZIndex::Unit(i) })
            },
            _ => syntax_error!("Unknown z-index instructions"),
        }
    }
    /// dispatch to [z-index](https://developer.mozilla.org/en-US/docs/Web/CSS/z-index)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: ZIndex::Arbitrary(arbitrary.to_owned()) })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/z-index#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["auto", "inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
