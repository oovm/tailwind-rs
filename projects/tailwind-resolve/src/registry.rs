use std::collections::BTreeMap;

use tailwind_ast::{CandidateSyntax, ModifierValue, UtilitySyntax, UtilityValue};
use tailwind_types::{DeclarationSet, Diagnostic, DiagnosticCode};

use crate::context::ResolveContext;
use crate::rules::{
    color, display,
    generic::{
        self, KeywordMapRule, MinMaxKind, StaticRule, ThemeColorRule, ThemeLengthRule,
    },
    spacing,
};

/// How a utility name is matched in the registry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleKind {
    Exact,
    Functional,
    ArbitraryProperty,
}

#[derive(Clone, Debug)]
pub enum RuleSpec {
    /// Exact utilities like `block`, `flex`, `hidden`, `inline-block`.
    Display,
    /// Spacing family parameterized by axis + box side kind.
    Spacing(spacing::SpacingRule),
    /// `bg-*` (keywords + color)
    BackgroundColor,
    /// Legacy text-color-only (prefer `Text`).
    TextColor,
    /// Smart `text-*`: color then font-size.
    Text,
    /// `[prop:value]`
    ArbitraryProperty,
    /// Exact multi-declaration utilities.
    Static(StaticRule),
    /// Functional name + keyword value map.
    KeywordMap(KeywordMapRule),
    /// Theme length / scale lookup.
    ThemeLength(ThemeLengthRule),
    /// Theme color + optional opacity modifier.
    ThemeColor(ThemeColorRule),
    /// `min` / `max` with `w-*` / `h-*` value prefix.
    MinMaxSize(MinMaxKind),
    /// `gap` / `gap-x-*` / `gap-y-*`.
    Gap,
    /// `border` width / style / color.
    Border,
    /// `rounded` / side radii.
    Rounded,
    /// `shadow` sizes.
    Shadow,
    /// `flex-*` direction / wrap / shorthand (not bare display).
    FlexFunctional,
    /// `z-*`.
    ZIndex,
    /// `opacity-*`.
    Opacity,
    /// Filter function (`blur`, `brightness`, …).
    FilterFn {
        property: &'static str,
        fn_name: &'static str,
        keywords: &'static [(&'static str, &'static str)],
        bare: &'static str,
    },
    /// `rotate-*`.
    Rotate,
    /// `duration-*` / `delay-*`.
    TimeMs(&'static str),
    /// `scale-*`.
    Scale,
}

/// Data-driven rule registry — no giant match over all utilities.
#[derive(Clone, Debug, Default)]
pub struct RuleRegistry {
    exact: BTreeMap<String, RuleSpec>,
    functional: BTreeMap<String, RuleSpec>,
    arbitrary_property: bool,
}

impl RuleRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn exact(&mut self, name: impl Into<String>, spec: RuleSpec) -> &mut Self {
        self.exact.insert(name.into(), spec);
        self
    }

    pub fn functional(&mut self, name: impl Into<String>, spec: RuleSpec) -> &mut Self {
        self.functional.insert(name.into(), spec);
        self
    }

    pub fn arbitrary_property(&mut self, enabled: bool) -> &mut Self {
        self.arbitrary_property = enabled;
        self
    }

    /// Stable registry fingerprint for resolve_cache keys (C5).
    pub fn content_version(&self) -> u64 {
        use crate::stable_hash::StableHasher;
        let mut h = StableHasher::new();
        h.write_str("tw-rules-v1");
        for name in self.exact.keys() {
            h.write_str(name);
        }
        for name in self.functional.keys() {
            h.write_str(name);
        }
        h.write_bool(self.arbitrary_property);
        h.finish()
    }

