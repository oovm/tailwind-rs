use super::*;

#[doc = include_str!("screen-reader.md")]
#[derive(Debug)]
pub struct TailwindScreenReader {
    inner: bool,
}

impl Display for TailwindScreenReader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.inner {
            f.write_str("not-")?
        }
        f.write_str("sr-only")
    }
}

impl TailwindInstance for TailwindScreenReader {
    fn selectors(&self, _: &TailwindBuilder) -> String {
        self.id()
    }
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        todo!()
    }
}

impl TailwindScreenReader {
    pub fn new(sr_only: bool) -> Self {
        match sr_only {
            true => Self { inner: true },
            false => Self { inner: false },
        }
    }
}
