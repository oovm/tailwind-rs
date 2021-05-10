
use super::*;


impl TailwindObject {
    pub fn new(id: impl Into<String>, css: BTreeSet<CssAttribute>) -> Box<dyn TailwindInstance> {
        Box::new(Self { id: id.into(), attributes: css })
    }
}

impl Display for TailwindObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.write_css(f) {
            Ok(_) => Ok(()),
            Err(_) => Err(std::fmt::Error),
        }
    }
}

impl TailwindInstance for TailwindObject {
    fn id(&self) -> String {
        self.id.to_owned()
    }
    fn attributes(&self) -> BTreeSet<CssAttribute> {
        self.attributes.to_owned()
    }
}

impl TailwindObject {
    pub fn parser<'a>(id: &'static str, css: &'static str) -> impl Fn(&'a str) -> ParsedItem<'a> {
        move |input| match tag(id)(input) {
            Ok((rest, _)) => {
                let lines = css.trim().lines();
                let mut out = BTreeSet::default();
                for i in lines.map(|s| s.trim()) {
                    if let Some((key, value)) = i.split_once(":") {
                        out.insert(CssAttribute::new(key, value));
                    }
                }
                Ok((rest, Self::new(id, out)))
            }
            Err(e) => Err(e),
        }
    }
}
