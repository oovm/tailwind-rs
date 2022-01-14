use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindOpacity {
    percent: NumericValue,
    backdrop: Backdrop,
}

impl Display for TailwindOpacity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.backdrop.write(f)?;
        write!(f, "opacity-{}", self.percent)
    }
}

impl TailwindInstance for TailwindOpacity {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let opacity = self.percent.get_properties(|f| format!("{}%", f));
        match self.backdrop.0 {
            true => css_attributes! {
                "backdrop-filter" => format!("opacity({})", opacity)
            },
            false => css_attributes! {
                "opacity" => opacity
            },
        }
    }
}

impl TailwindOpacity {
    /// <https://tailwindcss.com/docs/opacity>
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let percent = match backdrop {
            true => NumericValue::positive_parser("opacity", |_| false)(input, arbitrary)?,
            false => NumericValue::positive_parser("opacity", Self::check_valid)(input, arbitrary)?,
        };
        Ok(Self { percent, backdrop: Backdrop::from(backdrop) })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/opacity#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["always", "inherit", "initial", "normal", "unset"]);
        set.contains(mode)
    }
}
