use super::*;
use std::fmt::Write;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindTorch {
    kind: String,
}

impl TailwindTorch {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            [] => {
                debug_assert!(arbitrary.is_some(), "missing arbitrary after torch");
                arbitrary.to_string()
            },

            _ => {
                let input = pattern.join("-");
                debug_assert!(check_valid(&input), "missing arbitrary after torch");
                input
            },
        };
        Ok(Self { kind })
    }
}

impl Display for TailwindTorch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "touch-[{}]", self.kind)
    }
}

impl TailwindInstance for TailwindTorch {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "touch-action" => self.kind
        }
    }
}

#[cfg(debug_assertions)]
fn check_valid(mode: &str) -> bool {
    let set = BTreeSet::from_iter(vec![
        "auto",
        "none",
        "pan-x",
        "pan-y",
        "pan-left",
        "pan-right",
        "pan-up",
        "pan-down",
        "pinch-zoom",
        "manipulation",
        "inherit",
        "initial",
        "unset",
    ]);
    set.contains(mode)
}
