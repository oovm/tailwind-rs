use super::*;

pub struct Inlined {
    pub class: String,
    pub style: String,
}

impl Parse for Inlined {
    fn parse(input: ParseStream) -> Result<Self> {
        let query: LitStr = input.parse()?;
        let string = query.value();
        let mut parser = TailwindBuilder::default();
        let (class, style) = parser.inline(&string).unwrap();
        Ok(Inlined { class, style })
    }
}
