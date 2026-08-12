use tailwind_ast::{ArbitrarySyntax, UtilityValue};
use tailwind_types::{
    ColorValue, CssValue, Diagnostic, DiagnosticCode, LengthValue, ThemeKey, ThemeValue,
};

use crate::context::ResolveContext;
use crate::theme::ThemeLookup;

/// Shared typed value after theme / arbitrary resolution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypedValue {
    Length(LengthValue),
    Color(ColorValue),
    Keyword(String),
    Number(String),
    Raw(String),
}

impl TypedValue {
    pub fn into_css(self) -> CssValue {
        match self {
            Self::Length(v) => CssValue::Length(v),
            Self::Color(v) => CssValue::Color(v),
            Self::Keyword(v) => CssValue::Keyword(v),
            Self::Number(v) => CssValue::Number(v),
            Self::Raw(v) => CssValue::Raw(v),
        }
    }
}

pub struct LengthResolver;

impl LengthResolver {
    pub fn from_theme_spacing(
        ctx: &mut ResolveContext<'_>,
        token: &str,
    ) -> Result<TypedValue, Diagnostic> {
        Self::from_theme_namespace(ctx, "spacing", token)
    }

    /// Look up a length/keyword/raw value under an arbitrary theme namespace.
    pub fn from_theme_namespace(
        ctx: &mut ResolveContext<'_>,
        namespace: &str,
        token: &str,
    ) -> Result<TypedValue, Diagnostic> {
        let mut parts = vec![namespace.to_string()];
        parts.extend(token.split('-').map(str::to_string));
        let key = ThemeKey { path: parts };
        match ctx.lookup(key.clone()) {
            ThemeLookup::Resolved(ThemeValue::Length(v)) => Ok(TypedValue::Length(v)),
            ThemeLookup::Resolved(ThemeValue::Keyword(k)) => Ok(TypedValue::Keyword(k)),
            ThemeLookup::Resolved(ThemeValue::Number(n)) => Ok(TypedValue::Number(n)),
            ThemeLookup::Resolved(ThemeValue::Raw(r)) => Ok(TypedValue::Raw(r)),
            ThemeLookup::Resolved(other) => Err(Diagnostic::error(
                DiagnosticCode::ThemeInvalidType,
                format!(
                    "theme key `{}` is not a length (got {other:?})",
                    key.joined()
                ),
            )),
            ThemeLookup::Missing => Err(Diagnostic::error(
                DiagnosticCode::ThemeMissing,
                format!("missing theme key `{}`", key.joined()),
            )),
            ThemeLookup::Ambiguous => Err(Diagnostic::error(
                DiagnosticCode::ThemeAmbiguous,
                format!("ambiguous theme key `{}`", key.joined()),
            )),
            ThemeLookup::InvalidType => Err(Diagnostic::error(
                DiagnosticCode::ThemeInvalidType,
                format!("invalid type for `{}`", key.joined()),
            )),
        }
    }

    pub fn from_utility_value(
        ctx: &mut ResolveContext<'_>,
        value: &UtilityValue,
    ) -> Result<TypedValue, Diagnostic> {
        Self::from_utility_value_ns(ctx, "spacing", value)
    }

    pub fn from_utility_value_ns(
        ctx: &mut ResolveContext<'_>,
        namespace: &str,
        value: &UtilityValue,
    ) -> Result<TypedValue, Diagnostic> {
        match value {
            UtilityValue::Bare => Err(Diagnostic::error(
                DiagnosticCode::ResolveInvalidType,
                "length utility requires a value",
            )),
            UtilityValue::Named(n) => Self::from_theme_namespace(ctx, namespace, &n.text),
            UtilityValue::Arbitrary(a) => Self::from_arbitrary(a),
        }
    }

    pub fn from_arbitrary(a: &ArbitrarySyntax) -> Result<TypedValue, Diagnostic> {
        if let Some(hint) = &a.type_hint {
            if hint.text != "length" && hint.text != "size" {
                return Err(Diagnostic::error(
                    DiagnosticCode::ResolveInvalidType,
                    format!("expected length type hint, got `{}`", hint.text),
                ));
            }
        }
        Ok(TypedValue::Length(LengthValue {
            css: a.raw.clone(),
        }))
    }

    pub fn negate(value: TypedValue) -> Result<TypedValue, Diagnostic> {
        match value {
            TypedValue::Length(LengthValue { css }) => {
                if css.starts_with('-') {
                    Ok(TypedValue::Length(LengthValue {
                        css: css.trim_start_matches('-').to_string(),
                    }))
                } else if css == "0" || css == "0px" || css == "0rem" {
                    Ok(TypedValue::Length(LengthValue { css }))
                } else {
                    Ok(TypedValue::Length(LengthValue {
                        css: format!("-{css}"),
                    }))
                }
            }
            TypedValue::Keyword(k) if k == "auto" => Err(Diagnostic::error(
                DiagnosticCode::ResolveNegativeNotAllowed,
                "cannot negate `auto`",
            )),
            other => Err(Diagnostic::error(
                DiagnosticCode::ResolveNegativeNotAllowed,
                format!("cannot negate value {other:?}"),
            )),
        }
    }
}

