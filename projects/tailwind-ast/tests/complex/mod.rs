use tailwind_error::TextStorage;

use crate::test_expand;

#[test]
fn test_simple() {
    let mut store = TextStorage::default();
    test_expand(&mut store, "complex/test1").unwrap();
}
