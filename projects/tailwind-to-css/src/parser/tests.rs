impl AstArbitrary {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, r) = delimited(char('['), take_till1(|c| c != ']'), char(']'))(input)?;
        Ok((rest, Self(r.to_string())))
    }
    pub fn parse_maybe(input: &str) -> IResult<&str, Option<Self>> {
        let (rest, rhs) = opt(tuple((tag("-"), Self::parse)))(input)?;
        Ok((rest, rhs.map(|(_, e)| e)))
    }
}

impl AstElement {
    fn parse(input: &str) -> IResult<&str, Self> {
        let stop = |c: char| -> bool { matches!(c, ':' | '|' | '[' | ']' | '(' | ')' | ' ') };
        let (rest, s) = take_till(stop)(input)?;
        Ok((rest, Self(s.to_string())))
    }
}

impl AstElement {
    #[inline]
    pub fn is_self_reference(&self) -> bool {
        self.0 == "&"
    }

    pub fn as_usize(&self) -> Option<usize> {
        parse_integer(&self.0).ok().map(|(_, o)| o)
    }
    pub fn as_fraction(&self) -> Option<(usize, usize)> {
        parse_fraction(&self.0).ok().map(|(_, o)| o)
    }
}

impl TailwindInstruction {
    /// `v:v::-a-a-a-[A]`
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let es = separated_list0(tag("-"), AstElement::parse);
        let (rest, rhs) = tuple((TailwindVariant::parse_many, opt(tag("-")), es, AstArbitrary::parse_maybe))(input)?;
        let (variants, neg, atoms, arbitrary) = rhs;
        Ok((rest, Self { negative: neg.is_some(), variants, elements: atoms, arbitrary }))
    }
    pub fn parse_list(input: &str) -> IResult<&str, Vec<Self>> {
        separated_list0(multispace1, Self::parse)(input)
    }
}

impl TailwindInstruction {
    #[inline]
    pub fn view_elements(&self) -> Vec<&str> {
        self.elements.iter().map(|s| s.0.as_str()).collect()
    }
    #[inline]
    pub fn view_arbitrary(&self) -> &str {
        match &self.arbitrary {
            None => "",
            Some(setter) => setter.0.as_str(),
        }
    }
    // TODO
    pub fn normalization(self) -> Self {
        self
    }
}

impl AstGroup {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((Self::parse_grouped, Self::parse_standalone))(input)
    }
    pub fn parse_list(input: &str) -> IResult<&str, Vec<Self>> {
        separated_list0(multispace1, Self::parse)(input)
    }
    #[inline]
    fn parse_standalone(input: &str) -> IResult<&str, Self> {
        let (rest, inner) = TailwindInstruction::parse(input)?;
        Ok((rest, Self::Standalone { inner }))
    }
    #[inline]
    fn parse_grouped(input: &str) -> IResult<&str, Self> {
        let lhs = tuple((TailwindVariant::parse_many, opt(AstElement::parse)));
        let rhs = delimited(char('('), TailwindInstruction::parse_list, char(')'));
        let (rest, ((variants, elements), inner)) = tuple((lhs, rhs))(input)?;
        Ok((rest, Self::Grouped { variants, elements, inner }))
    }

    pub fn expand(s: Self, buffer: &mut Vec<TailwindInstruction>) {
        match s {
            Self::Standalone { inner } => buffer.push(inner),
            Self::Grouped { variants: vl, elements: el, inner } => {
                for TailwindInstruction { negative, variants: vr, elements: er, arbitrary } in inner {
                    let mut variants = vl.clone();
                    variants.extend(vr.into_iter());
                    let mut elements = match el.clone() {
                        None => vec![],
                        Some(s) => vec![s],
                    };
                    elements.extend(er.into_iter());
                    let new = TailwindInstruction { negative, variants, elements, arbitrary };
                    buffer.push(new);
                }
            }
        }
    }
    pub fn expand_list(v: Vec<Self>) -> Vec<TailwindInstruction> {
        let mut out = vec![];
        for i in v {
            Self::expand(i, &mut out)
        }
        out
    }
}

impl TailwindBuilder {
    pub fn parse_styles(input: &str) -> Result<Vec<TailwindInstruction>> {
        let g = AstGroup::parse_list(input.trim())?.1;
        Ok(AstGroup::expand_list(g))
    }
}

#[test]
fn test_style() {
    // w-full sm:w-auto text-lg uppercase text-gray-100 bg-purple-800 hover:bg-purple-700 focus:bg-purple-700 focus-visible:ring-4 ring-purple-400 px-6
    println!("{:#?}", TailwindInstruction::parse("not-hover:sm:text-red-200").unwrap().1);
    println!("{:#?}", TailwindInstruction::parse_list("w-full sm:w-auto").unwrap().1);
}

#[test]
fn test_group() {
    println!("{:#?}", AstGroup::parse_list("w(full sm:auto)").unwrap().1);
    println!("{:#?}", AstGroup::parse_list("not-hover:sm:text-red-200").unwrap().1);
}

#[test]
fn test_group_expand() {
    println!("{:#?}", TailwindBuilder::parse_styles("w(full sm:auto)"));
}
