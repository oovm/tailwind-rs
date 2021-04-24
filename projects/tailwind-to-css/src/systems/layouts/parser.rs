use super::*;

impl TailwindAspect {
    #[inline]
    pub fn new(kind: &'static str, ratio: &'static str) -> Box<dyn TailwindInstance> {
        box Self { kind, ratio }
    }
    pub fn parser<'a>(kind: &'static str, ratio: &'static str) -> TailwindParser<'a> {
        let id = format!("aspect-{}", kind);
        move |input| match tag(id)(input) {
            Ok(o) => Ok((o.0, Box::new(Self::new(kind, ratio)))),
            Err(e) => Err(e),
        }
    }
}

impl TailwindBreak {
    pub fn parser_before<'a>(kind: &'static str) -> TailwindParser<'a> {
        let id = format!("break-before-{}", kind);
        move |input| match tag(id)(input) {
            Ok(o) => Ok((o.0, Box::new(Self::Before(&id)))),
            Err(e) => Err(e),
        }
    }
    pub fn parser_after<'a>(kind: &'static str) -> TailwindParser<'a> {
        let id = format!("break-after-{}", kind);
        move |input| match tag(id)(input) {
            Ok(o) => Ok((o.0, Box::new(Self::After(&id)))),
            Err(e) => Err(e),
        }
    }
    pub fn parser_inside<'a>(kind: &'static str) -> TailwindParser<'a> {
        let id = format!("break-inside-{}", kind);
        move |input| match tag(id)(input) {
            Ok(o) => Ok((o.0, Box::new(Self::Inside(&id)))),
            Err(e) => Err(e),
        }
    }
}
