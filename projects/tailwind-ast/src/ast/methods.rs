use super::*;

impl AstStyle {}

#[inline]
fn merge_negative(lhs: bool, rhs: bool) -> bool {
    match (lhs, rhs) {
        (true, true) => true,
        (true, false) | (false, true) => true,
        (false, false) => false,
    }
}
