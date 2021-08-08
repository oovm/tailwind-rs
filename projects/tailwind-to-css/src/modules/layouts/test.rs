use crate::{tw_idempotency, TailwindBuilder};

#[test]
fn id_aspect() {
    let mut builder = TailwindBuilder::default();
    tw_idempotency("aspect-auto", &mut builder);
    tw_idempotency("aspect-square", &mut builder);
    tw_idempotency("aspect-video", &mut builder);
    tw_idempotency("aspect-1/1", &mut builder);
}

#[test]
fn build_columns() {
    let builder = TailwindBuilder::default();
    let out = builder.inline("columns-12");
    assert_eq!(out, "columns:12;");
    let out = builder.inline("columns-xs");
    assert_eq!(out, "columns:20rem;");
    let out = builder.inline("columns-auto");
    assert_eq!(out, "columns:auto;");
}
