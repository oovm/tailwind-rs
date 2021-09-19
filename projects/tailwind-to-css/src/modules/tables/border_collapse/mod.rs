use super::*;

#[derive(Copy, Clone, Debug)]
enum BorderCollapse {
    Collapse,
    Separate,
}

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindBorderCollapse {
    kind: BorderCollapse,
}

impl Display for TailwindBorderCollapse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Collapse => "border-collapse",
            Self::Separate => "border-separate",
        };
        f.write_str(text)
    }
}

impl TailwindInstance for TailwindBorderCollapse {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let collapse = match self {
            Self::Collapse => "collapse",
            Self::Separate => "separate",
        };
        css_attributes! {
            "border-collapse" => collapse
        }
    }
}

impl TailwindBorderCollapse {
    pub fn into_instance(self) -> Box<dyn TailwindInstance> {
        Box::new(self)
    }
}
