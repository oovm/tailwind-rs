use crate::JssError;
use pest::error::{Error, ErrorVariant};
use std::fmt::Debug;

impl<R> From<Error<R>> for JssError
where
    R: Debug,
{
    fn from(e: Error<R>) -> Self {
        let error = Self::from(e.variant);
        error
    }
}

impl<R> From<ErrorVariant<R>> for JssError
where
    R: Debug,
{
    fn from(e: ErrorVariant<R>) -> Self {
        let msg = match e {
            ErrorVariant::ParsingError { positives, negatives } => {
                format!("Positive attempts: {:?}\nNegative attempts: {:?}", positives, negatives)
            }
            ErrorVariant::CustomError { message } => message,
        };
        Self::syntax_error(msg)
    }
}
