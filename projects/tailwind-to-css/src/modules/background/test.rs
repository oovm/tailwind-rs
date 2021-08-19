use crate::{tw_idempotency, TailwindBuilder};

#[test]
fn build_bg_attach() {
    let builder = TailwindBuilder::default();
    let out = builder.inline("bg-scroll");
    assert_eq!(out, "background-attachment:scroll;");
}

#[test]
fn id_bg_attach() {
    let mut builder = TailwindBuilder::default();
    tw_idempotency("bg-local", &mut builder);
    tw_idempotency("bg-fixed", &mut builder);
    tw_idempotency("bg-local bg-fixed", &mut builder);
}

#[test]
fn build_bg_clip() {
    let builder = TailwindBuilder::default();
    let out = builder.inline("bg-clip-content");
    assert_eq!(out, "background-clip:content-box;");
}

#[test]
fn id_bg_clip() {
    let mut builder = TailwindBuilder::default();
    tw_idempotency("bg-clip-content", &mut builder);
    tw_idempotency("bg-clip-text", &mut builder);
    tw_idempotency("bg-clip-content bg-clip-text", &mut builder);
}
