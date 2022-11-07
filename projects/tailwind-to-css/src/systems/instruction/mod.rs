mod display;
mod methods;
mod resolver;
use crate::{TailwindBuilder, *};
use std::{
    fmt::{Debug, Display, Formatter},
};
use tailwind_ast::{AstStyle, TailwindArbitrary, TailwindVariant};

/// `v:v:-a-a-[A]`
#[derive(Debug, Clone)]
pub struct TailwindInstruction {
    negative: Negative,
    variants: Vec<TailwindVariant>,
    elements: TailwindElements,
    arbitrary: TailwindArbitrary,
}

/// <https://github.com/tw-in-js/twind/blob/main/src/twind/variants.ts>
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
