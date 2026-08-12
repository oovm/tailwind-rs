use crate::error::{ParseError, ParseResult};
use crate::lexer::{Lexer, Token, TokenKind};
use tailwind_ast::{
    ArbitrarySyntax, CandidateSyntax, Identifier, ModifierSyntax, ModifierValue, NamedValue,
    UtilitySyntax, UtilityValue, VariantBody, VariantSyntax,
};
use tailwind_types::TextRange;

/// Successful parse of a source string (possibly multiple candidates after desugar).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseOutput {
    pub candidates: Vec<CandidateSyntax>,
}

/// Parse a full source string: whitespace-separated candidates and groups.
///
/// Group syntax `hover:(a b)` is desugared into ordinary candidates before return.
pub fn parse_source(input: &str) -> ParseResult<ParseOutput> {
    if input.chars().all(|c| c.is_whitespace()) {
        return Err(ParseError::Empty);
    }

    let mut lexer = Lexer::new(input);
    let mut candidates = Vec::new();
    loop {
        skip_ws(&mut lexer);
        if lexer.peek_char().is_none() {
            break;
        }
        let start = lexer.position();
        parse_item(&mut lexer, &[], false, false, &mut candidates)?;
        if lexer.position() == start {
            return Err(ParseError::Unexpected {
                message: "failed to consume input".into(),
                span: (start, 1).into(),
            });
        }
    }

    if candidates.is_empty() {
        return Err(ParseError::Empty);
    }
    Ok(ParseOutput { candidates })
}

/// Parse a source that must yield exactly one candidate (no group expansion).
pub fn parse_candidate(input: &str) -> ParseResult<CandidateSyntax> {
    let out = parse_source(input)?;
    match out.candidates.len() {
        1 => Ok(out.candidates.into_iter().next().unwrap()),
        n => Err(ParseError::Unexpected {
            message: format!("expected 1 candidate, found {n}"),
            span: (0, input.len()).into(),
        }),
    }
}

fn skip_ws(lexer: &mut Lexer<'_>) {
    while matches!(lexer.peek_char(), Some(' ' | '\t' | '\n' | '\r')) {
        lexer.bump_char();
    }
}

struct Checkpoint {
    pos: usize,
}

impl Checkpoint {
    fn take(lexer: &Lexer<'_>) -> Self {
        Self {
            pos: lexer.position(),
        }
    }

    fn restore(self, lexer: &mut Lexer<'_>) {
        lexer.set_position(self.pos);
    }
}

fn parse_item(
    lexer: &mut Lexer<'_>,
    inherited_variants: &[VariantSyntax],
    inherited_important: bool,
    inherited_negative: bool,
    out: &mut Vec<CandidateSyntax>,
) -> ParseResult<()> {
    let start = lexer.position();
    let mut variants = inherited_variants.to_vec();

    loop {
        skip_ws(lexer);
        match try_parse_variant(lexer)? {
            Some(v) => variants.push(v),
            None => break,
        }
    }

    skip_ws(lexer);

    if lexer.peek_char() == Some('(') {
        return parse_group(lexer, &variants, inherited_important, inherited_negative, out);
    }

    let mut important = inherited_important;
    let mut negative = inherited_negative;

    if lexer.peek_char() == Some('!') {
        lexer.bump_char();
        important = true;
    }

    if lexer.peek_char() == Some('-') {
        let ck = Checkpoint::take(lexer);
        lexer.bump_char();
        let next = lexer.peek_char();
        if next == Some('[') || next.map(is_ident_start_char).unwrap_or(false) {
            negative = true;
        } else {
            ck.restore(lexer);
            return Err(ParseError::Unexpected {
                message: "dangling '-'".into(),
                span: (lexer.position(), 1).into(),
            });
        }
    }

    let utility = parse_utility(lexer)?;

    if lexer.peek_char() == Some('!') {
        lexer.bump_char();
        important = true;
    }

    let end = lexer.position();
    out.push(CandidateSyntax {
        variants,
        important,
        negative,
        utility,
        span: TextRange::new(start as u32, end as u32),
    });
    Ok(())
}

fn parse_group(
    lexer: &mut Lexer<'_>,
    variants: &[VariantSyntax],
    important: bool,
    negative: bool,
    out: &mut Vec<CandidateSyntax>,
) -> ParseResult<()> {
    let open = lexer.position();
    let _ = lexer.bump_char();

    loop {
        skip_ws(lexer);
        match lexer.peek_char() {
            None => {
                return Err(ParseError::Unbalanced {
                    opener: '(',
                    closer: ')',
                    span: (open, 1).into(),
                });
            }
            Some(')') => {
                lexer.bump_char();
                return Ok(());
            }
            _ => parse_item(lexer, variants, important, negative, out)?,
        }
    }
}

