use std::{
    convert::Infallible,
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use diagnostic::{
    term::{
        emit,
        termcolor::{ColorChoice, StandardStream},
        TerminalConfig,
    },
    Diagnostic, DiagnosticLevel, ErrorWithFile, ErrorWithFileSpan, FileID, TextStorage,
};

mod error_std;
mod for_diagnostic;

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
    #[inline]
    pub fn get_kind(&self) -> &TailwindErrorKind {
        &self.kind
    }
    /// Get error kind of this error
    #[inline]
    pub fn get_kind_mut(&mut self) -> &mut TailwindErrorKind {
        &mut self.kind
    }
    /// Get level
    #[inline]
    pub fn get_level(&self) -> DiagnosticLevel {
        self.level
    }
    /// Set
    #[inline]
    pub fn set_level(&mut self, level: DiagnosticLevel) {
        self.level = level;
    }
    /// Build
    pub fn with_level(mut self, level: DiagnosticLevel) -> Self {
        self.set_level(level);
        self
    }
    ///
    pub fn set_file(&mut self, file: &FileID) {
        match self.get_kind_mut() {
            TailwindErrorKind::IOError(_) => {},
            TailwindErrorKind::FormatError(_) => {},
            TailwindErrorKind::SyntaxError(e) => {
                e.file = file.clone();
            },
            TailwindErrorKind::TypeMismatch(_) => {},
            TailwindErrorKind::RuntimeError(_) => {},
            TailwindErrorKind::UndefinedVariable { .. } => {},
            TailwindErrorKind::Incomplete => {},
            TailwindErrorKind::Unreachable => {},
        }
    }
    ///
    pub fn with_file(mut self, file: &FileID) -> Self {
        self.set_file(file);
        self
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
    /// Emit diagnostic to std out
    pub fn emit(store: &TextStorage, diagnostics: &[Diagnostic]) -> Result<()> {
        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = TerminalConfig::default();
        for diagnostic in diagnostics {
            emit(&mut writer.lock(), &config, store, diagnostic)?;
        }
        Ok(())
    }
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
    pub fn syntax_error(error: ErrorWithFileSpan<String>) -> TailwindError {
        let kind = TailwindErrorKind::SyntaxError(error);
        Self { kind: Box::new(kind), level: Default::default() }
    }
    /// Constructor of [`NoteErrorKind::$t`]
    pub fn type_mismatch(msg: impl Into<String>) -> TailwindError {
        let kind = TailwindErrorKind::TypeMismatch(msg.into());
        Self { kind: Box::new(kind), level: Default::default() }
    }
    /// Constructor of [`NoteErrorKind::$t`]
    pub fn runtime_error(msg: impl Into<String>) -> TailwindError {
        let kind = TailwindErrorKind::RuntimeError(msg.into());
        Self { kind: Box::new(kind), level: Default::default() }
    }
}

impl Error for TailwindError {}

impl Display for TailwindError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl TailwindError {
    /// Pretty print
    pub fn as_report(&self) -> Diagnostic {
        self.kind.as_report(self.level)
    }
}
impl TailwindErrorKind {
    /// Pretty print
    pub fn as_report(&self, level: DiagnosticLevel) -> Diagnostic {
        match self {
            TailwindErrorKind::IOError(_) => {
                todo!()
            },
            TailwindErrorKind::FormatError(_) => {
                todo!()
            },
            TailwindErrorKind::SyntaxError(s) => s.as_report(level, "Syntax Error"),
            TailwindErrorKind::TypeMismatch(_) => {
                todo!()
            },
            TailwindErrorKind::RuntimeError(_) => {
                todo!()
            },
            TailwindErrorKind::UndefinedVariable { .. } => {
                todo!()
            },
            TailwindErrorKind::Incomplete => {
                todo!()
            },
            TailwindErrorKind::Unreachable => {
                todo!()
            },
        }
    }
}
