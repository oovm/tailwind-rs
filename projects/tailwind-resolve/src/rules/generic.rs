//! Generic / family rule resolvers used by the data-driven registry.

use std::collections::BTreeMap;

use tailwind_ast::UtilityValue;
use tailwind_types::{
    CssValue, Declaration, DeclarationSet, Diagnostic, DiagnosticCode, LengthValue, ThemeKey,
    ThemeValue,
};

use crate::context::ResolveContext;
use crate::theme::ThemeLookup;
use crate::typed::{ColorResolver, LengthResolver, TypedValue};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StaticRule {
    pub decls: Vec<(String, String)>,
}

impl StaticRule {
    pub fn pairs(pairs: &[(&str, &str)]) -> Self {
        Self {
            decls: pairs
                .iter()
                .map(|(p, v)| ((*p).to_string(), (*v).to_string()))
                .collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeywordMapEntry {
    pub token: String,
    pub property: String,
    pub css: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeywordMapRule {
    /// Default CSS property (arbitrary values / bare fallback).
    pub property: String,
    pub map: Vec<KeywordMapEntry>,
    pub bare: Option<String>,
}

impl KeywordMapRule {
    pub fn single(property: &str, map: &[(&str, &str)]) -> Self {
        Self {
            property: property.into(),
            map: map
                .iter()
                .map(|(k, v)| KeywordMapEntry {
                    token: (*k).to_string(),
                    property: property.into(),
                    css: (*v).to_string(),
                })
                .collect(),
            bare: None,
        }
    }

    /// Each entry is `(token, css_property, css_value)` — for axis utilities like `overflow-x-hidden`.
    pub fn entries(default_property: &str, map: &[(&str, &str, &str)]) -> Self {
        Self {
            property: default_property.into(),
            map: map
                .iter()
                .map(|(token, prop, css)| KeywordMapEntry {
                    token: (*token).to_string(),
                    property: (*prop).to_string(),
                    css: (*css).to_string(),
                })
                .collect(),
            bare: None,
        }
    }

    pub fn with_bare(mut self, bare: &str) -> Self {
        self.bare = Some(bare.into());
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThemeLengthRule {
    pub properties: Vec<String>,
    pub namespace: String,
    pub allow_negative: bool,
    /// Optional keyword shortcuts (token → css).
    pub keywords: Vec<(String, String)>,
    /// Bare utility value (e.g. `ring` without suffix).
    pub bare: Option<String>,
    /// If set, numeric named tokens become `{n}{suffix}` (suffix may be empty).
    pub numeric_suffix: Option<String>,
    /// Optional template wrapping the resolved css (`{v}` placeholder).
    pub wrap: Option<String>,
}

impl ThemeLengthRule {
    pub fn spacing(properties: &[&str]) -> Self {
        Self::props(properties, "spacing")
    }

    pub fn props(properties: &[&str], namespace: &str) -> Self {
        Self {
            properties: properties.iter().map(|s| (*s).to_string()).collect(),
            namespace: namespace.into(),
            allow_negative: false,
            keywords: Vec::new(),
            bare: None,
            numeric_suffix: None,
            wrap: None,
        }
    }

    pub fn allow_negative(mut self) -> Self {
        self.allow_negative = true;
        self
    }

    pub fn keywords(mut self, keywords: &[(&str, &str)]) -> Self {
        self.keywords = keywords
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect();
        self
    }

    pub fn bare(mut self, css: &str) -> Self {
        self.bare = Some(css.into());
        self
    }

    pub fn numeric_suffix(mut self, suffix: &str) -> Self {
        self.numeric_suffix = Some(suffix.into());
        self
    }

    pub fn wrap(mut self, template: &str) -> Self {
        self.wrap = Some(template.into());
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThemeColorRule {
    pub property: String,
}

impl ThemeColorRule {
    pub fn new(property: &str) -> Self {
        Self {
            property: property.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MinMaxKind {
    Min,
    Max,
}

pub fn resolve_static(rule: &StaticRule, negative: bool) -> Result<DeclarationSet, Diagnostic> {
    if negative {
        return Err(Diagnostic::error(
            DiagnosticCode::ResolveNegativeNotAllowed,
            "negative not allowed on static utilities",
        ));
    }
    Ok(DeclarationSet {
        declarations: rule
            .decls
            .iter()
            .map(|(p, v)| Declaration {
                property: p.clone(),
                value: CssValue::Keyword(v.clone()),
                important: false,
            })
            .collect(),
    })
}

pub fn resolve_keyword_map(
    rule: &KeywordMapRule,
    value: &UtilityValue,
    negative: bool,
) -> Result<DeclarationSet, Diagnostic> {
    if negative {
        return Err(Diagnostic::error(
            DiagnosticCode::ResolveNegativeNotAllowed,
            format!("negative not allowed on `{}`", rule.property),
        ));
    }
    let raw = match value {
        UtilityValue::Bare => rule.bare.as_deref().ok_or_else(|| {
            Diagnostic::error(
                DiagnosticCode::ResolveInvalidType,
                format!("`{}` requires a value", rule.property),
            )
        })?,
        UtilityValue::Named(n) => n.text.as_str(),
        UtilityValue::Arbitrary(a) => {
            return Ok(decl(&rule.property, CssValue::Raw(a.raw.clone())));
        }
    };
    if let Some(entry) = rule.map.iter().find(|e| e.token == raw) {
        return Ok(decl(&entry.property, CssValue::Keyword(entry.css.clone())));
    }
    Ok(decl(&rule.property, CssValue::Keyword(raw.into())))
}

pub fn resolve_theme_length(
    rule: &ThemeLengthRule,
    value: &UtilityValue,
    negative: bool,
    ctx: &mut ResolveContext<'_>,
) -> Result<DeclarationSet, Diagnostic> {
    if negative && !rule.allow_negative {
        return Err(Diagnostic::error(
            DiagnosticCode::ResolveNegativeNotAllowed,
            format!("negative not allowed for `{}`", rule.namespace),
        ));
    }

    let mut typed = match value {
        UtilityValue::Bare => {
            if let Some(css) = &rule.bare {
                TypedValue::Raw(css.clone())
            } else {
                return Err(Diagnostic::error(
                    DiagnosticCode::ResolveInvalidType,
                    "length utility requires a value",
                ));
            }
        }
        UtilityValue::Named(n) => {
            if let Some((_, css)) = rule.keywords.iter().find(|(k, _)| k == &n.text) {
                TypedValue::Raw(css.clone())
            } else if let Some(suffix) = &rule.numeric_suffix {
                if n.text.parse::<f64>().is_ok() {
                    TypedValue::Raw(format!("{}{suffix}", n.text))
                } else {
                    named_length_token(ctx, &rule.namespace, &n.text)?
                }
            } else {
                named_length_token(ctx, &rule.namespace, &n.text)?
            }
        }
        UtilityValue::Arbitrary(a) => LengthResolver::from_arbitrary(a)?,
    };

    if negative {
        typed = LengthResolver::negate(typed)?;
    }

    let mut css = typed.into_css();
    if let Some(template) = &rule.wrap {
        let inner = match &css {
            CssValue::Length(l) => l.css.clone(),
            CssValue::Keyword(k)
            | CssValue::Number(k)
            | CssValue::Ratio(k)
            | CssValue::Shadow(k)
            | CssValue::Image(k)
            | CssValue::Raw(k) => k.clone(),
            CssValue::Color(c) => c.css.clone(),
        };
        css = CssValue::Raw(template.replace("{v}", &inner));
    }
    Ok(DeclarationSet {
        declarations: rule
            .properties
            .iter()
            .map(|p| Declaration {
                property: p.clone(),
                value: css.clone(),
                important: false,
            })
            .collect(),
    })
}

fn named_length_token(
    ctx: &mut ResolveContext<'_>,
    namespace: &str,
    token: &str,
) -> Result<TypedValue, Diagnostic> {
    if let Some(pct) = fraction_to_percent(token) {
        return Ok(TypedValue::Length(LengthValue { css: pct }));
    }
    match token {
        "full" => {
            return Ok(TypedValue::Length(LengthValue {
                css: "100%".into(),
            }))
        }
        "screen" => {
            return Ok(TypedValue::Length(LengthValue {
                css: "100vh".into(),
            }))
        }
        "min" => return Ok(TypedValue::Keyword("min-content".into())),
        "max" => return Ok(TypedValue::Keyword("max-content".into())),
        "fit" => return Ok(TypedValue::Keyword("fit-content".into())),
        "auto" => return Ok(TypedValue::Keyword("auto".into())),
        "none" => return Ok(TypedValue::Keyword("none".into())),
        _ => {}
    }
    lookup_length(ctx, namespace, token)
}

fn lookup_length(
    ctx: &mut ResolveContext<'_>,
    namespace: &str,
    token: &str,
) -> Result<TypedValue, Diagnostic> {
    for ns in [namespace, "spacing"] {
        let key = ThemeKey::from_path([ns, token]);
        match ctx.lookup(key.clone()) {
            ThemeLookup::Resolved(ThemeValue::Length(v)) => return Ok(TypedValue::Length(v)),
            ThemeLookup::Resolved(ThemeValue::Keyword(k)) => return Ok(TypedValue::Keyword(k)),
            ThemeLookup::Resolved(ThemeValue::Raw(r)) => return Ok(TypedValue::Raw(r)),
            ThemeLookup::Resolved(ThemeValue::Number(n)) => {
                return Ok(TypedValue::Number(n));
            }
            ThemeLookup::Resolved(_) => {
                return Err(Diagnostic::error(
                    DiagnosticCode::ThemeInvalidType,
                    format!("theme key `{}` is not a length", key.joined()),
                ));
            }
            ThemeLookup::Missing => continue,
            ThemeLookup::Ambiguous => {
                return Err(Diagnostic::error(
                    DiagnosticCode::ThemeAmbiguous,
                    format!("ambiguous theme key `{}`", key.joined()),
                ));
            }
            ThemeLookup::InvalidType => {
                return Err(Diagnostic::error(
                    DiagnosticCode::ThemeInvalidType,
                    format!("invalid type for `{}`", key.joined()),
                ));
            }
        }
    }
    Err(Diagnostic::error(
        DiagnosticCode::ThemeMissing,
        format!("missing theme key `{namespace}.{token}`"),
    ))
}

pub fn resolve_theme_color(
    rule: &ThemeColorRule,
    value: &UtilityValue,
    opacity: Option<&str>,
    negative: bool,
    ctx: &mut ResolveContext<'_>,
) -> Result<DeclarationSet, Diagnostic> {
    if negative {
        return Err(Diagnostic::error(
            DiagnosticCode::ResolveNegativeNotAllowed,
            format!("negative not allowed on `{}`", rule.property),
        ));
    }
    let mut typed = ColorResolver::from_utility_value(ctx, value)?;
    if let Some(op) = opacity {
        typed = ColorResolver::with_opacity(typed, op)?;
    }
    Ok(decl(&rule.property, typed.into_css()))
}

pub fn resolve_text_smart(
    value: &UtilityValue,
    opacity: Option<&str>,
    negative: bool,
    ctx: &mut ResolveContext<'_>,
) -> Result<DeclarationSet, Diagnostic> {
    if negative {
        return Err(Diagnostic::error(
            DiagnosticCode::ResolveNegativeNotAllowed,
            "negative not allowed on text utilities",
        ));
    }
    match value {
        UtilityValue::Bare => Err(Diagnostic::error(
            DiagnosticCode::ResolveInvalidType,
            "text utility requires a value",
        )),
        UtilityValue::Named(n) => {
            if matches!(
                n.text.as_str(),
                "left" | "center" | "right" | "justify" | "start" | "end"
            ) {
                return Ok(decl("text-align", CssValue::Keyword(n.text.clone())));
            }
            let size_key = ThemeKey::from_path(["fontSize", n.text.as_str()]);
            match ctx.lookup(size_key) {
                ThemeLookup::Resolved(ThemeValue::Length(l)) => {
                    return Ok(decl("font-size", CssValue::Length(l)));
                }
                ThemeLookup::Resolved(ThemeValue::Keyword(k)) => {
                    return Ok(decl("font-size", CssValue::Keyword(k)));
                }
                _ => {}
            }
            let mut typed = ColorResolver::from_theme(ctx, &n.text)?;
            if let Some(op) = opacity {
                typed = ColorResolver::with_opacity(typed, op)?;
            }
            Ok(decl("color", typed.into_css()))
        }
        UtilityValue::Arbitrary(a) => {
            if a.type_hint.as_ref().map(|h| h.text.as_str()) == Some("length") {
                return Ok(decl(
                    "font-size",
                    CssValue::Length(LengthValue {
                        css: a.raw.clone(),
                    }),
                ));
            }
            let mut typed = ColorResolver::from_arbitrary(a)?;
            if let Some(op) = opacity {
                typed = ColorResolver::with_opacity(typed, op)?;
            }
            Ok(decl("color", typed.into_css()))
        }
    }
}

pub fn resolve_background_smart(
    value: &UtilityValue,
    opacity: Option<&str>,
    negative: bool,
    ctx: &mut ResolveContext<'_>,
    keywords: &BTreeMap<String, (&'static str, &'static str)>,
) -> Result<DeclarationSet, Diagnostic> {
    if negative {
        return Err(Diagnostic::error(
            DiagnosticCode::ResolveNegativeNotAllowed,
            "negative not allowed on background",
        ));
    }
    if let UtilityValue::Named(n) = value {
        if let Some((prop, css)) = keywords.get(&n.text) {
            return Ok(decl(prop, CssValue::Keyword((*css).into())));
        }
    }
    let mut typed = ColorResolver::from_utility_value(ctx, value)?;
    if let Some(op) = opacity {
        typed = ColorResolver::with_opacity(typed, op)?;
    }
    Ok(decl("background-color", typed.into_css()))
}

pub fn resolve_min_max_size(
    kind: MinMaxKind,
    value: &UtilityValue,
    negative: bool,
    ctx: &mut ResolveContext<'_>,
) -> Result<DeclarationSet, Diagnostic> {
    let (prop, ns, token_value) = match value {
        UtilityValue::Named(n) => {
            let text = n.text.as_str();
            if let Some(token) = text.strip_prefix("w-") {
                let prop = match kind {
                    MinMaxKind::Min => "min-width",
                    MinMaxKind::Max => "max-width",
                };
                (
                    prop,
                    "width",
                    UtilityValue::Named(tailwind_ast::NamedValue {
                        text: token.into(),
                        span: n.span,
                    }),
                )
            } else if let Some(token) = text.strip_prefix("h-") {
                let prop = match kind {
                    MinMaxKind::Min => "min-height",
                    MinMaxKind::Max => "max-height",
                };
                (
                    prop,
                    "height",
                    UtilityValue::Named(tailwind_ast::NamedValue {
                        text: token.into(),
                        span: n.span,
                    }),
                )
            } else {
                return Err(Diagnostic::error(
                    DiagnosticCode::ResolveInvalidType,
                    format!("expected w-* or h-* value, got `{text}`"),
                ));
            }
        }
        other => {
            return Err(Diagnostic::error(
                DiagnosticCode::ResolveInvalidType,
                format!("invalid min/max size value {other:?}"),
            ));
        }
    };
    let rule = ThemeLengthRule::props(&[prop], ns);
    resolve_theme_length(&rule, &token_value, negative, ctx)
}

pub fn resolve_gap(
    value: &UtilityValue,
    negative: bool,
    ctx: &mut ResolveContext<'_>,
) -> Result<DeclarationSet, Diagnostic> {
    match value {
        UtilityValue::Named(n) if n.text.starts_with("x-") => {
            let token = &n.text[2..];
            let inner = UtilityValue::Named(tailwind_ast::NamedValue {
                text: token.into(),
                span: n.span,
            });
            let rule = ThemeLengthRule::spacing(&["column-gap"]);
            resolve_theme_length(&rule, &inner, negative, ctx)
        }
        UtilityValue::Named(n) if n.text.starts_with("y-") => {
            let token = &n.text[2..];
            let inner = UtilityValue::Named(tailwind_ast::NamedValue {
                text: token.into(),
                span: n.span,
            });
            let rule = ThemeLengthRule::spacing(&["row-gap"]);
            resolve_theme_length(&rule, &inner, negative, ctx)
        }
        _ => {
            let rule = ThemeLengthRule::spacing(&["gap"]);
            resolve_theme_length(&rule, value, negative, ctx)
        }
    }
}

pub fn resolve_border_smart(
    value: &UtilityValue,
    opacity: Option<&str>,
    negative: bool,
    ctx: &mut ResolveContext<'_>,
) -> Result<DeclarationSet, Diagnostic> {
    if negative {
        return Err(Diagnostic::error(
            DiagnosticCode::ResolveNegativeNotAllowed,
            "negative not allowed on border",
        ));
    }
    match value {
        UtilityValue::Bare => Ok(decl(
            "border-width",
            CssValue::Length(LengthValue {
                css: "1px".into(),
            }),
        )),
        UtilityValue::Named(n) => match n.text.as_str() {
            "0" => Ok(decl(
                "border-width",
                CssValue::Length(LengthValue { css: "0px".into() }),
            )),
            "2" | "4" | "8" => Ok(decl(
                "border-width",
                CssValue::Length(LengthValue {
                    css: format!("{}px", n.text),
                }),
            )),
            "solid" | "dashed" | "dotted" | "double" | "none" | "hidden" => {
                Ok(decl("border-style", CssValue::Keyword(n.text.clone())))
            }
            _ => {
                let mut typed = ColorResolver::from_theme(ctx, &n.text)?;
                if let Some(op) = opacity {
                    typed = ColorResolver::with_opacity(typed, op)?;
                }
                Ok(decl("border-color", typed.into_css()))
            }
        },
        UtilityValue::Arbitrary(a) => Ok(decl("border-width", CssValue::Raw(a.raw.clone()))),
    }
}

pub fn resolve_rounded(
    value: &UtilityValue,
    negative: bool,
    ctx: &mut ResolveContext<'_>,
) -> Result<DeclarationSet, Diagnostic> {
    if negative {
        return Err(Diagnostic::error(
            DiagnosticCode::ResolveNegativeNotAllowed,
            "negative not allowed on rounded",
        ));
    }
    match value {
        UtilityValue::Bare => {
            let key = ThemeKey::from_path(["borderRadius", "DEFAULT"]);
            match ctx.lookup(key) {
                ThemeLookup::Resolved(ThemeValue::Length(l)) => {
                    Ok(decl("border-radius", CssValue::Length(l)))
                }
                _ => Ok(decl(
                    "border-radius",
                    CssValue::Length(LengthValue {
                        css: "0.25rem".into(),
                    }),
                )),
            }
        }
        UtilityValue::Named(n) => {
            let key = ThemeKey::from_path(["borderRadius", n.text.as_str()]);
            match ctx.lookup(key.clone()) {
                ThemeLookup::Resolved(ThemeValue::Length(l)) => {
                    Ok(decl("border-radius", CssValue::Length(l)))
                }
                ThemeLookup::Resolved(ThemeValue::Keyword(k)) => {
                    Ok(decl("border-radius", CssValue::Keyword(k)))
                }
                ThemeLookup::Missing if n.text == "none" => Ok(decl(
                    "border-radius",
                    CssValue::Length(LengthValue { css: "0".into() }),
                )),
                ThemeLookup::Missing if n.text == "full" => Ok(decl(
                    "border-radius",
                    CssValue::Length(LengthValue {
                        css: "9999px".into(),
                    }),
                )),
                ThemeLookup::Missing => Err(Diagnostic::error(
                    DiagnosticCode::ThemeMissing,
                    format!("missing theme key `{}`", key.joined()),
                )),
                other => Err(Diagnostic::error(
                    DiagnosticCode::ThemeInvalidType,
                    format!("invalid borderRadius: {other:?}"),
                )),
            }
        }
        UtilityValue::Arbitrary(a) => Ok(decl("border-radius", CssValue::Raw(a.raw.clone()))),
    }
}

pub fn resolve_shadow(value: &UtilityValue, negative: bool) -> Result<DeclarationSet, Diagnostic> {
    if negative {
        return Err(Diagnostic::error(
            DiagnosticCode::ResolveNegativeNotAllowed,
            "negative not allowed on shadow",
        ));
    }
    let css = match value {
        UtilityValue::Bare => {
            "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)"
        }
        UtilityValue::Named(n) => match n.text.as_str() {
            "sm" => "0 1px 2px 0 rgb(0 0 0 / 0.05)",
            "md" => "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)",
            "lg" => "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)",
            "xl" => "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)",
            "2xl" => "0 25px 50px -12px rgb(0 0 0 / 0.25)",
            "inner" => "inset 0 2px 4px 0 rgb(0 0 0 / 0.05)",
            "none" => "none",
            _ => {
                return Err(Diagnostic::error(
                    DiagnosticCode::ResolveInvalidType,
                    format!("unknown shadow `{}`", n.text),
                ));
            }
        },
        UtilityValue::Arbitrary(a) => {
            return Ok(decl("box-shadow", CssValue::Raw(a.raw.clone())));
        }
    };
    Ok(decl("box-shadow", CssValue::Raw(css.into())))
}

pub fn resolve_flex_functional(
    value: &UtilityValue,
    negative: bool,
) -> Result<DeclarationSet, Diagnostic> {
    if negative {
        return Err(Diagnostic::error(
            DiagnosticCode::ResolveNegativeNotAllowed,
            "negative not allowed on flex",
        ));
    }
    match value {
        UtilityValue::Named(n) => match n.text.as_str() {
            "row" => Ok(decl("flex-direction", CssValue::Keyword("row".into()))),
            "row-reverse" => Ok(decl(
                "flex-direction",
                CssValue::Keyword("row-reverse".into()),
            )),
            "col" => Ok(decl("flex-direction", CssValue::Keyword("column".into()))),
            "col-reverse" => Ok(decl(
                "flex-direction",
                CssValue::Keyword("column-reverse".into()),
            )),
            "wrap" => Ok(decl("flex-wrap", CssValue::Keyword("wrap".into()))),
            "wrap-reverse" => Ok(decl("flex-wrap", CssValue::Keyword("wrap-reverse".into()))),
            "nowrap" => Ok(decl("flex-wrap", CssValue::Keyword("nowrap".into()))),
            "1" => Ok(decl("flex", CssValue::Raw("1 1 0%".into()))),
            "auto" => Ok(decl("flex", CssValue::Raw("1 1 auto".into()))),
            "initial" => Ok(decl("flex", CssValue::Raw("0 1 auto".into()))),
            "none" => Ok(decl("flex", CssValue::Keyword("none".into()))),
            other => Err(Diagnostic::error(
                DiagnosticCode::ResolveInvalidType,
                format!("unknown flex value `{other}`"),
            )),
        },
        UtilityValue::Arbitrary(a) => Ok(decl("flex", CssValue::Raw(a.raw.clone()))),
        UtilityValue::Bare => Err(Diagnostic::error(
            DiagnosticCode::ResolveInvalidType,
            "use bare `flex` for display:flex",
        )),
    }
}

pub fn resolve_z_index(
    value: &UtilityValue,
    negative: bool,
    ctx: &mut ResolveContext<'_>,
) -> Result<DeclarationSet, Diagnostic> {
    match value {
        UtilityValue::Named(n) => {
            if n.text == "auto" {
                return Ok(decl("z-index", CssValue::Keyword("auto".into())));
            }
            let key = ThemeKey::from_path(["zIndex", n.text.as_str()]);
            match ctx.lookup(key.clone()) {
                ThemeLookup::Resolved(ThemeValue::Number(num)) => {
                    let css = if negative {
                        format!("-{num}")
                    } else {
                        num
                    };
                    Ok(decl("z-index", CssValue::Number(css)))
                }
                ThemeLookup::Missing => {
                    let n: i32 = n.text.parse().map_err(|_| {
                        Diagnostic::error(
                            DiagnosticCode::ThemeMissing,
                            format!("missing theme key `{}`", key.joined()),
                        )
                    })?;
                    let v = if negative { -n } else { n };
                    Ok(decl("z-index", CssValue::Number(v.to_string())))
                }
                other => Err(Diagnostic::error(
                    DiagnosticCode::ThemeInvalidType,
                    format!("invalid zIndex: {other:?}"),
                )),
            }
        }
        UtilityValue::Arbitrary(a) => Ok(decl("z-index", CssValue::Raw(a.raw.clone()))),
        UtilityValue::Bare => Err(Diagnostic::error(
            DiagnosticCode::ResolveInvalidType,
            "z requires a value",
        )),
    }
}

pub fn resolve_opacity(
    value: &UtilityValue,
    negative: bool,
    _ctx: &mut ResolveContext<'_>,
) -> Result<DeclarationSet, Diagnostic> {
    if negative {
        return Err(Diagnostic::error(
            DiagnosticCode::ResolveNegativeNotAllowed,
            "negative not allowed on opacity",
        ));
    }
    match value {
        UtilityValue::Named(n) => {
            let pct: u32 = n.text.parse().map_err(|_| {
                Diagnostic::error(
                    DiagnosticCode::ResolveInvalidType,
                    format!("invalid opacity `{}`", n.text),
                )
            })?;
            Ok(decl(
                "opacity",
                CssValue::Number(format!("{}", pct as f64 / 100.0)),
            ))
        }
        UtilityValue::Arbitrary(a) => Ok(decl("opacity", CssValue::Raw(a.raw.clone()))),
        UtilityValue::Bare => Err(Diagnostic::error(
            DiagnosticCode::ResolveInvalidType,
            "opacity requires a value",
        )),
    }
}

pub fn resolve_filter_fn(
    property: &str,
    fn_name: &str,
    value: &UtilityValue,
    negative: bool,
    keywords: &[(&str, &str)],
    bare: &str,
) -> Result<DeclarationSet, Diagnostic> {
    if negative && fn_name != "hue-rotate" {
        return Err(Diagnostic::error(
            DiagnosticCode::ResolveNegativeNotAllowed,
            format!("negative not allowed on `{fn_name}`"),
        ));
    }
    let arg = match value {
        UtilityValue::Bare => bare.to_string(),
        UtilityValue::Named(n) => keywords
            .iter()
            .find(|(k, _)| *k == n.text)
            .map(|(_, v)| (*v).to_string())
            .unwrap_or_else(|| n.text.clone()),
        UtilityValue::Arbitrary(a) => a.raw.clone(),
    };
    let css = if negative && fn_name == "hue-rotate" {
        format!("{fn_name}(-{arg})")
    } else {
        format!("{fn_name}({arg})")
    };
    Ok(decl(property, CssValue::Raw(css)))
}

pub fn resolve_rotate(value: &UtilityValue, negative: bool) -> Result<DeclarationSet, Diagnostic> {
    let deg = match value {
        UtilityValue::Named(n) => format!("{}deg", n.text),
        UtilityValue::Arbitrary(a) => a.raw.clone(),
        UtilityValue::Bare => {
            return Err(Diagnostic::error(
                DiagnosticCode::ResolveInvalidType,
                "rotate requires a value",
            ));
        }
    };
    let css = if negative {
        format!("rotate(-{deg})")
    } else {
        format!("rotate({deg})")
    };
    Ok(decl("transform", CssValue::Raw(css)))
}

pub fn resolve_time_ms(
    property: &str,
    value: &UtilityValue,
    negative: bool,
) -> Result<DeclarationSet, Diagnostic> {
    if negative {
        return Err(Diagnostic::error(
            DiagnosticCode::ResolveNegativeNotAllowed,
            format!("negative not allowed on `{property}`"),
        ));
    }
    match value {
        UtilityValue::Named(n) => {
            let ms: u32 = n.text.parse().map_err(|_| {
                Diagnostic::error(
                    DiagnosticCode::ResolveInvalidType,
                    format!("invalid time `{}`", n.text),
                )
            })?;
            Ok(decl(property, CssValue::Raw(format!("{ms}ms"))))
        }
        UtilityValue::Arbitrary(a) => Ok(decl(property, CssValue::Raw(a.raw.clone()))),
        UtilityValue::Bare => Err(Diagnostic::error(
            DiagnosticCode::ResolveInvalidType,
            format!("`{property}` requires a value"),
        )),
    }
}

pub fn resolve_scale(value: &UtilityValue, negative: bool) -> Result<DeclarationSet, Diagnostic> {
    if negative {
        return Err(Diagnostic::error(
            DiagnosticCode::ResolveNegativeNotAllowed,
            "negative not allowed on scale",
        ));
    }
    match value {
        UtilityValue::Named(n) => {
            let pct: f64 = n.text.parse().map_err(|_| {
                Diagnostic::error(
                    DiagnosticCode::ResolveInvalidType,
                    format!("invalid scale `{}`", n.text),
                )
            })?;
            Ok(decl(
                "transform",
                CssValue::Raw(format!("scale({})", pct / 100.0)),
            ))
        }
        UtilityValue::Arbitrary(a) => Ok(decl(
            "transform",
            CssValue::Raw(format!("scale({})", a.raw)),
        )),
        UtilityValue::Bare => Err(Diagnostic::error(
            DiagnosticCode::ResolveInvalidType,
            "scale requires a value",
        )),
    }
}

fn fraction_to_percent(text: &str) -> Option<String> {
    let (a, b) = text.split_once('/')?;
    let num: f64 = a.parse().ok()?;
    let den: f64 = b.parse().ok()?;
    if den == 0.0 {
        return None;
    }
    Some(format!("{}%", (num / den) * 100.0))
}

fn decl(property: &str, value: CssValue) -> DeclarationSet {
    DeclarationSet {
        declarations: vec![Declaration {
            property: property.into(),
            value,
            important: false,
        }],
    }
}