    pub fn resolve(
        &self,
        syntax: &CandidateSyntax,
        ctx: &mut ResolveContext<'_>,
    ) -> Result<(DeclarationSet, u32), Diagnostic> {
        match &syntax.utility {
            UtilitySyntax::ArbitraryProperty {
                property,
                value,
                modifier,
            } => {
                if !self.arbitrary_property {
                    return Err(Diagnostic::error(
                        DiagnosticCode::ResolveUnknownUtility,
                        "arbitrary properties are not enabled",
                    ));
                }
                if modifier.is_some() {
                    return Err(Diagnostic::error(
                        DiagnosticCode::ResolveModifierNotAllowed,
                        "modifiers are not allowed on arbitrary properties",
                    ));
                }
                if syntax.negative {
                    return Err(Diagnostic::error(
                        DiagnosticCode::ResolveNegativeNotAllowed,
                        "negative not allowed on arbitrary properties",
                    ));
                }
                let set = color::arbitrary_property(&property.text, value)?;
                Ok((set, 900))
            }
            UtilitySyntax::Standard {
                name,
                value,
                modifier,
            } => {
                // Compound exact: `inline-block` parses as name=inline + Named(block).
                if let UtilityValue::Named(n) = value {
                    let compound = format!("{}-{}", name.text, n.text);
                    if let Some(spec) = self.exact.get(&compound) {
                        if modifier.is_some() {
                            return Err(Diagnostic::error(
                                DiagnosticCode::ResolveModifierNotAllowed,
                                format!("utility `{compound}` does not allow modifiers"),
                            ));
                        }
                        return self.dispatch_exact(spec, &compound, syntax.negative);
                    }
                }

                if let Some(spec) = self.exact.get(&name.text) {
                    if !matches!(value, UtilityValue::Bare) {
                        // Fall through to functional (e.g. bare `flex` vs `flex-row`).
                    } else {
                        if modifier.is_some() {
                            return Err(Diagnostic::error(
                                DiagnosticCode::ResolveModifierNotAllowed,
                                format!("utility `{}` does not allow modifiers", name.text),
                            ));
                        }
                        return self.dispatch_exact(spec, &name.text, syntax.negative);
                    }
                }

                if let Some(spec) = self.functional.get(&name.text) {
                    return self.dispatch_functional(
                        spec,
                        &name.text,
                        value,
                        modifier.as_ref(),
                        syntax.negative,
                        ctx,
                    );
                }

                Err(Diagnostic::error(
                    DiagnosticCode::ResolveUnknownUtility,
                    format!("unknown utility `{}`", name.text),
                ))
            }
        }
    }

    fn dispatch_exact(
        &self,
        spec: &RuleSpec,
        name: &str,
        negative: bool,
    ) -> Result<(DeclarationSet, u32), Diagnostic> {
        if negative && !matches!(spec, RuleSpec::Static(_)) {
            // Static may still reject; others reject here.
            if !matches!(spec, RuleSpec::Display) {
                // Display never negative
            }
        }
        match spec {
            RuleSpec::Display => {
                if negative {
                    return Err(Diagnostic::error(
                        DiagnosticCode::ResolveNegativeNotAllowed,
                        format!("negative not allowed on `{name}`"),
                    ));
                }
                Ok((display::resolve_exact(name)?, 100))
            }
            RuleSpec::Static(rule) => Ok((generic::resolve_static(rule, negative)?, 110)),
            RuleSpec::KeywordMap(rule) => Ok((
                generic::resolve_keyword_map(rule, &UtilityValue::Bare, negative)?,
                120,
            )),
            _ => Err(Diagnostic::error(
                DiagnosticCode::ResolveAmbiguous,
                format!("exact rule mismatch for `{name}`"),
            )),
        }
    }

