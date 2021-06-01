use super::*;
use crate::TailwindInstance;
use std::str::FromStr;

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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out = match s {
            "normal" => Self::Normal,
            _ => return Err(()),
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
