use crate::{tw_idempotency, TailwindBuilder};

#[test]
fn build_screen_reader() {
    let builder = TailwindBuilder::default();
    let out = format!("{:?}", builder.inline("sr-only"));
    assert_eq!(out, "{aspect-ratio: 1 / 1;}")
}

#[test]
fn id_screen_reader() {
    let mut builder = TailwindBuilder::default();
    tw_idempotency("sr-only", &mut builder);
    tw_idempotency("not-sr-only", &mut builder);
}
