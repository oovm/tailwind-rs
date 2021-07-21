use super::*;
mod resolver;

/// `v:v:-a-a-[A]`
#[derive(Clone)]
pub struct TailwindInstruction {
    negative: bool,
    variants: Vec<TailwindVariant>,
    elements: Vec<String>,
    arbitrary: Option<TailwindArbitrary>,
}

#[derive(Debug, Clone)]
pub struct TailwindVariant {
    not: bool,
    pseudo: bool,
    names: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TailwindArbitrary {
    inner: String,
}

impl TailwindVariant {
    pub fn new(names: &[&str], not: bool, pseudo: bool) {
        Self {}
    }
}
