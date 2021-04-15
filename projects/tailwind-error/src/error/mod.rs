mod error_std;
use std::{
    convert::Infallible,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    ops::Range,
};
use url::Url;
use yggdrasil_shared::DiagnosticLevel;

/// All result about notedown
pub type Result<T> = std::result::Result<T, JssError>;
/// Maybe have ast position
pub type MaybeRanged = Option<Range<usize>>;
/// Error type for all Notedown operators
#[derive(Debug)]
pub struct JssError {
    /// Actual error kind
    pub kind: Box<JssErrorKind>,
    /// Error level for report
    pub level: DiagnosticLevel,
    /// File name where error occurred
    pub file: Option<Url>,
    /// Range offset where error occurred
    pub range: Option<Range<usize>>,
}

/// Actual error data for the error
#[derive(Debug)]
pub enum JssErrorKind {
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
    /// A forbidden cst_node encountered
    Unreachable,
    // #[error(transparent)]
    // UnknownError(#[from] anyhow::Error),
}

impl JssError {
    /// Set a new url for the error
    #[inline]
    pub fn set_url(&mut self, url: Url) {
        self.file = Some(url);
    }
    /// Set a local path for the error
    #[inline]
    #[cfg(any(unix, windows, target_os = "redox"))]
    pub fn set_path(&mut self, url: &std::path::Path) -> Result<()> {
        self.file = Some(Url::from_file_path(url)?);
        Ok(())
    }
    /// Set a new range for the error
    #[inline]
    pub fn set_range(&mut self, start: usize, end: usize) {
        self.range = Some(Range { start, end });
    }
    /// Constructor of [`NoteErrorKind::Unreachable`]
    #[inline]
    pub fn unreachable() -> Self {
        Self {
            kind: Box::new(JssErrorKind::Unreachable),
            level: DiagnosticLevel::None,
            file: None,
            range: None,
        }
    }

    /// Constructor of [`NoteErrorKind::UndefinedVariable`]
    #[inline]
    pub fn undefined_variable(name: impl Into<String>) -> JssError {
        let kind = JssErrorKind::UndefinedVariable { name: name.into() };
        Self {
            kind: Box::new(kind),
            level: DiagnosticLevel::None,
            file: None,
            range: None,
        }
    }
}

impl JssError {
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
        pub fn $name(msg: impl Into<String>) -> JssError {
            let kind = JssErrorKind::$t(msg.into());
            Self { kind: Box::new(kind), level: DiagnosticLevel::None, file: None, range: None }
        }
    };
    ($($name:ident => $t:ident),+ $(,)?) => (
        impl JssError { $(error_msg!($name=>$t);)+ }
    );
}

error_msg![
    syntax_error => SyntaxError,
    type_mismatch => TypeMismatch,
    runtime_error => RuntimeError,
];

impl Error for JssError {}

impl Display for JssError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let path = match &self.file {
            Some(s) => s.path(),
            None => "<Anonymous>",
        };
        match &self.range {
            Some(s) => writeln!(f, "at ({}, {}) of {}", s.start, s.end, path)?,
            None => writeln!(f, "at {}", path)?,
        }
        write!(f, "{:indent$}{}", self.kind, indent = 4)
    }
}

impl Display for JssErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::IOError(e) => {
                write!(f, "{}", e)
            }
            Self::FormatError(e) => {
                write!(f, "{}", e)
            }
            Self::SyntaxError(msg) => {
                f.write_str("SyntaxError: ")?;
                f.write_str(msg)
            }
            Self::TypeMismatch(msg) => {
                f.write_str("TypeError: ")?;
                f.write_str(msg)
            }
            Self::RuntimeError(msg) => {
                f.write_str("RuntimeError: ")?;
                f.write_str(msg)
            }
            Self::UndefinedVariable { name } => {
                write!(f, "RuntimeError: Variable {} not found in scope", name)
            }
            Self::Unreachable => {
                f.write_str("InternalError: ")?;
                f.write_str("Entered unreachable code!")
            }
        }
    }
}
