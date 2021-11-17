use super::*;

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindDivideWidth {
    axis: bool,
    width: usize,
}

impl Display for TailwindDivideWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.axis {
            true => write!(f, "divide-x-{}", self.width),
            false => write!(f, "divide-y-{}", self.width),
        }
    }
}

impl TailwindInstance for TailwindDivideWidth {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        match self.axis {
            true => css_attributes! {
                "border-right-width" => format!("{}px", self.width),
                "border-left-width" => "0"
            },
            false => css_attributes! {
                "border-top-width" => "0",
                "border-bottom-width" => format!("{}px", self.width)
            },
        }
    }
}

impl TailwindDivideWidth {
    /// https://tailwindcss.com/docs/divide-width
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary, axis: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after divide-width");
        let out = match input {
            [] => Self { axis, width: 1 },
            [n] => {
                let a = TailwindArbitrary::from(*n);
                Self { axis, width: a.as_integer()? }
            },
            _ => return syntax_error!("Unknown divide-width instructions"),
        };
        Ok(out)
    }
}
