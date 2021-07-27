use super::*;
mod display;
mod methods;
mod resolver;

/// `v:v:-a-a-[A]`
#[derive(Debug, Clone)]
pub struct TailwindInstruction {
    negative: bool,
    variants: Vec<TailwindVariant>,
    elements: TailwindElements,
    arbitrary: TailwindArbitrary,
}

#[derive(Debug, Clone)]
pub(crate) struct TailwindVariant {
    pub(crate) not: bool,
    pub(crate) pseudo: bool,
    pub(crate) names: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct TailwindElements {
    pub(crate) inner: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct TailwindArbitrary {
    pub(crate) inner: Option<String>,
}
