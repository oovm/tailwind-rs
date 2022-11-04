use std::{
    convert::Infallible,
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use diagnostic::{DiagnosticLevel, ErrorWithFile, ErrorWithFileSpan};

mod error_std;

/// All result about tailwind
pub type Result<T> = std::result::Result<T, TailwindError>;

/// Error type for all tailwind operators
#[derive(Debug)]
pub struct TailwindError {
    /// Actual error kind
    kind: Box<TailwindErrorKind>,
    level: DiagnosticLevel,
}

/// Actual error data for the error
#[derive(Debug)]
pub enum TailwindErrorKind {
    /// The error type for I/O operations
    IOError(ErrorWithFile<String>),
    /// The error type which is returned from formatting a message into a
    /// stream.
    FormatError(std::fmt::Error),
    /// The error type which is
    SyntaxError(ErrorWithFileSpan<String>),
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
    /// Get error kind of this error
    pub fn kind(&self) -> &TailwindErrorKind {
        &*self.kind
    }
    pub fn level(&self) -> DiagnosticLevel {
        self.level
    }
    /// Constructor of [`TailwindErrorKind::Incomplete`]
    #[inline]
    pub fn incomplete() -> Self {
        Self { kind: Box::new(TailwindErrorKind::Incomplete), level: DiagnosticLevel::Error }
    }
    /// Constructor of [`NoteErrorKind::Unreachable`]
    #[inline]
    pub fn unreachable() -> Self {
        Self { kind: Box::new(TailwindErrorKind::Unreachable), level: DiagnosticLevel::Error }
    }
    /// Constructor of [`NoteErrorKind::UndefinedVariable`]
    #[inline]
    pub fn undefined_variable(name: impl Into<String>) -> TailwindError {
        let kind = TailwindErrorKind::UndefinedVariable { name: name.into() };
        Self { kind: Box::new(kind), level: DiagnosticLevel::Error }
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

impl TailwindError {
    /// Constructor of [`NoteErrorKind::SyntaxError`]
    pub fn syntax_error(msg: impl Into<String>) -> TailwindError {
        let kind = TailwindErrorKind::SyntaxError(msg.into());
        Self { kind: Box::new(kind), level: Default::default() }
    }
    /// Constructor of [`NoteErrorKind::$t`]
    pub fn type_mismatch(msg: impl Into<String>) -> TailwindError {
        let kind = TailwindErrorKind::TypeMismatch(msg.into());
        Self { kind: Box::new(kind) }
    }
    /// Constructor of [`NoteErrorKind::$t`]
    pub fn runtime_error(msg: impl Into<String>) -> TailwindError {
        let kind = TailwindErrorKind::RuntimeError(msg.into());
        Self { kind: Box::new(kind) }
    }
}

impl Error for TailwindError {}

impl Display for TailwindError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Display for TailwindErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
