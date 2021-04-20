use crate::Result;
use std::fmt::{Arguments, Debug, Formatter, Write};

pub struct CssFormatter<'a> {
    buffer: &'a mut (dyn Write + 'a),
}

pub trait CssDisplay {
    fn display(&self, f: &mut CssFormatter) -> Result<()>;
}

impl<'a> Debug for CssFormatter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssFormatter {}")
    }
}

impl<'a> Write for CssFormatter<'a> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buffer.write_str(s)
    }

    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.buffer.write_char(c)
    }

    fn write_fmt(self: &mut Self, args: Arguments<'_>) -> std::fmt::Result {
        self.buffer.write_fmt(args)
    }
}
