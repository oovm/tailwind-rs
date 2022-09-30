use std::{
    borrow::Cow,
    convert::Infallible,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    ops::Range,
    path::PathBuf,
};

mod error_std;

/// All result about tailwind
pub type Result<T> = std::result::Result<T, TailwindError>;
/// Maybe have ast position
pub type MaybeRanged = Option<Range<usize>>;
/// Error type for all tailwind operators
#[derive(Debug)]
pub struct TailwindError {
    /// Actual error kind
    pub kind: Box<TailwindErrorKind>,
    /// File name where error occurred
    pub file: Option<PathBuf>,
    /// Range offset where error occurred
    pub range: Option<Range<usize>>,
}

/// Actual error data for the error
#[derive(Debug)]
pub enum TailwindErrorKind {
    /// The error type for I/O operations
    IOError(std::io::Error),
    /// The error type which is returned from formatting a message into a
    /// stream.
    FormatError(std::fmt::Error),
    /// The error type which is
    SyntaxError(String),
    /// The error type which is
    TypeMismatch(String),
    /// The error type which is occurred at runtime
    RuntimeError(String),
    /// Runtime error when variable is undefined
    UndefinedVariable {
        /// The name of the undefined variable
        name: String,
    },
    /// Parsing not complete
    Incomplete,
    /// A forbidden cst_node encountered
    Unreachable,
    // #[error(transparent)]
    // UnknownError(#[from] anyhow::Error),
}

impl TailwindError {
    /// Set a local path for the error
    #[inline]
    pub fn set_path(&mut self, url: &std::path::Path) -> Result<()> {
        self.file = Some(PathBuf::from(url));
        Ok(())
    }
    /// Set a new range for the error
    #[inline]
    pub fn set_range(&mut self, start: usize, end: usize) {
        self.range = Some(Range { start, end });
    }
    /// Constructor of [`NoteErrorKind::Incomplete`]
    #[inline]
    pub fn incomplete() -> Self {
        Self { kind: Box::new(TailwindErrorKind::Incomplete), file: None, range: None }
    }
    /// Constructor of [`NoteErrorKind::Unreachable`]
    #[inline]
    pub fn unreachable() -> Self {
        Self { kind: Box::new(TailwindErrorKind::Unreachable), file: None, range: None }
    }
    /// Constructor of [`NoteErrorKind::UndefinedVariable`]
    #[inline]
    pub fn undefined_variable(name: impl Into<String>) -> TailwindError {
        let kind = TailwindErrorKind::UndefinedVariable { name: name.into() };
        Self { kind: Box::new(kind), file: None, range: None }
    }
}

impl TailwindError {
    /// Deprecated or obsolete code.
    /// Clients are allowed to rendered diagnostics with this tag strike
    /// through.
    pub fn is_deprecated(&self) -> bool {
        false
    }
    /// Unused or unnecessary code.
    /// Clients are allowed to render diagnostics with this tag faded out
    /// instead of having an error squiggle.
    pub fn is_unnecessary(&self) -> bool {
        false
    }
}

macro_rules! error_msg {
    ($name:ident => $t:ident) => {
        /// Constructor of [`NoteErrorKind::$t`]
        pub fn $name(msg: impl Into<String>) -> TailwindError {
            let kind = TailwindErrorKind::$t(msg.into());
            Self { kind: Box::new(kind), file: None, range: None }
        }
    };
    ($($name:ident => $t:ident),+ $(,)?) => (
        impl TailwindError { $(error_msg!($name=>$t);)+ }
    );
}

error_msg![
    syntax_error => SyntaxError,
    type_mismatch => TypeMismatch,
    runtime_error => RuntimeError,
];

impl Error for TailwindError {}

impl Display for TailwindError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let path = match &self.file {
            Some(s) => s.to_string_lossy(),
            None => Cow::from("<Anonymous>"),
        };
        match &self.range {
            Some(s) => writeln!(f, "at ({}, {}) of {}", s.start, s.end, path)?,
            None => writeln!(f, "at {}", path)?,
        }
        write!(f, "{:indent$}{}", "", self.kind, indent = 4)
    }
}

impl Display for TailwindErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::IOError(e) => {
                write!(f, "{}", e)
            },
            Self::FormatError(e) => {
                write!(f, "{}", e)
            },
            Self::SyntaxError(msg) => {
                f.write_str("SyntaxError: ")?;
                f.write_str(msg)
            },
            Self::TypeMismatch(msg) => {
                f.write_str("TypeError: ")?;
                f.write_str(msg)
            },
            Self::RuntimeError(msg) => {
                f.write_str("RuntimeError: ")?;
                f.write_str(msg)
            },
            Self::UndefinedVariable { name } => {
                write!(f, "RuntimeError: Variable {} not found in scope", name)
            },
            Self::Incomplete => {
                f.write_str("InternalError: ")?;
                f.write_str("Parsing incomplete!")
            },
            Self::Unreachable => {
                f.write_str("InternalError: ")?;
                f.write_str("Entered unreachable code!")
            },
        }
    }
}
