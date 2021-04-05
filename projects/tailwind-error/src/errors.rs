#[derive(Debug, Copy, Clone)]
pub enum Error {
    UnknownError
}

pub type Result<T> = std::result::Result<T, Error>;
