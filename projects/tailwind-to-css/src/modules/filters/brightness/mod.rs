// https://tailwindcss.com/docs/background-origin
#[derive(Clone, Debug)]
pub struct TailwindBackgroundBrightness {
    brightness: TailwindBrightness,
}
impl TailwindBackgroundBrightness {
    #[inline]
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { brightness: TailwindBrightness::parse(rest, arbitrary, false)? })
    }
}

impl Display for TailwindBackgroundBrightness {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindBackgroundBrightness {}
