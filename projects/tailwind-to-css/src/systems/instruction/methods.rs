use super::*;

impl TailwindVariant {
    pub fn new(names: &[&str], not: bool, pseudo: bool) -> Self {
        Self { not, pseudo, names: names.iter().map(|s| s.to_string()).collect() }
    }
}
