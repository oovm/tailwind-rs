use crate::{MaybeRanged, TailwindError};
use rsass::{Error, ParseError, SourcePos};
use url::Url;

impl From<Error> for TailwindError {
    fn from(e: Error) -> Self {
        match e {
            Error::Input(path, io) => {
                let mut error = Self::from(io);
                if let Ok(o) = Url::from_file_path(path) {
                    error.file = Some(o)
                }
                error
            },
            Error::IoError(io) => Self::from(io),
            Error::BadCall(_, _, _) => {
                unimplemented!()
            },
            Error::InvalidFunctionName(r) => {
                // TODO: name
                let mut error = Self::runtime_error(&format!("Invalid function name"));
                error.range = get_range(&r);
                error
            },
            Error::BadValue(_) => {
                unimplemented!()
            },
            Error::BadArgument(_, _) => {
                unimplemented!()
            },
            Error::BadArguments(_, _) => {
                unimplemented!()
            },
            Error::BadRange(_) => {
                unimplemented!()
            },
            Error::ParseError(e) => Self::from(e),
            Error::UndefinedVariable(name) => Self::undefined_variable(name),
            Error::AtError(_, _) => {
                unimplemented!()
            },
            Error::S(s) => Self::runtime_error(s),
        }
    }
}

impl From<ParseError> for TailwindError {
    fn from(e: ParseError) -> Self {
        let error = Self::runtime_error(e.to_string());
        error
    }
}

fn get_range(_: &SourcePos) -> MaybeRanged {
    None
}
