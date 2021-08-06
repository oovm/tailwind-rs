use crate::{tw_idempotency, TailwindBuilder};

#[test]
fn build_bg_attach() {
    let builder = TailwindBuilder::default();
    let out = format!("{:?}", builder.try_inline("sr-only"));
    assert_eq!(out, "{aspect-ratio: 1 / 1;}")
}

#[test]
fn id_bg_attach() {
    let mut builder = TailwindBuilder::default();
    tw_idempotency("sr-only", &mut builder);
    tw_idempotency("not-sr-only", &mut builder);
}
