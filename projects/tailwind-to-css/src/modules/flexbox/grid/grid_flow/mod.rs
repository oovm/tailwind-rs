use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindGridFlow {
    kind: String,
}

impl<T> From<T> for TailwindGridFlow
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindGridFlow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("grid-flow-")?;
        let s = self.kind.as_str();
        match s {
            "column" => write!(f, "col"),
            "row dense" => write!(f, "row-dense"),
            "column dense" => write!(f, "col-dense"),
            _ => write!(f, "grid-flow-{}", s),
        }
    }
}

impl TailwindInstance for TailwindGridFlow {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "grid-auto-flow" => self.kind
        }
    }
}

impl TailwindGridFlow {
    /// https://tailwindcss.com/docs/grid-auto-flow
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after grid-flow");
        let kind = match pattern {
            ["col"] => Self::from("column"),
            ["col", "dense"] => Self::from("column dense"),
            ["row", "dense"] => Self::from("row dense"),
            _ => {
                let kind = pattern.join(" ");
                debug_assert!(Self::check_valid(&kind));
                Self { kind }
            },
        };
        Ok(kind)
    }
    /// https://developer.mozilla.org/zh-CN/docs/Web/CSS/grid-auto-flow
    pub fn check_valid(mode: &str) -> bool {
        let set =
            BTreeSet::from_iter(vec!["column", "column dense", "dense", "inherit", "initial", "row", "row dense", "unset"]);
        set.contains(mode)
    }
}
