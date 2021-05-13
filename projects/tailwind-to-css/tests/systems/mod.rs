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

#[test]
fn test_space_system() {
    build_target(
        "space_system",
        "p-auto pt-px px-px space-x-reverse",
        include_str!("space-inline.css"),
        include_str!("space-bundle.css"),
    )
}
