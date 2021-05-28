use super::*;
use tailwind_error::nom::{branch::alt, sequence::tuple};

impl TailwindBorderCollapse {
    pub fn into_instance(self) -> Box<dyn TailwindInstance> {
        Box::new(self)
    }
}

impl TailwindTableLayout {
    pub fn parse(input: &[&str]) -> Box<dyn TailwindInstance> {
        let out = match input {
            ["auto"] => Self::Auto,
            ["fixed"] => Self::Fixed,
            _ => {
                panic!("TODO")
            }
        };
        Box::new(out)
    }
}
