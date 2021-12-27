macro_rules! keyword_instance {
    ($t:ty => $a:literal) => {
        impl<T> From<T> for $t
        where
            T: Into<String>,
        {
            fn from(input: T) -> Self {
                Self { kind: StandardValue::from(input.into()) }
            }
        }
        impl TailwindInstance for $t {
            fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
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
macro_rules! css_insert {
    ($css:expr, "transform", $v:expr) => {
        $css.transform($v.to_string());
    };
    ($css:expr, $k:expr,  $v:expr) => {
        $css.insert($k.to_string(), $v.to_string());
    };
}

pub(crate) use color_instance;
pub(crate) use css_insert;
pub(crate) use keyword_instance;
