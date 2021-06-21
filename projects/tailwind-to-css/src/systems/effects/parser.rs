use super::*;
use tailwind_error::TailwindError;

impl TailwindOpacity {
    pub fn parse(input: &str) -> Box<dyn TailwindInstance> {
        todo!()
    }

    #[inline]
    pub fn number(input: usize) -> Box<dyn TailwindInstance> {
        Box::new(Self { opacity: input })
    }
}

impl TailwindBlendMode {}

impl FromStr for TailwindBlendMode {
    type Err = TailwindError;

    fn from_str(s: &str) -> Result<Self> {
        let out = match s {
            "normal" => Self::Normal,
            _ => return syntax_error!("mode error"),
        };
        Ok(out)
    }
}

impl TailwindBlend {
    #[inline]
    pub fn new(mode: &str, is_background: bool) -> Self {
        let mode = mode.parse().expect("???");
        Self { kind: TailwindBlendKind::new(is_background), mode }
    }
}

impl TailwindBlendKind {
    #[inline]
    pub fn new(is_background: bool) -> Self {
        match is_background {
            true => Self::Background,
            false => Self::Normal,
        }
    }
}

impl TailwindBlendMode {
    pub fn instance(self, is_background: bool) -> Box<dyn TailwindInstance> {
        TailwindBlend { kind: TailwindBlendKind::new(is_background), mode: self }.boxed()
    }
}

impl TailwindShadow {
    pub fn parse_box(input: &[&str], arbitrary: &str) -> Result<Self> {
        todo!()
    }
    pub fn parse_drop(input: &[&str], arbitrary: &str) -> Result<Self> {
        todo!()
    }
    pub fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        todo!()
    }
}
