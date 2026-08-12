use crate::error::{ParseError, ParseResult};
use tailwind_types::TextRange;

/// Lexical token kind. Parser owns all grammar decisions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Ident,
    Colon,
    Bang,
    Dash,
    Slash,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Whitespace,
    /// End of input.
    Eof,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub span: TextRange,
}

pub struct Lexer<'a> {
    src: &'a str,
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            bytes: src.as_bytes(),
            pos: 0,
        }
    }

    pub fn source(&self) -> &'a str {
        self.src
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn set_position(&mut self, pos: usize) {
        self.pos = pos.min(self.bytes.len());
    }

    pub fn peek_char(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    pub fn bump_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    pub fn span_from(&self, start: usize) -> TextRange {
        TextRange::new(start as u32, self.pos as u32)
    }

    pub fn slice(&self, span: TextRange) -> &'a str {
        &self.src[span.start as usize..span.end as usize]
    }

    pub fn next_token(&mut self) -> ParseResult<Token> {
        let start = self.pos;
        let Some(ch) = self.peek_char() else {
            return Ok(Token {
                kind: TokenKind::Eof,
                text: String::new(),
                span: self.span_from(start),
            });
        };

        let kind = match ch {
            ' ' | '\t' | '\n' | '\r' => {
                while matches!(self.peek_char(), Some(' ' | '\t' | '\n' | '\r')) {
                    self.bump_char();
                }
                TokenKind::Whitespace
            }
            ':' => {
                self.bump_char();
                TokenKind::Colon
            }
            '!' => {
                self.bump_char();
                TokenKind::Bang
            }
            '-' => {
                self.bump_char();
                TokenKind::Dash
            }
            '/' => {
                self.bump_char();
                TokenKind::Slash
            }
            '[' => {
                self.bump_char();
                TokenKind::LBracket
            }
            ']' => {
                self.bump_char();
                TokenKind::RBracket
            }
            '(' => {
                self.bump_char();
                TokenKind::LParen
            }
            ')' => {
                self.bump_char();
                TokenKind::RParen
            }
            _ if is_ident_start(ch) => {
                self.bump_char();
                while let Some(c) = self.peek_char() {
                    if is_ident_continue(c) {
                        self.bump_char();
                    } else {
                        break;
                    }
                }
                TokenKind::Ident
            }
            _ => {
                return Err(ParseError::Unexpected {
                    message: format!("unexpected character {ch:?}"),
                    span: (start, ch.len_utf8()).into(),
                });
            }
        };

        Ok(Token {
            kind,
            text: self.src[start..self.pos].to_string(),
            span: self.span_from(start),
        })
    }

    /// Read a balanced `[...]` starting at current `[`.
    /// Handles nested `()` / `[]`, quotes, and `\` escapes.
    pub fn read_arbitrary_brackets(&mut self) -> ParseResult<(String, TextRange)> {
        let start = self.pos;
        let Some('[') = self.bump_char() else {
            return Err(ParseError::Unexpected {
                message: "expected '['".into(),
                span: (start, 0).into(),
            });
        };

        let mut raw = String::new();
        let mut square = 1i32;
        let mut paren = 0i32;
        let mut quote: Option<char> = None;

        while let Some(ch) = self.peek_char() {
            if let Some(q) = quote {
                if ch == '\\' {
                    let esc_start = self.pos;
                    self.bump_char();
                    let Some(escaped) = self.bump_char() else {
                        return Err(ParseError::InvalidEscape {
                            span: (esc_start, 1).into(),
                        });
                    };
                    raw.push('\\');
                    raw.push(escaped);
                    continue;
                }
                raw.push(ch);
                self.bump_char();
                if ch == q {
                    quote = None;
                }
                continue;
            }

            match ch {
                '\\' => {
                    let esc_start = self.pos;
                    self.bump_char();
                    let Some(escaped) = self.bump_char() else {
                        return Err(ParseError::InvalidEscape {
                            span: (esc_start, 1).into(),
                        });
                    };
                    raw.push('\\');
                    raw.push(escaped);
                }
                '\'' | '"' => {
                    quote = Some(ch);
                    raw.push(ch);
                    self.bump_char();
                }
                '[' => {
                    square += 1;
                    raw.push(ch);
                    self.bump_char();
                }
                ']' => {
                    square -= 1;
                    self.bump_char();
                    if square == 0 && paren == 0 {
                        let span = self.span_from(start);
                        return Ok((raw, span));
                    }
                    raw.push(ch);
                }
                '(' => {
                    paren += 1;
                    raw.push(ch);
                    self.bump_char();
                }
                ')' => {
                    paren -= 1;
                    if paren < 0 {
                        return Err(ParseError::Unbalanced {
                            opener: '(',
                            closer: ')',
                            span: (self.pos, 1).into(),
                        });
                    }
                    raw.push(ch);
                    self.bump_char();
                }
                _ => {
                    raw.push(ch);
                    self.bump_char();
                }
            }
        }

        Err(ParseError::Unbalanced {
            opener: '[',
            closer: ']',
            span: (start, 1).into(),
        })
    }
}

fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '@' || ch == '&' || ch == '*' || ch == '#'
}

fn is_ident_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '%' || ch == '.'
}
