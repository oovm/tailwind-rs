use super::*;

impl TailwindListStyle {
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Custom(arbitrary.to_string()))
    }
}

impl TailwindIndent {
    pub fn parse(_input: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindWhiteSpace {
    pub fn parse(_input: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

impl TailwindContentElement {
    pub fn parse_arbitrary(_arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}
