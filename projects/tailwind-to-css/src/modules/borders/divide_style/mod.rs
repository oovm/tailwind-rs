use super::*;
///
#[derive(Copy, Clone, Debug)]
pub enum TailwindDivideStyle {
    None,
    Solid,
    Dashed,
    Dotted,
    Double,
    // Hidden,
}

impl Display for TailwindDivideStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindDivideStyle {}
