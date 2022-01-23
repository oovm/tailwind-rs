use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindResize {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindResize => "resize");

impl Display for TailwindResize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_class(f, "resize-", |f, s| match s {
            "both" => write!(f, "resize"),
            "horizontal" => write!(f, "resize-x"),
            "vertical" => write!(f, "resize-y"),
            _ => Err(std::fmt::Error),
        })
    }
}

impl TailwindResize {
    /// https://tailwindcss.com/docs/user-select
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            [] if arbitrary.is_none() => StandardValue::from("both"),
            ["x"] => StandardValue::from("horizontal"),
            ["y"] => StandardValue::from("vertical"),
            _ => StandardValue::parser("resize", &Self::check_valid)(pattern, arbitrary)?,
        };
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/user-select#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "block",
            "both",
            "horizontal",
            "inherit",
            "initial",
            "inline",
            "none",
            "revert",
            "unset",
            "vertical",
        ]);
        set.contains(mode)
    }
}
