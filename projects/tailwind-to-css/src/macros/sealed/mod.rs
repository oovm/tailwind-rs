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

pub(crate) use keyword_instance;
