use super::*;
mod display;
mod methods;
mod resolver;

/// `v:v:-a-a-[A]`
#[derive(Debug, Clone)]
pub struct TailwindInstruction {
    negative: bool,
    variants: Vec<TailwindVariant>,
    elements: Vec<String>,
    arbitrary: TailwindArbitrary,
}

#[derive(Debug, Clone)]
pub struct TailwindVariant {
    not: bool,
    pseudo: bool,
    names: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TailwindArbitrary {
    inner: Option<String>,
}
