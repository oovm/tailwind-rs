use super::*;

macro_rules! error_wrap {
    ($t:ty => $name:ident) => {
        impl From<$t> for TailwindError {
            fn from(e: $t) -> Self {
                Self { kind: Box::new(TailwindErrorKind::$name(e)), file: None, range: None }
            }
        }
    };
    ($($t:ty => $name:ident),+ $(,)?) => (
        $(error_wrap!($t=>$name);)+
    );
}

error_wrap![
    std::io::Error  => IOError,
    std::fmt::Error => FormatError,
];

impl From<Infallible> for TailwindError {
    fn from(_: Infallible) -> Self {
        Self::unreachable()
    }
}

impl From<()> for TailwindError {
    fn from(_: ()) -> Self {
        Self::unreachable()
    }
}
