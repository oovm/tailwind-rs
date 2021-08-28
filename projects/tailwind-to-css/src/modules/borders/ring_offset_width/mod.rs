use super::*;
use crate::LengthUnit;

///
#[derive(Copy, Clone, Debug)]
pub struct TailwindRingOffsetWidth {
    width: LengthUnit,
}

#[derive(Copy, Clone, Debug)]
enum RingOffsetWidth {
    Inset,
    Unit(usize),
}

impl Display for RingOffsetWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inset => write!(f, "inset"),
            Self::Unit(n) => write!(f, "{}", n),
        }
    }
}

impl Display for TailwindRingOffsetWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ring-offset-{}", self.kind)
    }
}

impl TailwindInstance for TailwindRingOffsetWidth {
    // --tw-ring-offset-width: 0px;
    // box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "--tw-ring-offset-width" => format!("{}px", self.kind)
        }
    }
}
impl TailwindRingOffsetWidth {
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after ring-width");
        let out = match input {
            [] => Self { kind: RingOffsetWidth::Unit(3) },
            ["inset"] => Self { kind: RingOffsetWidth::Inset },
            [n] => {
                let a = TailwindArbitrary::from(*n);
                Self { kind: RingOffsetWidth::Unit(a.as_integer()?) }
            }
            _ => return syntax_error!("Unknown ring-width instructions"),
        };
        Ok(out)
    }
}
