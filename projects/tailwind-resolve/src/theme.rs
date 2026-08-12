use std::collections::BTreeMap;

use tailwind_types::{Diagnostic, DiagnosticCode, ThemeEntry, ThemeInput, ThemeKey, ThemeValue};

use crate::canonical::hash_theme_maps;

/// Immutable, normalized theme used during resolve.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ThemeSnapshot {
    values: BTreeMap<ThemeKey, ThemeValue>,
    aliases: BTreeMap<ThemeKey, ThemeKey>,
    breakpoints: BTreeMap<String, String>,
    pub content_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ThemeLookup {
    Resolved(ThemeValue),
    Missing,
    Ambiguous,
    InvalidType,
}

/// Stable ordering / canonicalize version used by module-layer cache keys (C5).
pub const ORDERING_VERSION: u64 = 1;

impl ThemeSnapshot {
    pub fn empty() -> Self {
        Self::default()
    }

    /// Build from builtins + caller overrides. Alias cycles / duplicates → diagnostics.
    pub fn normalize(
        builtin: ThemeInput,
        input: ThemeInput,
        breakpoints: BTreeMap<String, String>,
    ) -> Result<Self, Vec<Diagnostic>> {
        let mut values = BTreeMap::new();
        let mut aliases = BTreeMap::new();
        let mut diagnostics = Vec::new();

        for entry in builtin.entries.into_iter().chain(input.entries) {
            insert_entry(&mut values, &mut aliases, entry, &mut diagnostics);
        }

        if let Some(cycle) = find_alias_cycle(&aliases) {
            diagnostics.push(Diagnostic::error(
                DiagnosticCode::ThemeAliasCycle,
                format!("theme alias cycle involving `{}`", cycle.joined()),
            ));
        }

        if !diagnostics.is_empty() {
            return Err(diagnostics);
        }

        let content_hash = simple_hash(&values, &aliases, &breakpoints);
        Ok(Self {
            values,
            aliases,
            breakpoints,
            content_hash,
        })
    }

    pub fn get(&self, key: &ThemeKey) -> ThemeLookup {
        let mut current = key.clone();
        let mut seen = Vec::new();
        loop {
            if seen.contains(&current) {
                return ThemeLookup::Ambiguous;
            }
            seen.push(current.clone());
            if let Some(target) = self.aliases.get(&current) {
                current = target.clone();
                continue;
            }
            return match self.values.get(&current) {
                Some(v) => ThemeLookup::Resolved(v.clone()),
                None => ThemeLookup::Missing,
            };
        }
    }

    pub fn breakpoint(&self, name: &str) -> Option<&str> {
        self.breakpoints.get(name).map(String::as_str)
    }

    pub fn values(&self) -> &BTreeMap<ThemeKey, ThemeValue> {
        &self.values
    }

    /// Hash only the theme slice actually read by a candidate (C5 resolve_cache).
    pub fn slice_hash(&self, keys: &[ThemeKey]) -> String {
        use crate::stable_hash::StableHasher;
        let mut sorted: Vec<_> = keys.to_vec();
        sorted.sort();
        sorted.dedup();
        let mut h = StableHasher::new();
        h.write_str("tw-theme-slice-v1");
        for key in sorted {
            h.write_str(&key.joined());
            if key.path.first().map(String::as_str) == Some("breakpoints") {
                match key.path.get(1).and_then(|n| self.breakpoint(n)) {
                    Some(q) => {
                        h.write_u8(1);
                        h.write_str(q);
                    }
                    None => h.write_u8(0),
                }
                continue;
            }
            match self.get(&key) {
                ThemeLookup::Resolved(v) => {
                    h.write_u8(1);
                    h.write_str(&format!("{v:?}"));
                }
                ThemeLookup::Missing => h.write_u8(0),
                ThemeLookup::Ambiguous => h.write_u8(2),
                ThemeLookup::InvalidType => h.write_u8(3),
            }
        }
        h.finish_hex()
    }

    /// Theme keys whose resolved value differs from `previous` (C5 invalidation).
    pub fn changed_keys_from(&self, previous: &Self) -> Vec<ThemeKey> {
        let mut out = std::collections::BTreeSet::new();
        for k in self.values.keys().chain(previous.values.keys()) {
            if self.get(k) != previous.get(k) {
                out.insert(k.clone());
            }
        }
        for k in self.aliases.keys().chain(previous.aliases.keys()) {
            if self.aliases.get(k) != previous.aliases.get(k) {
                out.insert(k.clone());
            }
        }
        for name in self.breakpoints.keys().chain(previous.breakpoints.keys()) {
            if self.breakpoint(name) != previous.breakpoint(name) {
                out.insert(ThemeKey::from_path(["breakpoints", name.as_str()]));
            }
        }
        out.into_iter().collect()
    }
}

fn insert_entry(
    values: &mut BTreeMap<ThemeKey, ThemeValue>,
    aliases: &mut BTreeMap<ThemeKey, ThemeKey>,
    entry: ThemeEntry,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if let ThemeValue::Raw(ref s) = entry.value {
        if let Some(path) = s.strip_prefix("alias:") {
            let target = ThemeKey::from_path(path.split('.'));
            if values.contains_key(&entry.key) {
                diagnostics.push(Diagnostic::error(
                    DiagnosticCode::ThemeAmbiguous,
                    format!(
                        "theme key `{}` cannot be both value and alias",
                        entry.key.joined()
                    ),
                ));
                return;
            }
            aliases.insert(entry.key, target);
            return;
        }
    }

    if aliases.contains_key(&entry.key) {
        diagnostics.push(Diagnostic::error(
            DiagnosticCode::ThemeAmbiguous,
            format!(
                "theme key `{}` cannot be both value and alias",
                entry.key.joined()
            ),
        ));
        return;
    }

    // Last write wins for values (caller overrides builtins).
    values.insert(entry.key, entry.value);
}

fn find_alias_cycle(aliases: &BTreeMap<ThemeKey, ThemeKey>) -> Option<ThemeKey> {
    for start in aliases.keys() {
        let mut current = start.clone();
        let mut seen = Vec::new();
        while let Some(next) = aliases.get(&current) {
            if seen.contains(&current) {
                return Some(start.clone());
            }
            seen.push(current.clone());
            current = next.clone();
        }
    }
    None
}

fn simple_hash(
    values: &BTreeMap<ThemeKey, ThemeValue>,
    aliases: &BTreeMap<ThemeKey, ThemeKey>,
    breakpoints: &BTreeMap<String, String>,
) -> String {
    hash_theme_maps(values, aliases, breakpoints)
}
