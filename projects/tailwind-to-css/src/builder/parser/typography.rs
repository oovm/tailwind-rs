use super::*;

fn antialias(input: &str) -> IResult<&str, Box<dyn TailwindInstance>> {
    match tag("antialiased")(input) {
        Ok(o) => Ok((o.0, Box::new(FontSmoothing::Normal))),
        Err(e) => Err(e),
    }
}

fn subpixel(input: &str) -> IResult<&str, Box<dyn TailwindInstance>> {
    match tag("subpixel-antialiased")(input) {
        Ok(o) => Ok((o.0, Box::new(FontSmoothing::Subpixel))),
        Err(e) => Err(e),
    }
}
