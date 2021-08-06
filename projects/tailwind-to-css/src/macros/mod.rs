#[macro_export]
macro_rules! css_attributes {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$(CssAttribute::new($k.to_string(), $v.to_string()),)*]))
    }};
}

#[macro_export]
macro_rules! attributes_ensure {
    ($f:tt, $field:tt, $t:ty) => {
        #[inline]
        fn $f(&mut self) -> &mut $t {
            if self.$field.is_none() {
                self.$field = Some(Default::default())
            }
            unsafe { self.$field.as_mut().unwrap_unchecked() }
        }
    };
}

#[macro_export]
macro_rules! syntax_error {
    ($msg:literal $(,)?) => {
        Err(tailwind_error::TailwindError::syntax_error($msg.to_string()))
    };
    // ($err:expr $(,)?) => {
    //     Err(TailwindError::from($err))
    // };
    ($fmt:expr, $($arg:tt)*) => {
        Err(tailwind_error::TailwindError::syntax_error(format!($fmt, $($arg)*)))
    };
}
