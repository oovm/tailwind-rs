use crate::JssError;
use num::{bigint::TryFromBigIntError, BigInt};
use rust_decimal::Error;

impl From<TryFromBigIntError<BigInt>> for JssError {
    fn from(e: TryFromBigIntError<BigInt>) -> Self {
        Self::runtime_error(format!("{}", e))
    }
}

impl From<Error> for JssError {
    fn from(e: Error) -> Self {
        Self::runtime_error(format!("{}", e))
    }
}
