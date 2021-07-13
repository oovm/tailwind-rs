use super::*;

impl TailwindBlur {
    pub fn parse(rest: &[&str], arbitrary: &str, backdrop: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_empty(), "forbidden arbitrary in blur");
        let px = match rest {
            ["none"] => 0,
            ["sm"] => 4,
            ["base"] => 8,
            [] if arbitrary.is_empty() => 8,
            ["md"] => 12,
            ["lg"] => 16,
            ["xl"] => 24,
            ["2xl"] => 40,
            ["3xl"] => 64,
            [n] => parse_i_px_maybe(n)?.1,
            _ => return syntax_error!("Unknown blur instructions"),
        };
        Ok(Self { px, backdrop })
    }
}

impl TailwindBrightness {
    pub fn parse(rest: &[&str], arbitrary: &str, backdrop: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_empty(), "forbidden arbitrary in brightness");
        match rest {
            [n] => Ok(Self { percent: parse_integer(n)?.1, backdrop }),
            _ => syntax_error!("Unknown brightness instructions"),
        }
    }
}

impl TailwindContrast {
    pub fn parse(rest: &[&str], arbitrary: &str, backdrop: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_empty(), "forbidden arbitrary in contrast");
        match rest {
            [n] => Ok(Self { percent: parse_integer(n)?.1, backdrop }),
            _ => syntax_error!("Unknown contrast instructions"),
        }
    }
}

impl TailwindGrayscale {
    pub fn parse(rest: &[&str], arbitrary: &str, backdrop: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_empty(), "forbidden arbitrary in grayscale");
        match rest {
            [n] => Ok(Self { percent: parse_integer(n)?.1, backdrop }),
            _ => syntax_error!("Unknown grayscale instructions"),
        }
    }
}
