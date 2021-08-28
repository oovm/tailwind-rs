use crate::{tw_idempotency, TailwindBuilder};

#[test]
fn build_rounded() {
    let mut builder = TailwindBuilder::default();
    let out = builder.inline("rounded-full");
    assert_eq!(out, "background-attachment:scroll;");
}

#[test]
fn id_rounded() {
    let mut builder = TailwindBuilder::default();
    tw_idempotency("bg-local", &mut builder);
    tw_idempotency("bg-fixed", &mut builder);
    tw_idempotency("bg-local bg-fixed", &mut builder);
}
