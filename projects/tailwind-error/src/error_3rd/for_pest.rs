use crate::TailwindError;
use pest::error::{Error, ErrorVariant};
use std::fmt::Debug;
use diagnostic::ErrorWithFileSpan;

impl<R> From<Error<R>> for TailwindError
where
    R: Debug,
{
    fn from(e: Error<R>) -> Self {
        let error = Self::from(e.variant);
        error
    }
}

impl<R> From<ErrorVariant<R>> for TailwindError
where
    R: Debug,
{
    fn from(error: ErrorVariant<R>) -> Self {
        let msg = match error {
            ErrorVariant::ParsingError { positives, negatives } => {
                format!("Positive attempts: {:?}\nNegative attempts: {:?}", positives, negatives)
            },
            ErrorVariant::CustomError { message } => message,
        };
        let e = ErrorWithFileSpan { error: msg, file: Default::default(), span: Default::default() };
        TailwindError::syntax_error(e)
    }
}
