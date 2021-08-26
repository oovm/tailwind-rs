use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBackgroundBlend {
    wrapper: TailwindBlend,
}

impl Display for TailwindBackgroundBlend {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bg-blend-{}", self.wrapper.get_class())
    }
}

impl TailwindInstance for TailwindBackgroundBlend {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "background-blend-mode" => self.wrapper.get_properties()
        }
    }
}

impl TailwindBackgroundBlend {
    /// https://tailwindcss.com/docs/background-blend-mode
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let wrapper = TailwindBlend::parse(input, arbitrary)?;
        Ok(Self { wrapper })
    }
}
