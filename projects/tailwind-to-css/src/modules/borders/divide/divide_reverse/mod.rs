use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindDivideReverse {
    axis: bool,
}

impl From<bool> for TailwindDivideReverse {
    fn from(axis: bool) -> Self {
        Self { axis }
    }
}

impl Display for TailwindDivideReverse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.axis {
            true => write!(f, "divide-x-reverse"),
            false => write!(f, "divide-y-reverse"),
        }
    }
}

impl TailwindInstance for TailwindDivideReverse {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let class = match self.axis {
            true => "--tw-divide-x-reverse",
            false => "--tw-divide-y-reverse",
        };
        css_attributes! {
            class => "1"
        }
    }
}
