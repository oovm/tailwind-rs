// use super::*;
// use std::str::FromStr;
//
// impl<'a> FromStr for AstStyle<'a> {
//     type Err = Err<Error<&'a str>>;
//
//     #[inline]
//     fn from_str(s: &str) -> Result<AstStyle<'a>, Self::Err> {
//         Ok(Self::parse(s)?.1)
//     }
// }
// impl<'a> FromStr for AstGroup<'a> {
//     type Err = Err<Error<&'a str>>;
//
//     #[inline]
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(Self::parse(s)?.1)
//     }
// }