fn try_parse_variant(lexer: &mut Lexer<'_>) -> ParseResult<Option<VariantSyntax>> {
    let ck = Checkpoint::take(lexer);

    // Arbitrary variant: `[...]:`
    if lexer.peek_char() == Some('[') {
        match lexer.read_arbitrary_brackets() {
            Ok((raw, span)) => {
                if lexer.peek_char() == Some(':') {
                    lexer.bump_char();
                    let double_colon = if lexer.peek_char() == Some(':') {
                        lexer.bump_char();
                        true
                    } else {
                        false
                    };
                    let _ = double_colon;
                    let arb = split_arbitrary(raw, span);
                    return Ok(Some(VariantSyntax {
                        not: false,
                        body: VariantBody::Arbitrary(arb),
                        span: TextRange::new(ck.pos as u32, lexer.position() as u32),
                    }));
                }
                ck.restore(lexer);
                return Ok(None);
            }
            Err(_) => {
                ck.restore(lexer);
                return Ok(None);
            }
        }
    }

    match try_named_variant(lexer)? {
        Some(v) => Ok(Some(v)),
        None => {
            ck.restore(lexer);
            Ok(None)
        }
    }
}

fn try_named_variant(lexer: &mut Lexer<'_>) -> ParseResult<Option<VariantSyntax>> {
    let start = lexer.position();
    if !lexer.peek_char().map(is_ident_start_char).unwrap_or(false) {
        return Ok(None);
    }

    let mut not = false;
    let mut names = Vec::new();
    let first = next_ident(lexer)?;

    if first.text == "not" && lexer.peek_char() == Some('-') {
        not = true;
        lexer.bump_char();
        if !lexer.peek_char().map(is_ident_start_char).unwrap_or(false) {
            return Ok(None);
        }
        names.push(next_ident(lexer)?);
    } else {
        names.push(first);
    }

    while lexer.peek_char() == Some('-') {
        let ck = Checkpoint::take(lexer);
        lexer.bump_char();
        if !lexer.peek_char().map(is_ident_start_char).unwrap_or(false) {
            ck.restore(lexer);
            break;
        }
        names.push(next_ident(lexer)?);
    }

    let double_colon = match lexer.peek_char() {
        Some(':') => {
            lexer.bump_char();
            if lexer.peek_char() == Some(':') {
                lexer.bump_char();
                true
            } else {
                false
            }
        }
        _ => return Ok(None),
    };

    Ok(Some(VariantSyntax {
        not,
        body: VariantBody::Named {
            names,
            double_colon,
        },
        span: TextRange::new(start as u32, lexer.position() as u32),
    }))
}

fn parse_utility(lexer: &mut Lexer<'_>) -> ParseResult<UtilitySyntax> {
    if lexer.peek_char() == Some('[') {
        let (raw, span) = lexer.read_arbitrary_brackets()?;
        let arb = split_arbitrary(raw, span);
        let property = arb.type_hint.clone().ok_or_else(|| ParseError::Unexpected {
            message: "arbitrary property requires `name:value`".into(),
            span: span.as_source_span(),
        })?;
        let value = ArbitrarySyntax {
            type_hint: None,
            raw: arb.raw,
            span: arb.span,
        };
        let modifier = try_parse_modifier(lexer)?;
        return Ok(UtilitySyntax::ArbitraryProperty {
            property,
            value,
            modifier,
        });
    }

    let name = next_ident(lexer)?;
    let mut value = UtilityValue::Bare;

    if lexer.peek_char() == Some('-') {
        let ck = Checkpoint::take(lexer);
        lexer.bump_char();
        match lexer.peek_char() {
            Some('[') => {
                let (raw, span) = lexer.read_arbitrary_brackets()?;
                value = UtilityValue::Arbitrary(split_arbitrary(raw, span));
            }
            Some(c) if is_value_start(c) => {
                let value_start = lexer.position();
                let mut text = String::new();
                loop {
                    match lexer.peek_char() {
                        Some(ch) if is_value_char(ch) => {
                            text.push(lexer.bump_char().unwrap());
                        }
                        Some('-') => {
                            let ck2 = Checkpoint::take(lexer);
                            lexer.bump_char();
                            if lexer.peek_char().map(is_value_char).unwrap_or(false) {
                                text.push('-');
                            } else {
                                ck2.restore(lexer);
                                break;
                            }
                        }
                        Some('/') => {
                            let ck2 = Checkpoint::take(lexer);
                            lexer.bump_char();
                            let after = lexer.peek_char();
                            if after.map(|c| c.is_ascii_digit()).unwrap_or(false)
                                && is_fraction_context(&text)
                            {
                                text.push('/');
                            } else {
                                ck2.restore(lexer);
                                break;
                            }
                        }
                        _ => break,
                    }
                }
                if text.is_empty() {
                    ck.restore(lexer);
                } else {
                    value = UtilityValue::Named(NamedValue {
                        text,
                        span: TextRange::new(value_start as u32, lexer.position() as u32),
                    });
                }
            }
            _ => ck.restore(lexer),
        }
    }

    let modifier = try_parse_modifier(lexer)?;
    Ok(UtilitySyntax::Standard {
        name,
        value,
        modifier,
    })
}

