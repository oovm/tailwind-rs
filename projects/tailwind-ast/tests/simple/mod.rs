use tailwind_error::TextStorage;

use crate::test_expand;

#[test]
fn test_simple() {
    let mut store = TextStorage::default();
    test_expand(&mut store, "simple/easy").unwrap();
    test_expand(&mut store, "simple/nest").unwrap();
    test_expand(&mut store, "simple/arbitrary").unwrap();
}
