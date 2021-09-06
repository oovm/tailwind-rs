use super::*;

#[derive(Debug, Copy, Clone)]
enum PlaceContent {
    Center,
    Start,
    End,
    Between,
    Around,
    Evenly,
    Stretch,
    Global(CssBehavior),
}

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindPlaceContent {
    kind: PlaceContent,
}

impl Display for PlaceContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlaceContent::Center => write!(f, "center"),
            PlaceContent::Start => write!(f, "start"),
            PlaceContent::End => write!(f, "end"),
            PlaceContent::Between => write!(f, "between"),
            PlaceContent::Around => write!(f, "around"),
            PlaceContent::Evenly => write!(f, "evenly"),
            PlaceContent::Stretch => write!(f, "stretch"),
            PlaceContent::Global(g) => write!(f, "{}", g),
        }
    }
}

impl Display for TailwindPlaceContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "place-content-{}", self.kind)
    }
}

impl PlaceContent {
    pub fn get_properties(&self) -> String {
        match self {
            PlaceContent::Center => format!("center"),
            PlaceContent::Start => format!("start"),
            PlaceContent::End => format!("end"),
            PlaceContent::Between => format!("space-between"),
            PlaceContent::Around => format!("space-around"),
            PlaceContent::Evenly => format!("space-evenly"),
            PlaceContent::Stretch => format!("stretch"),
            PlaceContent::Global(g) => format!("{}", g),
        }
    }
}

impl TailwindInstance for TailwindPlaceContent {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "place-content" => self.kind.get_properties()
        }
    }
}

impl TailwindPlaceContent {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after place-content");
        let kind = match pattern {
            ["center"] => PlaceContent::Center,
            ["start"] => PlaceContent::Start,
            ["end"] => PlaceContent::End,
            ["between"] => PlaceContent::Between,
            ["around"] => PlaceContent::Around,
            ["evenly"] => PlaceContent::Evenly,
            ["stretch"] => PlaceContent::Stretch,
            _ => return syntax_error!("Unknown sizing instructions: {}", pattern.join("-")),
        };
        Ok(Self { kind })
    }
}