    fn dispatch_functional(
        &self,
        spec: &RuleSpec,
        name: &str,
        value: &UtilityValue,
        modifier: Option<&tailwind_ast::ModifierSyntax>,
        negative: bool,
        ctx: &mut ResolveContext<'_>,
    ) -> Result<(DeclarationSet, u32), Diagnostic> {
        match spec {
            RuleSpec::Spacing(rule) => {
                if modifier.is_some() {
                    return Err(Diagnostic::error(
                        DiagnosticCode::ResolveModifierNotAllowed,
                        format!("utility `{name}` does not allow modifiers"),
                    ));
                }
                Ok((spacing::resolve(rule, value, negative, ctx)?, 200))
            }
            RuleSpec::BackgroundColor => {
                let opacity = modifier_opacity(modifier)?;
                let keywords = bg_keyword_map();
                Ok((
                    generic::resolve_background_smart(value, opacity, negative, ctx, &keywords)?,
                    300,
                ))
            }
            RuleSpec::TextColor => {
                let opacity = modifier_opacity(modifier)?;
                Ok((
                    color::resolve_text(value, opacity, negative, ctx)?,
                    310,
                ))
            }
            RuleSpec::Text => {
                let opacity = modifier_opacity(modifier)?;
                Ok((
                    generic::resolve_text_smart(value, opacity, negative, ctx)?,
                    310,
                ))
            }
            RuleSpec::ThemeColor(rule) => {
                let opacity = modifier_opacity(modifier)?;
                Ok((
                    generic::resolve_theme_color(rule, value, opacity, negative, ctx)?,
                    320,
                ))
            }
            RuleSpec::ThemeLength(rule) => {
                if modifier.is_some() {
                    return Err(Diagnostic::error(
                        DiagnosticCode::ResolveModifierNotAllowed,
                        format!("utility `{name}` does not allow modifiers"),
                    ));
                }
                Ok((
                    generic::resolve_theme_length(rule, value, negative, ctx)?,
                    210,
                ))
            }
            RuleSpec::KeywordMap(rule) => {
                if modifier.is_some() {
                    return Err(Diagnostic::error(
                        DiagnosticCode::ResolveModifierNotAllowed,
                        format!("utility `{name}` does not allow modifiers"),
                    ));
                }
                Ok((generic::resolve_keyword_map(rule, value, negative)?, 150))
            }
            RuleSpec::Static(rule) => Ok((generic::resolve_static(rule, negative)?, 110)),
            RuleSpec::MinMaxSize(kind) => {
                if modifier.is_some() {
                    return Err(Diagnostic::error(
                        DiagnosticCode::ResolveModifierNotAllowed,
                        format!("utility `{name}` does not allow modifiers"),
                    ));
                }
                Ok((
                    generic::resolve_min_max_size(*kind, value, negative, ctx)?,
                    220,
                ))
            }
            RuleSpec::Gap => {
                deny_modifier(name, modifier)?;
                Ok((generic::resolve_gap(value, negative, ctx)?, 230))
            }
            RuleSpec::Border => {
                let opacity = modifier_opacity(modifier)?;
                Ok((
                    generic::resolve_border_smart(value, opacity, negative, ctx)?,
                    330,
                ))
            }
            RuleSpec::Rounded => {
                deny_modifier(name, modifier)?;
                Ok((generic::resolve_rounded(value, negative, ctx)?, 340))
            }
            RuleSpec::Shadow => {
                deny_modifier(name, modifier)?;
                Ok((generic::resolve_shadow(value, negative)?, 350))
            }
            RuleSpec::FlexFunctional => {
                deny_modifier(name, modifier)?;
                Ok((generic::resolve_flex_functional(value, negative)?, 160))
            }
            RuleSpec::ZIndex => {
                deny_modifier(name, modifier)?;
                Ok((generic::resolve_z_index(value, negative, ctx)?, 170))
            }
            RuleSpec::Opacity => {
                deny_modifier(name, modifier)?;
                Ok((generic::resolve_opacity(value, negative, ctx)?, 360))
            }
            RuleSpec::FilterFn {
                property,
                fn_name,
                keywords,
                bare,
            } => {
                deny_modifier(name, modifier)?;
                Ok((
                    generic::resolve_filter_fn(property, fn_name, value, negative, keywords, bare)?,
                    400,
                ))
            }
            RuleSpec::Rotate => {
                deny_modifier(name, modifier)?;
                Ok((generic::resolve_rotate(value, negative)?, 410))
            }
            RuleSpec::TimeMs(prop) => {
                deny_modifier(name, modifier)?;
                Ok((generic::resolve_time_ms(prop, value, negative)?, 420))
            }
            RuleSpec::Scale => {
                deny_modifier(name, modifier)?;
                Ok((generic::resolve_scale(value, negative)?, 430))
            }
            RuleSpec::Display | RuleSpec::ArbitraryProperty => Err(Diagnostic::error(
                DiagnosticCode::ResolveAmbiguous,
                format!("functional rule mismatch for `{name}`"),
            )),
        }
    }
}

fn deny_modifier(
    name: &str,
    modifier: Option<&tailwind_ast::ModifierSyntax>,
) -> Result<(), Diagnostic> {
    if modifier.is_some() {
        Err(Diagnostic::error(
            DiagnosticCode::ResolveModifierNotAllowed,
            format!("utility `{name}` does not allow modifiers"),
        ))
    } else {
        Ok(())
    }
}

fn modifier_opacity(
    modifier: Option<&tailwind_ast::ModifierSyntax>,
) -> Result<Option<&str>, Diagnostic> {
    match modifier {
        None => Ok(None),
        Some(m) => match &m.value {
            ModifierValue::Named(id) => Ok(Some(id.text.as_str())),
            ModifierValue::Arbitrary(_) => Err(Diagnostic::error(
                DiagnosticCode::ResolveModifierNotAllowed,
                "arbitrary opacity modifiers not supported",
            )),
        },
    }
}

fn bg_keyword_map() -> BTreeMap<String, (&'static str, &'static str)> {
    let mut m = BTreeMap::new();
    for (k, prop, css) in [
        ("fixed", "background-attachment", "fixed"),
        ("local", "background-attachment", "local"),
        ("scroll", "background-attachment", "scroll"),
        ("auto", "background-size", "auto"),
        ("cover", "background-size", "cover"),
        ("contain", "background-size", "contain"),
        ("no-repeat", "background-repeat", "no-repeat"),
        ("repeat", "background-repeat", "repeat"),
        ("repeat-x", "background-repeat", "repeat-x"),
        ("repeat-y", "background-repeat", "repeat-y"),
        ("clip-border", "background-clip", "border-box"),
        ("clip-padding", "background-clip", "padding-box"),
        ("clip-content", "background-clip", "content-box"),
        ("clip-text", "background-clip", "text"),
        ("origin-border", "background-origin", "border-box"),
        ("origin-padding", "background-origin", "padding-box"),
        ("origin-content", "background-origin", "content-box"),
    ] {
        m.insert(k.to_string(), (prop, css));
    }
    m
}
