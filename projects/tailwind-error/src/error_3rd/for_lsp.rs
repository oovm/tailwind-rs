use crate::TailwindError;
use lsp_types::{Diagnostic, DiagnosticTag, Range};
use yggdrasil_shared::{LspTextAdaptor, TextIndex};

impl TailwindError {
    /// Get the range as [`Range`]
    #[inline]
    pub fn get_lsp_range(&self, text: &TextIndex) -> Option<Range> {
        self.range.as_ref().and_then(|r| text.offset_range_to_lsp_range(&r))
    }
    /// Get the tags as [`DiagnosticTag`]
    #[inline]
    pub fn get_lsp_tags(&self) -> Option<Vec<DiagnosticTag>> {
        let mut tags = vec![];
        if self.is_unnecessary() {
            tags.push(DiagnosticTag::UNNECESSARY)
        }
        if self.is_deprecated() {
            tags.push(DiagnosticTag::DEPRECATED)
        }
        return Some(tags);
    }
    /// Convert error to lsp [`Diagnostic`]
    #[inline]
    pub fn as_lsp_diagnostic(&self, text: &TextIndex) -> Diagnostic {
        Diagnostic {
            range: self.get_lsp_range(text).unwrap_or_default(),
            severity: self.level.into_severity(),
            code: None,
            code_description: None,
            source: None,
            message: "".to_string(),
            related_information: None,
            tags: self.get_lsp_tags(),
            data: None,
        }
    }
}
