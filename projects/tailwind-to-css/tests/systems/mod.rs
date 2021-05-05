
use super::*;

#[test]
fn test_table_system() {
    build_target(
        "table_system",
        "border-collapse border-separate table-auto table-fixed",
        include_str!("table-inline.css"),
        include_str!("table-bundle.css"),
    )
}