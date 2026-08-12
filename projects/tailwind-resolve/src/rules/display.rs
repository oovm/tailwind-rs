use tailwind_types::{CssValue, Declaration, DeclarationSet, Diagnostic, DiagnosticCode};

/// Exact display utilities. `hidden` maps to `display: none` (TW semantics).
pub fn resolve_exact(name: &str) -> Result<DeclarationSet, Diagnostic> {
    let keyword = match name {
        "block" => "block",
        "inline-block" => "inline-block",
        "inline" => "inline",
        "flex" => "flex",
        "inline-flex" => "inline-flex",
        "grid" => "grid",
        "inline-grid" => "inline-grid",
        "contents" => "contents",
        "hidden" => "none",
        "table" => "table",
        "flow-root" => "flow-root",
        "list-item" => "list-item",
        other => {
            return Err(Diagnostic::error(
                DiagnosticCode::ResolveUnknownUtility,
                format!("unknown display utility `{other}`"),
            ));
        }
    };

    Ok(DeclarationSet {
        declarations: vec![Declaration {
            property: "display".into(),
            value: CssValue::Keyword(keyword.into()),
            important: false,
        }],
    })
}
