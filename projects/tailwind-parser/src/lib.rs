//! Built-in Tailwind lexer + parser.
//!
//! No `nom` / PEG runtime. Errors use `miette` and convert to
//! [`tailwind_types::Diagnostic`].

#![forbid(unsafe_code)]

mod error;
mod lexer;
mod parser;

pub use error::{ParseError, ParseResult};
pub use lexer::{Lexer, Token, TokenKind};
pub use parser::{parse_candidate, parse_source, ParseOutput};

use tailwind_types::Diagnostic;

/// Parse one candidate token text into diagnostics-friendly output.
pub fn parse_token(token: &str) -> Result<ParseOutput, Vec<Diagnostic>> {
    match parse_source(token) {
        Ok(out) => Ok(out),
        Err(err) => Err(err.into_diagnostics(token)),
    }
}
