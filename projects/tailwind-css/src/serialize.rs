//! Canonical Style Module → CSS text (reference lowering).

use tailwind_types::{
    AtRuleCondition, CanonicalRule, CanonicalStyleModule, ConditionTree, CssValue, Declaration,
    SelectorCondition,
};

use crate::escape::escape_class_name;

/// Knobs for the reference serializer. Semantics of the module are never changed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SerializeOptions {
    /// When true, emit one rule per line (default). When false, concatenate compactly.
    pub pretty: bool,
}

impl Default for SerializeOptions {
    fn default() -> Self {
        Self { pretty: true }
    }
}

/// Serialize an entire canonical module to a CSS stylesheet string.
pub fn serialize_module(module: &CanonicalStyleModule) -> String {
    serialize_module_with(module, &SerializeOptions::default())
}

/// Serialize with explicit options.
pub fn serialize_module_with(module: &CanonicalStyleModule, options: &SerializeOptions) -> String {
    let sep = if options.pretty { "\n" } else { "" };
    module
        .rules
        .iter()
        .map(serialize_rule)
        .collect::<Vec<_>>()
        .join(sep)
}

/// Serialize one canonical rule to a CSS rule (possibly wrapped in at-rules).
pub fn serialize_rule(rule: &CanonicalRule) -> String {
    let class = escape_class_name(&rule.candidate_key);
    let base_selector = format!(".{class}");
    let body = format_declarations(&rule.declarations);
    emit_under_conditions(&rule.conditions, &base_selector, &body)
}

fn format_declarations(decls: &[Declaration]) -> String {
    decls
        .iter()
        .map(|d| {
            let value = format_css_value(&d.value);
            if d.important {
                format!("{}:{}!important", d.property, value)
            } else {
                format!("{}:{}", d.property, value)
            }
        })
        .collect::<Vec<_>>()
        .join(";")
}

fn format_css_value(value: &CssValue) -> String {
    match value {
        CssValue::Keyword(s)
        | CssValue::Number(s)
        | CssValue::Ratio(s)
        | CssValue::Shadow(s)
        | CssValue::Image(s)
        | CssValue::Raw(s) => s.clone(),
        CssValue::Length(l) => l.css.clone(),
        CssValue::Color(c) => c.css.clone(),
    }
}

fn emit_under_conditions(conditions: &ConditionTree, selector: &str, body: &str) -> String {
    match conditions {
        ConditionTree::None => format!("{selector}{{{body}}}"),
        ConditionTree::Selector(sel) => {
            let next = apply_selector_transform(selector, sel);
            format!("{next}{{{body}}}")
        }
        ConditionTree::AtRule(at) => {
            let inner = format!("{selector}{{{body}}}");
            wrap_at_rule(at, &inner)
        }
        ConditionTree::Compound(parts) => {
            let mut selector = selector.to_string();
            let mut at_rules: Vec<&AtRuleCondition> = Vec::new();
            flatten_conditions(parts, &mut selector, &mut at_rules);
            let mut css = format!("{selector}{{{body}}}");
            // First at-rule in source order is outermost.
            for at in at_rules.iter().rev() {
                css = wrap_at_rule(at, &css);
            }
            css
        }
    }
}

fn flatten_conditions<'a>(
    parts: &'a [ConditionTree],
    selector: &mut String,
    at_rules: &mut Vec<&'a AtRuleCondition>,
) {
    for part in parts {
        match part {
            ConditionTree::None => {}
            ConditionTree::Selector(sel) => {
                *selector = apply_selector_transform(selector, sel);
            }
            ConditionTree::AtRule(at) => at_rules.push(at),
            ConditionTree::Compound(inner) => flatten_conditions(inner, selector, at_rules),
        }
    }
}

fn apply_selector_transform(selector: &str, sel: &SelectorCondition) -> String {
    let transform = sel.transform.as_str();
    if transform.contains('&') {
        transform.replace('&', selector)
    } else {
        format!("{selector}{transform}")
    }
}

