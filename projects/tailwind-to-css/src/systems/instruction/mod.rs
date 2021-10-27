mod arbitrary;
mod display;
mod methods;
mod resolver;
pub use self::arbitrary::TailwindArbitrary;
use crate::{TailwindBuilder, *};
use css_color::Srgb;
use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter, Write},
    str::FromStr,
};
use tailwind_ast::{parse_f32, parse_fraction, parse_integer, ASTVariant, AstStyle};

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
