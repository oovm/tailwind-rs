use tailwind_ast::{ArbitrarySyntax, UtilityValue};
use tailwind_types::{CssValue, Declaration, DeclarationSet, Diagnostic, DiagnosticCode};

use crate::context::ResolveContext;
use crate::typed::ColorResolver;

pub fn resolve_text(
    value: &UtilityValue,
    opacity: Option<&str>,
    negative: bool,
    ctx: &mut ResolveContext<'_>,
) -> Result<DeclarationSet, Diagnostic> {
    if negative {
        return Err(Diagnostic::error(
            DiagnosticCode::ResolveNegativeNotAllowed,
            "negative not allowed on text color",
        ));
    }
    let mut typed = ColorResolver::from_utility_value(ctx, value)?;
    if let Some(op) = opacity {
        typed = ColorResolver::with_opacity(typed, op)?;
    }
    Ok(DeclarationSet {
        declarations: vec![Declaration {
            property: "color".into(),
            value: typed.into_css(),
            important: false,
        }],
    })
}

pub fn arbitrary_property(
    property: &str,
    value: &ArbitrarySyntax,
) -> Result<DeclarationSet, Diagnostic> {
    Ok(DeclarationSet {
        declarations: vec![Declaration {
            property: property.to_string(),
            value: CssValue::Raw(value.raw.clone()),
            important: false,
        }],
    })
}