fn wrap_at_rule(at: &AtRuleCondition, inner: &str) -> String {
    if at.query.is_empty() {
        format!("@{}{{{inner}}}", at.name)
    } else {
        format!("@{} {}{{{inner}}}", at.name, at.query)
    }
}

#[cfg(test)]
mod unit {
    use super::*;
    use tailwind_types::{ColorValue, LengthValue, OrderKey, Provenance, SourceRef};

    fn rule(key: &str, conditions: ConditionTree, decls: Vec<Declaration>) -> CanonicalRule {
        CanonicalRule {
            candidate_key: key.into(),
            conditions,
            declarations: decls,
            order: OrderKey {
                layer: 3,
                variant_rank: 0,
                utility_rank: 0,
                tie: key.into(),
            },
            provenance: vec![Provenance {
                source: SourceRef::new("t"),
                candidate_index: Some(0),
            }],
        }
    }

    #[test]
    fn base_utility() {
        let r = rule(
            "p-4",
            ConditionTree::None,
            vec![Declaration {
                property: "padding".into(),
                value: CssValue::Length(LengthValue {
                    css: "1rem".into(),
                }),
                important: false,
            }],
        );
        assert_eq!(serialize_rule(&r), ".p-4{padding:1rem}");
    }

    #[test]
    fn hover_and_important() {
        let r = rule(
            "hover:bg-red-500",
            ConditionTree::Selector(SelectorCondition {
                transform: ":hover".into(),
            }),
            vec![Declaration {
                property: "background-color".into(),
                value: CssValue::Color(ColorValue {
                    css: "#ef4444".into(),
                }),
                important: false,
            }],
        );
        assert_eq!(
            serialize_rule(&r),
            r".hover\:bg-red-500:hover{background-color:#ef4444}"
        );

        let r = rule(
            "!p-4",
            ConditionTree::None,
            vec![Declaration {
                property: "padding".into(),
                value: CssValue::Length(LengthValue {
                    css: "1rem".into(),
                }),
                important: true,
            }],
        );
        assert_eq!(serialize_rule(&r), r".\!p-4{padding:1rem!important}");
    }

    #[test]
    fn media_breakpoint() {
        let r = rule(
            "md:p-4",
            ConditionTree::AtRule(AtRuleCondition {
                name: "media".into(),
                query: "(min-width: 768px)".into(),
            }),
            vec![Declaration {
                property: "padding".into(),
                value: CssValue::Length(LengthValue {
                    css: "1rem".into(),
                }),
                important: false,
            }],
        );
        assert_eq!(
            serialize_rule(&r),
            r"@media (min-width: 768px){.md\:p-4{padding:1rem}}"
        );
    }

    #[test]
    fn dark_ampersand_transform() {
        let r = rule(
            "dark:p-4",
            ConditionTree::Selector(SelectorCondition {
                transform: ".dark &".into(),
            }),
            vec![Declaration {
                property: "padding".into(),
                value: CssValue::Length(LengthValue {
                    css: "1rem".into(),
                }),
                important: false,
            }],
        );
        assert_eq!(serialize_rule(&r), r".dark .dark\:p-4{padding:1rem}");
    }

    #[test]
    fn compound_md_hover() {
        let r = rule(
            "md:hover:p-4",
            ConditionTree::Compound(vec![
                ConditionTree::AtRule(AtRuleCondition {
                    name: "media".into(),
                    query: "(min-width: 768px)".into(),
                }),
                ConditionTree::Selector(SelectorCondition {
                    transform: ":hover".into(),
                }),
            ]),
            vec![Declaration {
                property: "padding".into(),
                value: CssValue::Length(LengthValue {
                    css: "1rem".into(),
                }),
                important: false,
            }],
        );
        assert_eq!(
            serialize_rule(&r),
            r"@media (min-width: 768px){.md\:hover\:p-4:hover{padding:1rem}}"
        );
    }
}
