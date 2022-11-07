use super::*;

mod traits;

/// Used to represent those attributes that only have keywords
#[derive(Debug, Clone)]
pub enum StandardValue {
    Keyword(String),
    Arbitrary(TailwindArbitrary),
}

pub type KeywordMap = &'static [(&'static str, Option<&'static str>)];

#[derive(Default, Debug, Clone)]
pub struct KeywordInstance {
    pub pattern: String,
    pub display: String,
    pub value: String,
    pub arbitrary: TailwindArbitrary,
}

impl KeywordInstance {
    pub fn parse<'i>(
        mut self,
        id: &'static str,
        pattern: &'i [&'i str],
        keyword_map: KeywordMap,
        arbitrary: &'i TailwindArbitrary,
    ) -> Result<()> {
        let value = match pattern {
            [] => StandardValue::Arbitrary(TailwindArbitrary::from(arbitrary)),
            _ => {
                let keyword = pattern.join("-");

                if cfg!(compile_time) {
                    for (name, display) in keyword_map {
                        if keyword == name {
                            self.display
                        }
                    }
                    return syntax_error!("{} does not a valid value of {}", keyword, id);
                }
                StandardValue::Keyword(keyword)
            },
        };

        Ok(Box::new(self))
    }
}

impl StandardValue {
    pub fn parser(id: &'static str, check_valid: &'static impl Fn(&str) -> bool) -> impl Fn(&[&str], &TailwindArbitrary) -> Result<Self> {
        move |pattern: &[&str], arbitrary: &TailwindArbitrary| match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            _ => Self::parse_keyword(pattern, id, check_valid),
        }
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary(TailwindArbitrary::from(arbitrary)))
    }
    pub fn parse_keyword(pattern: &[&str], id: &str, checker: &'static impl Fn(&str) -> bool) -> Result<Self> {
        let keyword = pattern.join("-");
        if cfg!(compile_time) && !checker(&keyword) {
            return syntax_error!("{} does not a valid value of {}", keyword, id);
        }
        Ok(Self::Keyword(keyword))
    }
    pub fn get_properties(&self) -> &str {
        match self {
            Self::Keyword(s) => s.as_str(),
            Self::Arbitrary(s) => s.as_str(),
        }
    }
    pub fn get_value(&self) -> &str {
        match self {
            Self::Keyword(s) => s.as_str(),
            Self::Arbitrary(s) => s.as_str(),
        }
    }
    pub fn write_class(&self, fmt: &mut Formatter, class: &str, special: fn(&mut Formatter, &str) -> std::fmt::Result) -> std::fmt::Result {
        match self {
            StandardValue::Keyword(s) => match special(fmt, s) {
                Ok(o) => Ok(o),
                Err(_) => write!(fmt, "{}", class),
            },
            StandardValue::Arbitrary(s) => s.write_class(fmt, class),
        }
    }
}
