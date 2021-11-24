use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBackgroundAttachment {
    kind: String,
}

impl<T> From<T> for TailwindBackgroundAttachment
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindBackgroundAttachment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bg-")?;
        let s = self.kind.as_str();
        match s {
            s @ ("fixed" | "local" | "scroll") => write!(f, "{}", s),
            _ => write!(f, "attach-{}", s),
        }
    }
}

impl TailwindInstance for TailwindBackgroundAttachment {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "background-attachment" => self.kind
        }
    }
}

impl TailwindBackgroundAttachment {
    /// <https://tailwindcss.com/docs/background-attachment>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: NeverArbitrary::parser("bg-attach", &check_valid)(pattern, arbitrary)? })
    }
}

/// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-attachment#syntax>
fn check_valid(mode: &str) -> bool {
    let set = BTreeSet::from_iter(vec![
        // Keyword values
        "auto",
        "avoid",
        // Page break values
        "avoid-page",
        // Column break values
        "avoid-column",
        // Region break values
        "avoid-region",
        // Global values
        "inherit",
        "initial",
        "revert",
        "unset",
    ]);
    set.contains(mode)
}
