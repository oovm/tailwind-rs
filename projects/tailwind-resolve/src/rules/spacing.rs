use tailwind_ast::UtilityValue;
use tailwind_types::{Declaration, DeclarationSet, Diagnostic, DiagnosticCode};

use crate::context::ResolveContext;
use crate::typed::LengthResolver;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpacingKind {
    Padding,
    Margin,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpacingAxis {
    All,
    X,
    Y,
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpacingRule {
    pub kind: SpacingKind,
    pub axis: SpacingAxis,
}

impl SpacingRule {
    pub const fn new(kind: SpacingKind, axis: SpacingAxis) -> Self {
        Self { kind, axis }
    }
}

pub fn resolve(
    rule: &SpacingRule,
    value: &UtilityValue,
    negative: bool,
    ctx: &mut ResolveContext<'_>,
) -> Result<DeclarationSet, Diagnostic> {
    if negative && rule.kind == SpacingKind::Padding {
        return Err(Diagnostic::error(
            DiagnosticCode::ResolveNegativeNotAllowed,
            "negative padding is not allowed",
        ));
    }

    let mut typed = LengthResolver::from_utility_value(ctx, value)?;
    if negative {
        typed = LengthResolver::negate(typed)?;
    }
    let css = typed.into_css();
    let props = properties(rule);
    Ok(DeclarationSet {
        declarations: props
            .into_iter()
            .map(|property| Declaration {
                property: property.to_string(),
                value: css.clone(),
                important: false,
            })
            .collect(),
    })
}

fn properties(rule: &SpacingRule) -> Vec<&'static str> {
    let prefix = match rule.kind {
        SpacingKind::Padding => "padding",
        SpacingKind::Margin => "margin",
    };
    match rule.axis {
        SpacingAxis::All => vec![prefix],
        SpacingAxis::X => vec![
            match rule.kind {
                SpacingKind::Padding => "padding-left",
                SpacingKind::Margin => "margin-left",
            },
            match rule.kind {
                SpacingKind::Padding => "padding-right",
                SpacingKind::Margin => "margin-right",
            },
        ],
        SpacingAxis::Y => vec![
            match rule.kind {
                SpacingKind::Padding => "padding-top",
                SpacingKind::Margin => "margin-top",
            },
            match rule.kind {
                SpacingKind::Padding => "padding-bottom",
                SpacingKind::Margin => "margin-bottom",
            },
        ],
        SpacingAxis::Top => vec![match rule.kind {
            SpacingKind::Padding => "padding-top",
            SpacingKind::Margin => "margin-top",
        }],
        SpacingAxis::Right => vec![match rule.kind {
            SpacingKind::Padding => "padding-right",
            SpacingKind::Margin => "margin-right",
        }],
        SpacingAxis::Bottom => vec![match rule.kind {
            SpacingKind::Padding => "padding-bottom",
            SpacingKind::Margin => "margin-bottom",
        }],
        SpacingAxis::Left => vec![match rule.kind {
            SpacingKind::Padding => "padding-left",
            SpacingKind::Margin => "margin-left",
        }],
    }
}
