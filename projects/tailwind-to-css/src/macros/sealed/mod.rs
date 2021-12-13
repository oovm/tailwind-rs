macro_rules! keyword_instance {
    ($t:ty => $a:literal) => {
        impl<T> From<T> for $t
        where
            T: Into<String>,
        {
            fn from(input: T) -> Self {
                Self { kind: KeywordOnly::from(input) }
            }
        }
        impl TailwindInstance for $t {
            fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
                css_attributes! {
                    $a => self.kind.get_properties()
                }
            }
        }
    };
}

macro_rules! color_instance {
    ($t:ty) => {
        impl From<TailwindColor> for $t {
            fn from(color: TailwindColor) -> Self {
                Self { color }
            }
        }
        impl $t {
            ///
            pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
                Ok(Self { color: TailwindColor::parse(input, arbitrary)? })
            }
            ///
            pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
                Ok(Self { color: TailwindColor::parse_arbitrary(arbitrary)? })
            }
        }
    };
}

pub(crate) use color_instance;
pub(crate) use keyword_instance;
