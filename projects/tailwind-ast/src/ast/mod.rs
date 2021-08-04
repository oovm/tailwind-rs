mod display;
mod methods;
mod parse;
#[cfg(test)]
mod tests;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    character::complete::{alphanumeric1, char, multispace1},
    combinator::opt,
    error::Error,
    multi::{many0, separated_list0},
    sequence::{delimited, tuple},
    Err, IResult,
};
use std::{
    fmt::{Display, Formatter},
    ops::{Add, AddAssign},
};

///
pub fn parse_tailwind(input: &str) -> Result<Vec<AstStyle>, Err<Error<&str>>> {
    let rest = many0(tuple((multispace1, AstGroupItem::parse)));
    let (head, groups) = match tuple((AstGroupItem::parse, rest))(input.trim()) {
        Ok(o) => o.1,
        Err(e) => return Err(e),
    };
    let mut out = vec![];
    head.expand(&mut out);
    for (_, g) in groups {
        g.expand(&mut out)
    }
    Ok(out)
}

/// `variant:ast-style(grouped)`
#[derive(Clone, Debug, PartialEq)]
pub struct AstGroup<'a> {
    ///
    pub head: AstStyle<'a>,
    ///
    pub children: Vec<AstGroupItem<'a>>,
}

/// one of [`AstGroup`] and [`AstStyle`]
#[derive(Clone, Debug, PartialEq)]
pub enum AstGroupItem<'a> {
    ///
    Grouped(AstGroup<'a>),
    ///
    Styled(AstStyle<'a>),
    // SelfReference(AstReference),
}

/// `not-variant:pseudo::-ast-element-[arbitrary]`
#[derive(Clone, Debug, PartialEq)]
pub struct AstStyle<'a> {
    ///
    pub negative: bool,
    ///
    pub variants: Vec<ASTVariant<'a>>,
    ///
    pub elements: Vec<&'a str>,
    ///
    pub arbitrary: Option<&'a str>,
}

/// `-[.+]`
#[derive(Clone, Debug, PartialEq)]
pub struct AstArbitrary<'a> {
    /// `[.]`
    pub arbitrary: &'a str,
}

/// `ast-elements`
#[derive(Clone, Debug, PartialEq, Default)]
pub struct AstElements<'a> {
    /// `name-space`
    pub elements: Vec<&'a str>,
}

/// `&`
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AstReference {}

/// `(not-)?variant:pseudo::`
#[derive(Clone, Debug, PartialEq)]
pub struct ASTVariant<'a> {
    /// `not-`
    pub not: bool,
    /// `::`
    pub pseudo: bool,
    /// `name-space`
    pub names: Vec<&'a str>,
}
