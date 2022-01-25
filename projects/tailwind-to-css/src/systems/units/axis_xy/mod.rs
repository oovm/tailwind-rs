use std::fmt::{Display, Formatter};

mod traits;

#[derive(Copy, Clone, Debug)]
pub enum AxisXY {
    X,
    Y,
    N,
}

impl AxisXY {
    pub fn split_xy<'a, 'b>(pattern: &'a [&'b str]) -> (Self, &'a [&'b str]) {
        match pattern {
            ["x", rest @ ..] => (AxisXY::X, rest),
            ["y", rest @ ..] => (AxisXY::Y, rest),
            _ => unreachable!(),
        }
    }
    pub(crate) fn write_xy<A, B>(&self, f: &mut Formatter<'_>, before: B, after: A) -> std::fmt::Result
    where
        A: Display,
        B: Display,
    {
        match self {
            AxisXY::X => write!(f, "{}-x-{}", before, after),
            AxisXY::Y => write!(f, "{}-y-{}", before, after),
            AxisXY::N => unreachable!(),
        }
    }
    pub fn split_xyn<'a, 'b>(pattern: &'a [&'b str]) -> (Self, &'a [&'b str]) {
        match pattern {
            ["x", rest @ ..] => (AxisXY::X, rest),
            ["y", rest @ ..] => (AxisXY::Y, rest),
            _ => (AxisXY::N, pattern),
        }
    }
    pub(crate) fn write_xyn<A, B>(&self, f: &mut Formatter<'_>, before: B, after: A) -> std::fmt::Result
    where
        A: Display,
        B: Display,
    {
        match self {
            AxisXY::X => write!(f, "{}-x-{}", before, after),
            AxisXY::Y => write!(f, "{}-y-{}", before, after),
            AxisXY::N => write!(f, "{}-{}", before, after),
        }
    }
    pub fn format_xyn<B>(&self, before: B) -> String
    where
        B: Display,
    {
        match self {
            AxisXY::X => format!("{}-x", before),
            AxisXY::Y => format!("{}-y", before),
            AxisXY::N => format!("{}", before),
        }
    }
}
