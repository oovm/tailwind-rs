use std::collections::BTreeSet;

use tailwind_types::{CanonicalRule, ThemeKey};

use crate::theme::{ThemeLookup, ThemeSnapshot};

/// Per-candidate resolve state. Tracks theme read-set for incremental invalidation (C5).
pub struct ResolveContext<'a> {
    theme: &'a ThemeSnapshot,
    reads: BTreeSet<ThemeKey>,
}

impl<'a> ResolveContext<'a> {
    pub fn new(theme: &'a ThemeSnapshot) -> Self {
        Self {
            theme,
            reads: BTreeSet::new(),
        }
    }

    pub fn lookup(&mut self, key: ThemeKey) -> ThemeLookup {
        self.reads.insert(key.clone());
        self.theme.get(&key)
    }

    /// Record a breakpoint read (`breakpoints.<name>`) and return its media query.
    pub fn lookup_breakpoint(&mut self, name: &str) -> Option<&str> {
        let key = ThemeKey::from_path(["breakpoints", name]);
        self.reads.insert(key);
        self.theme.breakpoint(name)
    }

    pub fn snapshot(&self) -> &ThemeSnapshot {
        self.theme
    }

    pub fn into_theme_reads(self) -> Vec<ThemeKey> {
        self.reads.into_iter().collect()
    }
}

/// One resolved rule plus the theme keys it actually read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedRule {
    pub rule: CanonicalRule,
    pub theme_reads: Vec<ThemeKey>,
}
