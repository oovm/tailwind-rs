#[doc = include_str!("word-break.md")]
#[derive(Debug, Clone)]
pub enum TailwindBreakWord {
    Normal,
    Words,
    All,
}

impl Display for TailwindBreakWord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindBreakWord {}
