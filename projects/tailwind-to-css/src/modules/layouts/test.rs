use crate::{tw_idempotency, TailwindBuilder};

#[test]
fn build_aspect() {
    let builder = TailwindBuilder::default();
    let out = format!("{:?}", builder.try_inline("aspect-square"));
    assert_eq!(out, "{aspect-ratio: 1/1;}")
}

#[test]
fn id_aspect() {
    let mut builder = TailwindBuilder::default();
    tw_idempotency("aspect-auto", &mut builder);
    tw_idempotency("aspect-square", &mut builder);
    tw_idempotency("aspect-video", &mut builder);
    tw_idempotency("aspect-1/1", &mut builder);
}
