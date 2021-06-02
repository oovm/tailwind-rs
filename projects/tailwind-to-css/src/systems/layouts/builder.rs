use super::*;
use crate::{parse_integer, syntax_error};
use tailwind_error::{
    nom::{
        character::streaming::digit1,
        combinator::{map_res, opt, recognize},
        sequence::tuple,
    },
    Result, TailwindError,
};

impl TailwindAspect {
    /// https://tailwindcss.com/docs/aspect-ratio
    pub fn parse(kind: &[&str]) -> Result<Self> {
        let name = match kind {
            [name] => *name,
            _ => return syntax_error!("unknown aspect-ratio elements"),
        };
        let out = match name {
            "auto" => Self { kind: "auto", ratio: "auto" },
            "square" => Self { kind: "square", ratio: "1/1" },
            "video" => Self { kind: "video", ratio: "16/9" },
            _ => return syntax_error!("unknown aspect-ratio elements"),
        };
        Ok(out)
    }
}

impl TailwindColumns {
    /// https://tailwindcss.com/docs/columns
    pub fn parse(kind: &[&str]) -> Result<Self> {
        let name = match kind {
            [name] => *name,
            _ => return syntax_error!("unknown column elements"),
        };
        let out = match name {
            "auto" => Self::Auto,
            "3xs" => Self::Size(16),
            "2xs" => Self::Size(18),
            "xs" => Self::Size(20),
            "sm" => Self::Size(24),
            "md" => Self::Size(28),
            "lg" => Self::Size(32),
            "xl" => Self::Size(36),
            "2xl" => Self::Size(42),
            "3xl" => Self::Size(48),
            "4xl" => Self::Size(56),
            "5xl" => Self::Size(64),
            "6xl" => Self::Size(72),
            "7xl" => Self::Size(80),
            _ => Self::Columns(parse_integer(name)?.1),
        };
        Ok(out)
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