pub struct ColorResolver;

impl ColorResolver {
    pub fn from_theme(
        ctx: &mut ResolveContext<'_>,
        token: &str,
    ) -> Result<TypedValue, Diagnostic> {
        // `red-500` → colors.red.500 ; `white` → colors.white
        let parts: Vec<&str> = std::iter::once("colors")
            .chain(token.split('-'))
            .collect();
        let key = ThemeKey::from_path(parts);
        match ctx.lookup(key.clone()) {
            ThemeLookup::Resolved(ThemeValue::Color(v)) => Ok(TypedValue::Color(v)),
            ThemeLookup::Resolved(ThemeValue::Raw(r)) => Ok(TypedValue::Raw(r)),
            ThemeLookup::Resolved(other) => Err(Diagnostic::error(
                DiagnosticCode::ThemeInvalidType,
                format!(
                    "theme key `{}` is not a color (got {other:?})",
                    key.joined()
                ),
            )),
            ThemeLookup::Missing => Err(Diagnostic::error(
                DiagnosticCode::ThemeMissing,
                format!("missing theme key `{}`", key.joined()),
            )),
            ThemeLookup::Ambiguous => Err(Diagnostic::error(
                DiagnosticCode::ThemeAmbiguous,
                format!("ambiguous theme key `{}`", key.joined()),
            )),
            ThemeLookup::InvalidType => Err(Diagnostic::error(
                DiagnosticCode::ThemeInvalidType,
                format!("invalid type for `{}`", key.joined()),
            )),
        }
    }

    pub fn from_utility_value(
        ctx: &mut ResolveContext<'_>,
        value: &UtilityValue,
    ) -> Result<TypedValue, Diagnostic> {
        match value {
            UtilityValue::Bare => Err(Diagnostic::error(
                DiagnosticCode::ResolveInvalidType,
                "color utility requires a value",
            )),
            UtilityValue::Named(n) => Self::from_theme(ctx, &n.text),
            UtilityValue::Arbitrary(a) => Self::from_arbitrary(a),
        }
    }

    pub fn from_arbitrary(a: &ArbitrarySyntax) -> Result<TypedValue, Diagnostic> {
        if let Some(hint) = &a.type_hint {
            if hint.text != "color" {
                return Err(Diagnostic::error(
                    DiagnosticCode::ResolveInvalidType,
                    format!("expected color type hint, got `{}`", hint.text),
                ));
            }
        }
        Ok(TypedValue::Color(ColorValue {
            css: a.raw.clone(),
        }))
    }

    /// Apply `/modifier` opacity when modifier is 0–100.
    pub fn with_opacity(value: TypedValue, opacity: &str) -> Result<TypedValue, Diagnostic> {
        let alpha = parse_opacity(opacity)?;
        match value {
            TypedValue::Color(ColorValue { css }) => Ok(TypedValue::Color(ColorValue {
                css: format!("color-mix(in oklab, {css} {alpha}%, transparent)"),
            })),
            TypedValue::Raw(css) => Ok(TypedValue::Raw(format!(
                "color-mix(in oklab, {css} {alpha}%, transparent)"
            ))),
            other => Err(Diagnostic::error(
                DiagnosticCode::ResolveModifierNotAllowed,
                format!("opacity modifier not allowed on {other:?}"),
            )),
        }
    }
}

fn parse_opacity(token: &str) -> Result<u32, Diagnostic> {
    let n: u32 = token.parse().map_err(|_| {
        Diagnostic::error(
            DiagnosticCode::ResolveInvalidType,
            format!("invalid opacity modifier `/{token}`"),
        )
    })?;
    if n > 100 {
        return Err(Diagnostic::error(
            DiagnosticCode::ResolveInvalidType,
            format!("opacity `/{token}` out of range 0–100"),
        ));
    }
    Ok(n)
}

pub struct KeywordResolver;

impl KeywordResolver {
    pub fn expect(name: &str, allowed: &[&str]) -> Result<TypedValue, Diagnostic> {
        if allowed.contains(&name) {
            Ok(TypedValue::Keyword(name.to_string()))
        } else {
            Err(Diagnostic::error(
                DiagnosticCode::ResolveInvalidType,
                format!("invalid keyword `{name}`"),
            ))
        }
    }
}
