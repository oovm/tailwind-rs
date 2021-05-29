use super::*;
use crate::TailwindInstance;

impl TailwindOpacity {
    pub fn parse(input: &str) -> Box<dyn TailwindInstance> {
        todo!()
    }

    #[inline]
    pub fn number(input: usize) -> Box<dyn TailwindInstance> {
        Box::new(Self { opacity: input })
    }
}

impl TailwindBlend {
    pub fn parse(mode: &str, is_background: bool) -> Box<dyn TailwindInstance> {
        let mode = TailwindBlendMode::new(mode).expect("???");
        let out = Self { kind: TailwindBlendKind::new(is_background), mode };
        Box::new(out)
    }
}

impl TailwindBlendKind {
    pub fn new(is_background: bool) -> Self {
        match is_background {
            true => Self::Background,
            false => Self::Normal,
        }
    }
}

impl TailwindBlendMode {
    pub fn instance(self, is_background: bool) -> Box<dyn TailwindInstance> {
        box TailwindBlend { kind: TailwindBlendKind::new(is_background), mode: self }
    }
}
