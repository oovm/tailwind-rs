use super::*;
use tailwind_error::TailwindError;

impl TailwindOpacity {
    /// https://tailwindcss.com/docs/opacity
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after duration");
        match input {
            [n] => Ok(Self { opacity: parse_usize(n)? }),
            _ => syntax_error!("Unknown opacity instructions: {}", input.join("-")),
        }
    }
}

impl TailwindBlendMode {}

impl FromStr for TailwindBlendMode {
    type Err = TailwindError;

    fn from_str(s: &str) -> Result<Self> {
        let out = match s {
            "normal" => Self::Normal,
            _ => return syntax_error!("mode error"),
        };
        Ok(out)
    }
}

impl TailwindBlend {
    #[inline]
    pub fn new(mode: &str, is_background: bool) -> Self {
        let mode = mode.parse().expect("???");
        Self { kind: TailwindBlendKind::new(is_background), mode }
    }
}

impl TailwindBlendKind {
    #[inline]
    pub fn new(is_background: bool) -> Self {
        match is_background {
            true => Self::Background,
            false => Self::Normal,
        }
    }
}

impl TailwindBlendMode {
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindShadow {
    pub fn parse_box(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
    pub fn parse_drop(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

#[inline(always)]
fn parse_usize(deg: &str) -> Result<usize> {
    Ok(parse_integer(deg)?.1)
}
