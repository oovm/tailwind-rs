use super::*;

impl From<bool> for AxisXY {
    fn from(s: bool) -> Self {
        match s {
            true => Self::X,
            false => Self::Y,
        }
    }
}

impl From<Option<bool>> for AxisXY {
    fn from(s: Option<bool>) -> Self {
        match s {
            Some(true) => Self::X,
            Some(false) => Self::Y,
            None => Self::N,
        }
    }
}
