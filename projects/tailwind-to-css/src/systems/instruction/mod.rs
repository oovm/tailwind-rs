use super::*;
mod display;
mod methods;
mod resolver;

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
pub struct TailwindVariant {
    not: bool,
    pseudo: bool,
    names: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TailwindElements {
    inner: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TailwindArbitrary {
    inner: Option<String>,
}

/// https://github.com/tw-in-js/twind/blob/main/src/twind/variants.ts
#[derive(Copy, Clone, Debug)]
pub enum TailwindVariantKind {
    Dark,
    Sticky,
    MotionReduce,
    MotionSafe,
    First,
    Last,
    Even,
    Odd,
    Children,
    Siblings,
    Sibling,
    Override,
}
