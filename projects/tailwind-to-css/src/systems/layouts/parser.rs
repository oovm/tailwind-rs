use super::*;
use crate::parse_integer;
use tailwind_error::{
    nom::{
        branch::alt,
        character::streaming::digit1,
        combinator::{map_res, opt, recognize},
        sequence::tuple,
    },
    Result,
};

impl TailwindAspect {
    /// https://tailwindcss.com/docs/aspect-ratio
    pub fn parse<'a>(kind: &[&str]) -> Box<dyn TailwindInstance> {
        let name = match kind {
            [name] => *name,
            _ => panic!("unknown aspect-ratio elements"),
        };
        match name {
            "auto" => Self::instance("auto", "auto"),
            "square" => Self::instance("square", "1 / 1"),
            "video" => Self::instance("video", "16 / 9"),
            _ => {
                panic!("unknown aspect-ratio")
            }
        }
    }
    #[inline]
    fn instance(kind: &'static str, ratio: &'static str) -> Box<dyn TailwindInstance> {
        Box::new(Self { kind, ratio })
    }
}

impl TailwindBreak {
    /// https://tailwindcss.com/docs/break-before
    pub fn parse_before<'a>(kind: &[&str]) -> Box<dyn TailwindInstance> {
        match kind.len() {
            1 | 2 => {}
            r => panic!("break-before expected 1 or 2 elements but found {} elements", r),
        }
        let instance = match kind {
            ["auto"] => Self::Before("auto"),
            ["avoid"] => Self::Before("avoid"),
            ["all"] => Self::Before("all"),
            ["avoid", "page"] => Self::Before("avoid-page"),
            ["page"] => Self::Before("page"),
            ["left"] => Self::Before("left"),
            ["right"] => Self::Before("right"),
            ["column"] => Self::Before("column"),
            _ => {
                panic!("unknown aspect-ratio")
            }
        };
        Box::new(instance)
    }
    /// https://tailwindcss.com/docs/break-after
    pub fn parse_after<'a>(kind: &[&str]) -> Box<dyn TailwindInstance> {
        match kind.len() {
            1 | 2 => {}
            r => panic!("break-after expected 1 or 2 elements but found {} elements", r),
        }
        let instance = match kind {
            ["auto"] => Self::After("auto"),
            ["avoid"] => Self::After("avoid"),
            ["all"] => Self::After("all"),
            ["avoid", "page"] => Self::After("avoid-page"),
            ["page"] => Self::After("page"),
            ["left"] => Self::After("left"),
            ["right"] => Self::After("right"),
            ["column"] => Self::After("column"),
            _ => {
                panic!("unknown aspect-ratio")
            }
        };
        Box::new(instance)
    }
    /// https://tailwindcss.com/docs/break-inside
    pub fn parse_inside<'a>(kind: &[&str]) -> Box<dyn TailwindInstance> {
        match kind.len() {
            1 | 2 => {}
            r => panic!("break-inside expected 1 or 2 elements but found {} elements", r),
        }
        let instance = match kind {
            ["auto"] => Self::Inside("auto"),
            ["avoid"] => Self::Inside("avoid"),
            ["avoid", "page"] => Self::Inside("avoid-page"),
            ["avoid", "column"] => Self::Inside("avoid-column"),
            _ => {
                panic!("unknown aspect-ratio")
            }
        };
        Box::new(instance)
    }
}

impl TailWindZIndex {
    pub fn parse(kind: &[&str], neg: bool) -> Box<dyn TailwindInstance> {
        match kind.len() {
            1 => {}
            r => panic!("break-inside expected 1 element but found {} elements", r),
        }
        let instance = match kind {
            ["auto"] => Self::Auto,
            [r] => Self::parse_number(r, neg).expect("not number"),
            _ => {
                panic!("Unknown aspect-ratio pattern")
            }
        };
        Box::new(instance)
    }
    #[inline]
    fn parse_number(input: &str, neg: bool) -> Result<Self> {
        let n = parse_integer(input)?.1;
        match neg {
            true => Ok(Self::Negative(n)),
            false => Ok(Self::Positive(n)),
        }
    }
}