fn is_fraction_context(text: &str) -> bool {
    !text.is_empty() && text.chars().all(|c| c.is_ascii_digit() || c == '.')
}

fn try_parse_modifier(lexer: &mut Lexer<'_>) -> ParseResult<Option<ModifierSyntax>> {
    if lexer.peek_char() != Some('/') {
        return Ok(None);
    }
    let start = lexer.position();
    lexer.bump_char();
    if lexer.peek_char() == Some('[') {
        let (raw, span) = lexer.read_arbitrary_brackets()?;
        return Ok(Some(ModifierSyntax {
            value: ModifierValue::Arbitrary(split_arbitrary(raw, span)),
            span: TextRange::new(start as u32, lexer.position() as u32),
        }));
    }
    let ident = next_ident(lexer)?;
    Ok(Some(ModifierSyntax {
        value: ModifierValue::Named(ident),
        span: TextRange::new(start as u32, lexer.position() as u32),
    }))
}

fn split_arbitrary(raw: String, span: TextRange) -> ArbitrarySyntax {
    if let Some(idx) = find_type_hint_colon(&raw) {
        let hint = raw[..idx].to_string();
        if is_simple_ident(&hint) {
            let hint_start = span.start + 1;
            let hint_end = hint_start + hint.len() as u32;
            return ArbitrarySyntax {
                type_hint: Some(Identifier::new(hint, TextRange::new(hint_start, hint_end))),
                raw: raw[idx + 1..].to_string(),
                span,
            };
        }
    }
    ArbitrarySyntax {
        type_hint: None,
        raw,
        span,
    }
}

fn find_type_hint_colon(raw: &str) -> Option<usize> {
    let mut paren = 0i32;
    let mut quote: Option<char> = None;
    let mut chars = raw.char_indices().peekable();
    while let Some((i, ch)) = chars.next() {
        if let Some(q) = quote {
            if ch == '\\' {
                let _ = chars.next();
                continue;
            }
            if ch == q {
                quote = None;
            }
            continue;
        }
        match ch {
            '\'' | '"' => quote = Some(ch),
            '(' => paren += 1,
            ')' => paren -= 1,
            ':' if paren == 0 => return Some(i),
            _ => {}
        }
    }
    None
}

fn is_simple_ident(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

fn next_ident(lexer: &mut Lexer<'_>) -> ParseResult<Identifier> {
    let tok = read_ident_token(lexer)?;
    Ok(Identifier::new(tok.text, tok.span))
}

fn read_ident_token(lexer: &mut Lexer<'_>) -> ParseResult<Token> {
    let start = lexer.position();
    let Some(ch) = lexer.peek_char() else {
        return Err(ParseError::Incomplete {
            span: (start, 0).into(),
        });
    };
    if !is_ident_start_char(ch) && !ch.is_ascii_digit() {
        return Err(ParseError::Unexpected {
            message: format!("expected identifier, found {ch:?}"),
            span: (start, ch.len_utf8()).into(),
        });
    }
    lexer.bump_char();
    while let Some(c) = lexer.peek_char() {
        if is_ident_continue_char(c) {
            lexer.bump_char();
        } else {
            break;
        }
    }
    Ok(Token {
        kind: TokenKind::Ident,
        text: lexer.source()[start..lexer.position()].to_string(),
        span: TextRange::new(start as u32, lexer.position() as u32),
    })
}

fn is_ident_start_char(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_' || ch == '@' || ch == '&' || ch == '*' || ch == '#'
}

fn is_ident_continue_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '%' || ch == '.'
}

fn is_value_start(ch: char) -> bool {
    is_ident_start_char(ch) || ch.is_ascii_digit()
}

fn is_value_char(ch: char) -> bool {
    is_ident_continue_char(ch)
}
