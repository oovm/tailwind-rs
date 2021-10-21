use super::*;
#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindGridFlow {
    Row,
    RowDense,
    Column,
    ColumnDense,
}

impl Display for TailwindGridFlow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindGridFlow {}

impl TailwindGridFlow {
    pub fn parse(_pattern: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}
