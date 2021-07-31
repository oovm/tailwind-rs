use super::*;
mod display;
mod methods;
mod resolver;

pub fn parse_tailwind(input: &str) -> Vec<TailwindInstruction> {
    todo!()
}

#[cfg(test)]
pub fn tw_idempotency(input1: &str, builder: &mut TailwindBuilder) {
    let input2 = &builder.trace(input1);
    assert_eq!(builder.inline(input1), builder.inline(input2))
}

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
