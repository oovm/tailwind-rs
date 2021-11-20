use super::*;

#[derive(Debug, Clone)]
pub enum NeverArbitrary {}

impl NeverArbitrary {
    pub fn parser(
        id: &'static str,
        check_valid: &'static impl Fn(&str) -> bool,
    ) -> impl Fn(&[&str], &TailwindArbitrary) -> Result<String> {
        move |pattern: &[&str], arbitrary: &TailwindArbitrary| Self::parse_standard(pattern, arbitrary, id, check_valid)
    }
    pub fn parse_standard(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
        id: &str,
        checker: &'static impl Fn(&str) -> bool,
    ) -> Result<String> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after {}", id);
        let kind = pattern.join("-");
        debug_assert!(checker(&kind));
        Ok(kind)
    }
}
