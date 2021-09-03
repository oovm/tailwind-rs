mod animate;
mod delay;
mod display;
mod duration;
mod ease;
mod parser;
mod trans;
use super::*;

#[doc = include_str!("scale.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindTransition {}

#[doc = include_str!("scale.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindDuration {
    ms: usize,
}

#[doc = include_str!("scale.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindEase {}

#[doc = include_str!("scale.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindDelay {
    ms: usize,
}

#[doc = include_str!("scale.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindAnimate {}
